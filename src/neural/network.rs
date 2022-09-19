use nalgebra::DVector;

use super::{ecuations::ErrorEcuation, layer::Layer};

pub struct Network {
    pub layers: Vec<Box<dyn Layer>>,
    pub learning_rate: f32,
}

impl Network {
    pub fn new(learning_rate: f32) -> Self {
        Network {
            layers: Vec::new(),
            learning_rate,
        }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn full_forward(&self, inputs: &DVector<f32>) -> DVector<f32> {
        let mut output = inputs.clone();
        for layer in self.layers.iter() {
            output = layer.forward(&output);
        }
        output
    }

    pub fn epoch<T: ErrorEcuation>(
        &mut self,
        inputs: &DVector<f32>,
        expected: &DVector<f32>,
    ) -> DVector<f32> {
        let mut layer_inputs = Vec::<DVector<f32>>::with_capacity(self.layers.len());
        layer_inputs.push(inputs.clone_owned());
        for (i, layer) in self.layers.iter().enumerate() {
            layer_inputs.push(layer.forward(&layer_inputs[i]));
        }
        let final_layer_index = self.layers.len() - 1;
        let output = layer_inputs.last().unwrap();
        let errors = T::calc(output, expected);
        let errors_deriv = T::deriv(output, expected);
        let mut layer_error = errors_deriv.as_slice().to_vec();
        for (i, layer) in self.layers.iter_mut().rev().enumerate() {
            let propagate = layer.back(&layer_error, &layer_inputs[final_layer_index - i]);
            layer.upate_weights(self.learning_rate, &propagate.gradients);
            layer_error = propagate.errors;
        }
        errors
    }
}
