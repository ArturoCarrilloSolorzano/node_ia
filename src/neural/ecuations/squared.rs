use super::ErrorEcuation;

pub struct SquaredError {}

impl ErrorEcuation for SquaredError {
    fn calc(
        output: &nalgebra::DVector<f32>,
        expected: &nalgebra::DVector<f32>,
    ) -> nalgebra::DVector<f32> {
        let dif = output - expected;
        let squared = dif.component_mul(&dif);
        squared.scale(0.5)
    }

    fn deriv(
        output: &nalgebra::DVector<f32>,
        expected: &nalgebra::DVector<f32>,
    ) -> nalgebra::DVector<f32> {
        output - expected
    }
}
