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
        let left_side: Vec<String> = self.left_side.iter().map(format_term).collect();
        let right_side: Vec<String> = self.right_side.iter().map(format_term).collect();

        let ls = left_side.join(" ");
        let rs = right_side.join(" ");

        write!(
            f,
            "{} = {}",
            ls.trim_start_matches('+').trim_start(),
            rs.trim_start_matches('+').trim_start()
        )
    }
}

fn format_term(term: &Term) -> String {
    match term.coefficient {
        c if c == 0.0 => String::new(), // Skip zero coefficients
        c if (c - 1.0).abs() < f64::EPSILON && term.power != 0 => format!("+ X^{}", term.power),
        c if (c + 1.0).abs() < f64::EPSILON && term.power != 0 => format!("- X^{}", term.power),
        c if c < 0.0 => format!("{} * X^{}", c, term.power),
        _ => format!("+ {} * X^{}", term.coefficient, term.power),
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
