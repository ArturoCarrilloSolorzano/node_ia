use super::{writer::file_output, reader::FileOutput2};
use rand::Rng;

pub struct  Partition{
    pub train: FileOutput2,
    pub test: FileOutput2
}
#[derive(Default)]
pub struct Datasets {
    pub dataset: Vec<Partition>
}



pub fn main (file_output: FileOutput2)-> Datasets{

    let mut data: Datasets = Datasets::default();

    data.dataset.push(randomPartition(&file_output));
    data.dataset.push(first200Partition(&file_output));
    data.dataset.push(last200Partition(&file_output));
    data.dataset.push(first100last100Partition(&file_output));
    data.dataset.push(on300And700Partition(&file_output));
    data.dataset.push(on200And800Partition(&file_output));
    data.dataset.push(on0And300Partition(&file_output));
    data.dataset.push(on100And400Partition(&file_output));
    data.dataset.push(on800And900Partition(&file_output));
    data.dataset.push(middle200Partition(&file_output));

    return  data;
    
}


pub fn randomPartition(file_output: &FileOutput2) -> Partition{
    let mut rng = rand::thread_rng();
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    let mut randomNum: i8;

    for (index, value) in file_output.inputs.iter().enumerate(){
        if inputsTestValues.len() < 1000 && inputsTrainValues.len() < 4000{
            randomNum = rng.gen_range(1..10);
            if randomNum < 9{
                inputsTrainValues.push(value.clone());
                outputsTrainValues.push(file_output.expected[index].clone());
            }
            else {
                inputsTestValues.push(value.clone());
                outputsTestValues.push(file_output.expected[index].clone());
            }
        }

        else if inputsTestValues.len() == 1000 {
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
        else if inputsTrainValues.len() == 4000 {
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };

}

pub fn first200Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if index < 2000{
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}

pub fn last200Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if index >= 4000{
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}

pub fn first100last100Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if index < 500 || index >= 4500{
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}

pub fn middle200Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if index >= 3000 && index < 4000{
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}

pub fn on300And700Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if (index >= 3500 && index < 4000) || (index >= 2500 && index < 3000){
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}

pub fn on200And800Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if (index >= 1500 && index < 2000) || (index >= 0 && index < 500){
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}

pub fn on0And300Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if (index >= 1500 && index < 2500){
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}
pub fn on100And400Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if (index >= 500 && index < 1500){
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}

pub fn on800And900Partition(file_output: &FileOutput2) -> Partition{
    let mut train: FileOutput2 = FileOutput2::default();
    let mut test: FileOutput2 = FileOutput2::default();

    let mut inputsTrainValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTrainValues: Vec<f32> =Vec::new();

    let mut inputsTestValues: Vec<(f32,f32,f32)> =Vec::new();
    let mut outputsTestValues: Vec<f32> =Vec::new();

    for (index, value) in file_output.inputs.iter().enumerate(){
        if (index >= 3500 && index < 4500){
            inputsTestValues.push(value.clone());
            outputsTestValues.push(file_output.expected[index].clone());
        }
        else{
            inputsTrainValues.push(value.clone());
            outputsTrainValues.push(file_output.expected[index].clone());
        }
    }
    train = FileOutput2 {inputs: inputsTrainValues, expected: outputsTrainValues};
    test = FileOutput2 {inputs: inputsTestValues, expected: outputsTestValues};
    return Partition { train: train, test: test };
}