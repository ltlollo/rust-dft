#![feature(test)]

extern crate dsp;
extern crate test;
extern crate num;

use num::complex::Complex;
use test::Bencher;
use dsp::*;

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

#[bench]
fn mesure_fhwt(b: &mut Bencher) {
    let mut sig = vec![1f64; 2048];
    b.iter(|| fhwt(&mut sig[..]).unwrap());
}

#[bench]
fn mesure_fihwt(b: &mut Bencher) {
    let mut sig = vec![1f64; 2048];
    b.iter(|| fihwt(&mut sig[..]).unwrap());
}
