#[macro_use]
extern crate lazy_static;

extern crate num;

use std::env;
use std::fs::File;
use std::io::Read;

use num::complex::Complex;

lazy_static! {
    static ref TAU: f64 = 6.283185307179586476925286766559005768394338798750211641949;
    static ref J: Complex<f64> = Complex::i();
}

fn dft(signal: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let length = signal.len();
    let n = length as f64;
    let mut fourier: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); length];
    for iter1 in 0..length {
        for iter2 in 0..length {
            let i = iter1 as f64;
            let j = iter2 as f64;
            
            fourier[iter1] += signal[iter2] * (-i*j*(*TAU)*(*J)/n).exp();
        }
    }
    return fourier;
}

fn fft(signal: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let length = signal.len();
    if length <= 1 {
        return signal;
    }

    let n = length as f64;
    
    let mut even: Vec<Complex<f64>> = Vec::new();
    even.extend(signal.iter().step_by(2));
    even = fft(even);

    let mut odd: Vec<Complex<f64>> = Vec::new();
    odd.extend(signal.iter().skip(1).step_by(2));
    odd = fft(odd);
    
    let mut fourier: Vec<Complex<f64>> = vec![Complex::new(0.0, 0.0); length];
    for iter in 0..length/2 {
        let i = iter as f64;
        fourier[iter] += even[iter] + (-i*(*TAU)*(*J)/n).exp() * odd[iter];
        fourier[iter+length/2] += even[iter] - (-i*(*TAU)*(*J)/n).exp() * odd[iter];
    }
    return fourier;
}

fn main() {  
    // Open file
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect number of arguments");
        return;
    }
    let mut file = File::open(&args[1]).expect("file not found");

    // Read file
    let mut text = String::new();    
    file.read_to_string(&mut text).expect("error reading file");
    let data = text.split(' ');
    let mut signal: Vec<Complex<f64>> = Vec::new();
    for s in data {
        signal.push(s.parse().unwrap());
    }

    // Store original signal
    let original = signal.to_vec();

    // Compute DFT result
    let fourier1: Vec<Complex<f64>> = dft(signal);

    // Print DFT result
    println!("With DFT:");
    let length = fourier1.len();
    let scaling = length as f64 / 2.0;
    for iter in 0..length/2 {
        println!("{0} hz: {1:.10}", iter, fourier1[iter].norm()/scaling);
    }

    // Compute FFT result
    let fourier2: Vec<Complex<f64>> = fft(original);

    // Print FFT result    
    println!("With FFT:");
    for iter in 0..length/2 {
        println!("{0} hz: {1:.10}", iter, fourier2[iter].norm()/scaling);
    }
}
