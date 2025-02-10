use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HeatSource {
    q: f64,
}

impl HeatSource {
    pub fn new(q: f64) -> Self {
        Self { q }
    }

    pub fn q(&self) -> f64 {
        self.q
    }
}
