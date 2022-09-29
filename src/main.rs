extern crate libm;
use std::io::stdin;

use file::reader::FileOutput2;
use nalgebra::DVector;

use crate::neural::{
    ecuations::{squared::SquaredError, tanh::TanH},
    layer::BaseLayer,
    network::Network,
};

use self::file::{particioner, particioner2};

pub mod chart;
pub mod file;
pub mod neural;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    println!("Numero de iteraciones:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer)?;

    let max_iterations = match buffer.trim_end() {
        "" => Ok(50 as i64),
        value => value.parse::<i64>(),
    }
    .expect("input invalido");
    let mut buffer = String::new();
    println!("Error deseado:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer)?;

    let min_error = match buffer.trim_end() {
        "" => Ok(0.1 as f32),
        value => value.parse::<f32>(),
    }
    .expect("input invalido");
    let mut buffer = String::new();
    println!("Tasa de aprendizaje:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer)?;

    let learning_rate = match buffer.trim_end() {
        "" => Ok(0.5 as f32),
        value => value.parse::<f32>(),
    }
    .expect("input invalido");

    let data = file::reader::main2("spheres1d10.csv");

    let partitioned_data = particioner::main(data);

    for (i, partition) in partitioned_data.dataset[..5].iter().enumerate() {
        println!("\nPartición {}\n", i);
        full(
            min_error,
            max_iterations as u32,
            &partition.train,
            &partition.test,
            learning_rate,
            &format!("graphs/spheres1d10-{}.png", i),
        );
    }

    let data10 = file::reader::main2("spheres2d10.csv");
    let data50 = file::reader::main2("spheres2d50.csv");
    let data70 = file::reader::main2("spheres2d70.csv");

    let random_partition = particioner2::randomPartition(&data10);
    let first200_partition = particioner2::first200Partition(&data10);
    let last200_partition = particioner2::last200Partition(&data10);
    full(
        min_error,
        max_iterations as u32,
        &random_partition.train,
        &random_partition.test,
        learning_rate,
        "graphs/spheres2d10-random.png",
    );
    full(
        min_error,
        max_iterations as u32,
        &first200_partition.train,
        &first200_partition.test,
        learning_rate,
        "graphs/spheres2d10-first200.png",
    );
    full(
        min_error,
        max_iterations as u32,
        &last200_partition.train,
        &last200_partition.test,
        learning_rate,
        "graphs/spheres2d10-last200.png",
    );

    let first100_partition = particioner2::first100last100Partition(&data50);
    let on300_partition = particioner2::on300And700Partition(&data50);
    let on200_partition = particioner2::on200And800Partition(&data50);
    full(
        min_error,
        max_iterations as u32,
        &first100_partition.train,
        &first100_partition.test,
        learning_rate,
        "graphs/spheres2d50-first100.png",
    );
    full(
        min_error,
        max_iterations as u32,
        &on300_partition.train,
        &on300_partition.test,
        learning_rate,
        "graphs/spheres2d50-on300.png",
    );
    full(
        min_error,
        max_iterations as u32,
        &on200_partition.train,
        &on200_partition.test,
        learning_rate,
        "graphs/spheres2d50-on200.png",
    );

    let on0_partition = particioner2::on0And300Partition(&data70);
    let on100_partition = particioner2::on0And300Partition(&data70);
    let on800_partition = particioner2::on0And300Partition(&data70);
    let middle200_partition = particioner2::on0And300Partition(&data70);
    full(
        min_error,
        max_iterations as u32,
        &on0_partition.train,
        &on0_partition.test,
        learning_rate,
        "graphs/spheres2d70-on0.png",
    );
    full(
        min_error,
        max_iterations as u32,
        &on100_partition.train,
        &on100_partition.test,
        learning_rate,
        "graphs/spheres2d70-on100.png",
    );
    full(
        min_error,
        max_iterations as u32,
        &on800_partition.train,
        &on800_partition.test,
        learning_rate,
        "graphs/spheres2d70-on800.png",
    );
    full(
        min_error,
        max_iterations as u32,
        &middle200_partition.train,
        &middle200_partition.test,
        learning_rate,
        "graphs/spheres2d70-middle200.png",
    );
    Ok(())
}

fn full(
    min_error: f32,
    max_iterations: u32,
    input: &FileOutput2,
    test: &FileOutput2,
    learning_rate: f32,
    name: &str,
) {
    let network = train(min_error, max_iterations, input, learning_rate);
    test_and_chart(&network, name, test);
}
fn train(min_error: f32, max_iterations: u32, train: &FileOutput2, learning_rate: f32) -> Network {
    let mut network = Network {
        layers: vec![
            Box::new(BaseLayer::<TanH>::new(5, 3)),
            Box::new(BaseLayer::<TanH>::new(1, 5)),
        ],
        learning_rate,
    };
    let mut error_avg = 1.0;
    let mut gen = 0;
    while error_avg > min_error && gen < max_iterations {
        error_avg = 0.0;
        println!("----- Entrenando generación {} -----", gen);
        for (i, input) in train.inputs.iter().enumerate() {
            let (x, y, z) = input;
            let vector_input = DVector::from_vec(vec![x.to_owned(), y.to_owned(), z.to_owned()]);
            let vector_expected = DVector::from_vec(vec![train.expected[i]]);
            let error = network.epoch::<SquaredError>(&vector_input, &vector_expected);
            error_avg += error[0];
        }
        gen += 1;
        error_avg = error_avg / train.inputs.len() as f32;
        println!("\tError promedio: {}", error_avg);
    }
    network
}

fn test_and_chart(network: &Network, name: &str, test: &FileOutput2) {
    let mut scatter_positive: Vec<(f32, f32, f32)> = Vec::new();
    let mut scatter_negative: Vec<(f32, f32, f32)> = Vec::new();
    for input in test.inputs.iter() {
        let (x, y, z) = input;
        let vector_input = DVector::from_vec(vec![x.to_owned(), y.to_owned(), z.to_owned()]);
        let output = network.full_forward(&vector_input);
        if output[0] > 0.0 {
            scatter_positive.push(input.clone());
        } else {
            scatter_negative.push(input.clone());
        }
    }
    chart::main(scatter_positive, scatter_negative, name);
}
