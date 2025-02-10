use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    name: String,
    density: f64,
    thermal_conductivity: f64,
    specific_heat_capacity: f64,
}

impl Material {
    pub fn new(
        name: String,
        density: f64,
        thermal_conductivity: f64,
        specific_heat_capacity: f64,
    ) -> Self {
        Self {
            name,
            density,                // ρ
            thermal_conductivity,   // λ
            specific_heat_capacity, // c
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_density(&self) -> f64 {
        self.density
    }

    pub fn get_thermal_conductivity(&self) -> f64 {
        self.thermal_conductivity
    }

    pub fn get_specific_heat_capacity(&self) -> f64 {
        self.specific_heat_capacity
    }
}
