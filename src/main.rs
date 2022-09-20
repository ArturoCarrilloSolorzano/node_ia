extern crate libm;
use std::io::stdin;

use nalgebra::DVector;

use crate::neural::{
    ecuations::{squared::SquaredError, tanh::TanH, ErrorEcuation},
    layer::BaseLayer,
    network::Network,
};

pub mod file;
pub mod chart;
pub mod neural;

fn main() {
    // numero de iteraciones, o error, learning rate

    let mut buffer = String::new();
    println!("Numero de iteraciones:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer);


    let max_iterations =  match buffer.trim_end() {
        "" =>  Ok(50 as i64),
        value => value.parse::<i64>(),
    }.expect("input invalido");
    let mut buffer = String::new();
    println!("Error deseado:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer);


    let min_error =  match buffer.trim_end() {
        "" =>  Ok(0.0001 as f32),
        value => value.parse::<f32>(),
    }.expect("input invalido");
    let mut buffer = String::new();
    println!("Tasa de aprendizaje:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer);


    let learning_rate =  match buffer.trim_end() {
        "" =>  Ok(0.76 as f32),
        value => value.parse::<f32>(),
    }.expect("input invalido");


    let train_reader = file::reader::main("XOR_trn.csv");
    let test_reader = file::reader::main("XOR_tst.csv");
    let mut scatter_positive:Vec<(f32, f32)> = Vec::new();
    let mut scatter_negative:Vec<(f32, f32)> = Vec::new();
    // Ejemplo compuerta OR
    let mut network = Network {
        layers: vec![
            Box::new(BaseLayer::<TanH>::new(2, 2)),
            Box::new(BaseLayer::<TanH>::new(1, 2)),
        ],
        learning_rate: learning_rate,
    };

    let mut error_avg = 1.0;
    let mut gen = 1;
    while error_avg > min_error && gen < max_iterations{
        error_avg = 0.0;
        println!("----- Entrenando generación {} -----", gen);
        for (i, input) in train_reader.inputs.iter().enumerate() {
            let (x,y) = input;
            let vector_input = DVector::from_vec(vec![x.to_owned(),y.to_owned()]);
            let vector_expected = DVector::from_vec(vec![train_reader.expected[i]]);
            let error = network.epoch::<SquaredError>(&vector_input, &vector_expected);
            error_avg += error[0];
        }
        gen += 1;
        error_avg = error_avg / train_reader.inputs.len() as f32;
        println!("\tError promedio: {}", error_avg);
    }

    for input in test_reader.inputs.iter() {
        let (x,y) = input;
        let vector_input = DVector::from_vec(vec![x.to_owned(),y.to_owned()]);
        let output = network.full_forward(&vector_input);
        if output[0] > 0.0{
            scatter_positive.push(input.clone());
        }
        else{
            scatter_negative.push(input.clone());
        }
    }
    println!("\tError promedio: {}", error_avg);

    chart::main(scatter_positive, scatter_negative);
}
