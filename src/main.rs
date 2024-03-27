use computorv1::parse::get_equation;
use computorv1::Error;
use computorv1::solve_equation;

/// # Errors
/// This function will return an error if it fails to read user input.
fn main() -> Result<(), Box<dyn Error>> {
    // let mut equation = initial_prompt()?;
    let input = "-4 * X^0 + 8 * X^1 - 1 * x^2 = -2.5 * X^0 - 0 * X^1 + 4 * X^2".to_string();
    let equation = get_equation(&input)?;
    println!("Input:\n{input}");
    println!("Your equation is:\n{equation}");
    println!("=====================================");
    println!("Solving");
    println!("=====================================");
    let solved = solve_equation(equation);
    println!("=====================================");
    println!("Solved:");
    println!("{solved}");
    Ok(())
}
