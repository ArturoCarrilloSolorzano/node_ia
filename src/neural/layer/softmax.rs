use crate::neural::ecuations::softmax::Softmax;
use nalgebra::DVector;

pub struct SoftMaxLayer {}

impl SoftMaxLayer {
    pub fn forward(inputs: &DVector<f32>) -> DVector<f32> {
        return Softmax::calc(inputs);
    }

    pub fn back(hot_one: &DVector<f32>, inputs: &DVector<f32>) -> Vec<f32> {
        let neuron_error = Softmax::calc(inputs);
        let beta = neuron_error - hot_one;

        let mut res = Vec::new();
        for err in beta.iter() {
            res.push(*err)
        }
        res
    }
}
