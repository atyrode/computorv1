mod io;
use io::initial_prompt;


fn computorv1() -> String {
    let equation: String;

    match initial_prompt() {
        Ok(user_input) => equation = user_input,
        Err(err) => return format!("{}", err),
    }

    equation
}

fn main() {
    loop {
        println!("{}", "-".repeat(80));
        let result: String = computorv1();
        println!("{}", result);
    }
}
