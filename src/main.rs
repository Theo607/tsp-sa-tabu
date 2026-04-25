mod io;
mod annealing;
mod math_util;
mod simulate_annealing;
mod tabu_search;

use io::{parse_tsp, FILE_PATH_IN, FILE_PATH_OUT, FILE_NAMES};
use simulate_annealing::run_simulation;
use tabu_search::{run_tabu_search, TabuParameters};
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!("Rozpoczynanie symulacji (Simulated Annealing & Tabu Search)...");
    let start_time = Instant::now();

    FILE_NAMES.into_par_iter().for_each(|file_name| {
        let input_path = format!("{}{}", FILE_PATH_IN, file_name);
        // Usuwamy rozszerzenie .tsp z nazwy instancji dla czystszego nazewnictwa plików wyjściowych
        let instance_pure_name = file_name.replace(".tsp", "");
        
        if let Ok(cities) = parse_tsp(&input_path) {
            let n = cities.len();

            if true {
                println!(
                    "Wątek {:?}: Przetwarzanie {} (Rozmiar: {})", 
                    std::thread::current().id(), 
                    instance_pure_name, 
                    n
                );

                // --- 1. SIMULATED ANNEALING ---
                let ann_output = format!("{}BT_ANN_{}.tour", FILE_PATH_OUT, instance_pure_name);
                let ann_dist = run_simulation(&cities, &ann_output);
                
                // --- 2. TABU SEARCH ---
                let tabu_output = format!("{}BT_TABU_{}.tour", FILE_PATH_OUT, instance_pure_name);
                
                let tabu_params = TabuParameters {
                    max_iterations: 10000, 
                    tabu_tenure: (n as f64 * 0.2) as usize, 
                    neighborhood_size: (n * 2).min(500),   
                };
                
                let tabu_dist = run_tabu_search(&cities, &tabu_output, &tabu_params);
                
                println!(
                    "Zakończono {}: ANN = {:.2}, TABU = {:.2}", 
                    instance_pure_name, ann_dist, tabu_dist
                );
            } else {
                println!("Pominięto {}: Zbyt duży rozmiar ({})", instance_pure_name, n);
            }
        } else {
            eprintln!("Nie można otworzyć pliku: {}", input_path);
        }
    });

    let duration = start_time.elapsed();
    println!("---");
    println!("Wszystkie symulacje zakończone w czasie: {:?}", duration);
}
