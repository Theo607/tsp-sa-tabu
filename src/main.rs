mod io;
mod annealing;
mod math_util;
mod simulate_annealing;

use io::{parse_tsp, FILE_PATH_IN, FILE_PATH_OUT, FILE_NAMES};
use simulate_annealing::run_simulation;
use rayon::prelude::*;
use std::time::Instant;

fn main() {
    println!("Rozpoczynanie symulacji dla plików poniżej 1000 miast...");
    let start_time = Instant::now();

    FILE_NAMES.into_par_iter().for_each(|file_name| {
        let input = format!("{}{}", FILE_PATH_IN, file_name);
        
        if let Ok(cities) = parse_tsp(&input) {
            let n = cities.len();

            if n <= 5000 {
                let output = format!("{}{}.tour", FILE_PATH_OUT, file_name);
                
                println!(
                    "Wątek {:?}: Przetwarzanie {} (Rozmiar: {})", 
                    std::thread::current().id(), 
                    file_name, 
                    n
                );

                let best_dist = run_simulation(&cities, &output);
                
                println!("Zakończono {}: Najlepszy dystans = {:.2}", file_name, best_dist);
            } else {
                println!("Pominięto {}: Zbyt duży rozmiar ({})", file_name, n);
            }
        } else {
            eprintln!("Nie można otworzyć pliku: {}", input);
        }
    });

    let duration = start_time.elapsed();
    println!("---");
    println!("Wszystkie zadania zakończone w czasie: {:?}", duration);
}
