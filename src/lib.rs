#![crate_type = "lib"]
#![desc = "Time and frequency decimation discrete fourier tranform"]
#![license = "GPLv2"]

extern crate num;
extern crate core;

use num::complex::Complex;
use num::{Zero, One, Num};
use std::num::FromPrimitive;
use std::num::{Float, FloatMath};

pub fn dit<T>(sig: &mut [Complex<T>])
    where T : Num + Zero + One + FloatMath + FromPrimitive {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n: T = FromPrimitive::from_uint(len).unwrap();
    let mut even_vec = Vec::from_elem(len/2, Zero::zero());
    let mut odd_vec = Vec::from_elem(len/2, Zero::zero());
    let even = even_vec.as_mut_slice();
    let odd = odd_vec.as_mut_slice();
    for i in range(0, len/2 as uint) {
        even[i] = sig[2*i];
        odd[i] = sig[2*i+1];
    }
    dit(even);
    dit(odd);
    for i in range(0, len/2 as uint) {
        let k: T = FromPrimitive::from_uint(i).unwrap();
        let th: T = -k*Float::two_pi()/n;
        odd[i] = odd[i] * Complex::from_polar(&One::one(), &th);
        sig[i] = even[i] + odd[i];
        sig[i+len/2] = even[i] - odd[i];
    }
}

pub fn dif<T>(sig: &mut [Complex<T>])
    where T : Num + One + FloatMath + FromPrimitive {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n: T = FromPrimitive::from_uint(len).unwrap();
    let mut vec = sig.to_vec();
    let (first, second) = vec.split_at_mut(len/2);
    for i in range(0, len/2 as uint) {
        let k: T = FromPrimitive::from_uint(i).unwrap();
        let th: T = -k*Float::two_pi()/n;
        first[i] = first[i] + sig[i+len/2];
        second[i] = (sig[i]-second[i])*Complex::from_polar(&One::one(), &th);
    }
    dif(first);
    dif(second);
    for i in range(0, len/2 as uint) {
        sig[2*i] = first[i];
        sig[2*i+1] = second[i];
    }
}
