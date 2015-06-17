use std::ops::{Add, Div, Mul, Neg, Sub};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Complex {
    pub re: f32,
    pub im: f32
}

impl Complex {
    #[inline]
    pub fn new(re: f32, im: f32) -> Complex {
        Complex { re: re, im: im }
    }

    #[inline]
    pub fn scale(&mut self, f: f32) {
        self.re *= f;
        self.im *= f;
    }

    #[inline]
    pub fn scale_new(&self, f: f32) -> Complex {
        Complex::new(self.re * f, self.im * f)
    }

    #[inline]
    pub fn norm(&self) -> f32 {
        self.re.hypot(self.im)
    }

    #[inline]
    pub fn from_polar(r: f32, theta: f32) -> Complex {
        Complex::new(r * theta.cos(), r * theta.sin())
    }
}

impl Add for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

impl Mul for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        Complex::new(self.re * other.re - self.im * other.im, self.re * other.im + self.im * other.re)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{}-{}i", self.re, 0.0 - self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}
