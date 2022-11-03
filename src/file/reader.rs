use std::error::Error;
use std::path::Path;

pub struct Reader {
    input_len: usize,
    output_len: usize,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub inputs: Vec<f32>,
    pub expected: Vec<f32>,
}

pub type FileOutput = Vec<Record>;

impl Reader {
    pub fn new(input_len: usize, output_len: usize) -> Self {
        Reader {
            input_len,
            output_len,
        }
    }
    pub fn read(&self, file_name: &str) -> Result<FileOutput, Box<dyn Error>> {
        let path = Path::new("./files/inputs/");
        let path = path.join(file_name);
        let mut rdr = csv::Reader::from_path(path)?;
        let mut records: Vec<Record> = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let mut inputs = Vec::<f32>::with_capacity(self.input_len);
            let mut expected = Vec::<f32>::with_capacity(self.input_len);
            for i in 0..self.input_len {
                inputs.push(record[i].parse()?);
            }
            for i in self.input_len..self.input_len + self.output_len {
                expected.push(record[i].parse()?);
            }
            records.push(Record { inputs, expected })
        }
        Ok(records)
    }
}
