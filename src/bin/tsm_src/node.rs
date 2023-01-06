use rand::Rng;

#[derive(Clone, Copy)]
pub struct Node {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

pub fn random_nodes(n: usize) -> Vec<Node> {
    let mut rng = rand::thread_rng();
    let mut nodes: Vec<Node> = Vec::new();
    for i in 0..n {
        nodes.push(Node {
            id: i,
            x: (rng.gen::<f64>() * 50.0).round(),
            y: (rng.gen::<f64>() * 50.0).round(),
        })
    }
    return nodes;
}

pub fn swap_2_nodes(nodes: &Vec<Node>, idx1: usize, idx2: usize) -> Vec<Node> {
    let mut new_nodes = nodes.to_vec();
    new_nodes.swap(idx1, idx2);
    new_nodes
}
