use crate::facets::heat::boundary::Boundary;
use crate::facets::heat::heat_source::HeatSource;
use crate::facets::heat::materials::material::Material;
use crate::facets::heat::numerical::root;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ThermalZone {
    boundaries: Vec<Boundary>,
    sources: Vec<HeatSource>,
    exterior_temperature: f64,
    interior_material: Material,
    zone_volume: f64,
}

impl ThermalZone {
    pub fn new(
        boundaries: Vec<Boundary>,
        sources: Vec<HeatSource>,
        outside_temperature: f64,
        interior_material: Material,
        zone_volume: f64,
    ) -> Self {
        ThermalZone {
            boundaries,
            sources,
            exterior_temperature: outside_temperature,
            interior_material,
            zone_volume,
        }
    }

    pub fn zone_mass(&self) -> f64 {
        self.zone_volume * self.interior_material.get_density()
    }

    pub fn a(&self) -> f64 {
        self.boundaries
            .iter()
            .map(|x| x.u_value() * x.area())
            .sum::<f64>()
            / (self.zone_mass() * self.interior_material.get_specific_heat_capacity())
    }

    pub fn b(&self) -> f64 {
        (self.sources.iter().map(|x| x.q()).sum::<f64>()
            + self
                .boundaries
                .iter()
                .map(|x| x.u_value() * x.area())
                .sum::<f64>()
                * self.exterior_temperature)
            / (self.zone_mass() * self.interior_material.get_specific_heat_capacity())
    }

    pub fn temperature(&self, initial_interior_temperature: f64, time: f64) -> f64 {
        self.b() / self.a()
            + (initial_interior_temperature - self.b() / self.a()) * f64::exp(-self.a() * time)
    }

    pub fn temperature_gradient(&self, initial_interior_temperature: f64, time: f64) -> f64 {
        -self.a()
            * f64::exp(-self.a() * time)
            * (initial_interior_temperature - self.b() / self.a())
    }

    pub fn time_to_temperature(&self, initial_interior_temperature: f64, temperature: f64) -> f64 {
        let answer = root(
            |time| self.temperature(initial_interior_temperature, time) - temperature,
            |time| self.temperature_gradient(initial_interior_temperature, time),
            initial_interior_temperature,
            1.0e-3,
            1000,
        )
        .unwrap();

        answer
    }
}
