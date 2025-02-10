use crate::facets::heat::materials::material::Material;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    material: Material,
    thickness: f64,
}

impl Layer {
    pub fn new(material: Material, thickness: f64) -> Self {
        Self {
            material,
            thickness,
        }
    }
    pub fn get_material(&self) -> &Material {
        &self.material
    }
    pub fn get_thickness(&self) -> f64 {
        self.thickness
    }

    pub fn get_thermal_resistance(&self) -> f64 {
        self.thickness / self.material.get_thermal_conductivity()
    }
}
