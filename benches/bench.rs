#![feature(test)]

extern crate dsp;
extern crate test;
extern crate num;

use num::complex::Complex;
use test::Bencher;
use dsp::*;

#[bench]
fn mesure_dif(b: &mut Bencher) {
    let mut sig : Vec<_> = (0..2048).map(|x| {
        Complex::new( x as f64, x as f64 + 1f64)
    }).collect();
    b.iter(|| dif(&mut sig[..]));
}

#[bench]
fn mesure_dif_table(b: &mut Bencher) {
    let mut t : TwiddleTable<f64> = TwiddleTable::precompute(2048);
    let mut sig : Vec<_> = (0..2048).map(|x| {
        Complex::new( x as f64, x as f64 + 1f64)
    }).collect();
    b.iter(|| t.dif(&mut sig[..]));
}

#[bench]
fn mesure_dit_table(b: &mut Bencher) {
    let mut t : TwiddleTable<f64> = TwiddleTable::precompute(2048);
    let mut sig : Vec<_> = (0..2048).map(|x| {
        Complex::new( x as f64, x as f64 + 1f64)
    }).collect();
    b.iter(|| t.dit(&mut sig[..]));
}

#[bench]
fn mesure_dit(b: &mut Bencher) {
    let mut sig : Vec<_> = (0..2048).map(|x| {
        Complex::new(x as f64, x as f64 + 1f64)
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
