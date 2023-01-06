use rand::Rng;
// use plotpy::{Histogram};
use plotly::{Histogram, Layout, Plot, Scatter};


#[derive(Clone, Copy)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
}

fn random_nodes(N: usize) -> Vec<Node> {
    let mut rng = rand::thread_rng();
    let mut nodes: Vec<Node> = Vec::new();
    for i in 0..N {
        nodes.push(Node{
             id: i,
             x: (rng.gen::<f64>()*10.0).round(),
             y: (rng.gen::<f64>()*10.0).round()
        })
    }
    return nodes;  
}

fn distance(x1: f64, x2: f64, y1: f64, y2: f64) -> f64{
    ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt()
}

fn distance_total(nodes: &Vec<Node>) -> f64{
    return nodes
    .into_iter()
    .enumerate()
    .fold(0.0,  |s, (i, node)| {
        if i == nodes.len() -1 {
            let start_node = &nodes[0];
            return  s + distance(node.x, start_node.x, node.y, start_node.y);
        }else{
            let next = &nodes[i+1];
            s + distance(next.x,node.x, next.y, node.y)
        }
    });
}

fn swap_2_nodes(nodes: &Vec<Node>, idx1: usize, idx2: usize) -> Vec<Node> {
    let mut new_nodes = nodes.to_vec();
    new_nodes.swap( idx1, idx2);
    new_nodes
}

fn get_random_variations(nodes: &Vec<Node>, num: usize) -> Vec<Vec<Node>>{
    let mut rng = rand::thread_rng();
    let mut variations: Vec<Vec<Node>> = Vec::new();
    for n in 1..num {
        let mut random = | | (rng.gen::<f64>()*((nodes.len()-1) as f64)).round() as usize;
        variations.push(swap_2_nodes(nodes, random(), random()))
    }
    variations
}

fn move_all_variations(nodes: &Vec<Node>) -> Vec<Vec<Node>>{
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


fn find_best(current_nodes: &Vec<Node>, current_best_d: f64, variations:Vec<Vec<Node>>) -> (Vec<Node>, f64){
    let new_best = variations.iter()
    .map(|var| (var, distance_total(var)))
    .fold((current_nodes, current_best_d), |best, variance| {
        if variance.1 < best.1 {
            return variance
        }else{
            return best
        }
    });
    return (new_best.0.to_vec(), new_best.1)
}

fn tsp_hill_climb(nodes: &Vec<Node>) -> Vec<f64> {
    let mut best_nodes = nodes.to_vec();
    let mut best_score = distance_total(&best_nodes);
    let mut num_evaluations = 1;
    let max_evaluations = 50000;

    let mut distances: Vec<f64> = Vec::new();

    while num_evaluations < max_evaluations{
        let mut move_made = false;
        let variations = move_all_variations(nodes);
        //let variations = get_random_variations(nodes, 10);
        for i in 0..variations.len(){
            if(num_evaluations > max_evaluations){break;}
            let variation = &variations[i];
            let variation_score = distance_total(&variation);
            num_evaluations+=1;
            if(variation_score < best_score){
                best_nodes = variation.clone();
                best_score = variation_score;
                move_made = true;
                distances.push(best_score);
                //break;
            }
        }
        if(!move_made){
            //break;
        }
    }
    return distances;
}

fn tsp_hill_climb2(nodes: &Vec<Node>) -> Vec<f64> {
    let mut distances: Vec<f64> = Vec::new();

    let mut current_solution = &nodes.clone();
    let mut current_dist = distance_total(&current_solution);
    let mut variations = move_all_variations(&current_solution);
    let (mut best_varation, mut best_variation_dist) = find_best(&current_solution, current_dist, variations);

    loop {
        if !(best_variation_dist < current_dist) {
            break;
        }
        current_solution = &best_varation;
        current_dist = best_variation_dist;
        variations = move_all_variations(&current_solution);
        (best_varation, best_variation_dist) = find_best(&current_solution, current_dist, variations);
        distances.push(current_dist);
    }


    return distances;
}

fn tsp_simulated_annealing(nodes: &Vec<Node>) -> Vec<f64> {
    Vec::new()
}




fn main() {
    const N: usize = 100;
    let nodes = random_nodes(N);

    let hc_history = tsp_hill_climb(&nodes);
    let hc_history2 = tsp_hill_climb2(&nodes);
   // let sa_history2 = tsp_simulated_annealing(&nodes);

    let trace = Scatter::new((0..hc_history.len()-1).collect(), hc_history).name("hc_history");
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();

    let trace2 = Scatter::new((0..hc_history2.len()-1).collect(), hc_history2).name("sa_history");
    let mut plot2 = Plot::new();
    plot2.add_trace(trace2);
    plot2.show();

    // let trace3 = Scatter::new((0..sa_history2.len()-1).collect(), sa_history2).name("sa_history");
    // let mut plot3 = Plot::new();
    // plot3.add_trace(trace3);
    // plot3.show();


}


mod tests {
    use crate::distance;
    use crate::Node;
    use crate::distance_total;
    use crate::move_all_variations;


    #[test]
    fn test_distance(){
       assert_eq!(format!("{:.5}", distance(1.0, 4.0, 2.0, 5.0)), "4.24264");
    }

    #[test]
    fn test_distance_total(){
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
        assert_eq!(format!("{:.5}", distance_total(&nodes)), "8.48528");

        nodes.push(Node{
            id: 3,
            x: 6.0,
            y: 7.0
        });

        assert_eq!(format!("{:.5}", distance_total(&nodes)), "14.14214");
     }

     #[test]
     fn test_move_all_variations(){
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
         let variations = move_all_variations(&nodes);
 
         assert_eq!(variations.len(), 3);
      }


}
