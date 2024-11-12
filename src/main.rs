use email_pest_parser::ParsedEmail;
use std::env;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "help" => print_help(),
        "credits" => print_credits(),
        file_path => {
            let content = fs::read_to_string(file_path)?;
            match ParsedEmail::from_email(&content) {
                Ok(parsed_email) => println!("{:#?}", parsed_email),
                Err(e) => eprintln!("Error parsing email: {}", e),
            }
        }
    }

    Ok(())
}

fn print_help() {
    println!("Email Parser Usage:");
    println!("  cargo run <email_file>      Parses an email file.");
    println!("  cargo run help              Displays help information.");
    println!("  cargo run credits           Shows project credits.");
}

fn print_credits() {
    println!("Email Parser");
    println!("Created by Tymofii Hrynchuk");
}
