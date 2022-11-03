use rand::Rng;

use super::reader::{FileOutput, Record};

pub struct Partition {
    pub train: FileOutput,
    pub test: FileOutput,
}

pub fn random_partition(data: &FileOutput, train_percentage: f32) -> Partition {
    let traning_len = (data.len() as f32 * train_percentage).ceil() as usize;
    let test_len = data.len() - traning_len;
    let mut train = Vec::<Record>::with_capacity(traning_len);
    let mut test = Vec::<Record>::with_capacity(test_len);
    let mut rng = rand::thread_rng();
    for entry in data.iter() {
        if test.len() >= test_len {
            train.push(entry.clone());
            continue;
        }
        let r = rng.gen_range(0.0..1.0);
        if r < train_percentage && train.len() < traning_len {
            train.push(entry.clone());
            continue;
        }
        test.push(entry.clone());
    }
    Partition { train, test }
}
