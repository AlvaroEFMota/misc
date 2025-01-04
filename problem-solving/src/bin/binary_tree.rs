use std::fmt;

#[derive(Debug)]
pub struct Node {
    value: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn main() {
    let mut node = insert(None, 50);
    node = insert(node.take(), 10);
    node = insert(node.take(), 70);
    node = insert(node.take(), 5);
    println!("{:?}", node);
}

fn insert(root_node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
    let new_node = Node {
        value: value,
        height: 0,
        left: None,
        right: None,
    };

    // pegando a propriedade da variável root_node
    // diferente de &mut root_node, que apenas acessa um endereço mutável
    // sem pegar a propriedade. Nessa implementação está correto desta forma
    if let Some(mut node) = root_node {
        if value < node.value {
            node.left = insert(node.left.take(), value);
        } else {
            node.right = insert(node.right.take(), value);
        }
        Some(node)
    } else {
        Some(Box::new(new_node))
    }
}
