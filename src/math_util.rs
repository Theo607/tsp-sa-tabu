use crate::io::City;
use rand::Rng;

pub fn create_distance_matrix(cities: &[City]) -> Vec<Vec<f64>> {
    let n = cities.len();
    let mut matrix = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in i + 1..n {
            let dx = (cities[i].x - cities[j].x) as f64;
            let dy = (cities[i].y - cities[j].y) as f64;
            let dist = (dx * dx + dy * dy).sqrt();
            matrix[i][j] = dist;
            matrix[j][i] = dist;
        }
    }
    matrix
}

pub fn calculate_total_distance(path: &[u16], dist_matrix: &Vec<Vec<f64>>) -> f64 {
    let mut total = 0.0;
    let n = path.len();
    for i in 0..n {
        let city_a = (path[i] - 1) as usize;
        let city_b = (path[(i + 1) % n] - 1) as usize;
        total += dist_matrix[city_a][city_b];
    }
    total
}

pub fn calculate_2opt_delta(
    path: &[u16],
    i: usize,
    j: usize,
    dist_matrix: &Vec<Vec<f64>>,
) -> f64 {
    let n = path.len();

    if n < 2 { return 0.0; }

    let idx_before_i = if i == 0 { n - 1 } else { i - 1 };
    let idx_after_j = (j + 1) % n;

    let city_before_i = (path[idx_before_i] - 1) as usize;
    let city_i = (path[i] - 1) as usize;
    let city_j = (path[j] - 1) as usize;
    let city_after_j = (path[idx_after_j] - 1) as usize;

    let old_dist = dist_matrix[city_before_i][city_i] + dist_matrix[city_j][city_after_j];
    
    let new_dist = dist_matrix[city_before_i][city_j] + dist_matrix[city_i][city_after_j];

    new_dist - old_dist
}

pub fn apply_2opt(path: &mut Vec<u16>, i: usize, j: usize) {
    path[i..=j].reverse();
}

pub fn get_random_indices(n: usize) -> (usize, usize) {
    let mut rng = rand::thread_rng();
    loop {
        let i = rng.gen_range(0..n);
        let j = rng.gen_range(0..n);
        
        let (start, end) = if i < j { (i, j) } else { (j, i) };
        
        if end - start >= 1 && end - start < n - 1 {
            return (start, end);
        }
    }
}
pub fn generate_random_path(n: usize) -> Vec<u16> {
    use rand::seq::SliceRandom;
    let mut path: Vec<u16> = (1..=(n as u16)).collect();
    let mut rng = rand::thread_rng();
    path.shuffle(&mut rng);
    path
}
