use file::reader::{FileOutput, Reader};
use nalgebra::DVector;
use neural::network::classificator::MultiClassClassificator;

use crate::file::{particioner::random_partition, randomizer::no_duplicates_random};

pub mod chart;
pub mod file;
pub mod inputs;
pub mod neural;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lerning_rules = inputs::rules::main();

    let reader = Reader::new(4, 3);
    let data = reader.read("irisbin.csv")?;
    let partition = random_partition(&data, 0.8);

    let mut network = MultiClassClassificator::new(4, 3, &[125, 255], 0.001);
    test_and_chart(&network, "help.png", &partition.test);
    full(
        &mut network,
        lerning_rules.min_error,
        lerning_rules.max_iterations as u32,
        &partition.train,
        &partition.test,
        "help1.png",
    );

    Ok(())
}

fn full(
    network: &mut MultiClassClassificator,
    min_error: f32,
    max_iterations: u32,
    input: &FileOutput,
    test: &FileOutput,
    name: &str,
) {
    train(network, min_error, max_iterations, input);
    test_and_chart(network, name, test);
}

fn tanh_to_hotone(tanh: &Vec<f32>) -> DVector<f32> {
    let mut vector = DVector::<f32>::zeros(tanh.len());
    for (index, value) in tanh.iter().enumerate() {
        if *value > 0.0 {
            vector[index] = 1.0;
        }
    }
    vector
}

fn separate_data(data: FileOutput) -> (Vec<DVector<f32>>, Vec<DVector<f32>>) {
    let mut inputs = Vec::<DVector<f32>>::new();
    let mut expected = Vec::<DVector<f32>>::new();
    for record in data.iter() {
        inputs.push(DVector::from_vec(record.inputs.clone()));
        expected.push(tanh_to_hotone(&record.expected));
    }
    (inputs, expected)
}

fn train(
    network: &mut MultiClassClassificator,
    min_error: f32,
    max_iterations: u32,
    train: &FileOutput,
) {
    let mut error_avg = 1.0;
    let mut gen = 0;
    while error_avg > min_error && gen < max_iterations {
        let random = no_duplicates_random(train);
        let (inputs, expected) = separate_data(random);
        println!("----- Entrenando generación {} -----", gen);
        error_avg = network.sgd_epoch(&inputs, &expected).expect("error");
        gen += 1;
        println!("\tError promedio: {}", error_avg);
    }
}

fn hotone_to_class(hotone: &Vec<f32>) -> usize {
    let mut class: usize = 0;
    for (index, item) in hotone.iter().enumerate() {
        if hotone[class] < *item {
            class = index;
        }
    }
    class
}

fn test_and_chart(network: &MultiClassClassificator, name: &str, test: &FileOutput) {
    let mut map = Vec::with_capacity(network.num_classes);
    for _ in 0..network.num_classes {
        let row = Vec::from_iter(std::iter::repeat(0).take(network.num_classes));
        map.push(row);
    }
    let mut right_predicctions = 0;
    for record in test.iter() {
        let prediction = network.full_forward(&DVector::from_vec(record.inputs.clone()));
        let predicted_class = hotone_to_class(&prediction.as_slice().to_vec());
        let acctual_class = hotone_to_class(&record.expected);
        if acctual_class == predicted_class {
            right_predicctions += 1;
        }
        map[acctual_class][predicted_class] += 1;
    }
    println!(
        "Porcentage de acierto {}",
        right_predicctions as f32 / test.len() as f32
    );
    chart::main(&map, name);
}
