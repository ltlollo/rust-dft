#![feature(core)]
#![crate_type = "lib"]

extern crate num;
extern crate core;

use num::complex::Complex;
use num::{Zero, One};
use num::traits::Float;
use std::option::Option;

pub trait MathConsts : Float + Send {
    fn two_pi() -> Self;
    fn from_usize(val: usize) -> Self;
}

impl MathConsts for f64 {
    fn two_pi() -> f64 { 6.28318530717958647692528676655900576f64 }
    fn from_usize(val: usize) -> f64 { val as f64 }
}

impl MathConsts for f32 {
    fn two_pi() -> f32 { 6.28318530717958647692528676655900576f32 }
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
    }
    {
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

pub struct TwiddleTable<T> {
    table: Vec<Vec<Complex<T>>>,
}

impl<T> TwiddleTable<T> where T: MathConsts {
    fn dit_noalloc(&self, sig: &mut [Complex<T>]) {
        let len = sig.len();
        if len <= 1 {
            return;
        }
        let mut vec = vec![Zero::zero(); len];
        let (even, odd) = vec.split_at_mut(len/2);
        for i in (0..len/2 as usize) {
            even[i] = sig[2*i];
            odd[i] = sig[2*i+1];
        }
        dit(even);
        dit(odd);
        let w = &self.table[len/2];
        for i in (0..len/2 as usize) {
            odd[i] = odd[i]*w[i];
            sig[i] = even[i] + odd[i];
            sig[i+len/2] = even[i] - odd[i];
        }
    }
    pub fn dit(&mut self, sig: &mut [Complex<T>]) {
        self.preallocate(sig.len());
        self.dit_noalloc(sig);
    }
    fn dif_noalloc(&self, sig: &mut [Complex<T>]) {
        let len = sig.len();
        if len <= 1 {
            return;
        }
        {
            let (first, second) = sig.split_at_mut(len/2);
            let w = &self.table[len/2];
            for i in (0..len/2 as usize) {
                let f = first[i];
                first[i] = f + second[i];
                second[i] = (f - second[i])*w[i];
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
    pub fn dif(&mut self, sig: &mut [Complex<T>]) {
        self.preallocate(sig.len());
        self.dif_noalloc(sig);
    }
    fn preallocate(&mut self, siglen: usize) {
        if siglen/2 > self.table.len()+1 {
            for j in (self.table.len()..siglen/2+1) {
                let w: Vec<Complex<T>> = (0..j).map(|x| {
                    let th = T::two_pi()*T::from_usize(x)/T::from_usize(j);
                    Complex::from_polar(&One::one(), &th)
                }).collect();
                self.table.push(w);
            }
        }
    } 
    pub fn precompute(len: usize) -> TwiddleTable<T> {
        let n = len/2+1;
        let mut t: Vec<Vec<Complex<T>>> = Vec::with_capacity(n);
        for j in (0..n) {
            let w: Vec<Complex<T>> = (0..j).map(|x| {
                let th = T::two_pi()*T::from_usize(x)/T::from_usize(j);
                Complex::from_polar(&One::one(), &th)
            }).collect();
            t.push(w);
        }
        TwiddleTable{ table: t }
    }
    pub fn new() -> TwiddleTable<T> {
        TwiddleTable{ table: Vec::new() }
    }
}
fn is_pow2(n: usize) -> bool {
    (2 as usize).pow((n as f32).log2() as u32) == n
}

/// This function computes the fast haar wavelet transform
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
            let p = (sig[2*i*j] + sig[2*i*j+i])/T::from_usize(2);
            let m = (sig[2*i*j] - sig[2*i*j+i])/T::from_usize(2);
            sig[2*i*j] = p;
            sig[2*i*j+i] = m;
        }
        i = i*2;
        len = len/2;
    }
    Some(())
}

/// This function computes the fast inverse haar wavelet transform
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
