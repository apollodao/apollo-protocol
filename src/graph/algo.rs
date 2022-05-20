use crate::graph::entities::{Edge, Node, Paths};
use crate::graph::state::get_edges;

use cosmwasm_std::{Decimal, StdError, StdResult, Storage};
use schemars::JsonSchema;
use schemars::Set;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{BTreeSet, BinaryHeap, HashMap};
use std::iter::FromIterator;

pub fn shortest_paths(storage: &dyn Storage, start: Node, target: Node) -> StdResult<Paths> {
    let mut visited_nodes: BTreeSet<Node> = Set::new();
    let mut scores = HashMap::new();
    let mut visit_next = BinaryHeap::new();
    let zero_score = Paths {
        paths: vec![],
        cost: 0,
    };
    scores.insert(start.clone(), zero_score.clone());
    visit_next.push(MinScored(zero_score, start.clone()));
    while let Some(MinScored(mut cur_node_paths, cur_node)) = visit_next.pop() {
        if visited_nodes.contains(&cur_node) || &target == &cur_node {
            continue;
        }
        if &target == &cur_node {
            break;
        }
        cur_node_paths = scores.get(&cur_node).unwrap_or(&cur_node_paths).clone();
        for edge in get_edges(storage, &cur_node) {
            let next_node = edge.to.clone();

            match scores.entry(next_node.clone()) {
                Occupied(ent) => {
                    let prev_path_len = ent.get().cost;
                    let mut next_path_len = cur_node_paths.cost + 1;
                    let mut next_node_paths = cur_node_paths.clone();
                    next_node_paths.add_edge_to_paths(&edge);

                    let new_paths = if prev_path_len == next_path_len || &next_node == &target {
                        let mut new_paths = ent.get().to_owned();
                        new_paths
                            .paths
                            .append(&mut (next_node_paths.paths).to_owned());
                        new_paths.cost = prev_path_len.max(next_path_len);
                        new_paths.clone()
                    } else if prev_path_len > next_path_len {
                        next_node_paths.clone()
                    } else {
                        continue;
                    };
                    *ent.into_mut() = new_paths.clone();
                    visit_next.push(MinScored(new_paths.clone(), next_node.clone()));
                }
                Vacant(ent) => {
                    let mut new_paths = cur_node_paths.clone();
                    new_paths.add_edge_to_paths(&edge);
                    new_paths.cost += 1;
                    ent.insert(new_paths.clone());
                    visit_next.push(MinScored(new_paths, next_node));
                }
            };
        }
        visited_nodes.insert(cur_node.clone());
    }

    let min_path_to_target_res = scores.get(&target);
    let min_path_to_target = min_path_to_target_res.ok_or_else(|| {
        StdError::generic_err(format!("no paths found from {:?} to {:?}", start, target))
    })?;

    Ok(min_path_to_target.clone())
}

#[derive(Copy, Clone, Debug)]
pub struct MinScored<K, T>(pub K, pub T);

impl<K: PartialOrd, T> PartialEq for MinScored<K, T> {
    #[inline]
    fn eq(&self, other: &MinScored<K, T>) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K: PartialOrd, T> Eq for MinScored<K, T> {}

impl<K: PartialOrd, T> PartialOrd for MinScored<K, T> {
    #[inline]
    fn partial_cmp(&self, other: &MinScored<K, T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: PartialOrd, T> Ord for MinScored<K, T> {
    #[inline]
    fn cmp(&self, other: &MinScored<K, T>) -> Ordering {
        let a = &self.0;
        let b = &other.0;
        if a == b {
            Ordering::Equal
        } else if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if a.ne(a) && b.ne(b) {
            // these are the NaN cases
            Ordering::Equal
        } else if a.ne(a) {
            // Order NaN less, so that it is last in the MaxScore order
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
