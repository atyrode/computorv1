use regex::Regex;

use crate::Error;
use crate::{Equation, Term};

fn remove_spaces(equation: &str) -> String {
    equation.replace(' ', "")
}

fn get_sides(equation: &str) -> Vec<&str> {
    equation.split('=').collect()
}

fn get_terms(side: &str) -> Result<Vec<Term>, Box<dyn Error>> {
    let re = Regex::new(
        r"(?<coefficient>[+-]?\d(?:\.\d+)?)?(?:[\*+-]?(?<variable>[Xx](?:\^(?<exponent>\d))?))?",
    )?;

    let mut terms: Vec<Term> = Vec::new();

    for cap in re.captures_iter(side) {
        let mut term = Term::default();

        if let Some(coefficient) = cap.name("coefficient") {
            term.coefficient = coefficient.as_str().parse::<f64>()?;
        }
        if let Some(exponent) = cap.name("exponent") {
            term.power = exponent.as_str().parse::<i32>()?;
        }

        terms.push(term);
    }
    Ok(terms)
}

/// # Errors
/// This function will return an error if the equation is invalid.
pub fn get_equation(user_input: &str) -> Result<Equation, Box<dyn Error>> {
    let mut equation = Equation::default();

    let trimmed = remove_spaces(user_input);

    // Return error if there are no equal signs in the equation
    let sides: Vec<&str> = match get_sides(&trimmed) {
        sides if sides.len() != 2 => return Err("Invalid equation".into()),
        sides => sides,
    };

    equation.left_side = get_terms(sides[0])?;
    equation.right_side = get_terms(sides[1])?;

    println!("{equation}");
    Ok(equation)
}
