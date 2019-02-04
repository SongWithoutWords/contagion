use std::cmp::*;

use crate::core::scalar::Scalar;
use crate::core::vector::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub pos: Vector2,
    pub h: Scalar
}

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub start: Node,
    pub end: Node,
    pub cost: Scalar
}

#[derive(Clone, Debug)]
pub struct Path {
    pub edges: Vec<Edge>,
    pub cost: Scalar
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Path {
    pub fn from_edge(e: Edge) -> Path {
        Path {
            edges: vec![e],
            cost: e.cost
        }
    }

    pub fn append_edge(&mut self, e: Edge) -> Self {
        self.cost += &e.cost;
        self.edges.push(e);
        *self
    }

    pub fn f_stat(&self) -> Scalar {
        match self.edges.last() {
            Some(edge) => edge.end.h + self.cost,
            None => 0.0
        }
    }
}
