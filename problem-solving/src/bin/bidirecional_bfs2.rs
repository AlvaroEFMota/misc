use problem_solving::{generate_graph, WGraph};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut graph = generate_graph();
    bidirecional_bfs2(&mut graph.graph, 'a', 'f');
}

fn bidirecional_bfs2(graph: &mut HashMap<char, Vec<char>>, s: char, t: char) {
    let mut parent: HashMap<char, Option<char>> = HashMap::new();
    let mut vtx_type = HashMap::new();
    let mut s_queue = Vec::new();
    let mut t_queue = Vec::new();
    let mut found = false;
    let mut joint: Option<(char, char)> = None;

    parent.insert(s.clone(), None);
    parent.insert(t.clone(), None);
    vtx_type.insert(s.clone(), 's');
    vtx_type.insert(t.clone(), 't');
    s_queue.push(s.clone());
    t_queue.push(t.clone());

    while (s_queue.len() > 0 || t_queue.len() > 0) && !found {
        if s_queue.len() > 0 {
            let vertex = s_queue.remove(0);

            for adj in graph.get(&vertex).unwrap() {
                let current_type = vtx_type.get(&vertex).unwrap();
                if vtx_type.contains_key(adj) {
                    if vtx_type.get(adj).unwrap() != current_type {
                        joint = Some((vertex, adj.clone()));
                        found = true;
                        break;
                    }
                } else {
                    parent.insert(adj.clone(), Some(vertex.clone()));
                    vtx_type.insert(adj.clone(), current_type.clone());
                    s_queue.push(adj.clone());
                }
            }
        }

        if t_queue.len() > 0 {
            let vertex = t_queue.remove(0);

            for adj in graph.get(&vertex).unwrap() {
                let current_type = vtx_type.get(&vertex).unwrap();
                if vtx_type.contains_key(adj) {
                    if vtx_type.get(adj).unwrap() != current_type {
                        joint = Some((adj.clone(), vertex));
                        found = true;
                        break;
                    }
                } else {
                    parent.insert(adj.clone(), Some(vertex.clone()));
                    vtx_type.insert(adj.clone(), current_type.clone());
                    t_queue.push(adj.clone());
                }
            }
        }
    }

    println!("{:?}", joint);
    println!("{:?}", parent);
}
