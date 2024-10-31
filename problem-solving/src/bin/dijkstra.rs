use std::{cmp::Reverse, collections::BinaryHeap, collections::HashMap};

use problem_solving::generate_graph;
fn main() {
    let wgraph = generate_graph();

    let mut dist: HashMap<&char, i32> = HashMap::new();
    let mut visited: HashMap<&char, bool> = HashMap::new();
    let mut min_heap: BinaryHeap<Reverse<(i32, char)>> = BinaryHeap::new();

    for vertex in wgraph.graph.keys() {
        dist.insert(vertex, i32::MAX);
        visited.insert(vertex, false);
    }

    min_heap.push(Reverse((0, 'a')));
    *dist.get_mut(&'a').unwrap() = 0;

    while let Some(Reverse((_, vertex))) = min_heap.pop() {
        if *visited.get(&vertex).unwrap() {
            continue;
        }

        *visited.get_mut(&vertex).unwrap() = true;
        for adj in wgraph.graph.get(&vertex).unwrap() {
            if !visited.get(&adj).unwrap() {
                let new_distance =
                    *dist.get(&vertex).unwrap() + *wgraph.weights.get(&(vertex, *adj)).unwrap();
                if new_distance < *dist.get(&adj).unwrap() {
                    *dist.get_mut(&adj).unwrap() = new_distance;
                    min_heap.push(Reverse((new_distance, *adj)));
                }
            }
        }
    }

    println!("{:?}", dist);
}
