use std::ops::Div;
use num::{FromPrimitive, Num, Signed};

pub trait Root {
    fn is_root(squareroot: &Vec<f64>, input: &Vec<f64>, delta: f64) -> Option<bool>;
}
