#[cfg(test)]
mod tests {
    use crate::graph::entities::{Edge, Node};
    use crate::graph::state::{add_edges, get_edges};
    use crate::graph::tests::mock_querier::mock_dependencies_with_querier;
    use apollo_asset::asset::AssetInfo;
    use cosmwasm_std::Addr;

    #[test]
    fn test_read_write_graph() {
        let mut deps = mock_dependencies_with_querier(&[]);

        let node_a = Node {
            asset: AssetInfo::NativeToken {
                denom: "uusd".to_string(),
            },
        };
        let node_b = Node {
            asset: AssetInfo::NativeToken {
                denom: "uluna".to_string(),
            },
        };

        // nodes dont exist in graph yet, should error
        let res = get_edges(&deps.storage, &node_a);
        assert!(res.is_empty());
        let res = get_edges(&deps.storage, &node_b);
        assert!(res.is_empty());

        // add edges from a to b and b to a
        add_edges(&mut deps.storage, node_a.clone(), node_b.clone(), Some(1)).unwrap();

        let edges_a = get_edges(&deps.storage, &node_a);
        let edges_b = get_edges(&deps.storage, &node_b);
        assert_eq!(1, edges_a.len());
        assert_eq!(1, edges_b.len());
        // edges should be equal even tho to/from are in different order
        assert_eq!(edges_a[0], edges_b[0]);
        assert_eq!(edges_a[0].to, edges_b[0].from);
        assert_eq!(edges_a[0].from, edges_b[0].to);
        assert_ne!(edges_a[0].from, edges_b[0].from);
        assert_ne!(edges_a[0].to, edges_b[0].to);
    }

    #[test]
    fn test_dijkstra() {
        let mut deps = mock_dependencies_with_querier(&[]);

        let ust = Node {
            asset: AssetInfo::NativeToken {
                denom: "uusd".to_string(),
            },
        };
        let luna = Node {
            asset: AssetInfo::NativeToken {
                denom: "uluna".to_string(),
            },
        };
        let bluna = Node {
            asset: AssetInfo::Token {
                contract_addr: Addr::unchecked("bluna"),
            },
        };
        let nluna = Node {
            asset: AssetInfo::Token {
                contract_addr: Addr::unchecked("nluna"),
            },
        };
        let psi = Node {
            asset: AssetInfo::Token {
                contract_addr: Addr::unchecked("psi"),
            },
        };

        let astro = Some(Addr::unchecked("astro"));

        // ust > luna (native/non-native) > bluna
        // ust > psi > (luna | nluna) > bluna
        add_edges(&mut deps.storage, ust.clone(), luna.clone(), Some(1)).unwrap();
        add_edges(&mut deps.storage, ust.clone(), luna.clone(), None).unwrap();
        add_edges(&mut deps.storage, ust.clone(), psi.clone(), Some(1)).unwrap();
        add_edges(&mut deps.storage, luna.clone(), psi.clone(), Some(1)).unwrap();
        add_edges(&mut deps.storage, nluna.clone(), psi.clone(), Some(1)).unwrap();
        add_edges(&mut deps.storage, nluna.clone(), bluna.clone(), Some(1)).unwrap();
        add_edges(&mut deps.storage, luna.clone(), bluna.clone(), Some(1)).unwrap();

        // get min paths from ust to all others in graph
        let res = dijkstra(&deps.storage, ust.clone(), None, |e| {
            if let Some(dex_id) = e.pair_info {
                1
            } else {
                2
            }
        })
        .unwrap();

        let min_path_bluna = res.get(&bluna).unwrap();
        assert_eq!(
            min_path_bluna.edges,
            vec![
                Edge {
                    from: ust.clone(),
                    to: luna.clone(),
                    pair_info: None
                },
                Edge {
                    from: luna.clone(),
                    to: bluna.clone(),
                    pair_info: Some(1)
                }
            ]
        );
        assert_eq!(min_path_bluna.cost, 3);

        let min_path_nluna = res.get(&nluna).unwrap();
        assert_eq!(
            min_path_nluna.edges,
            vec![
                Edge {
                    from: ust.clone(),
                    to: psi.clone(),
                    pair_info: Some(1)
                },
                Edge {
                    from: psi.clone(),
                    to: nluna.clone(),
                    pair_info: Some(1)
                }
            ]
        );
        assert_eq!(min_path_nluna.cost, 4);

        let min_path_psi = res.get(&psi).unwrap();
        assert_eq!(
            min_path_psi.edges,
            vec![Edge {
                from: ust.clone(),
                to: psi.clone(),
                pair_info: Some(1)
            }]
        );
        assert_eq!(min_path_psi.cost, 2);

        let min_path_ust = res.get(&ust).unwrap();
        assert_eq!(min_path_ust.edges, vec![]);
        assert_eq!(min_path_ust.cost, 0);
    }
}
