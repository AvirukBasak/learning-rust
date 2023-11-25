use crate::helperfn::get_filename;
use crate::helperfn::input;

mod numbers {
    pub struct Number {
        base: i32,
        exp: i32,
    }

    impl Number {
        pub fn new(base: i32, exp: i32) -> Self {
            Number { base, exp }
        }
        pub fn get_base(&self) -> i32 {
            self.base
        }
        pub fn get_exp(&self) -> i32 {
            self.exp
        }
        pub fn pow(&self) -> i32 {
            let mut result = 1;
            for _ in 0..self.exp {
                result *= self.base;
            }
            result
        }
        pub fn sqrt(&self) -> f32 {
            (self.base as f32).sqrt()
        }
    }
}

pub fn main() {
    let base: i32 = input(&format!("{}: Enter base: ", get_filename(file!())));
    let exp: i32 = input(&format!("{}: Enter exponent: ", get_filename(file!())));
    let num = numbers::Number::new(base, exp);
    println!(
        "{}: {}^{} = {}",
        get_filename(file!()),
        num.get_base(),
        num.get_exp(),
        num.pow()
    );
    println!(
        "{}: sqrt({}) = {}",
        get_filename(file!()),
        num.get_base(),
        num.sqrt()
    );
}
