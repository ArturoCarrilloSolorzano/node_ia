use libm::log;

use nalgebra::DVector;

pub struct CrossEntropy {}

impl CrossEntropy {
    pub fn calc(outpus: &DVector<f32>, expected: &DVector<f32>) -> f32 {
        let mut index = 0;
        for (i, value) in expected.iter().enumerate() {
            if *value > 0.0 {
                index = i;
                break;
            }
        }
        return -log(outpus[index as usize] as f64) as f32;
    }
}
