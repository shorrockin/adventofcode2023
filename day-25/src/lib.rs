use pathfinding::directed::bfs::bfs_reach;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> usize {
    let mut graph = parse(input);
    for _ in 0..3 {
        let edge = most_connected_edge(&graph);
        graph.get_mut(&edge.0).unwrap().remove(edge.1);
        graph.get_mut(&edge.1).unwrap().remove(edge.0);
    }

    let subgraph_length =
        bfs_reach(*graph.keys().next().unwrap(), |n| graph[n].iter().copied()).count();
    subgraph_length * (graph.len() - subgraph_length)
}

pub fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (left, connections) = line.split_once(": ").unwrap();
        for right in connections.split_whitespace() {
            graph.entry(left).or_default().insert(right);
            graph.entry(right).or_default().insert(left);
        }
    }
    graph
}

fn most_connected_edge<'a>(graph: &HashMap<&'a str, HashSet<&'a str>>) -> (&'a str, &'a str) {
    let mut paths: HashMap<(&str, &str), usize> = HashMap::new();

    // loop through all the keys
    for start in graph.keys().copied() {
        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut visited = HashSet::new();
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            for n in graph[&node].iter().copied() {
                if visited.contains(&n) {
                    continue;
                }

                queue.push_back(n);
                visited.insert(n);

                let edge = match n < node {
                    true => (n, node),
                    false => (node, n),
                };
                *paths.entry(edge).or_default() += 1;
            }
        }
    }
    paths.into_iter().max_by_key(|&(_, v)| v).unwrap().0
}

#[cfg(test)]
mod tests {
    use super::*;

    //static INPUT: &str = include_str!("input.txt");
    static EXAMPLE: &str = include_str!("input.example.txt");

    #[test]
    fn part_one_works() {
        assert_eq!(54, part_one(EXAMPLE));
        // takes a little over a second to run
        //assert_eq!(596376, part_one(INPUT));
    }
}
