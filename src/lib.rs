#![crate_type = "lib"]
#![desc = "Some algoritms"]
#![license = "GPLv2"]

extern crate num;
extern crate core;

use num::complex::Complex;
use std::num::FromPrimitive;
use core::num::Float;

pub fn dit<T: FloatMath + FromPrimitive>(sig: &mut [Complex<T>]) {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n: T = FromPrimitive::from_uint(len).unwrap();
    let r: T = FromPrimitive::from_uint(1).unwrap();
    let zero: T = FromPrimitive::from_uint(0).unwrap();
    let mut even_vec = Vec::from_elem(len/2, Complex::new(zero, zero));
    let mut odd_vec = Vec::from_elem(len/2, Complex::new(zero, zero));
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
        odd[i] = odd[i] * Complex::from_polar(&r, &th);
        sig[i] = even[i] + odd[i];
        sig[i+len/2] = even[i] - odd[i];
    }
}

pub fn dif<T: FloatMath + FromPrimitive>(sig: &mut [Complex<T>]) {
    let len = sig.len();
    if len <= 1 {
        return;
    }
    let n: T = FromPrimitive::from_uint(len).unwrap();
    let r: T = FromPrimitive::from_int(-1).unwrap();
    let mut vec = sig.to_vec();
    let (first, second) = vec.split_at_mut(len/2);
    for i in range(0, len/2 as uint) {
        let k: T = FromPrimitive::from_uint(i).unwrap();
        let th: T = -k*Float::two_pi()/n;
        first[i] = first[i] + sig[i+len/2];
        second[i] = (second[i]-sig[i])*Complex::from_polar(&r, &th);
    }
    dif(first);
    dif(second);
    for i in range(0, len/2 as uint) {
        sig[2*i] = first[i];
        sig[2*i+1] = second[i];
    }
}
