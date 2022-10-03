extern crate dyn_clone;
use std::marker::PhantomData;

use nalgebra::{DMatrix, DVector};

use super::{ecuations::Ecuation, neuron::Neuron, BackpropagtionByproduct};

pub trait Layer {
    fn forward(&self, inputs: &DVector<f32>) -> DVector<f32>;
    fn back(&self, errors: &Vec<f32>, inputs: &DVector<f32>) -> BackpropagtionByproduct;
    fn upate_weights(&mut self, learning_rate: f32, gradients: &DMatrix<f32>);
    fn len(&self) -> usize;
    fn inputs_len(&self) -> usize;
    fn get_weights(&self) -> DMatrix<f32>;
}

#[derive(Clone)]
pub struct BaseLayer<T: Ecuation> {
    pub neurons: Vec<Neuron>,
    inputs: usize,
    phantom_type: PhantomData<T>,
}

impl<T: Ecuation> BaseLayer<T> {
    pub fn new(nodes: usize, inputs: usize) -> Self {
        let mut layer = BaseLayer {
            neurons: Vec::with_capacity(nodes),
            phantom_type: PhantomData,
            inputs,
        };
        for i in 0..nodes {
            layer.neurons.push(Neuron::new(inputs, i as u32));
        }
        layer
    }

    pub fn from_neurons(neurons: &Vec<Neuron>) -> Self {
        let first = neurons.first().unwrap();
        BaseLayer {
            neurons: neurons.to_vec(),
            inputs: first.weights.len(),
            phantom_type: PhantomData,
        }
    }

    pub fn from_matrix(weights: &DMatrix<f32>) -> Self {
        let mut neurons = Vec::<Neuron>::with_capacity(weights.nrows());
        for (i, row) in weights.row_iter().enumerate() {
            neurons.push(Neuron::from_weights(&row.transpose(), i as u32));
        }
        let first_len = neurons.first().unwrap().weights.len();
        BaseLayer {
            neurons,
            inputs: first_len,
            phantom_type: PhantomData,
        }
    }
}

impl<T: Ecuation> Layer for BaseLayer<T> {
    fn forward(&self, inputs: &DVector<f32>) -> DVector<f32> {
        let mut activations = DVector::zeros(self.neurons.len());
        for (i, neuron) in self.neurons.iter().enumerate() {
            activations[i] = neuron.activate::<T>(inputs);
        }
        activations
    }

    fn back(&self, errors: &Vec<f32>, inputs: &DVector<f32>) -> BackpropagtionByproduct {
        let mut gradients = DMatrix::<f32>::zeros(self.neurons.len(), inputs.len() + 1);
        let mut layer_errors = DMatrix::<f32>::zeros(self.neurons.len(), inputs.len());
        for (i, neuron) in self.neurons.iter().enumerate() {
            let next_layer_error_deriv = errors[i];
            let neuron_error = neuron.calc_error_deriv::<T>(inputs, next_layer_error_deriv);
            let gradient = &neuron.gradient(inputs, neuron_error);
            let neuron_error_with_w = &neuron.errors_times_weights(neuron_error);
            gradients.set_row(i, &gradient.transpose());
            layer_errors.set_row(i, &neuron_error_with_w.transpose());
        }
        let added_errors = layer_errors.row_sum().as_slice().to_vec();
        BackpropagtionByproduct {
            gradients,
            errors: added_errors,
        }
    }

    fn upate_weights(&mut self, learning_rate: f32, gradients: &DMatrix<f32>) {
        for (i, neuron) in self.neurons.iter_mut().enumerate() {
            neuron.update_weights(learning_rate, &gradients.row(i).transpose());
        }
    }

    fn len(&self) -> usize {
        self.neurons.len()
    }

    fn get_weights(&self) -> DMatrix<f32> {
        let mut weights = DMatrix::<f32>::zeros(self.neurons.len(), self.neurons[0].weights.len());
        for (i, neuron) in self.neurons.iter().enumerate() {
            weights.set_row(i, &neuron.weights.transpose());
        }
        weights
    }
    fn inputs_len(&self) -> usize {
        self.inputs
   }
}
