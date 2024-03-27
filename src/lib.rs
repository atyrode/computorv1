pub mod io;
pub mod solve;
pub use solve::solve_equation;

pub mod parse;

pub use std::error::Error;
pub use std::fmt::{Debug, Display, Formatter, Result};

pub struct Equation {
    left: Vec<Term>,
    right: Vec<Term>,
}

impl Equation {
    pub fn underline_term(&self, term: &Term) -> String {
        let mut underline = String::new();

        for _ in 0..term.to_string().len() {
            underline.push('-');
        }
        underline
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        fn format_terms(terms: &[Term]) -> String {
            let fmt_terms = terms
                .iter()
                .map(|term| format!("{term}"))
                .collect::<Vec<String>>()
                .join(" ");

            if let Some(fmt_terms) = fmt_terms.strip_prefix('+') {
                return fmt_terms.to_string();
            } else if fmt_terms.starts_with("- ") {
                return fmt_terms.replacen("- ", "-", 1);
            }
            fmt_terms
        }

        write!(
            f,
            "{} = {}",
            format_terms(&self.left),
            format_terms(&self.right)
        )
    }
}

struct Term {
    pub coefficient: Option<Coefficient>,
    pub variable: Option<Variable>,
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match (&self.coefficient, &self.variable) {
            (Some(coefficient), Some(variable)) => write!(f, "{} * {}", coefficient, variable),
            (Some(coefficient), None) => write!(f, "{}", coefficient),
            (None, Some(variable)) => write!(f, "{}", variable),
            (None, None) => write!(f, ""),
        }
    }
}

struct Coefficient {
    value: f64,
    power: Exponent,
}

impl Display for Coefficient {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", self.value, self.power)
    }
}

struct Variable {
    negated: bool,
    power: Exponent,
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, 
            "{}X{}",
            if self.negated { "-" } else { "" },
            self.power)
    }
}

struct Exponent {
    value: Option<f64>,
}

impl Display for Exponent {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.value {
            Some(value) => write!(f, "^{}", value),
            None => write!(f, ""),
        }
    }
}