use regex::Regex;

use crate::Error;
use crate::{Coefficient, Equation, Sign, Term, Variable};

fn remove_spaces(equation: &str) -> String {
    equation.replace(' ', "")
}

fn get_sides(equation: &str) -> Vec<&str> {
    equation.split('=').collect()
}

fn get_terms(side: &str, pattern: &Regex) -> Result<Vec<Term>, Box<dyn Error>> {

    let mut terms: Vec<Term> = Vec::new();

    for cap in pattern.captures_iter(side) {

        let coef_sign = cap.name("coef_sign").and_then(|sign| match sign.as_str() {
            "+" => Some(Sign::Positive),
            "-" => Some(Sign::Negative),
            _ => None,
        });

        let coef = match cap.name("coef") {
            Some(coef) => coef.as_str().parse::<f64>()?,
            None => Err("Invalid coefficient")?,
        };
        
        let coef_exponent = match cap.name("coef_exponent") {
            Some(exponent) => Some(exponent.as_str().parse::<f64>()?.abs()),
            None => None,
        };

        let var_sign = cap.name("var_sign").and_then(|sign| match sign.as_str() {
            "+" => Some(Sign::Positive),
            "-" => Some(Sign::Negative),
            _ => None,
        });

        match cap.name("var") {
            Some(_) => (),
            None => Err("Missing variable")?,
        };

        let var_exponent = match cap.name("var_exponent") {
            Some(exponent) => exponent.as_str().parse::<f64>()?,
            None => Err("Missing variable exponent")?,
        };

        let term = Term {
            coefficient: Coefficient {
                sign: coef_sign,
                value: coef,
                power: coef_exponent,
            },
            variable: Variable {
                sign: var_sign,
                power: var_exponent,
            },
        };
        terms.push(term);
    }
    Ok(terms)
}

/// # Errors
/// This function will return an error if the equation is invalid.
pub fn get_equation(user_input: &str) -> Result<Equation, Box<dyn Error>> {
    let trimmed = remove_spaces(user_input);
    let pattern = Regex::new(
        r"(?:(?<coef_sign>[+-]|^)(?<coef>\d(?:\.\d+)?)(?:\^(?<coef_exponent>\d))?)?\*(?:(?<var_sign>[+-])?(?<var>[Xx])(?:\^(?<var_exponent>\d))?)?",
    )?;

    // Return error if there are no equal signs in the equation
    match get_sides(&trimmed) {
        sides if sides.len() != 2 => Err("Invalid equation".into()),
        sides => {
            let left: Vec<Term> = get_terms(sides[0], &pattern)?;
            let right: Vec<Term> = get_terms(sides[1], &pattern)?;

            let equation = Equation { left, right };

            Ok(equation)
        }
    }
}
