use material::Material;
use once_cell::sync::Lazy;

pub mod material;

pub static CONCRETE: Lazy<Material> =
    Lazy::new(|| Material::new("concrete".to_string(), 2400.0, 1.75, 0.84 * 1.0e3));
pub static BRICK: Lazy<Material> =
    Lazy::new(|| Material::new("brick".to_string(), 1920.0, 0.9, 0.84 * 1.0e3));
pub static POLYSTYRENE: Lazy<Material> =
    Lazy::new(|| Material::new("polystyrene".to_string(), 25.0, 0.035, 1.3 * 1.0e3));
pub static AIR: Lazy<Material> = Lazy::new(|| Material::new("air".to_string(), 1.2, 0.024, 1.0e3));
pub static VOID: Lazy<Material> =
    Lazy::new(|| Material::new("void".to_string(), 0.0, f64::INFINITY, 0.0));

pub fn all() -> Vec<Material> {
    vec![
        CONCRETE.clone(),
        BRICK.clone(),
        POLYSTYRENE.clone(),
        AIR.clone(),
        VOID.clone(),
    ]
}
