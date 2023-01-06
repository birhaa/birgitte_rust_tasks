use plotly::{Plot, Scatter};
use std::time::{Instant};

mod helpers;
mod tsm_src;
pub use tsm_src::node::*;
pub use tsm_src::distance::*;
pub use tsm_src::hill_climb::*;
pub use tsm_src::simulated_aannealing::*;

use crate::tsm_src::variations::swap_random_2_nodes;

fn main() {
    const N: usize = 100;
    const MAX_EVALUATION_NO_BETTER: usize = 300;
    let nodes = random_nodes(N);
    let mut solutions: Vec<Vec<f64>> = Vec::new();

    let start = Instant::now();
    solutions.push(tsp_hill_climb(&nodes, swap_random_2_nodes, MAX_EVALUATION_NO_BETTER));
    let mut dur = start.elapsed().as_millis();
    println!("Finished nr1 with duration {dur}");
    solutions.push(tsp_hill_climb_2(&nodes, swap_random_2_nodes));
    dur = start.elapsed().as_millis();
    println!("Finished nr2 with duration {dur}");
    solutions.push(tsp_simulated_annealing(&nodes, swap_random_2_nodes, MAX_EVALUATION_NO_BETTER));
    dur = start.elapsed().as_millis();
    println!("Finished nr3 with duration {dur}");
    solutions.push(tsp_simulated_annealing_2(&nodes, swap_random_2_nodes));
    dur = start.elapsed().as_millis();
    println!("Finished nr4 with duration {dur}");
    // solutions.push(tsp_hill_climb(&nodes, swap_random_best_of));
    // solutions.push(tsp_simulated_annealing(&nodes, swap_random_2_nodes, MAX_EVALUATION_NO_BETTER));
    // solutions.push(tsp_simulated_annealing(&nodes, swap_random_best_of));

    solutions.iter().for_each(|sol| {
        let trace = Scatter::new((0..sol.len()-1).collect(), sol.to_vec()).name("hc_history");
        let mut plot = Plot::new();
        plot.add_trace(trace);
        plot.show();
    })
}
