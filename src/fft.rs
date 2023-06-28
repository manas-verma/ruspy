use crate::array::Array;
use num_complex::Complex;
use num_traits::Zero;
use pyo3::prelude::{pyfunction, PyResult, Python};
use rustfft::FftPlanner;
use std::f64::consts::PI;

fn num_bits(x: usize) -> usize {
    (x as f64).log2().floor() as usize
}
fn fft_iterative(buffer: &mut [Complex<f64>]) {
    let n = buffer.len();
    if n.count_ones() > 1 {
        panic!("Length of buffer must be a power of 2.");
    }

    // Might be off by one.
    for s in 0..n {
        let m = 1 << s;
        if m > n {
            return;
        }
        let omega_m = Complex::from_polar(1.0, -2.0 * PI / (m as f64));
        for k in (0..n).step_by(m) {
            let mut omega = Complex::new(1.0, 0.0);
            for j in 0..m / 2 {
                let t = omega * buffer[k + j + m / 2];
                let u = buffer[k + j];
                buffer[k + j] = u + t;
                buffer[k + j + m / 2] = u - t;
                omega *= omega_m;
            }
        }
    }
}

pub fn bit_reversal(x: usize, num_bits: usize) -> usize {
    let mut bit_reversed = 0;
    for i in 0..num_bits {
        let bit = (x >> i) & 1;
        bit_reversed = (bit_reversed << 1) | bit;
    }
    bit_reversed
}

pub fn _fft_basic(buffer: &mut Vec<Complex<f64>>) {
    let n = buffer.len();
    let num_bits_needed = num_bits(n - 1) + 1;
    print!("num_bits_needed: {}\n", num_bits_needed);
    let p: usize = 1 << num_bits_needed;

    let mut padded_buffer = vec![Complex::zero(); p];

    for i in 0..p {
        let bri = bit_reversal(i, num_bits_needed);
        padded_buffer[bri] = buffer[i % n];
    }

    fft_iterative(&mut padded_buffer);
    for i in 0..n {
        buffer[i] = padded_buffer[i];
    }
}

#[pyfunction]
pub fn fft_basic(_py: Python, array: &Array) -> PyResult<Array> {
    let mut data = array.data.clone();
    _fft_basic(&mut data);
    Ok(Array { data })
}

fn _fft(buffer: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(buffer.len());
    let mut output = buffer.to_vec();
    fft.process(&mut output);
    output
}

#[pyfunction]
pub fn fft(_py: Python, array: &Array) -> PyResult<Array> {
    let output = _fft(&array.data);
    Ok(Array { data: output })
}
