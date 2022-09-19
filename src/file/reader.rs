use std::error::Error;
use std::process;
use std::path::Path;

pub struct file_output {
    pub inputs: Vec<Vec<f32>>,
    pub expected: Vec<Vec<f32>>,
}

fn reader(file_name: &str) -> Result< file_output, Box<dyn Error>>{
    let path = Path::new("./files/inputs/");
    let path = path.join(file_name);
    print!("{:?}", path);
    let mut rdr = csv::Reader::from_path(path)?;
    let mut inputs:Vec<Vec<f32>> = Vec::new();
    let mut outputs:Vec<Vec<f32>> = Vec::new();
    
    for result in rdr.records(){
        let record = result?;
        inputs.push(vec![record[0].parse()?, record[1].parse()?]);
        outputs.push(vec![record[2].parse()?]);
        println!("{:?}", record);
    }
    Ok(file_output{inputs, expected: outputs})
}

pub fn main() -> file_output{
    match reader("XOR_trn.csv"){
        Ok(output) => return output,
        Err(err)=>{
            println!("error running example: {}", err);
            process::exit(1);
        }
    }

}