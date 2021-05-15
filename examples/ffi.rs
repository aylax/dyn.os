// ===============================================
// ffi.rs --- FFI 
// Author: AyLax.Zhou < zhoubye@foxmail.com >
// Copyright (c) 2021-20xx AyLax.Zhou
// License: GPLv3
// ===============================================


#[link(name = "m")]
extern {
    fn csqrtf(z: Complex) -> Complex;

    fn ccosf(z: Complex) -> Complex;
}

#[repr(C)]
#[derive(Clone,Copy)]
struct Complex {
    re: f32,
    im: f32,
}

impl Complex {
    fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }
}

impl std::fmt::Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.im < 0. {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}-{}i", self.re, self.im)
        }
    }
}

fn cos(z: Complex) -> Complex { unsafe { ccosf(z) } }

fn sqrt(z: Complex) -> Complex { unsafe { csqrtf(z) } }

fn main() {
    let z = Complex::new(-1., 0.);
    println!("squre root of {:?} is {:?}", z, sqrt(z));
    println!("cos of {:?} is {:?}", z, cos(z));
}

