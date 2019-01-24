struct Node {
    pos: Vector2,
    h: Scalar
}

struct Edge {
    start: Node,
    end: Node,
    cost: Scalar
}

struct Graph {
    start: Node,
    goal: Node,
    edges: Vec<Edge>,
    nodes: Vec<Node>
}

impl Graph {
    fn new(start_pos: Vector2, end_pos: Vector2) -> Self {
        let start_node = Node { pos: start_pos, h: euclidean_dist(start_pos, end_pos) };
        let end_node = Node { pos: end_pos, h: 0 };

        Graph {
            start: start_node,
            goal: end_node,
            edges: Vec::new(),
            nodes: Vec::new()
        }
    }

    // Add a new node to the graph, unless it already exists
    fn add_node(&mut self, n: Vector2) -> &Node {
        match self.get_node_by_pos(n) {
            Some(node) => node,
            None => {
                self.nodes.push(Node { pos: n, h: euclidean_dist(n, self.goal.pos) })
            }
        }
    }

    // Find a node from its coordinates
    fn get_node_by_pos(&self, query_pos: Vector2) -> Option<&Node> {
        self.nodes.iter().find(|&&node| node.pos.x == query_pos.x && node.pos.y == query_pos.y)
    }

    // Add a new edge to the graph, unless it already exists.
    // Creates new nodes if necessary.
    // Does not check if edge is disconnected from graph
    fn add_edge(&mut self, start_pos: Vector2, end_pos: Vector2) -> &Edge {
        let start_node = self.add_node(start_pos);
        let end_node = self.add_node(end_pos);

        match self.edges.iter().find(|&&edge| edge.start == start_node && edge.end == end_node) {
            Some(edge) => edge,
            None => {
                self.edges.push(Edge {
                    start: start_node,
                    end: end_node,
                    cost: euclidean_dist(start_pos, end_pos)
                })
            }
        }
    }
}

struct Path {
    edges: Vec<Edge>,
    cost: Scalar
}

impl Path {
    fn from_edge(e: Edge) -> Self {
        Path {
            edges: vec![e],
            cost: e.cost
        }
    }

    fn from_edges(e: Vec<Edge>) -> Self {
        Path {
            edges: e,
            cost: e.iter().map(|&e| e.cost).sum()
        }
    }

    fn append_edge(&self, e: &Edge) -> Self {
        Path {
            edges: self.edges.push(e),
            cost: self.cost + e.cost
        }
    }

    fn f_stat(&self) -> Scalar {
        match self.edges.last() {
            Some(edge) => edge.end.h + self.cost,
            None => 0
        }
    }
}

// Could probably refactor this to be a Vec<Edge>
struct Polygon(Vec<Vector2>);

impl Polygon {
    // Find the normals of all edges of the polygon
    fn normals(&self) -> Vec<Vector2> {
        let out = Vec::new();

        for i in 0..self.len() {
            let ab = self[(i + 1) % self.len()].sub(self[i]);
            let ac = self[(i - 1) % self.len()].sub(self[i]);
            let n = ac.mul(-1);
            let d = n.dot(ac);

            if d > 0 {
                out.push(-n);
            }
            else {
                out.push(n);
            }
        }

        out
    }

    // Find the position of all intersections of the line spanned by start and end
    fn intersects(&self, start: Vector2, end: Vector2) -> Vec<Vector2> {
        let mut out = Vec::new();

        let m1 = (start.y - end.y) / (start.x - end.x);
        let b1 = start.y - m1 * start.x;

        for i in 0..self.len()-1 {
            let r = self[i];
            let s = self[(i + 1) % self.len()];
            let m2 = (r.y - s.y) / (r.x - s.x);
            let b2 = r.y - m2 * r.x;

            if m1*b2 - m2*b1 != 0 {
                let x_intercept = (b2 - b1) / (m1 - m2);
                let y_intercept = m1 * x_intercept + b1;

                if x_intercept >= start.x.min(end.x) && x_intercept <= start.x.max(end.x) &&
                    y_intercept >= start.y.min(end.y) && y_intercept <= start.y.min(end.y) {
                    out.push(Vector2 { x: x_intercept, y: y_intercept })
                }
            }
        }

        out
    }
}

