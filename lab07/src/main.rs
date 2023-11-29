use std::fmt;
use std::{io, ops::Add, ops::Mul, ops::Neg, ops::Sub};
#[derive(Debug, Copy, Clone, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}
impl Complex {
    fn new<T, K>(r: T, i: K) -> Self
    where
        T: Into<f64> + Copy,
        K: Into<f64> + Copy,
    {
        Self {
            real: r.into(),
            imag: i.into(),
        }
    }
    fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }
}
impl Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}
impl Add<i32> for Complex {
    type Output = Complex;
    fn add(self, rhs: i32) -> Self::Output {
        Self {
            real: self.real + rhs as f64,
            imag: self.imag,
        }
    }
}
impl Add<f64> for Complex {
    type Output = Complex;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real + rhs as f64,
            imag: self.imag,
        }
    }
}
impl Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}
impl Sub<i32> for Complex {
    type Output = Complex;
    fn sub(self, rhs: i32) -> Self::Output {
        Self {
            real: self.real - rhs as f64,
            imag: self.imag,
        }
    }
}
impl Sub<f64> for Complex {
    type Output = Complex;
    fn sub(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real - rhs as f64,
            imag: self.imag,
        }
    }
}
impl Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Self::Output {
        Self {
            real: (self.real * rhs.real) - (self.imag * rhs.imag),
            imag: (self.real * rhs.imag) + (self.imag * rhs.real),
        }
    }
}
impl Mul<i32> for Complex {
    type Output = Complex;
    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            real: self.real * rhs as f64,
            imag: self.imag * rhs as f64,
        }
    }
}
impl Mul<f64> for Complex {
    type Output = Complex;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Self {
            real: -self.real,
            imag: -self.imag,
        }
    }
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.real == 0_f64 && self.imag == 0_f64 {
            return write!(f, "0");
        }
        let mut equation: String = String::new();
        if self.real < 0_f64 {
            equation.push('-');
        }
        if self.real != 0_f64 {
            let s = format!("{}", self.real);
            equation.push_str(s.as_str());
            if self.imag > 0_f64 {
                equation.push('+');
            }
        }
        if self.imag != 0_f64 {
            let s = format!("{}{}", self.imag, "i");
            equation.push_str(s.as_str());
        }

        write!(f, "{}", equation)
    }
}
trait From<T>: Sized {
    // Required method
    fn from(value: T) -> Self;
}
impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Self {
            real: value as f64,
            imag: 0_f64,
        }
    }
}
impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Self {
            real: value as f64,
            imag: 0_f64,
        }
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}
