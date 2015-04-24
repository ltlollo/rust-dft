extern crate dsp;
extern crate num;

use num::complex::Complex;
use dsp::*;

#[test]
fn test_dfts() {
    let (four, one, zero) = (Complex::new(4.0f64, 0.0),
                             Complex::new(1.0f64, 0.0),
                             Complex::new(0.0f64, 0.0));
    let result = vec![four, zero, zero, zero, four, zero, zero, zero];
    let sig_orig = vec![one, zero, one, zero, one, zero, one, zero];
    let mut sig = sig_orig.clone();
    dif(&mut sig[..]);
    assert!(sig == result, "testing dif");

    sig = sig_orig.clone();
    dit(&mut sig[..]);
    assert!(sig == result, "testing dit");
}

#[test]
fn test_fhwts() {
    let mut sig = vec![4f64, 2f64, 2f64, 4f64];
    let res = vec![3f64, 1f64, 0f64, -1f64];
    let orig = sig.clone();
    fhwt(&mut sig[..]);
    assert!(sig == res, "testing dif");
    fihwt(&mut sig[..]);
    assert!(sig == orig, "testing dit");
}
