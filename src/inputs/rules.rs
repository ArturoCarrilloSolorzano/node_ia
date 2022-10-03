use std::io::stdin;
pub struct LearningRules{
    pub max_iterations: i64,
    pub min_error: f32,
    pub learning_rate: f32
}

pub fn main() -> LearningRules{
    let mut buffer = String::new();
    println!("Numero de iteraciones:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer).unwrap();

    let max_iterations = match buffer.trim_end() {
        "" => Ok(600 as i64),
        value => value.parse::<i64>(),
    }
    .expect("input invalido");
    let mut buffer = String::new();
    println!("Error deseado:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer).unwrap();

    let min_error = match buffer.trim_end() {
        "" => Ok(0.1 as f32),
        value => value.parse::<f32>(),
    }
    .expect("input invalido");
    let mut buffer = String::new();
    println!("Tasa de aprendizaje:");
    // `read_line` returns `Result` of bytes read
    stdin().read_line(&mut buffer).unwrap();

    let learning_rate = match buffer.trim_end() {
        "" => Ok(0.1 as f32),
        value => value.parse::<f32>(),
    }
    .expect("input invalido");

    return LearningRules{max_iterations, min_error, learning_rate}
}
