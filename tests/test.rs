extern crate dfts;
extern crate num;

use num::complex::Complex;
use dfts::*;

#[test]
fn calc_dfts() {
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
