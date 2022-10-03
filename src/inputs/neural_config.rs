use std::io::stdin;

pub fn main() -> Vec<usize>{
    let mut network_size: Vec<usize> = Vec::new();
    let mut buffer = String::new();
    println!("Numero de capas:");
    stdin().read_line(&mut buffer).unwrap();

    let layers = match buffer.trim_end() {
        "" => Ok(3 as usize),
        value => value.parse::<usize>(),
    }
    .expect("input invalido");

    for i in 0..layers{
        println!("Neuronas para la capa: {}", i);
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
    
        let neuronas = match buffer.trim_end() {
            "" => Ok(10 as usize),
            value => value.parse::<usize>(),
        }
        .expect("input invalido");

        network_size.push(neuronas);
    }

    return network_size;
}
