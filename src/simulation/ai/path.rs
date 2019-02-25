use std::cmp::*;

use crate::core::scalar::Scalar;
use crate::core::vector::Vector2;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub pos: Vector2,
    pub h: u64
}

#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub start: Node,
    pub end: Node,
    pub cost: u64
}

#[derive(Clone, Debug)]
pub struct Path {
    pub edges: Vec<Edge>,
    pub cost: u64
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.f_stat().partial_cmp(&other.f_stat())
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

    pub fn append_edge(&mut self, e: Edge) {
        self.cost += e.cost;
        self.edges.push(e);
    }

    pub fn f_stat(&self) -> Scalar {
        match self.edges.last() {
            Some(edge) => (edge.end.h + self.cost) as Scalar,
            None => 0.0
        }
    }

    pub fn to_vec(&self) -> Vec<Vector2> {
        let mut out = vec!();

        if self.edges.len() > 0 {
            out.push(self.edges[0].start.pos);

            for i in 0..self.edges.len() {
                out.push(self.edges[i].end.pos);
            }
        }

        out
    }
}
