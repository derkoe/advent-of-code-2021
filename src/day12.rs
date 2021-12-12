use aoc_runner_derive::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    nodes: HashSet<String>,
    edges: HashMap<String, HashSet<String>>,
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Graph {
    let mut graph = Graph {
        nodes: HashSet::new(),
        edges: HashMap::new(),
    };
    input.lines().for_each(|line| {
        let mut parts = line.split('-');
        let node_a = parts.next().unwrap();
        let node_b = parts.next().unwrap();
        graph.nodes.insert(node_a.to_string());
        graph.nodes.insert(node_b.to_string());
        graph
            .edges
            .entry(node_a.to_string())
            .or_default()
            .insert(node_b.to_string());
        graph
            .edges
            .entry(node_b.to_string())
            .or_default()
            .insert(node_a.to_string());
    });
    graph
}

#[aoc(day12, part1)]
fn part1(input: &Graph) -> usize {
    bfs(input, "start", Vec::new(), HashSet::new(), false)
}

fn bfs(
    graph: &Graph,
    current_node: &str,
    mut path: Vec<String>,
    mut visited: HashSet<String>,
    first_small_cave: bool,
) -> usize {
    let mut path_count = 0;
    visited.insert(current_node.to_string());
    path.push(current_node.to_string());

    if current_node == "end" {
        return 1;
    }

    if graph.edges.contains_key(current_node) {
        for node in graph.edges[current_node].iter() {
            if !visited.contains(node) || node.chars().all(|c| c >= 'A' && c <= 'Z') {
                path_count += bfs(graph, node, path.clone(), visited.clone(), first_small_cave);
            } else if first_small_cave && node != "start" {
                path_count += bfs(graph, node, path.clone(), visited.clone(), false);
            }
        }
    }

    path_count
}

#[aoc(day12, part2)]
fn part2(input: &Graph) -> usize {
    bfs(input, "start", Vec::new(), HashSet::new(), true)
}
