
use std::io;

static P:f64 =0.49;
static INV_P:f64=0.51;

fn multiply_matrix(m1: &Vec<Vec<f64>>,m2: &Vec<Vec<f64>> )-> Vec<Vec<f64>>
{
    let n = m1.len();
    let mut result= vec![vec![0.0; n]; n];
    for i in 0..n
    {
        for j in 0..n
        {
            for k in 0..n
            {
                result[i][j]+=m1[i][k]*m2[k][j];
            }
        }
    }
    result
}

fn generate_markov_chain(n: usize, markov_chain: &mut Vec<Vec<f64>>) {
    for i in 1..(n - 1) {
        markov_chain[i-1][i] = INV_P;
        markov_chain[i+1][i] = P;
    }

    markov_chain[0][0] = 1.0;
    markov_chain[n - 1][n - 1] = 1.0;
}

fn main(){
    let mut input=String::new();
    println!("Write the number of states");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid integer input");
    let mut markov_chain = vec![vec![0.0; n]; n];
    let mut markov_pow=vec![vec![0.0; n]; n];
    for i in 0..n
    {
        markov_pow[i][i]=1.0;
    }
    generate_markov_chain(n,&mut markov_chain);
    for row in markov_chain.iter(){
        println!("{:?}",row);
    }
    input.clear();
    println!("Write the initial state");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let state:usize = input.trim().parse().expect("Invalid integer input");
    input.clear();
    println!("Write the exponent");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut k:i32 =input.trim().parse().expect("Invalid integer input");
    while k>0 {
        if k%2==1 
        {
            markov_pow=multiply_matrix(&markov_chain,&markov_pow);
        }
        k/=2;
        markov_chain=multiply_matrix(&markov_chain,&markov_chain);
    }
    for i in 0..n{
        println!("{:?}",markov_pow[i][state]);
    }
}