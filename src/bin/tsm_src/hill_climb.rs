use std::collections::{hash_map, HashMap};

use rand::Rng;
use plotly::{Histogram, Layout, Plot, Scatter};

use crate::Node;
use crate::distance_total;
use crate::tsm_src::variations::*;

// algortihms 

pub fn tsp_hill_climb(nodes: &Vec<Node>, variation_fn: fn(nodes: &Vec<Node>) -> Vec<Node>, max_evaluations: usize ) -> Vec<f64> {
    let mut best_nodes = nodes.to_vec();
    let mut best_dist = distance_total(&best_nodes);
    let mut num_evaluations = 1;

    let mut distances: Vec<f64> = Vec::new();

    while num_evaluations < max_evaluations{
        let variation = variation_fn(&best_nodes);
        let variation_dist = distance_total(&variation);
        if(variation_dist < best_dist){
            num_evaluations = 0;
            best_nodes = variation;
            best_dist = variation_dist;
        }else{
            num_evaluations +=1;
        }
        distances.push(best_dist)
    }

    return distances;
}

pub fn tsp_hill_climb_2(nodes: &Vec<Node>, variation_fn: fn(nodes: &Vec<Node>) -> Vec<Node>) -> Vec<f64> {
    let mut best_nodes = nodes.to_vec();
    let mut best_score = distance_total(&best_nodes);
    let mut num_evaluations = 1;
    let max_evaluations = 50000;

    let mut distances: Vec<f64> = Vec::new();

    while num_evaluations < max_evaluations{
        let mut move_made = false;
        let variations = all_variations(&variation_fn(&best_nodes));

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



