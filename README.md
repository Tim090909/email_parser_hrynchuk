# email_parser_hrynchuk

**email_parser_hrynchuk** is a Rust tool for parsing and analyzing complete email structures. Using `pest` grammar rules, this parser extracts key email components, making it ideal for validation and automated email processing.

## Parsing Process

The parser in `src/grammar.pest` breaks down an email into the following components:

- **Email Address**: Extracts and validates addresses in `username@domain` format.
  - **Username**: Allows alphanumeric characters, underscores, dots, and hyphens.
  - **Domain**: Composed of one or more subdomains separated by dots.
- **Headers**: Captures key headers like `To`, `From`, `Subject`, and `Date`.
- **Body**: Processes plain text and basic HTML content for the main body of the email.

This structure allows for detailed email parsing, enabling applications to analyze headers, validate email addresses, and manage body content efficiently.

## Usage

The command-line tool can parse entire emails provided as input:
```bash
cargo run -- "path/to/email_file.eml"
