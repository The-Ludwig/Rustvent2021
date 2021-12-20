use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;

#[derive(PartialEq)]
enum NodeType {
    Upper,
    Lower,
}

struct Node {
    connections: HashSet<String>,
    ntype: NodeType,
}

#[derive(Clone)]
struct Route {
    path: Vec<String>,
    doubled: bool,
}

impl NodeType {
    fn from_str(name: &str) -> NodeType {
        match name.to_lowercase() == name {
            true => Self::Lower,
            false => Self::Upper,
        }
    }
}

impl Node {
    fn from_str(name: &str) -> Node {
        Node {
            connections: HashSet::new(),
            ntype: NodeType::from_str(name),
        }
    }
}

fn parse(filename: &str) -> io::Result<HashMap<String, Node>> {
    let mut graph = HashMap::new();
    for line in fs::read_to_string(filename)?.lines() {
        let names: Vec<_> = line.split("-").collect();
        for (i, j) in [(0, 1), (1, 0)] {
            // this 'if' is only relevant for part two
            // consider this dirty
            if names[j] != "start" {
                match graph.get_mut(names[i]) {
                    None => {
                        let mut node = Node::from_str(names[i]);
                        node.connections.insert(names[j].to_string());
                        graph.insert(names[i].to_string(), node);
                    }
                    Some(x) => {
                        x.connections.insert(names[j].to_string());
                    }
                }
            }
        }
    }

    Ok(graph)
}

fn part_two(graph: &HashMap<String, Node>) -> usize {
    let mut routes: Vec<Route> = vec![Route {
        path: vec![String::from("start")],
        doubled: false,
    }];

    let mut finished_routes: Vec<_> = Vec::new();

    loop {
        let mut current_route = match routes.pop() {
            Some(x) => x,
            None => break,
        };

        let current_name = current_route.path.last().unwrap();
        let current_node = graph.get(current_name).unwrap();

        if current_name == "end" {
            finished_routes.push(current_route);
            continue;
        }

        match current_node.ntype {
            NodeType::Lower => {
                if current_route
                    .path
                    .iter()
                    .filter(|&x| (*current_name).eq(x))
                    .count()
                    > 1
                {
                    if current_route.doubled {
                        continue;
                    } else {
                        current_route.doubled = true;
                    }
                }
            }
            NodeType::Upper => (),
        }

        for next in &current_node.connections {
            let mut add = current_route.clone();
            add.path.push(next.to_string());
            routes.push(add);
        }
    }

    finished_routes.len()
}

fn part_one(graph: &HashMap<String, Node>) -> usize {
    let mut routes: Vec<Vec<String>> = vec![vec![String::from("start")]];

    let mut finished_routes: Vec<Vec<String>> = Vec::new();

    loop {
        let current_route = match routes.pop() {
            Some(x) => x,
            None => break,
        };

        let current_name = current_route.last().unwrap();
        let current_node = graph.get(current_name).unwrap();

        match current_node.ntype {
            NodeType::Lower => {
                if current_route
                    .iter()
                    .filter(|&x| (*current_name).eq(x))
                    .count()
                    > 1
                {
                    continue;
                }
            }
            NodeType::Upper => (),
        }

        for next in &current_node.connections {
            let mut add = current_route.clone();
            add.push(next.to_string());
            routes.push(add);
        }

        if current_name == "end" {
            finished_routes.push(current_route);
        }
    }

    finished_routes.len()
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let graph = parse("inputs/day12").unwrap();
    println!("Answer Part One: {} ", part_one(&graph));

    println!("Answer Part Two: {} ", part_two(&graph));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let r = parse("inputs/day12_test").unwrap();
        assert!(r["HN"].ntype == NodeType::Upper);
        assert!(r["dc"].ntype == NodeType::Lower);
        assert!(r.contains_key("start"));
        assert!(r.contains_key("end"));
        assert!(r.contains_key("kj"));
        assert!(!r.contains_key("ksdfdsfj"));
    }

    #[test]
    fn test_part_one() {
        let graph = parse("inputs/day12_test").unwrap();
        assert_eq!(part_one(&graph), 19)
    }

    #[test]
    fn test_part_two() {
        let graph = parse("inputs/day12_test").unwrap();
        assert_eq!(part_two(&graph), 103)
    }
}
