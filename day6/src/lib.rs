use std::collections::hash_map::HashMap;

pub struct MapTreeNode<K> {
    parent: Option<K>,
    children: Vec<K>,
}

impl<K> MapTreeNode<K> {
    fn new() -> Self {
        MapTreeNode {
            parent: None,
            children: vec![],
        }
    }
}

/// parse the input into a tree contained in a hashmap
/// elements reference the keys of their parent and children
pub fn get_parsed_input() -> HashMap<String, MapTreeNode<String>> {
    let input = include_str!("input/input");

    let mut node_map: HashMap<String, MapTreeNode<String>> = HashMap::new();

    for line in input.lines() {
        let parts = line.split(')').collect::<Vec<_>>();
        let orbited_key = parts[0].trim();
        let orbiter_key = parts[1].trim();
        if !node_map.contains_key(orbited_key) {
            node_map.insert(orbited_key.to_owned(), MapTreeNode::new());
        }

        if !node_map.contains_key(orbiter_key) {
            node_map.insert(orbiter_key.to_owned(), MapTreeNode::new());
        }

        node_map.get_mut(orbiter_key).unwrap().parent = Some(orbited_key.to_owned());

        node_map
            .get_mut(orbited_key)
            .unwrap()
            .children
            .push(String::from(orbiter_key));
    }
    node_map
}

/// the total number of direct + indirect orbits
/// for a node n at depth d, n directly & indirectly orbits d objects
pub fn part1(node_map: &HashMap<String, MapTreeNode<String>>) {
    let root = String::from("COM");

    let mut stack = vec![(&root, 0)];
    let mut count = 0;
    while let Some((key, level)) = stack.pop() {
        count += level;
        for child in &node_map[key].children {
            stack.push((child, level + 1))
        }
    }

    println!("Part 1 = {}", count);
}

/// the number of edges between me and santa!
/// finds a common parent and uses the distance from the child to that parent
pub fn part2(node_map: &HashMap<String, MapTreeNode<String>>) {
    let mut santa_parents = get_parents(&node_map, &String::from("SAN"));
    let mut you_parents = get_parents(&node_map, &String::from("YOU"));

    while santa_parents.last() == you_parents.last() {
        santa_parents.pop();
        you_parents.pop();
    }

    println!("Part 2 = {}", santa_parents.len() + you_parents.len());
}

/// get the parents of the given src node
fn get_parents(node_map: &HashMap<String, MapTreeNode<String>>, src: &String) -> Vec<String> {
    let mut current = &node_map[src];
    let mut parents = Vec::with_capacity(node_map.len());
    while let Some(k) = &current.parent {
        parents.push(k.clone());
        current = &node_map[k];
    }
    parents
}
