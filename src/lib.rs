use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct EmailParser;

#[derive(Debug)]
pub struct ParsedEmail {
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub email_addresses: Vec<String>,
}

#[derive(Debug, Error)]
pub enum EmailParseError {
    #[error("Parsing error occurred: {0}")]
    ParseError(String),
    #[error("Invalid email address: {0}")]
    InvalidEmailAddress(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
}

impl ParsedEmail {
    pub fn from_email(input: &str) -> Result<Self, EmailParseError> {
        let pairs = EmailParser::parse(Rule::email, input)
            .map_err(|e| EmailParseError::ParseError(e.to_string()))?;

        let mut headers = Vec::new();
        let mut body = String::new();
        let mut email_addresses = Vec::new();

        let email_pair = pairs.peek().unwrap();
        for pair in email_pair.into_inner() {
            match pair.as_rule() {
                Rule::headers => {
                    for header_pair in pair.into_inner() {
                        let mut inner_rules = header_pair.into_inner();
                        let name = inner_rules.next().unwrap().as_str().to_string();
                        let value = inner_rules.next().unwrap().as_str().to_string();

                        if name == "From" || name == "To" {
                            if EmailParser::parse(Rule::email_address, &value).is_err() {
                                return Err(EmailParseError::InvalidEmailAddress(value));
                            }
                            email_addresses.push(value.clone());
                        } else {
                            if (!name.is_empty() && value.is_empty())
                                || (name.is_empty() && !value.is_empty())
                            {
                                return Err(EmailParseError::MissingField(name));
                            }
                        }

                        headers.push((name, value));
                    }
                }
                Rule::body => body = pair.as_str().to_string(),
                _ => {}
            }
        }

        if email_addresses.len() < 2 {
            return Err(EmailParseError::MissingField(
                "Both 'From' and 'To' fields are required".to_string(),
            ));
        }

        Ok(ParsedEmail {
            headers,
            body,
            email_addresses,
        })
    }
}
