use nalgebra::{DMatrix, DVector};
use crate::neural::ecuations::softmax::Softmax;

use crate::neural::{ neuron::Neuron, BackpropagtionByproduct};
use super::Layer;

pub struct SoftMaxLayer{
    pub neurons: Vec<Neuron>,
    inputs: usize,
}

impl SoftMaxLayer{
    pub fn forward(&self, inputs: &DVector<f32>) -> DVector<f32> {
        return Softmax::calc(inputs);
    }

    pub fn back(&self, hot_one: &DVector<f32>, inputs: &DVector<f32>) -> Vec<f32> {
        let mut layer_errors = DMatrix::<f32>::zeros(self.neurons.len(), inputs.len()); 
        let neuron_error = Softmax::calc(inputs);
        let beta = neuron_error - hot_one;


        for (i, neuron) in self.neurons.iter().enumerate() {
            let neuron_error_with_w = &neuron.errors_times_weights(beta[i]);
            layer_errors.set_row(i, &neuron_error_with_w.transpose());
        }


        return layer_errors.row_sum().as_slice().to_vec();

    }

}