use crate::core::vector::*;
use crate::core::scalar::*;
use crate::simulation::ai::path::{Node, Edge, Path};
use crate::core::geo::polygon::*;
use crate::simulation::barricade::*;

struct Graph {
    _start: Node,
    goal: Node,
    edges: Vec<Edge>
}

impl Graph {
    fn new(start_pos: Vector2, end_pos: Vector2) -> Self {
        let start_node = Node { pos: start_pos, h: euclidean_dist(start_pos, end_pos) };
        let end_node = Node { pos: end_pos, h: 0.0 };

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
                    start: Node { pos: start_pos, h: euclidean_dist(start_pos, self.goal.pos) },
                    end: Node { pos: end_pos, h: euclidean_dist(end_pos, self.goal.pos) },
                    cost: euclidean_dist(start_pos, end_pos)
                };
                self.edges.push(edge);

                edge
            }
        }
    }
}

struct Intersection {
    pos: Vector2,
    index: usize,
    intersect_type: IntersectType
}

enum IntersectType {
    Building,
    Barricade
}

// Find the shortest path from start_pos to end_pos, accounting for obstacles
pub fn find_path(
    start_pos: Vector2,
    end_pos: Vector2,
    obstacles: &Vec<Polygon>,
    outlines: &Vec<Polygon>,
    barricades: &Vec<Barricade>) -> Option<Path> {

    // Initialize the graph representing the viable paths from start to end
    let mut graph = Graph::new(start_pos, end_pos);

    // Find all intervening obstacles between the start and the goal and store the position of the
    // intersect and the index of the obstacle
    let mut intersections: Vec<Intersection> = vec!();

    for i in 0..obstacles.len() {
        for intersect in obstacles[i].intersects(start_pos, end_pos) {
            intersections.push(Intersection {
                pos: intersect,
                index: i,
                intersect_type: IntersectType::Building
            });
        }
    }

    for i in 0..barricades.len() {
        for intersect in barricades[i].poly.intersects(start_pos, end_pos) {
            intersections.push(Intersection {
                pos: intersect,
                index: i,
                intersect_type: IntersectType::Barricade
            });
        }
    }

    // Find the closest intersection point
    match get_min_index_by_dist(start_pos, intersections) {

        // Nothing between start and end, answer is a straight line
        None => return Some(Path::from_edge(Edge {
            start: Node { pos: start_pos, h: euclidean_dist(start_pos, end_pos) },
            end: Node { pos: end_pos, h: 0.0 },
            cost: euclidean_dist(start_pos, end_pos)
        })),
        Some(intersection) => {
            // Initializing the frontier
            let mut frontier = vec!();

            let mut outline;
            let mut obstacle;

            match intersection.intersect_type {
                IntersectType::Building => {
                    outline = outlines[intersection.index].clone();
                    obstacle = obstacles[intersection.index].clone();
                },
                IntersectType::Barricade => {
                    outline = barricades[intersection.index].clone().outline;
                    obstacle = barricades[intersection.index].clone().poly;
                }
            };

            // Add paths to the frontier based on outline of closest obstacle
            for i in 0..outline.num_sides() {
                if obstacle.intersects(start_pos, outline.get(i)).len() == 0 {
                    frontier.push(Path::from_edge(graph.add_edge(start_pos, outline.get(i))));
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
                                    return Some(path)
                                }

                                // Find all intersections between current edge and goal
                                intersections = vec!();
                                for i in 0..obstacles.len() {
                                    for intersect in obstacles[i].intersects(edge.end.pos, end_pos) {
                                        intersections.push(Intersection {
                                            pos: intersect,
                                            index: i,
                                            intersect_type: IntersectType::Building
                                        });
                                    }
                                }
                                for i in 0..barricades.len() {
                                    for intersect in barricades[i].poly.intersects(edge.end.pos, end_pos) {
                                        intersections.push(Intersection {
                                            pos: intersect,
                                            index: i,
                                            intersect_type: IntersectType::Barricade
                                        });
                                    }
                                }

                                match get_min_index_by_dist(edge.end.pos, intersections) {
                                    None => {
                                        // No intersections, go straight to the goal
                                        path.append_edge(graph.add_edge(edge.end.pos, end_pos));
                                        frontier.push(path);
                                    }
                                    Some(intersection) => {
                                        match intersection.intersect_type {
                                            IntersectType::Building => {
                                                outline = outlines[intersection.index].clone();
                                                obstacle = obstacles[intersection.index].clone();
                                            },
                                            IntersectType::Barricade => {
                                                outline = barricades[intersection.index].clone().outline;
                                                obstacle = barricades[intersection.index].clone().poly;
                                            }
                                        };

                                        // Add paths to the frontier based on outline of closest obstacle
                                        for j in 0..outline.num_sides() {
                                            if edge.end.pos != outline.get(j) && obstacle.intersects(edge.end.pos, outline.get(j)).len() == 0 {
                                                let mut new_path = path.clone();
                                                new_path.append_edge(graph.add_edge(edge.end.pos, outline.get(j)));
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

fn get_min_index_by_dist(pos: Vector2, intersections: Vec<Intersection>) -> Option<Intersection> {
    if intersections.len() == 0 { return None }

    let mut intersection = None;
    let mut dist = INFINITY;

    for i in intersections {
        if euclidean_dist(pos, i.pos) < dist {
            dist = euclidean_dist(pos, i.pos);
            intersection = Some(i);
        }
    }

    intersection
}

fn euclidean_dist(a: Vector2, b: Vector2) -> Scalar {
    ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt()
}
