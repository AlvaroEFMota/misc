use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use problem_solving::generate_graph;
fn main() {
    let wgraph = generate_graph();
    
    let mut visited = HashSet::new();
    let mut min_heap: BinaryHeap<Reverse<(i32, char)>> = BinaryHeap::new();
    let mut dist: HashMap<char, i32> = HashMap::new();
    
    for vertex in wgraph.graph.keys() {
        dist.insert(*vertex, i32::MAX);
    }
    
    
    let source = 'a';

    *dist.get_mut(&source).unwrap() = 0;
    min_heap.push(Reverse((0, source)));
    
    while let Some(Reverse((_, vertex))) = min_heap.pop() {
        visited.insert(vertex);
        for adj in wgraph.graph.get(&vertex).unwrap() {
            if !visited.contains(&adj) {
                let adj_dist = dist.get(adj).unwrap();
                let vertex_dist = *dist.get(&vertex).unwrap();
                let weight = *wgraph.weights.get(&(vertex, *adj)).unwrap();
                let new_dist = vertex_dist + weight;
                if new_dist < *adj_dist {
                    *dist.get_mut(adj).unwrap() = new_dist;
                    min_heap.push(Reverse((new_dist, adj.clone())));
                }
            }
        }
    }
    
    println!("{:?}", dist);
    
    
}