#[cfg(test)]
use crate::fft::{_fft_basic as fft, bit_reversal};
#[cfg(test)]
use num_complex::Complex;

#[test]
pub fn test() {
    let mut buffer = vec![Complex::new(0.0, 0.0); 5];
    for i in 0..5 {
        buffer[i] = Complex::new(1.0 + i as f64, 0.0);
    }
    fft(&mut buffer);
    let first_value = Complex::new(15.0, 0.0);
    // let second_value = Complex::new(-2.5, 3.4409548011777);
    println!("{:?}", buffer);
    assert_eq!(buffer[0], first_value);
    // assert_eq!(buffer[1].re, second_value.re);
}

#[test]
pub fn test2() {
    let mut buffer = vec![Complex::new(0.0, 0.0); 8];
    for i in 0..8 {
        buffer[i] = Complex::new(1.0 + i as f64, 0.0);
    }
    fft(&mut buffer);
    let first_value = Complex::new(36.0, 0.0);
    // let second_value = Complex::new(-2.5, 3.4409548011777);
    println!("{:?}", buffer);
    assert_eq!(buffer[0], first_value);
    // assert_eq!(buffer[1].re, second_value.re);
}

#[test]
pub fn test_bit_reversal() {
    assert_eq!(bit_reversal(0, 3), 0);
    assert_eq!(bit_reversal(1, 3), 4);
    assert_eq!(bit_reversal(2, 3), 2);
    assert_eq!(bit_reversal(3, 3), 6);
    assert_eq!(bit_reversal(4, 3), 1);
    assert_eq!(bit_reversal(5, 3), 5);
    assert_eq!(bit_reversal(6, 3), 3);
    assert_eq!(bit_reversal(7, 3), 7);

    assert_eq!(bit_reversal(0, 4), 0);
    assert_eq!(bit_reversal(1, 4), 8);
    assert_eq!(bit_reversal(2, 4), 4);
    assert_eq!(bit_reversal(3, 4), 12);
    assert_eq!(bit_reversal(4, 4), 2);
    assert_eq!(bit_reversal(5, 4), 10);
    assert_eq!(bit_reversal(6, 4), 6);
    assert_eq!(bit_reversal(7, 4), 14);
}