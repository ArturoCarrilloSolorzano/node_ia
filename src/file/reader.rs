use std::error::Error;
use std::process;
use std::path::Path;

pub struct FileOutput {
    pub inputs: Vec<(f32, f32)>,
    pub expected: Vec<f32>,
}

fn reader(file_name: &str) -> Result< FileOutput, Box<dyn Error>>{
    let path = Path::new("./files/inputs/");
    let path = path.join(file_name);
    let mut rdr = csv::Reader::from_path(path)?;
    let mut inputs:Vec<(f32, f32)> = Vec::new();
    let mut outputs:Vec<f32> = Vec::new();
    
    for result in rdr.records(){
        let record = result?;
        inputs.push((record[0].parse()?, record[1].parse()?));
        outputs.push(record[2].parse()?);
    }
    Ok(FileOutput{inputs, expected: outputs})
}

pub fn main(file_name: &str) -> FileOutput{
    match reader(file_name){
        Ok(output) => return output,
        Err(err)=>{
            println!("error running example: {}", err);
            process::exit(1);
        }
    }

}
