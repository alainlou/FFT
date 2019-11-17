extern crate num;

use std::env;
use std::fs::File;
use std::io::Read;

use num::complex::Complex;

const TAU: f64 = 6.283185307179586476925286766559005768394338798750211641949;

fn main() {
    // Open file
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(&args[1]).expect("file not found");

    // Read file
    let mut text = String::new();    
    file.read_to_string(&mut text).expect("error reading file");
    let data = text.split(' ');
    let mut signal: Vec<f64> = Vec::new();
    for s in data {
        signal.push(s.parse().unwrap());
    }
    
    // Compute result
    let length = signal.len();
    let n = length as f64;
    let mut fourier: Vec<Complex<f64>> = Vec::with_capacity(length);
    for _ in 0..length {
        fourier.push(Complex::new(0.0, 0.0));
    }
    for iter1 in 0..length {
        for iter2 in 0..length {
            let i = iter1 as f64;
            let j = iter2 as f64;
            fourier[iter1].re += signal[iter2] * (TAU/n*i*j).cos();
            fourier[iter1].im += -signal[iter2] * (TAU/n*i*j).sin();
        }
    }

    // Print result
    let scaling = length as f64 / 2.0;
    for iter1 in 0..length/2 {
        println!("{0} hz: {1:.10}", iter1, fourier[iter1].norm()/scaling);
    }
}
