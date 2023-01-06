use rand::Rng;

use crate::Node;
use crate::helpers::random::*;
use crate::distance;
use crate::swap_2_nodes;
use crate::distance_total;

pub fn swap_random_2_nodes(nodes: &Vec<Node>) -> Vec<Node>{
    let mut rng = rand::thread_rng();
    let mut x1 = 0;
    let mut x2 = 0;
    let mut random = | | (rng.gen::<f64>()*((nodes.len()-1) as f64)).round() as usize;
    while(x1 == x2){
        x1 = random();
        x2 = random();
    }
    return swap_2_nodes(nodes, x1, x2)
}

pub fn swap_random_chunk(nodes: &Vec<Node>) -> Vec<Node>{
    let mut x1 = 0;
    let mut x2 = 0;
    let chunk_size = random_size(1)+1;
    while(x1 == x2){
        x1 = random_size(nodes.len() -1);
        x2 = random_size(nodes.len() -1);
    }
    let mut new_nodes = nodes.to_vec();
    for i in 0..chunk_size{
        x1 = if x1+i >= new_nodes.len() {0} else {x1+i};
        x2 = if x2+i >= new_nodes.len() {0} else {x2+i};
        new_nodes.swap( x1, x2);
    }
    new_nodes
}

pub fn move_closest_node_to_random(nodes: &Vec<Node>) -> Vec<Node>{
    let variation = nodes.clone();
    let idx = random_size(nodes.len()-1);
    let random_node =  &nodes[idx];

    let close_x_node = variation.iter().reduce(|prev, current| {
        let d1 = distance(prev.x, random_node.x, prev.y, random_node.y).abs();
        let d2 = distance(current.x, random_node.x, current.y, random_node.y).abs();
        if current.id == random_node.id || d1 < d2 {
            return prev;
        }
        return current;
    }).unwrap();
    let idx2 = variation.iter().position(|&n| n.id == close_x_node.id).unwrap();
    return swap_2_nodes(&variation, if idx == nodes.len()-1 {0} else {idx +1}, idx2);
}

pub fn swap_random_best_of(nodes: &Vec<Node>) -> Vec<Node>{
    let variations = get_random_variations(nodes, 10);
    return find_best( variations).0;
}

// Variations

pub fn get_random_variations(nodes: &Vec<Node>, num: usize) -> Vec<Vec<Node>>{
    let mut rng = rand::thread_rng();
    let mut variations: Vec<Vec<Node>> = Vec::new();
    for n in 1..num {
        let mut random = | | (rng.gen::<f64>()*((nodes.len()-1) as f64)).round() as usize;
        variations.push(swap_2_nodes(nodes, random(), random()))
    }
    variations
}

pub fn all_variations(nodes: &Vec<Node>) -> Vec<Vec<Node>>{
    let mut variations: Vec<Vec<Node>> = Vec::new();
    for i in 0..nodes.len() {
        for j in (i+1)..nodes.len(){
            let mut variation = nodes.clone();
            variation[i] = nodes[j];
            variation[j] = nodes[i];
            variations.push(variation)
        }
    }
    variations
}


pub fn find_best( variations:Vec<Vec<Node>>) -> (Vec<Node>, f64){
    let new_best = variations.iter()
    .map(|var| (var, distance_total(var)))
    .reduce( |best, variance| {
        if variance.1 <= best.1 {
            return variance
        }else{
            return best
        }
    }).unwrap();
    return (new_best.0.to_vec(), new_best.1)
}


mod tests {
    use crate::Node;
    use crate::tsm_src::variations::all_variations;

     #[test]
     fn test_move_all_variations(){
         let nodes = {
            let mut nodes : Vec<Node> = Vec::new();
            nodes.push(Node{
                id: 0,
                x: 4.0,
                y: 5.0
            });
            nodes.push(Node{
                id: 1,
                x: 1.0,
                y: 2.0
            });
            nodes.push(Node{
                id: 3,
                x: 6.0,
                y: 7.0
            });
            nodes
        };
         let variations = all_variations(&nodes);
 
         assert_eq!(variations.len(), 3);
      }

}