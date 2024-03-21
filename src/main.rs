use computorv1::parse::get_equation;
use computorv1::Error;

/// # Errors
/// This function will return an error if it fails to read user input.
fn main() -> Result<(), Box<dyn Error>> {
    // let mut equation = initial_prompt()?;
    let input = "4 *X^0 + 8 * X^1 - 1 * X^2 = 2.5 * X^0 - 0 * X^1 + 4 * X^2".to_string();
    let _equation = get_equation(&input)?;
    let input: String = "5+4*X+X^2=X^2".to_string();
    let _equation = get_equation(&input)?;
    Ok(())
}
