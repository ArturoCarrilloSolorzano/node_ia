extern crate rand;
use super::{ecuations::Ecuation, na::DVector};
use rand::{prelude::Rng, thread_rng};

#[derive(Clone)]
pub struct Neuron {
    pub weights: DVector<f32>,
    pub index: u32,
}

impl Neuron {
    pub fn new(inputs: usize, index: u32) -> Self {
        let mut rng = thread_rng();
        return Neuron {
            weights: DVector::from_fn(inputs + 1, |r, _| -> f32 {
                if r == inputs {
                    return 0.0;
                }
                rng.gen_range(0.0..0.4)
            }),
            index,
        };
    }

    pub fn from_weights(weights: &DVector<f32>, index: u32) -> Self {
        Neuron {
            weights: weights.clone_owned(),
            index,
        }
    }

    pub fn from_vec(weights: Vec<f32>, index: u32) -> Self {
        Neuron {
            weights: DVector::from_vec(weights),
            index,
        }
    }

    fn extend_input_to_bias(inputs: &DVector<f32>) -> DVector<f32> {
        inputs.clone_owned().insert_row(inputs.len(), 1.0)
    }

    pub fn calc_z_vector(&self, inputs: &DVector<f32>) -> DVector<f32> {
        let inputs_with_bias = Self::extend_input_to_bias(inputs);
        let z_vector = inputs_with_bias.component_mul(&self.weights);
        z_vector
    }

    pub fn activate<T: Ecuation>(&self, inputs: &DVector<f32>) -> f32 {
        T::calc(&self.calc_z_vector(inputs), self.index)
    }

    pub fn errors_times_weights(&self, error: f32) -> DVector<f32> {
        let weights_no_bias = self
            .weights
            .clone_owned()
            .remove_row(self.weights.len() - 1);
        weights_no_bias.scale(error)
    }

    pub fn calc_error_deriv<T: Ecuation>(
        &self,
        inputs: &DVector<f32>,
        next_layer_error_deriv: f32,
    ) -> f32 {
        let z_vector = self.calc_z_vector(inputs);
        let a_deriv = T::derivate(&z_vector, self.index);
        next_layer_error_deriv * a_deriv
    }

    pub fn gradient(&self, inputs: &DVector<f32>, error_deriv: f32) -> DVector<f32> {
        let inputs_cloned = Self::extend_input_to_bias(inputs);
        inputs_cloned * error_deriv
    }

    pub fn update_weights(&mut self, learning_rate: f32, gradient: &DVector<f32>) {
        let delta = &gradient.scale(-1.0 * learning_rate);
        self.weights = self.weights.clone() + delta;
    }
}
