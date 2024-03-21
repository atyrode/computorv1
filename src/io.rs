use std::io::{stdin, stdout, Write};

use crate::Error;

/// # Errors
/// This function will return an error if it fails flush stdout.
fn question_user(question: &str, prompt: &str) -> Result<(), Box<dyn Error>> {
    println!("{question}");
    print!("{prompt}");
    stdout().flush()?;
    Ok(())
}

/// # Errors
/// This function will return an error if it fails to read ustdin.
fn get_user_input() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input)
}

/// # Errors
/// This function will return an error if it fails to read user input or print and flush to stdout.
pub fn initial_prompt() -> Result<String, Box<dyn Error>> {
    question_user("Enter your polynomial equation:", "> ")?;
    get_user_input()
}
