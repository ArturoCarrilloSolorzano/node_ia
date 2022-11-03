pub mod tanh;
pub mod squared;
pub mod sigmoid;
pub mod relu;
pub mod softmax;
pub mod crossentropy;
use na::DVector;

use super::na;
pub trait Ecuation {
    fn calc(z: &DVector<f32>, index: u32) -> f32;
    fn derivate(z: &DVector<f32>, index: u32) -> f32;
}

pub trait ErrorEcuation {
    fn calc(output: &DVector<f32>, expected: &DVector<f32>) -> DVector<f32>;
    fn deriv(output: &DVector<f32>, expected: &DVector<f32>) -> DVector<f32>;
}
