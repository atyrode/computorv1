use std::io::{self, Error, Write};

fn question_user(question: &str, prompt: &str) -> Result<(), Error> {
    println!("{question}");
    print!("{prompt}");
    io::stdout().flush()?;
    Ok(())
}

fn get_user_input() -> Result<String, Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

pub fn initial_prompt() -> Result<String, Error> {
    question_user("Enter your polynomial equation:", "> ")?;
    get_user_input()
}
