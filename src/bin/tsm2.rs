use plotly::{Histogram, Layout, Plot, Scatter};

mod helpers;
mod tsm_src;
pub use tsm_src::node::*;
pub use tsm_src::distance::*;
use crate::tsm_src::variations::*;



fn tsp_hill_climb(nodes: &Vec<Node>) -> Vec<f64> {
    let mut best_nodes = nodes.to_vec();
    let mut best_score = distance_total(&best_nodes);
    let mut num_evaluations = 1;
    let max_evaluations = 50000;

    let mut distances: Vec<f64> = Vec::new();

    while num_evaluations < max_evaluations{
        let mut move_made = false;
        let variations = all_variations(&swap_random_2_nodes(&best_nodes));

        for i in 0..variations.len(){
            if num_evaluations > max_evaluations {break;}
            num_evaluations+=1;

            let variation = &variations[i];
            let variation_score = distance_total(&variation);

            if variation_score < best_score {
                best_nodes = variation.clone();
                best_score = variation_score;
                move_made = true;
                distances.push(best_score);
                break;
            }
        }
        //No better solution, local minima
        if(!move_made){
            //break;
        }
    }
    return distances;
}

fn tsp_simulated_annealing(nodes: &Vec<Node>) -> Vec<f64> {
    let mut best_nodes = nodes.to_vec();
    let mut current_best_score = distance_total(&best_nodes);
    let mut best_score = current_best_score;
    let mut num_evaluations = 1;
    let max_evaluations = 500000;
    let mut temperature = nodes.len() as f64*50.0;
    let alpha = 0.99;

    let mut distances: Vec<f64> = Vec::new();

    while temperature > 0.1{
        let mut done = false;
        let variations = all_variations(&swap_random_2_nodes(&best_nodes));

        for i in 0..(variations.len()-1) {
            if(num_evaluations > max_evaluations){done = true; break;}
            num_evaluations+=1;

            let variation = &variations[i];
            let variation_score = distance_total(&variation);

            let p = ((current_best_score - variation_score )/temperature).exp();
            let random =  helpers::random::random_number(1.0);

            if variation_score < current_best_score || random < p {
                best_nodes = variation.clone();
                current_best_score = variation_score;
                distances.push(current_best_score);
                if(current_best_score < best_score){
                    best_score = current_best_score;
                }
                break;
            }
        }
        temperature *= alpha;
        //Finish
        if done {break;}
    }
    return distances;
}




fn main() {
    const N: usize = 10;
    let nodes = random_nodes(N);

    let hc_history = tsp_hill_climb(&nodes);
    let sa_history = tsp_simulated_annealing(&nodes);

    let trace = Scatter::new((0..hc_history.len()-1).collect(), hc_history).name("hc_history");
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();

    let trace2 = Scatter::new((0..sa_history.len()-1).collect(), sa_history).name("sa_history");
    let mut plot2 = Plot::new();
    plot2.add_trace(trace2);
    plot2.show();

    // let trace3 = Scatter::new((0..sa_history2.len()-1).collect(), sa_history2).name("sa_history");
    // let mut plot3 = Plot::new();
    // plot3.add_trace(trace3);
    // plot3.show();


}
