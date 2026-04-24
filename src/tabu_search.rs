use crate::io::{City, save_tour};
use crate::math_util::*;
use std::collections::VecDeque;

pub struct TabuParameters {
    pub max_iterations: usize,
    pub tabu_tenure: usize,
    pub neighborhood_size: usize,
}

pub fn run_tabu_search(cities: &[City], output_path: &str, params: &TabuParameters) -> f64 {
    let n = cities.len();
    if n == 0 { return 0.0; }

    let dist_matrix = create_distance_matrix(cities);
    let mut current_path = generate_random_path(n);
    let mut current_distance = calculate_total_distance(&current_path, &dist_matrix);

    let mut best_path = current_path.clone();
    let mut best_distance = current_distance;

    let mut tabu_list: VecDeque<(usize, usize)> = VecDeque::with_capacity(params.tabu_tenure);

    println!("Starting Tabu Search for {} cities. Initial distance: {:.2}", n, current_distance);

    for iter in 0..params.max_iterations {
        let mut best_neighbor_delta = f64::MAX;
        let mut best_move = None;

        for _ in 0..params.neighborhood_size {
            let (i, j) = get_random_indices(n);

            let is_tabu = tabu_list.contains(&(i, j)) || tabu_list.contains(&(j, i));
            
            let delta = calculate_2opt_delta(&current_path, i, j, &dist_matrix);

            if !is_tabu || (current_distance + delta < best_distance) {
                if delta < best_neighbor_delta {
                    best_neighbor_delta = delta;
                    best_move = Some((i, j));
                }
            }
        }

        if let Some((i, j)) = best_move {
            apply_2opt(&mut current_path, i, j);
            current_distance += best_neighbor_delta;

            if tabu_list.len() >= params.tabu_tenure {
                tabu_list.pop_front();
            }
            tabu_list.push_back((i, j));

            if current_distance < best_distance {
                best_distance = current_distance;
                best_path = current_path.clone();
            }
        }

        if iter % 1000 == 0 && iter > 0 {
            println!("Iteration {}: Best Distance = {:.2}", iter, best_distance);
        }
    }

    println!("Tabu Search finished. Best distance: {:.2}", best_distance);

    if let Err(e) = save_tour(&best_path, output_path) {
        eprintln!("Failed to save tour: {}", e);
    }

    best_distance
}
