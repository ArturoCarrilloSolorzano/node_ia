extern crate libm;

use file::reader::{FileOutput, FileOutput2};
use nalgebra::DVector;
use neural::layer::Layer;

use crate::neural::{
    ecuations::{squared::SquaredError, tanh::TanH},
    layer::BaseLayer,
    network::Network,
};

use self::file::{particioner, particioner2};

pub mod chart;
pub mod file;
pub mod inputs;
pub mod neural;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lerning_rules = inputs::rules::main();
    let network_size = inputs::neural_config::main();

    let train_data = file::reader::main("concentlite.csv");
    let _test_data = file::reader::main("concentlite_tst.csv");
    let mut layers = Vec::<Box<dyn Layer>>::with_capacity(network_size.len()+1);
    let mut input_len = 2;
    let final_out_len = 1;

    for size in network_size.iter() {
        layers.push(Box::from(BaseLayer::<TanH>::new(*size, input_len)));
        input_len = *size;
    }
    layers.push(Box::from(BaseLayer::<TanH>::new(final_out_len, input_len)));

    let mut network = Network{
        layers,
        learning_rate: lerning_rules.learning_rate
    };

    full(
        &mut network,
        lerning_rules.min_error,
        lerning_rules.max_iterations as u32,
        &train_data,
        &train_data,
        &format!("graphs/concentile.png"),
    );

    Ok(())
}

fn full(
    network: &mut Network,
    min_error: f32,
    max_iterations: u32,
    input: &FileOutput,
    test: &FileOutput,
    name: &str,
) {
    train(network, min_error, max_iterations, input);
    test_and_chart(network, name, test);
}

fn train(network: &mut Network, min_error: f32, max_iterations: u32, train: &FileOutput) {
    let mut inputs = Vec::<DVector<f32>>::new();
    let mut expected = Vec::<DVector<f32>>::new();
    for (i, input) in train.inputs.iter().enumerate() {
        let (x, y) = input;
        inputs.push(DVector::from_vec(vec![x.to_owned(), y.to_owned()]));
        expected.push(DVector::from_vec(vec![train.expected[i]]));
    }
    let mut error_avg = 1.0;
    let mut gen = 0;
    while gen < max_iterations {
        println!("----- Entrenando generación {} -----", gen);
       // error_avg = network
       //     .mini_batch_epoch::<SquaredError>(&inputs, &expected, 32)
       //     .expect("error");
        error_avg = network
            .sgd_epoch::<SquaredError>(&inputs, &expected)
            .expect("error");
        gen += 1;
        println!("\tError promedio: {}", error_avg);
    }
}

fn test_and_chart(network: &Network, name: &str, test: &FileOutput) {
    let mut scatter_positive: Vec<(f32, f32)> = Vec::new();
    let mut scatter_negative: Vec<(f32, f32)> = Vec::new();
    for input in test.inputs.iter() {
        let (x, y) = input;
        let vector_input = DVector::from_vec(vec![x.to_owned(), y.to_owned()]);
        let output = network.full_forward(&vector_input);
        println!("{}", output[0]);
        if output[0] > 0.0 {
            scatter_positive.push(input.clone());
        } else {
            scatter_negative.push(input.clone());
        }
    }
    chart::main(scatter_positive, scatter_negative, name);
}
