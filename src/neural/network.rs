use nalgebra::{DMatrix, DVector};

use super::{ecuations::ErrorEcuation, layer::Layer};

pub struct Network {
    pub layers: Vec<Box<dyn Layer>>,
    pub learning_rate: f32,
}

pub struct NetworkError {
    pub gradients: Vec<DMatrix<f32>>,
    pub raw_output: DVector<f32>,
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

    pub fn calc_error<T: ErrorEcuation>(
        &self,
        inputs: &DVector<f32>,
        expected: &DVector<f32>,
    ) -> NetworkError {
        let mut layer_inputs = Vec::<DVector<f32>>::with_capacity(self.layers.len());
        layer_inputs.push(inputs.clone_owned());
        for (i, layer) in self.layers.iter().enumerate() {
            layer_inputs.push(layer.forward(&layer_inputs[i]));
        }
        let final_layer_index = self.layers.len() - 1;
        let output = layer_inputs.last().unwrap();
        let errors_deriv = T::deriv(output, expected);
        let mut layer_error = errors_deriv.as_slice().to_vec();
        let mut gradients = Vec::<DMatrix<f32>>::with_capacity(self.layers.len());
        for (i, layer) in self.layers.iter().rev().enumerate() {
            let propagate = layer.back(&layer_error, &layer_inputs[final_layer_index - i]);
            gradients.push(propagate.gradients);
            layer_error = propagate.errors;
        }
        NetworkError {
            gradients,
            raw_output: output.clone_owned(),
        }
    }

    pub fn update_weights(&mut self, gradients: &Vec<DMatrix<f32>>) -> Result<(), ()> {
        if gradients.len() != self.layers.len() {
            return Err(());
        }
        for (i, layer) in self.layers.iter_mut().enumerate() {
            layer.upate_weights(self.learning_rate, &gradients[gradients.len() - 1 - i]);
        }
        return Ok(());
    }

    pub fn sgd_epoch<T: ErrorEcuation>(
        &mut self,
        inputs: &Vec<DVector<f32>>,
        expected: &Vec<DVector<f32>>,
    ) -> Result<f32, ()> {
        if inputs.len() != expected.len() {
            return Err(());
        }
        let mut avg = 0.0;
        for (i, input) in inputs.iter().enumerate() {
            let error = self.calc_error::<T>(&input, &expected[i]);
            self.update_weights(&error.gradients)?;
            avg = T::calc(&error.raw_output, &expected[i]).sum() / error.raw_output.len() as f32;
        }
        avg /= inputs.len() as f32;
        Ok(avg)
    }

    pub fn mini_batch_epoch<T: ErrorEcuation>(
        &mut self,
        inputs: &Vec<DVector<f32>>,
        expected: &Vec<DVector<f32>>,
        batch_len: usize,
    ) -> Result<f32, ()> {
        if inputs.len() != expected.len() {
            return Err(());
        }
        let mut avg = 0.0;
        let mut errors = <Vec<DMatrix<f32>>>::new();
        for layer in self.layers.iter().rev() {
            errors.push(DMatrix::zeros(layer.len(), layer.inputs_len() + 1));
        }
        let mut example_counter = 1;
        for (i, input) in inputs.iter().enumerate() {
            let error = self.calc_error::<T>(&input, &expected[i]);
            for (i, gradient) in error.gradients.iter().enumerate() {
                errors[i] += gradient;
            }
            avg = T::calc(&error.raw_output, &expected[i]).sum() / error.raw_output.len() as f32;
            if example_counter == batch_len {
                for gradient in errors.iter_mut() {
                    gradient.scale_mut(1.0/batch_len as f32);
                }
                example_counter = 0;
                self.update_weights(&errors).expect("error");
                errors = <Vec<DMatrix<f32>>>::new();
                for layer in self.layers.iter().rev() {
                    errors.push(DMatrix::zeros(layer.len(), layer.inputs_len() + 1));
                }
            }
            example_counter += 1;
        }
        avg /= inputs.len() as f32;
        Ok(avg)
    }

    pub fn get_weights(&self) -> Vec<DMatrix<f32>> {
        let mut weights = Vec::<DMatrix<f32>>::with_capacity(self.layers.len());
        for layer in self.layers.iter() {
            weights.push(layer.get_weights());
        }
        weights
    }
}
