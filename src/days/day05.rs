use petgraph::algo::toposort;
use petgraph::prelude::*;
use petgraph::Graph;
use std::collections::HashMap;

pub fn part_one(input: &str) -> i32 {
    let app = App::from_input(input);
    app.good_updates
        .iter()
        .map(|update| {
            let len = update.len();
            update[len / 2]
        })
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    let app = App::from_input(input);
    app.bad_updates
        .iter()
        .map(|update| fix_ordering(update, &app.dependencies))
        .map(|update| {
            let len = update.len();
            update[len / 2]
        })
        .sum()
}

#[derive(Debug)]
struct App {
    dependencies: HashMap<i32, Vec<i32>>,
    good_updates: Vec<Vec<i32>>,
    bad_updates: Vec<Vec<i32>>,
}

impl App {
    fn from_input(input: &str) -> Self {
        let dependencies: HashMap<i32, Vec<i32>> = input
            .trim()
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let mut parts = line.split('|');
                let a = parts.next().unwrap().parse().unwrap();
                let b = parts.next().unwrap().parse().unwrap();
                (a, b)
            })
            .fold(HashMap::new(), |mut acc, (a, b)| {
                acc.entry(b).or_default().push(a);
                acc
            });

        let updates: Vec<_> = input
            .trim()
            .lines()
            .filter(|line| line.contains(','))
            .map(|line| {
                line.split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect();

        let (good, bad): (Vec<_>, Vec<_>) = updates
            .iter()
            .cloned()
            .partition(|update| is_good_update(update, &dependencies));

        App {
            dependencies,
            good_updates: good,
            bad_updates: bad,
        }
    }
}

fn is_good_update(update: &[i32], dep_map: &HashMap<i32, Vec<i32>>) -> bool {
    for (idx, item) in update.iter().enumerate() {
        if !dep_map.contains_key(item) {
            continue;
        }
        if let Some(dependencies) = dep_map.get(item) {
            for dependency in dependencies.iter().filter(|&dep| update.contains(dep)) {
                if let Some(dep_idx) = update.iter().position(|&x| x == *dependency) {
                    if dep_idx > idx {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn fix_ordering(update: &[i32], dep_map: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut graph = Graph::<i32, ()>::new();

    let node_indices: HashMap<i32, NodeIndex> =
        update.iter().map(|&n| (n, graph.add_node(n))).collect();

    for &page in update {
        if let Some(deps) = dep_map.get(&page) {
            for &dep in deps {
                if let (Some(&from), Some(&to)) = (node_indices.get(&dep), node_indices.get(&page))
                {
                    graph.add_edge(from, to, ());
                }
            }
        }
    }

    match toposort(&graph, None) {
        Ok(sorted) => sorted.into_iter().map(|idx| graph[idx]).collect(),
        Err(_) => update.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
75,97,47,61,53
97,61,53,29,13
75,29,13
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_part_one() {
        let result = part_one(INPUT);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(INPUT);
        assert_eq!(result, 123);
    }
}
