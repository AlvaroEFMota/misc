use core::option::Option;
use problem_solving::{generate_graph, WGraph};
use std::collections::{HashMap, HashSet};

fn main() {
    bidirecional_bfs(generate_graph(), 'a', 'f');
}

fn bidirecional_bfs(graph: WGraph, s: char, t: char) -> Vec<char> {
    let mut visited = HashSet::new();
    let mut v_type = HashMap::new();
    let mut parent = HashMap::new();

    let mut s_queue: Vec<&char> = Vec::new();
    let mut t_queue: Vec<&char> = Vec::new();
    s_queue.push(&s);
    t_queue.push(&t);
    v_type.insert(s.clone(), 's');
    v_type.insert(t.clone(), 't');
    parent.insert(s.clone(), None);
    parent.insert(t.clone(), None);
    let mut joint: Option<(char, char)> = None;
    let mut found = false;
    while s_queue.len() > 0 && t_queue.len() > 0 && !found {
        let s_vertex = s_queue.remove(0);
        visited.insert(s_vertex.clone());

        for adj in &graph.graph[s_vertex] {
            let vertex_type = v_type.get(s_vertex).unwrap();

            if v_type.contains_key(adj) {
                let adj_type = v_type.get(adj).unwrap();
                if vertex_type != adj_type {
                    joint = Some((s_vertex.clone(), adj.clone()));
                    found = true;
                    break;
                }
            } else {
                parent.insert(adj.clone(), Some(s_vertex.clone()));
                v_type.insert(adj.clone(), vertex_type.clone());
            }

            if !visited.contains(adj) {
                s_queue.push(adj);
            }
        }

        let t_vertex = t_queue.remove(0);
        visited.insert(t_vertex.clone());

        for adj in &graph.graph[t_vertex] {
            let vertex_type = v_type.get(t_vertex).unwrap();

            if v_type.contains_key(adj) {
                let adj_type = v_type.get(adj).unwrap();
                if vertex_type != adj_type {
                    joint = Some((adj.clone(), t_vertex.clone()));
                    found = true;
                    break;
                }
            } else {
                parent.insert(adj.clone(), Some(t_vertex.clone()));
                v_type.insert(adj.clone(), vertex_type.clone());
            }

            if !visited.contains(adj) {
                t_queue.push(adj);
            }
        }
    }

    println!("{:?}", joint);
    println!("{:?}", parent);
    Vec::new()
}
