use crate::facets::heat::geometry::PlanePolygon;
use crate::facets::heat::layer::Layer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Boundary {
    geometry: PlanePolygon,
    layers: Vec<Layer>,
    openings: Vec<Boundary>,
}

impl Boundary {
    pub fn new(geometry: PlanePolygon, layers: Vec<Layer>) -> Self {
        Self {
            geometry,
            layers,
            openings: vec![],
        }
    }

    pub fn add_opening(&mut self, opening: Boundary) {
        self.openings.push(opening);
    }

    pub fn thickness(&self) -> f64 {
        self.layers.iter().map(|layer| layer.get_thickness()).sum()
    }

    pub fn area(&self) -> f64 {
        self.geometry.area()
    }

    pub fn net_area(&self) -> f64 {
        self.geometry.area()
            - self
                .openings
                .iter()
                .map(|opening| opening.net_area())
                .sum::<f64>()
    }

    pub fn openings_area(&self) -> f64 {
        self.openings.iter().map(|opening| opening.net_area()).sum()
    }

    pub fn r_value(&self) -> f64 {
        (self
            .layers
            .iter()
            .map(|layer| layer.get_thermal_resistance())
            .sum::<f64>()
            * self.net_area()
            + self.openings_area() * self.openings.iter().map(|x| x.r_value()).sum::<f64>())
            / (self.net_area() + self.openings_area())
    }

    // U-value W / ( m ^ 2 * K )
    pub fn u_value(&self) -> f64 {
        1.0 / self.r_value()
    }

    pub fn q(&self, delta_temperature: f64) -> f64 {
        self.area() * self.u_value() * delta_temperature
    }
}
