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

fn get_variations(nodes: &Vec<Node>, num: usize) -> Vec<Vec<Node>>{
    let mut rng = rand::thread_rng();
    let mut variations: Vec<Vec<Node>> = Vec::new();
    for n in 1..num {
        let mut random = | | (rng.gen::<f64>()*((nodes.len()-1) as f64)).round() as usize;
        variations.push(swap_2_nodes(nodes, random(), random()))
    }
    variations
}

fn find_best(current_nodes: &Vec<Node>, current_best_d: f64) -> (Vec<Node>, f64){
    let variations = get_variations(current_nodes, 10);
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
    let mut num_evaluations= 1;
    let num_max_evaluations= 5000;
    let mut current_best = distance_total(nodes);
    let mut current_nodes = nodes.to_vec();
    let mut distances: Vec<f64> = Vec::new();

    while num_evaluations < num_max_evaluations{
        let (new_best_nodes, new_best_distance) = find_best(&current_nodes, current_best);
        current_best = new_best_distance;
        current_nodes = new_best_nodes;
        distances.push(current_best);
        num_evaluations +=1;
    }

    return distances;
}

fn tsp_simulated_annealing(nodes: &Vec<Node>) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    let init_temp: f64 = 50.0;
    let alpha = 0.99;

    let mut current_temp = init_temp;
    let mut current_nodes = nodes.to_vec();
    let mut same_solution = 0;
    let mut same_cost_diff = 0;

    let mut distances: Vec<f64> = Vec::new();


    while same_solution < 1500 && same_cost_diff < 150000{
        let mut random = | | (rng.gen::<f64>()*((nodes.len()-1) as f64)).round() as usize;
        let other_nodes = swap_2_nodes(nodes, random(), random());
        let distance = distance_total(&current_nodes);
        let distance_other = distance_total(&other_nodes);
        let cost_diff =  distance - distance_other;//get_cost( distance_other, nodes.len()) - get_cost(distance, nodes.len());
        match cost_diff {
            0.0 => {
                current_nodes = other_nodes;
                same_cost_diff += 1;
                same_solution = 0;
                distances.push(distance_other);
            }
            cf if cf > 0.0  => {
                current_nodes = other_nodes;
                same_cost_diff = 0;
                same_solution = 0;
                distances.push(distance_other);
            }
            _ =>{
                let random = rng.gen::<f64>();
                let p = ((cost_diff)/current_temp).exp();
                if(random <= p){
                    current_nodes = other_nodes;
                    same_cost_diff = 0;
                    same_solution = 0;
                    distances.push(distance_other);
                }else{
                    same_solution  +=1;
                    same_cost_diff +=1;
                    distances.push(distance);
                }
                current_temp = current_temp*alpha;
            }
        }

    }
    distances
}


fn tsp_simulated_annealing2(nodes: &Vec<Node>) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    let init_temp: f64 = 5000.0;
    let alpha = 0.99;

    let mut num_evaluations= 1;
    let num_max_evaluations= 50;
    let mut distances: Vec<f64> = Vec::new();

    let mut current_nodes = nodes.to_vec();
    let mut current_best = distance_total(&current_nodes);


    while num_evaluations < num_max_evaluations {

        let mut current_temp = init_temp;
        // let mut best_nodes = nodes.to_vec();

        while current_temp > 0.1{
            let mut random = | | (rng.gen::<f64>()*((nodes.len()-1) as f64)).round() as usize;
            
            let new_nodes = swap_2_nodes(&current_nodes, random(), random());
            //let new_nodes = find_best(&current_nodes, current_best).0;

            let distance_new = distance_total(&new_nodes);
            let p = (-(distance_new - current_best )/current_temp).exp();
            let random =  rng.gen::<f64>();

            if(distance_new < current_best){
                current_best = distance_new;
                current_nodes = new_nodes.clone();
                // best_nodes = new_nodes.clone();
            }else if (random < p){
                current_nodes = new_nodes;
                // current_best = distance_new;
            }

            // if ( diff < 0.0 || p < random ){
            //     current_nodes = new_nodes;
            //     distance = distance_new;
            //     distances.push(distance_new);
            // }

            distances.push(current_best);
            current_temp *= alpha;

        }
        num_evaluations +=1;
    }
    distances
}


fn tsp_s_a(nodes: &Vec<Node>) -> Vec<f64> {

    let start_temp = 100.0;
    let mut t = start_temp;
    let mut best_distance = distance_total(&nodes);
    println!("Initial distance of travel: {best_distance}");
    let best_solution = nodes.to_vec();
    let mut current_solution = best_solution.clone();

    let mut distances: Vec<f64> = Vec::new();


    for n in 1..150000 {
        let mut rng = rand::thread_rng();
        let mut random = | | (rng.gen::<f64>()*((nodes.len()-1) as f64)).round() as usize;

        if( t > 0.0001){
            let old_solution = current_solution.clone();
            current_solution = swap_2_nodes(&current_solution, random(), random());
            let current_distance = distance_total(&current_solution);

            let p = ((best_distance - current_distance )/t).exp();
            let random =  rng.gen::<f64>();

            if(current_distance < best_distance ){
                best_distance = current_distance;
            }else if(p < random){
                current_solution = old_solution;
            }
            distances.push(best_distance);
            t *= 0.99;
        }else{
            continue;
        }
    }
        distances

    }


fn main() {
    const N: usize = 100;
    let nodes = random_nodes(N);
    println!("Finished");
    // let temp: f64 = 5000.0;
    // let t: f64 = (-0.02/temp as f64).exp();
    // let t11 = (-200.0/temp as f64).exp();
    // let t1 = (-2000.0/temp as f64).exp();
    // let t2 = (-20000.0/temp as f64).exp();

    let hc_history = tsp_hill_climb(&nodes);
    let sa_history = tsp_s_a(&nodes);
    let sa_history2 = tsp_simulated_annealing(&nodes);

    let trace = Scatter::new((0..hc_history.len()-1).collect(), hc_history).name("hc_history");
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();

    let trace2 = Scatter::new((0..sa_history.len()-1).collect(), sa_history).name("sa_history");
    let mut plot2 = Plot::new();
    plot2.add_trace(trace2);
    plot2.show();

    let trace3 = Scatter::new((0..sa_history2.len()-1).collect(), sa_history2).name("sa_history");
    let mut plot3 = Plot::new();
    plot3.add_trace(trace3);
    plot3.show();


}
