struct Path {
    curr: Vector2,
    next: Option<Path>
}

impl Path {
    fn new(start: Vector2) -> Path {
        Path {
            curr: Vector2,
            next: None
        }
    }

    fn append(self, node: Vector2) -> Path {
        Path {
            curr: node,
            next: self
        }
    }

    fn cost(&self) -> Scalar {
        match self.next {
            Some(node) => euclid_dist(self.curr, node.curr),
            None => 0
        }
    }
}

// could optimize by caching cost of path
// Returns shortest Path from start to end using A*
pub fn find_shortest_path(start: Vector2, end: Vector2) -> Path {
    let mut frontier = Vec::new();
    frontier.push(Path.new().append(start));

    while frontier.len() > 0 {
        // Sort the frontier by f-stat
        frontier.sort_by_key(|x| x.cost() + euclid_dist(x.curr, end))
    }

    Path.new(start)
}

fn euclid_dist(start: Vector2, end: Vector2) -> Scalar {
    ((start.x - end.x).powf(2.0) + (start.y - end.y).powf(2.0)).sqrt()
}
