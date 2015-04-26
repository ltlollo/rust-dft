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

/// This function computes the fourier transformation of the input signal
/// using the fast in-place time decimation algoritm
pub fn dit<T>(sig: &mut [Complex<T>]) where T : MathConsts {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n = T::from_usize(len);
    let mut vec = vec![Zero::zero(); len];
    let (even, odd) = vec.split_at_mut(len/2);
    for i in (0..len/2 as usize) {
        even[i] = sig[2*i];
        odd[i] = sig[2*i+1];
    }
    dit(even);
    dit(odd);
    for i in (0..len/2 as usize) {
        let k = T::from_usize(i);
        let th = -k*T::two_pi()/n;
        odd[i] = odd[i] * Complex::from_polar(&One::one(), &th);
        sig[i] = even[i] + odd[i];
        sig[i+len/2] = even[i] - odd[i];
    }
}

/// This function computes the fourier transformation of the input signal
/// using the fast in-place time decimation algoritm
pub fn dif<T>(sig: &mut [Complex<T>]) where T : MathConsts {
    let len = sig.len();
    if len <= 1 {
        return;
    } {
        let n = T::from_usize(len);
        let (first, second) = sig.split_at_mut(len/2);
        for i in (0..len/2 as usize) {
            let k = T::from_usize(i);
            let th = -k*T::two_pi()/n;
            let f = first[i];
            first[i] = f + second[i];
            second[i] = (f - second[i])*Complex::from_polar(&One::one(), &th);
        }
        dif(first);
        dif(second);
    }
    let vec = sig.to_vec();
    let (first, second) = vec.split_at(len/2);
    for i in (0..len/2 as usize) {
        sig[2*i] = first[i];
        sig[2*i+1] = second[i];
    }
}

fn is_pow2(n: usize) -> bool {
    (2 as usize).pow((n as f32).log2() as u32) == n
}

/// This function computes the haar wavelet transform
/// the signal size must be a power of 2
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
            let p = (sig[2*i*j] + sig[2*i*j+i])/T::two();
            let m = (sig[2*i*j] - sig[2*i*j+i])/T::two();
            sig[2*i*j] = p;
            sig[2*i*j+i] = m;
        }
        i = i*2;
        len = len/2;
    }
    Some(())
}

/// This function computes the inverse haar wavelet transform
/// the signal size must be a power of 2
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
