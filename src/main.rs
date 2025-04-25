use std::fs::File;
use std::io::{Write, BufWriter};
use ndarray::{Array2};

static P: f64 = 0.49;
static INV_P: f64 = 0.51;

fn multiply_matrix(m1: &Array2<f64>, m2: &Array2<f64>) -> Array2<f64> {
    m1.dot(m2) // Multiplicación optimizada de matrices
}

fn generate_markov_chain(n: usize) -> Array2<f64> {
    let mut markov_chain = Array2::<f64>::zeros((n, n)); // Inicializa con ceros
    for i in 1..(n - 1) {
        markov_chain[(i - 1, i)] = INV_P;
        markov_chain[(i + 1, i)] = P;
    }
    markov_chain[(0, 0)] = 1.0;
    markov_chain[(n - 1, n - 1)] = 1.0;
    markov_chain
}

fn game(n: usize, state: usize, mut k: i32, markov_chain: &Array2<f64>, filename: &str) {
    let mut markov_pow = Array2::<f64>::eye(n); 
    let mut local_markov_chain = markov_chain.clone(); // Crea una copia mutable

    while k > 0 {
        if k % 2 == 1 {
            markov_pow = multiply_matrix(&local_markov_chain, &markov_pow);
        }
        k /= 2;
        local_markov_chain = multiply_matrix(&local_markov_chain, &local_markov_chain);
    }

    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    for i in 0..n {
        write!(writer, "{:.6}", markov_pow[(i, state)]).unwrap();
        if i < n - 1 {
            write!(writer, ", ").unwrap();
        }
    }
    writeln!(writer).unwrap();
}

fn main() {
    let n: usize = 1002;
    let k: i32 = 1000;

    let markov_chain = generate_markov_chain(n);

    game(n, 10, k, &markov_chain, "start_10.txt");
    game(n, 100, k, &markov_chain, "start_100.txt");
    game(n, 1000, k, &markov_chain, "start_1000.txt");
}