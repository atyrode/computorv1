pub mod io;
pub mod parse;

pub use std::error::Error;

#[derive(Default)]
pub struct Equation {
    pub left_side: Vec<Term>,
    pub right_side: Vec<Term>,
}

impl Equation {
    #[must_use]
    pub fn new(left_side: Vec<Term>, right_side: Vec<Term>) -> Self {
        Self {
            left_side,
            right_side,
        }
    }
}

impl std::fmt::Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn parse(side: &Vec<Term>) -> String {
            let mut result = String::new();
            for term in side {
                if term.coefficient <= 0.0 {
                    result.push_str(&format!("- {}", term.coefficient.abs()));
                } else {
                    result.push_str(&format!("+ {term} "));
                }
            }
            result
        }

        write!(
            f,
            "{} = {}",
            parse(&self.left_side),
            parse(&self.right_side)
        )
    }
}

pub struct Term {
    pub coefficient: f64,
    pub power: i32,
}

impl Term {
    #[must_use]
    pub const fn new(coefficient: f64, power: i32) -> Self {
        Self { coefficient, power }
    }
}

impl std::default::Default for Term {
    fn default() -> Self {
        Self {
            coefficient: 0.0,
            power: 0,
        }
    }
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} * X^{}", self.coefficient, self.power)
    }
}
