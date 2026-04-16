use std::f64;

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
    pub fn new(size: u64) -> Self {
        Self {
            size,
            base_temperature_k: 0.5,
            epoch_k: 0.3,
            tau_k: 0.1,
            epsilon: 0.001,
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
}

impl SimulationConstants {
    pub fn from(p: &SimulationParameters) -> Self {
        let n = p.size as f64;

        let base_temperature = p.base_temperature_k * n.ln();

        let epoch_min = (p.epoch_k * n) as u64;
        let epoch_max = (5.0 * p.epoch_k * n) as u64;
        let epoch_range = epoch_max - epoch_min;

        let tau = p.tau_k * base_temperature;

        Self {
            size: p.size,
            base_temperature,
            epoch_min,
            epoch_max,
            epoch_range,
            tau,
        }
    }
}
