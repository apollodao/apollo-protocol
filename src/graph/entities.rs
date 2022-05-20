use apollo_asset::asset::AssetInfo;
use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct Node {
    pub asset: AssetInfo,
}

impl Into<AssetInfo> for Node {
    fn into(self) -> AssetInfo {
        self.asset
    }
}

impl From<AssetInfo> for Node {
    fn from(asset: AssetInfo) -> Self {
        Self { asset }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct PairInfo {
    pub dex_id: u8,
    pub contract_addr: Addr,
}

impl Hash for PairInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contract_addr.hash(state);
    }
}

impl PartialEq for PairInfo {
    fn eq(&self, other: &Self) -> bool {
        self.contract_addr == other.contract_addr
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct Edge {
    pub from: Node,
    pub to: Node,
    pub pair_info: Option<PairInfo>,
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
        self.pair_info.hash(state);
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.asset.as_bytes())
    }
}

impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.asset.partial_cmp(&other.asset)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.asset
            .partial_cmp(&other.asset)
            .unwrap_or(Ordering::Greater)
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        let other_node = [&other.from, &other.to];
        self.pair_info == other.pair_info
            && other_node.contains(&&self.from)
            && other_node.contains(&&self.to)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct Path {
    pub edges: Vec<Edge>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct PathResponse {
    pub best_shortest_path: Vec<Edge>,
    pub best_split_path: Vec<(Decimal, Edge)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(deny_unknown_fields)]
pub struct Paths {
    pub paths: Vec<Path>,
    pub cost: usize,
}

impl Paths {
    pub(crate) fn add_edge_to_paths(&mut self, edge: &Edge) -> () {
        if self.paths.is_empty() {
            self.paths = vec![Path {
                edges: vec![edge.clone()],
            }]
        } else {
            self.paths.iter_mut().for_each(|mut p| {
                p.edges.push(edge.clone());
            });
        }
    }
}

impl PartialOrd for Paths {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}
