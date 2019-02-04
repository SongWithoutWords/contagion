use crate::core::vector::*;

use crate::simulation::ai::path::{Node, Edge, Path};
use crate::simulation::ai::polygon::Polygon;

struct Graph {
    start: Node,
    goal: Node,
    edges: Vec<Edge>
}

impl Graph {
    fn new(start_pos: Vector2, end_pos: Vector2) -> Self {
        let start_node = Node { pos: start_pos, h: euclidean_dist(start_pos, end_pos) };
        let end_node = Node { pos: end_pos, h: 0 };

        Graph {
            start: start_node,
            goal: end_node,
            edges: Vec::new()
        }
    }

    // Add a new edge to the graph, unless it already exists.
    // Creates new nodes if necessary.
    // Does not check if edge is disconnected from graph
    fn add_edge(&mut self, start_pos: Vector2, end_pos: Vector2) -> Edge {
        match self.edges.iter().find(|&&edge| edge.start.pos == start_pos && edge.end.pos == end_pos) {
            Some(edge) => *edge,
            None => {
                let edge = Edge {
                    start: Node { pos: start_pos, h: euclidean_dist(start_pos, self.goal.pos) },
                    end: Node { pos: start_pos, h: euclidean_dist(end_pos, self.goal.pos) },
                    cost: euclidean_dist(start_pos, end_pos)
                };
                self.edges.push(edge);

                edge
            }
        }
    }
}

// Find the shortest path from start_pos to end_pos, accounting for obstacles
pub fn find_path(start_pos: Vector2, end_pos: Vector2, obstacles: Vec<Polygon>) -> Option<Path> {
    // Placeholder for how far out we need to space the actor we are finding the path for
    let actor_radius = 5.0;

    // Initialize the graph representing the viable paths from start to end
    let mut graph = Graph::new(start_pos, end_pos);

    // Find all intervening obstacles between the start and the goal
    let mut intersections = Vec::<(Vector2, Polygon)>::new();
    for poly in obstacles.iter() {
        for intersect in poly.intersects(start_pos, end_pos) {
            intersections.push((intersect, poly.clone()));
        }
    }

    // Find the closest intersection point
    match intersections.iter().min_by_key(|i| euclidean_dist(start_pos, i.0)) {
        // Nothing between start and end, answer is a straight line
        None => return Some(Path::from_edge(Edge {
            start: Node { pos: start_pos, h: euclidean_dist(start_pos, end_pos) },
            end: Node { pos: end_pos, h: 0 },
            cost: euclidean_dist(start_pos, end_pos)
        })),
        Some(intersect) => {
            // Initializing the frontier
            let mut frontier = Vec::new();

            // Need to offset nodes based on actor radius
            let norms = intersect.1.normals();

            for i in 0..intersect.1.num_sides() {
                // Use normals of edges of polygon to determine direction
                let norm_sum = norms[i] + norms[(i - 1) % norms.len()];
                let node_pos = intersect.1.get(i) + (norm_sum / (euclidean_dist(Vector2::zero(), norm_sum) as f64).sqrt()) * actor_radius;

                // If there is a direct path to the offset node(s), add it/them to the graph/frontier
                if intersect.1.intersects(start_pos, node_pos).len() == 0 {
                    frontier.push(Path::from_edge(graph.add_edge(start_pos, node_pos)));
                }
            }

            while frontier.len() > 0 {
                frontier.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

                match frontier.pop() {
                    Some(mut path) => {
                        // Check if end of the path is the goal node
                        match path.edges.last() {
                            Some(edge) => {
                                if edge.end.pos == end_pos {
                                    return Some(path)
                                }

                                // Find all intersections between current edge and goal
                                intersections = Vec::<(Vector2, Polygon)>::new();
                                for poly in obstacles.iter() {
                                    for intersect in poly.intersects(edge.end.pos, end_pos) {
                                        intersections.push((intersect, poly.clone()));
                                    }
                                }

                                match intersections.iter().min_by_key(|i| euclidean_dist(edge.end.pos, i.0)) {
                                    None => {
                                        // No intersections, go straight to the goal
                                        path.append_edge(graph.add_edge(edge.end.pos, end_pos));
                                        frontier.push(path);
                                    }
                                    Some(intersect) => {
                                        // Need to offset nodes based on actor radius
                                        let norms = intersect.1.normals();

                                        for i in 0..intersect.1.num_sides() {
                                            // Use normals of edges of polygon to determine direction
                                            let norm_sum = norms[i] + norms[(i - 1) % norms.len()];
                                            let node_pos = intersect.1.get(i) + (norm_sum / (euclidean_dist(Vector2::zero(), norm_sum) as f64).sqrt()) * actor_radius;

                                            // If there is a direct path to the offset node(s), add it/them to the graph/frontier
                                            if intersect.1.intersects(start_pos, node_pos).len() == 0 {
                                                let mut new_path = path.clone();
                                                new_path.append_edge(graph.add_edge(edge.end.pos, node_pos));
                                                frontier.push(new_path);
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

fn euclidean_dist(a: Vector2, b: Vector2) -> u64 {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt() as u64
}
