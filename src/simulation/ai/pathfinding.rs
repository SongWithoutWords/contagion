use crate::core::vector::*;
use crate::core::scalar::*;
use crate::simulation::ai::path::{Node, Edge, Path};
use crate::core::geo::polygon::*;

struct Graph {
    _start: Node,
    goal: Node,
    edges: Vec<Edge>
}

impl Graph {
    fn new(start_pos: Vector2, end_pos: Vector2) -> Self {
        let start_node = Node { pos: start_pos, h: euclidean_dist(start_pos, end_pos) as u64 };
        let end_node = Node { pos: end_pos, h: 0 };

        Graph {
            _start: start_node,
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
                    start: Node { pos: start_pos, h: euclidean_dist(start_pos, self.goal.pos) as u64 },
                    end: Node { pos: end_pos, h: euclidean_dist(end_pos, self.goal.pos) as u64 },
                    cost: euclidean_dist(start_pos, end_pos) as u64
                };
                self.edges.push(edge);

                edge
            }
        }
    }
}

// Find the shortest path from start_pos to end_pos, accounting for obstacles
pub fn find_path(
    start_pos: Vector2,
    end_pos: Vector2,
    obstacles: &Vec<Polygon>,
    outlines: &Vec<Polygon>) -> Option<Vec<Vector2>> {

    // Initialize the graph representing the viable paths from start to end
    let mut graph = Graph::new(start_pos, end_pos);

    // Find all intervening obstacles between the start and the goal and store the position of the
    // intersect and the index of the obstacle
    let mut intersections: Vec<(Vector2, usize)> = vec!();
    for i in 0..obstacles.len() {
        for intersect in obstacles[i].intersects(start_pos, end_pos) {
            intersections.push((intersect, i));
        }
    }

    // Find the closest intersection point
    match get_min_index_by_dist(start_pos, intersections) {

        // Nothing between start and end, answer is a straight line
        None => return Some(vec![end_pos]),
        Some(i) => {
            // Initializing the frontier
            let mut frontier = vec!();

            // Add paths to the frontier based on outline of closest obstacle
            for j in 0..outlines[i].num_sides() {
                if obstacles[i].intersects(start_pos, outlines[i].get(j)).len() == 0 {
                    frontier.push(Path::from_edge(graph.add_edge(start_pos, outlines[i].get(j))));
                }
            }

            while frontier.len() > 0 {
                // Sort frontier so that path with lowest f-stat is last (so it can be popped)
                frontier.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

                match frontier.pop() {
                    Some(mut path) => {
                        // Check if end of the path is the goal node
                        match path.edges.last() {
                            Some(edge) => {
                                if edge.end.pos == end_pos {
                                    return Some(path.to_vec())
                                }

                                // Find all intersections between current edge and goal
                                intersections = vec!();
                                for j in 0..obstacles.len() {
                                    for intersect in obstacles[j].intersects(edge.end.pos, end_pos) {
                                        intersections.push((intersect, j));
                                    }
                                }

                                match get_min_index_by_dist(edge.end.pos, intersections) {
                                    None => {
                                        // No intersections, go straight to the goal
                                        path.append_edge(graph.add_edge(edge.end.pos, end_pos));
                                        frontier.push(path);
                                    }
                                    Some(j) => {
                                        // Add paths to the frontier based on outline of closest obstacle
                                        for k in 0..outlines[j].num_sides() {
                                            if edge.end.pos != outlines[j].get(k) && obstacles[j].intersects(edge.end.pos, outlines[j].get(k)).len() == 0 {
                                                let mut new_path = path.clone();
                                                new_path.append_edge(graph.add_edge(edge.end.pos, outlines[j].get(k)));
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

fn get_min_index_by_dist(pos: Vector2, intersections: Vec<(Vector2, usize)>) -> Option<usize> {

    let mut index = 0;
    let mut dist = INFINITY;

    if intersections.len() == 0 { return None }

    for intersection in intersections {
        if euclidean_dist(pos, intersection.0) < dist {
            dist = euclidean_dist(pos, intersection.0);
            index = intersection.1;
        }
    }

    Some(index)
}

fn euclidean_dist(a: Vector2, b: Vector2) -> Scalar {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt()
}
