extern crate libm;

use super::na;
use libm::exp;
use libm::log;

use na::DMatrix;
use na::DVector;

#[derive(Clone)]
pub struct Softmax {}

impl Softmax {
    pub fn calc(z: &DVector<f32>) -> DVector<f32> {
        let mut matrix = z.clone();
        let mut m;
        let mut sum;
        let constant;

        m = matrix[0];
        for (i, _) in matrix.iter().enumerate() {
            if m < matrix[i] {
                m = matrix[i];
            }
        }

        sum = 0.0;
        for (i, _) in matrix.iter().enumerate() {
            sum += exp((matrix[i] - m) as f64) as f32
        }

        constant = m + log(sum as f64) as f32;
        for value in matrix.iter_mut() {
            *value = exp((*value - constant) as f64) as f32;
        }

        return matrix;
    }

    pub fn derivate(z: &DVector<f32>) -> DMatrix<f32> {
        let matrix = Softmax::calc(z);
        let mut square_matrix = DMatrix::zeros(matrix.len(), matrix.len());

        for x in 0..matrix.len() {
            for y in 0..matrix.len() {
                let sigmoid = if x == y { 1 } else { 0 };
                square_matrix[(x, y)] = matrix[x] * (sigmoid as f32 - matrix[y]);
            }
        }

        return square_matrix;
    }
}
