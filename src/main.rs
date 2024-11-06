use hrynchuk_email_parser::parse_email;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an email to validate.");
        return;
    }
    let email = &args[1];
    match parse_email(email) {
        Ok(_) => println!("Email is valid."),
        Err(err) => println!("Invalid email: {}", err),
    }
}
