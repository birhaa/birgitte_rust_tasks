use plotly::{Plot, Scatter};
use std::time::Instant;

mod helpers;
mod tsm_src;
pub use tsm_src::distance::*;
pub use tsm_src::hill_climb::*;
pub use tsm_src::node::*;
pub use tsm_src::simulated_aannealing::*;

use crate::tsm_src::variations::*;

fn get_variation_fn(v: usize) -> fn(nodes: &Vec<Node>) -> Vec<Node> {
    match v {
        0 => swap_random_2_nodes,
        1 => swap_random_chunk,
        2 => swap_closest_node_to_random,
        3 => swap_random_best_of,
        _ => swap_random_2_nodes,
    }
}

type AlgorithmFn = dyn Fn(&Vec<Node>, fn(nodes: &Vec<Node>) -> Vec<Node>) -> Vec<f64>;

fn main() {
    const N: usize = 100;
    const V: usize = 0;
    const DRAW_RESULTS: bool = true;
    let nodes = random_nodes(N);
    let mut solutions: Vec<Vec<f64>> = Vec::new();
    let variation_fn = get_variation_fn(V);
    let algorithms: Vec<(&str, &AlgorithmFn)> = vec![
        ("Hill climb subset variation", &tsp_hill_climb),
        ("Hill climb all variation", &tsp_hill_climb_2),
        (
            "Simulated annealing subset variation",
            &tsp_simulated_annealing,
        ),
        (
            "Simulated annealing all variation",
            &tsp_simulated_annealing_2,
        ),
    ];

    let mut start = Instant::now();
    algorithms.iter().for_each(|a| {
        let last = a.1(&nodes, variation_fn);
        match last.last() {
            Some(l) => println!(
                "Finished {:?} with last result {:?} with duration {:?}",
                a.0,
                l,
                start.elapsed().as_millis()
            ),
            None => println!("No result found"),
        }
        solutions.push(last);
        start = Instant::now();
    });

    if DRAW_RESULTS {
        solutions.iter().for_each(|sol| {
            let trace = Scatter::new((0..sol.len() - 1).collect(), sol.to_vec()).name("hc_history");
            let mut plot = Plot::new();
            plot.add_trace(trace);
            plot.show();
        })
    }
}
