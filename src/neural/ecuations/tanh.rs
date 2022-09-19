extern crate libm;

use super::na;
use crate::neural::ecuations::Ecuation;
use libm::tanh;
use na::DVector;

#[derive(Clone)]
pub struct TanH {}

impl Ecuation for TanH {
    fn calc(z: &DVector<f32>, _index: u32) -> f32 {
        tanh(z.sum() as f64) as f32
    }

    fn derivate(z: &DVector<f32>, _index: u32) -> f32 {
        1.0 - tanh(z.sum() as f64).powi(2) as f32
    }
}
