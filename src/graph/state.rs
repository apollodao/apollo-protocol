use crate::graph::entities::{Edge, Node, PairInfo};

use crate::router::PairsResponseItem;
use apollo_asset::asset::AssetInfo;
use cosmwasm_std::{to_binary, Addr, Order, StdError, StdResult, Storage};
use cw_storage_plus::{Bound, Key, Map, PrimaryKey};

pub const GRAPH: Map<&Node, Vec<Edge>> = Map::new("graph");

impl PrimaryKey<'_> for &Node {
    type Prefix = ();
    type SubPrefix = ();

    fn key(&self) -> std::vec::Vec<Key<'_>> {
        vec![cw_storage_plus::Key::Ref(self.asset.as_bytes())]
    }

    type Suffix = ();

    type SuperSuffix = ();
}

pub fn get_edges(storage: &dyn Storage, node: &Node) -> Vec<Edge> {
    GRAPH.load(storage, node).unwrap_or_default()
}

// // settings for pagination
// const MAX_LIMIT: u32 = 30;
// const DEFAULT_LIMIT: u32 = 10;
// pub fn get_all_edges(
//     storage: &dyn Storage,
//     limit: Option<u32>,
//     start_after: Option<AssetInfo>,
// ) -> StdResult<Vec<PairsResponseItem>> {
//     let t = if let Some(start_after) = start_after {
//         Some(to_binary(&Node::from(start_after))?)
//     } else {
//         None
//     };
//     let start = t.map(Bound::exclusive);
//     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

//     GRAPH
//         .range(storage, start, None, Order::Ascending)
//         .take(limit)
//         .map(|p| {
//             let edges = p?.1;
//             let node = &edges[0].from;
//             Ok(PairsResponseItem {
//                 asset: node.asset.clone(),
//                 pairs: edges,
//             })
//         })
//         .collect()
// }

pub fn add_edge(storage: &mut dyn Storage, edge: &Edge) -> StdResult<()> {
    let mut edges = get_edges(storage, &edge.from);
    match edges.contains(&edge) {
        false => {
            edges.push(edge.clone());
            GRAPH.save(storage, &edge.from, &edges)
        }
        true => Err(StdError::generic_err(format!(
            "edge {:?} already exists",
            edge
        ))),
    }
}

// add edges from a to b and b to a
pub fn add_edges(
    storage: &mut dyn Storage,
    a: Node,
    b: Node,
    pair_info: Option<PairInfo>,
) -> StdResult<()> {
    add_edge(
        storage,
        &Edge {
            from: a.clone(),
            to: b.clone(),
            pair_info: pair_info.clone(),
        },
    )?;
    add_edge(
        storage,
        &Edge {
            from: b,
            to: a,
            pair_info,
        },
    )
}
