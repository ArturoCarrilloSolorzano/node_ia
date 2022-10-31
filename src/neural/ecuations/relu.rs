use super::Ecuation;

pub struct ReLU {}

impl Ecuation for ReLU {
    fn calc(z: &nalgebra::DVector<f32>, index: u32) -> f32 {
        let sum = z.sum();
        if sum > 0.0 {
            sum
        } else {
            0.0
        }
    }
    fn derivate(z: &nalgebra::DVector<f32>, index: u32) -> f32 {
        let sum = z.sum();
        if sum > 0.0 {
            1.0
        } else {
            0.0
        }
    }
}
