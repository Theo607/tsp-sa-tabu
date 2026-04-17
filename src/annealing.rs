use std::f64;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct SimulationParameters {
    pub size: u64,

    pub base_temperature_k: f64,
    pub epoch_k: f64,
    pub tau_k: f64,
    pub epsilon: f64,
    pub min_delta: f64,
}

impl SimulationParameters {
    pub const fn new(size: u64) -> Self {
        Self {
            size,
            base_temperature_k: 100.0, 
            epoch_k: 500.0,            
            tau_k: 1.0,
            epsilon: 1e-7,            
            min_delta: -1e-9,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationConstants {
    pub size: u64,

    pub base_temperature: f64,

    pub epoch_min: u64,
    pub epoch_max: u64,
    pub epoch_range: u64,

    pub tau: f64,
    pub temp_constant : f64,
    pub min_delta : f64,
    pub epsilon : f64
}

impl SimulationConstants {
    pub fn from(p: &SimulationParameters) -> Self {
        let n = p.size as f64;

        let base_temperature = p.base_temperature_k * n.ln();

        let epoch_min = (p.epoch_k * n) as u64;
        let epoch_max = (5.0 * p.epoch_k * n) as u64;
        let epoch_range = epoch_max - epoch_min;

        let tau = p.tau_k * base_temperature;
        let temp_constant = (-1.0)/n.ln();
        let min_delta = p.min_delta;
        let epsilon = p.epsilon;

        Self {
            size: p.size,
            base_temperature,
            epoch_min,
            epoch_max,
            epoch_range,
            tau,
            temp_constant,
            min_delta,
            epsilon
        }
    }
}

pub fn update_temperature(
    temperature: &mut f64,
    sim_consts: &SimulationConstants,
) {
    *temperature *= sim_consts.temp_constant.exp();
}

pub fn epoch_length(
    temperature : f64, 
    c : &SimulationConstants
    ) -> u64 {
    let tau = c.tau;
    let n_min = c.epoch_min as f64;
    let n_delta = c.epoch_range as f64;

    let x = temperature / tau;
    let weight = 1.0 / (1.0 + x);

    (n_min + n_delta * weight) as u64
}

pub fn accept(
    delta : f64,
    temperature: f64, 
    c : &SimulationConstants
    ) -> bool {
    if delta <= c.min_delta {
        return true;
    } 
    if temperature <= 1e-12 {
        return false;
    }

    let p = (-delta / temperature).exp();
    rand::random::<f64>() < p
}

pub fn should_stop(
    temp: f64,
    avg_delta: f64,
    no_improve_epochs: u64,
    p: &SimulationConstants,
) -> bool {
    temp < 0.001 || no_improve_epochs >= 50
}
