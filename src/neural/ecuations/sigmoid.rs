use super::Ecuation;

pub struct Sigmoid {}

impl Ecuation for Sigmoid {
    fn calc(z: &nalgebra::DVector<f32>, _index: u32) -> f32 {
        let z_value = z.sum();
        1.0 / (1.0 + libm::expf(-1.0 * z_value))
    }

    fn derivate(z: &nalgebra::DVector<f32>, _index: u32) -> f32 {
        let z_value = z.sum();
        let sigmoid = 1.0 / (1.0 + libm::expf(-1.0 * z_value));
        sigmoid * (1.0 - sigmoid)
    }
}
