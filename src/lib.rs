#![feature(core)]
#![crate_type = "lib"]

extern crate num;
extern crate core;

use num::complex::Complex;
use num::{Zero, One};
use num::traits::Float;
use std::f32::consts::PI_2 as PI32_2;
use std::f64::consts::PI_2 as PI64_2;
use std::option::Option;

pub trait MathConsts : Float {
    fn two_pi() -> Self;
    fn two() -> Self;
    fn from_usize(val: usize) -> Self;
}

impl MathConsts for f64 {
    fn two_pi() -> f64 { PI64_2 }
    fn two() -> f64 { 2f64 }
    fn from_usize(val: usize) -> f64 { val as f64 }
}

impl MathConsts for f32 {
    fn two_pi() -> f32 { PI32_2 }
    fn two() -> f32 { 2f32 }
    fn from_usize(val: usize) -> f32 { val as f32 }
}

pub fn dit<T>(sig: &mut [Complex<T>])
    where T : MathConsts {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n: T = MathConsts::from_usize(len);
    let mut even_vec = vec![Zero::zero(); len/2];
    let mut odd_vec = vec![Zero::zero(); len/2];
    let even = &mut even_vec[..];
    let odd = &mut odd_vec[..];
    for i in (0..len/2 as usize) {
        even[i] = sig[2*i];
        odd[i] = sig[2*i+1];
    }
    dit(even);
    dit(odd);
    for i in (0..len/2 as usize) {
        let k: T = MathConsts::from_usize(i);
        let th: T = -k*MathConsts::two_pi()/n;
        odd[i] = odd[i] * Complex::from_polar(&One::one(), &th);
        sig[i] = even[i] + odd[i];
        sig[i+len/2] = even[i] - odd[i];
    }
}

pub fn dif<T>(sig: &mut [Complex<T>])
    where T : MathConsts {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n: T = MathConsts::from_usize(len);
    let mut vec = sig.to_vec();
    let (first, second) = vec.split_at_mut(len/2);
    for i in (0..len/2 as usize) {
        let k: T = MathConsts::from_usize(i);
        let th: T = -k*MathConsts::two_pi()/n;
        first[i] = first[i] + sig[i+len/2];
        second[i] = (sig[i]-second[i])*Complex::from_polar(&One::one(), &th);
    }
    dif(first);
    dif(second);
    for i in (0..len/2 as usize) {
        sig[2*i] = first[i];
        sig[2*i+1] = second[i];
    }
}

fn is_pow2(n: usize) -> bool {
    (2 as usize).pow((n as f32).log2() as u32) == n
}

pub fn fhwt<T>(sig: &mut [T]) -> Option<()> where T: MathConsts {
    if !is_pow2(sig.len()) {
        return None;
    }
    if sig.len() < 2 {
        return Some(());
    }
    let times = (sig.len() as f32).log2() as usize;
    let mut len  = sig.len()/2;
    let mut i = 1;
    for _ in(0..times) {
        for j in (0..len) {
            let p = (sig[2*i*j] + sig[2*i*j+i])/MathConsts::two();
            let m = (sig[2*i*j] - sig[2*i*j+i])/MathConsts::two();
            sig[2*i*j] = p;
            sig[2*i*j+i] = m;
        }
        i = i*2;
        len = len/2;
    }
    Some(())
}

pub fn fihwt<T>(sig: &mut [T]) -> Option<()> where T: Float {
    if !is_pow2(sig.len()) {
        return None;
    }
    if sig.len() < 2 {
        return Some(());
    }
    let times = (sig.len() as f32).log2() as usize;
    let mut len = sig.len()/2;
    let mut i = 1;
    for _ in (0..times) {
        for j in (0..i) {
            let p = sig[2*len*j] + sig[2*len*j+len];
            let m = sig[2*len*j] - sig[2*len*j+len];
            sig[2*len*j] = p;
            sig[2*len*j+len] = m;
        }
        i = i*2;
        len = len/2;
    }
    Some(())
}
