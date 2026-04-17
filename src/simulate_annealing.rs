use crate::io::{City, save_tour};
use crate::math_util::*;
use crate::annealing::*;

pub fn run_simulation(cities: &[City], output_path: &str) -> f64 {
    let n = cities.len();
    if n == 0 { return 0.0; }

    let params = SimulationParameters::new(n as u64);
    let constants = SimulationConstants::from(&params);
    let dist_matrix = create_distance_matrix(cities);

    let mut current_path = generate_random_path(n);
    let mut current_distance = calculate_total_distance(&current_path, &dist_matrix);

    let mut best_path = current_path.clone();
    let mut best_distance = current_distance;

    let mut temperature = constants.base_temperature;
    let mut no_improve_epochs = 0;
    let mut total_iterations = 0;

    println!("Starting SA for {} cities. Initial distance: {:.2}", n, current_distance);

    while !should_stop(temperature, current_distance, no_improve_epochs, &constants) {
        let mut improved_in_epoch = false;
        let current_epoch_len = epoch_length(temperature, &constants);

        for _ in 0..current_epoch_len {
            let (i, j) = get_random_indices(n);
            
            let delta = calculate_2opt_delta(&current_path, i, j, &dist_matrix);

            if accept(delta, temperature, &constants) {
                apply_2opt(&mut current_path, i, j);
                current_distance += delta;

                if current_distance < best_distance {
                    best_distance = current_distance;
                    best_path = current_path.clone();
                    improved_in_epoch = true;
                }
            }
            total_iterations += 1;
        }

        if improved_in_epoch {
            no_improve_epochs = 0;
        } else {
            no_improve_epochs += 1;
        }

        update_temperature(&mut temperature, &constants);
    }

    println!("Simulation finished. Best distance: {:.2} (Total iterations: {})", best_distance, total_iterations);

    if let Err(e) = save_tour(&best_path, output_path) {
        eprintln!("Failed to save tour: {}", e);
    }

    best_distance
}
