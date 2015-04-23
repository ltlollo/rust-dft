#![feature(core)]
#![crate_type = "lib"]

extern crate num;
extern crate core;

use num::complex::Complex;
use num::{Zero, One};
use std::num::FromPrimitive;
use num::traits::Float;
use std::f32::consts::PI_2 as PI32_2;
use std::f64::consts::PI_2 as PI64_2;

pub trait MathConsts : Float {
    fn two_pi() -> Self;
}

impl MathConsts for f64 {
    fn two_pi() -> f64 {
        return PI64_2;
    }
}

impl MathConsts for f32 {
    fn two_pi() -> f32 {
        return PI32_2;
    }
}

pub fn dit<T>(sig: &mut [Complex<T>])
    where T : FromPrimitive + MathConsts {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n: T = FromPrimitive::from_usize(len).unwrap();
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
        let k: T = FromPrimitive::from_usize(i).unwrap();
        let th: T = -k*MathConsts::two_pi()/n;
        odd[i] = odd[i] * Complex::from_polar(&One::one(), &th);
        sig[i] = even[i] + odd[i];
        sig[i+len/2] = even[i] - odd[i];
    }
}

pub fn dif<T>(sig: &mut [Complex<T>])
    where T :  FromPrimitive + MathConsts {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n: T = FromPrimitive::from_usize(len).unwrap();
    let mut vec = sig.to_vec();
    let (first, second) = vec.split_at_mut(len/2);
    for i in (0..len/2 as usize) {
        let k: T = FromPrimitive::from_usize(i).unwrap();
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
