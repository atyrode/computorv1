use crate::{Equation, Term};

fn simplify(mut lhs: Vec<Term>, mut rhs: Vec<Term>) -> (Vec<Term>, Vec<Term>) {
    println!("o) First, we simplify each term.");

    for term in lhs.iter_mut() {
        let pre_simplify = format!("{}", term);

        println!("{pre_simplify} => {term}");
    }
    
    (lhs, rhs)
}

pub fn solve_equation(equation: Equation) -> String {
    let lhs: Vec<Term> = equation.left;
    let rhs: Vec<Term> = equation.right;
    let (simplified_lhs, simplified_rhs) = simplify(lhs, rhs);

    let solved_equation = Equation { left: simplified_lhs, right: simplified_rhs };

    format!("{}", solved_equation)
}