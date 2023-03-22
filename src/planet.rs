use nalgebra::Vector2;
// use num_bigfloat::BigFloat;
pub struct Planet {
    pub label: String,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub acceleration: Vector2<f64>,
    pub radius: f32,
    pub mass: f64,
    pub color: [f32; 4],
}
