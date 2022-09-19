extern crate libm;

use nalgebra::DVector;

use crate::neural::{
    ecuations::{squared::SquaredError, tanh::TanH, ErrorEcuation},
    layer::BaseLayer,
    network::Network,
};

pub mod neural;
pub mod file;

fn main() {
    let reader = file::reader::main();
    // Ejemplo compuerta OR
    let mut network = Network {
        layers: vec![Box::new(BaseLayer::<TanH>::new(1, 2))],
        learning_rate: 0.4,
    };

    /*
        let inputs = vec![
        vec![0.0, 0.0],
        vec![0.0, 1.0],
        vec![1.0, 0.0],
        vec![1.0, 1.0],
    ]; */

    //let expected = vec![vec![-1.0], vec![1.0], vec![1.0], vec![1.0]];

    let mut error_avg = 1.0;
    while error_avg > 0.001 {
        error_avg = 0.0;
        for (i, input) in reader.inputs.iter().enumerate() {
            let vector_input = DVector::from_vec(input.clone());
            let vector_expected = DVector::from_vec(reader.expected[i].clone());
            let output = network.full_forward(&vector_input);
            let error = SquaredError::calc(&output, &vector_expected)[0];
            error_avg += error;
            println!(
                "got {} expected {}, error {}",
                output[0], vector_expected[0], error
            );
            network.epoch::<SquaredError>(&vector_input, &vector_expected);
        }
        error_avg = error_avg / reader.inputs.len() as f32;
    }

    let vector_input = DVector::from_vec(vec![0.0, 0.0]);
    let vector_expected = DVector::from_vec(vec![-1.0]);
    let output = network.full_forward(&vector_input);
    println!("got {} expected {}", output[0], vector_expected[0]);
}