// Find the shortest path from start_pos to end_pos, accounting for obstacles
pub fn find_path(start_pos: Vector2, end_pos: Vector2, obstacles: Vec<Polygon>) -> Some<Path> {
    // Placeholder for how far out we need to space the actor we are finding the path for
    let actor_radius = 5.0;

    // Initialize the graph representing
    let mut graph = Graph.new(start_pos, end_pos);

    // Find all intervening obstacles between the start and the goal
    let mut intersections = Vec::<(Vector2, Polygon)>::new();
    for poly in obstacles.iter() {
        for intersect in poly.intersects(start_pos, end_pos) {
            intersections.push((intersect, poly));
        }
    }

    // Find the closest intersection point
    match intersections.iter().enumerate().min_by(|&(_, i)| euclidean_dist(start_pos, i.0)) {
        // Nothing between start and end, answer is a straight line
        None => return Path.from_edge(Edge {
            start: Node { pos: start_pos, h: euclidean_dist(start_pos, end_pos) },
            end: Node { pos: end_pos, h: 0 },
            cost: euclidean_dist(start_pos, end_pos)
        }),
        Some(intersect) => {
            // Initializing the frontier
            let mut frontier = Vec::new();

            // Need to offset nodes based on actor radius
            let norms = intersect.1.normals();

            for i in intersect.1.len() {
                // Use normals of edges of polygon to determine direction
                let norm_sum = &norms[i].add(&norms[(i - 1) % norms.len()]);
                let node_pos = intersect.1.get(i) + norm_sum.div(norm_sum.length()).mul(actor_radius);

                // If there is a direct path to the offset node(s), add it/them to the graph/frontier
                if intersect.1.intersects(start_pos, node_pos).len() == 0 {
                    frontier.push(Path.from_edge(graph.add_edge(start_pos, node_pos)));
                }
            }

            while frontier.len() > 0 {
                frontier.sort_by(|a, b| b.f_stat().cmp(a.f_stat()));

                match frontier.pop() {
                    Some(path) => {
                        // Check if end of the path is the goal node
                        match path.edges.last() {
                            Some(edge) => {
                                if edge.end.x == end_pos.x && edge.y == end_pos.y {
                                    return path
                                }

                                // Find all intersections between current edge and goal
                                intersections = Vec::<(Vector2, Polygon)>::new();
                                for poly in obstacles.iter() {
                                    for intersect in poly.intersects(edge.end, end_pos) {
                                        intersections.push((intersect, poly));
                                    }
                                }

                                match intersections.iter().enumerate().min_by(|&(_, i)| euclidean_dist(edge.end, i.0)) {
                                    None => {
                                        frontier.push(path.append_edge(graph.add_edge(edge.end, end_pos)));
                                    }
                                    Some(intersect) => {
                                        // Need to offset nodes based on actor radius
                                        let norms = intersect.1.normals();

                                        for i in intersect.1.len() {
                                            // Use normals of edges of polygon to determine direction
                                            let norm_sum = &norms[i].add(&norms[(i - 1) % norms.len()]);
                                            let node_pos = intersect.1.get(i) + norm_sum.div(norm_sum.length()).mul(actor_radius);

                                            // If there is a direct path to the offset node(s), add it/them to the graph/frontier
                                            if intersect.1.intersects(start_pos, node_pos).len() == 0 {
                                                frontier.push(path.append_edge(graph.add_edge(edge.end, node_pos)));
                                            }
                                        }
                                    }
                                }
                            }
                            None => return None
                        }
                    }
                    None => return None
                }

            }

            None
        }
    }
}

fn euclidean_dist(a: Vector2, b: Vector2) -> Scalar {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt()
}

