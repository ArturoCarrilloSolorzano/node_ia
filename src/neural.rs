use na::DMatrix;

extern crate nalgebra as na;

pub mod ecuations;
pub mod layer;
pub mod network;
pub mod neuron;

#[derive(Clone)]
pub struct BackpropagtionByproduct {
    gradients: DMatrix<f32>,
    errors: Vec<f32>,
}
