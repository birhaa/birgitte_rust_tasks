use std::collections::{hash_map, HashMap};

use rand::Rng;
use plotly::{Histogram, Layout, Plot, Scatter};

use crate::Node;
use crate::distance_total;
use crate::helpers;
use crate::tsm_src::variations::*;

pub fn tsp_simulated_annealing(nodes: &Vec<Node>, variation_fn: fn(nodes: &Vec<Node>) -> Vec<Node>, max_evaluations: usize) -> Vec<f64> {

    let mut best_nodes = nodes.to_vec();
    let mut best_dist = distance_total(&best_nodes);
    let mut num_evaluations = 1;

    let mut current_temp = nodes.len() as f64 *10.0;

    let mut distances: Vec<f64> = Vec::new();

    while num_evaluations < max_evaluations{
        let variation = variation_fn(&best_nodes);
        let variation_dist = distance_total(&variation);

        let p = ((best_dist - variation_dist )/current_temp).exp();
        let random =  helpers::random::random_number(1.0);
  
        if variation_dist < best_dist || random < p {
            num_evaluations = 0;
            best_nodes = variation;
            best_dist = variation_dist;
        }

        num_evaluations +=1;
        distances.push(best_dist);
        if current_temp > 0.01 {
            current_temp*=0.99;
        }
    }

    return distances;
}


pub fn tsp_simulated_annealing_2(nodes: &Vec<Node>, variation_fn: fn(nodes: &Vec<Node>) -> Vec<Node>) -> Vec<f64> {
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
        let variations = all_variations(&variation_fn(&best_nodes));

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
