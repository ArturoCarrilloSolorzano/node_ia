extern crate libm;

use nalgebra::DVector;

use crate::neural::{
    ecuations::{squared::SquaredError, tanh::TanH, ErrorEcuation},
    layer::BaseLayer,
    network::Network,
};

pub mod file;
pub mod neural;

fn main() {
    let reader = file::reader::main("XOR_trn.csv");
    // Ejemplo compuerta OR
    let mut network = Network {
        layers: vec![
            Box::new(BaseLayer::<TanH>::new(2, 2)),
            Box::new(BaseLayer::<TanH>::new(1, 2)),
        ],
        learning_rate: 0.76,
    };

    let mut error_avg = 1.0;
    let mut gen = 1;
    while error_avg > 0.0001 {
        error_avg = 0.0;
        println!("----- Entrenando generación {} -----", gen);
        for (i, input) in reader.inputs.iter().enumerate() {
            let vector_input = DVector::from_vec(input.clone());
            let vector_expected = DVector::from_vec(reader.expected[i].clone());
            let output = network.full_forward(&vector_input);
            let error = SquaredError::calc(&output, &vector_expected)[0];
            error_avg += error;
            network.epoch::<SquaredError>(&vector_input, &vector_expected);
        }
        gen += 1;
        error_avg = error_avg / reader.inputs.len() as f32;
        println!("\tError promedio: {}", error_avg);
    }

    let vector_input = DVector::from_vec(vec![-1.0, -1.0]);
    let vector_expected = DVector::from_vec(vec![-1.0]);
    let output = network.full_forward(&vector_input);
    println!("got {} expected {}", output[0], vector_expected[0]);
}
