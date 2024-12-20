use std::collections::HashMap;

// wgeighted graph
#[derive(Debug)]
pub struct WGraph {
    pub graph: HashMap<char, Vec<char>>,
    pub weights: HashMap<(char, char), i32>,
}
/*
 *       b -4- d
 *      / \   / \
 *    10   \ /   9
 *   /      \     \
 * a       / \     \
 *  \     2   1     f
 *   3   /     \   2
 *    \ /       \ /
 *     c -- 8 -- e
 * */

pub fn generate_graph() -> WGraph {
    let mut graph = HashMap::new();
    let mut weights = HashMap::new();

    graph.insert('a', vec!['b', 'c']);
    graph.insert('b', vec!['a', 'd', 'e']);
    graph.insert('c', vec!['a', 'd', 'e']);
    graph.insert('d', vec!['b', 'c', 'f']);
    graph.insert('e', vec!['b', 'c', 'f']);
    graph.insert('f', vec!['d', 'e']);

    weights.insert(('a', 'b'), 10);
    weights.insert(('b', 'a'), 10);
    weights.insert(('a', 'c'), 3);
    weights.insert(('c', 'a'), 3);
    weights.insert(('b', 'd'), 4);
    weights.insert(('d', 'b'), 4);
    weights.insert(('b', 'e'), 1);
    weights.insert(('e', 'b'), 1);
    weights.insert(('c', 'e'), 8);
    weights.insert(('e', 'c'), 8);
    weights.insert(('c', 'd'), 2);
    weights.insert(('d', 'c'), 2);
    weights.insert(('d', 'f'), 9);
    weights.insert(('f', 'd'), 9);
    weights.insert(('e', 'f'), 2);
    weights.insert(('f', 'e'), 2);

    WGraph { graph, weights }
}

#[allow(dead_code)]
enum Filetype {
    RegularFile,
}
#[allow(dead_code)]
struct Inode {
    file: String,
    size: u64,
    blocks: u8,
    io_block: u8,
    file_type: Filetype,
    //device
    inode: u32,
}
