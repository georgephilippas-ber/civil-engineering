use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PlaneVector {
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlanePolygon {
    points: Vec<PlaneVector>,
}

impl PlanePolygon {
    pub fn new(points: Vec<PlaneVector>) -> Self {
        Self { points }
    }

    pub fn area(&self) -> f64 {
        let vertices = self.points.len();

        let mut cumulative_sum_ = 0.;
        for vertex in 0..vertices {
            let current_vertex = &self.points[vertex];
            let next_vertex = &self.points[(vertex + 1) % vertices];

            cumulative_sum_ += current_vertex.x * next_vertex.y - next_vertex.x * current_vertex.y;
        }

        (cumulative_sum_ / 2.).abs()
    }

    pub fn rectangle(width: f64, height: f64) -> Self {
        Self::new(vec![
            PlaneVector { x: 0.0, y: 0.0 },
            PlaneVector { x: width, y: 0.0 },
            PlaneVector {
                x: width,
                y: height,
            },
            PlaneVector { x: 0.0, y: height },
        ])
    }

    pub fn regular_polygon(sides: usize, radius: f64) -> Self {
        Self::new(
            (0..sides)
                .map(|i| {
                    let angle = i as f64 * 2.0 * std::f64::consts::PI / sides as f64;

                    PlaneVector {
                        x: radius * angle.cos(),
                        y: radius * angle.sin(),
                    }
                })
                .collect(),
        )
    }

    pub fn scale(&mut self, factor: f64) {
        for vector_ in &mut self.points {
            vector_.x *= factor;
            vector_.y *= factor;
        }
    }
}
