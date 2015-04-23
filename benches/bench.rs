#![feature(test)]

extern crate dfts;
extern crate test;
extern crate num;

use num::complex::Complex;
use test::Bencher;
use dfts::*;

#[bench]
fn mesure_dif(b: &mut Bencher) {
    let mut sig : Vec<_> = (0..2048).map(|_| {
        Complex::new(1 as f64, 1 as f64)
    }).collect();
    b.iter(|| dif(&mut sig[..]));
}

#[bench]
fn mesure_dit(b: &mut Bencher) {
    let mut sig : Vec<_> = (0..2048).map(|_| {
        Complex::new(1 as f64, 1 as f64)
    }).collect();
    b.iter(|| dit(&mut sig[..]));
}
