use anyhow::{Context, Result};
use email_pest_parser::ParsedEmail;

#[test]
fn test_parse_email() {
    let email = "From: sender@example.com\nTo: recipient@example.com\n\nThis is the body.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_headers() {
    let email = "From: sender@example.com\nTo: recipient@example.com\n\nBody content.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.headers.len(), 2);
    assert_eq!(
        parsed.headers[0],
        ("From".to_string(), "sender@example.com".to_string())
    );
    assert_eq!(
        parsed.headers[1],
        ("To".to_string(), "recipient@example.com".to_string())
    );
}

#[test]
fn test_parse_header_line() {
    let email = "From: sender@example.com\nTo: recipient@example.com\nX-Test-Header: some-value\n\nBody content.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(
        parsed.headers[2],
        ("X-Test-Header".to_string(), "some-value".to_string())
    );
}

#[test]
fn test_parse_field_name() {
    let email =
        "From: sender@example.com\nTo: recipient@example.com\nCustom-Field: Value\n\nBody content.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.headers[2].0, "Custom-Field");
}

#[test]
fn test_parse_field_value() {
    let email = "From: sender@example.com\nTo: recipient@example.com\nField: Value with symbols!@#\n\nBody content.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.headers[2].1, "Value with symbols!@#");
}

#[test]
fn test_parse_body() {
    let email = "From: sender@example.com\nTo: recipient@example.com\nSubject: Testing\n\nThis is the email body.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.body, "This is the email body.");
}

#[test]
fn test_parse_email_address() {
    let email = "From: sender@example.com\nTo: recipient@example.com\n\nThis is the email body.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());
}

#[test]
fn test_parse_username() {
    let email = "From: user_name123@example.com\nTo: recipient@example.com\n\nBody content.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.headers[0].1, "user_name123@example.com");
}

#[test]
fn test_parse_domain() {
    let email = "From: sender@example.com\nTo: recipient@example.com\n\nBody content.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.headers[1].1, "recipient@example.com");
}

#[test]
fn test_parse_subdomain() {
    let email = "From: sender@example.com\nTo: recipient@subdomain.example.com\n\nBody content.";
    let parsed = ParsedEmail::from_email(email);
    assert!(parsed.is_ok());

    let parsed = parsed.unwrap();
    assert_eq!(parsed.headers[1].1, "recipient@subdomain.example.com");
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;
    use email_pest_parser::{EmailParseError, ParsedEmail};

    #[test]
    fn test_parse_valid_email() -> Result<()> {
        let email = "From: sender@example.com\nTo: recipient@example.com\n\nBody content.";
        let parsed = ParsedEmail::from_email(email).context("Expected successful email parsing")?;
        assert_eq!(parsed.email_addresses.len(), 2);
        Ok(())
    }

    #[test]
    fn test_invalid_email_missing_at() {
        let email = "To: recipientexample.com\n\nBody content.";
        let parsed = ParsedEmail::from_email(email);

        assert!(
            matches!(parsed, Err(EmailParseError::InvalidEmailAddress(_))),
            "Expected InvalidEmailAddress error but got: {:?}",
            parsed
        );
    }

    #[test]
    fn test_missing_from_field() {
        let email =
            "From: sender@example.com\nTo: recipient@example.com\nSubject: \n\nBody content.";
        let parsed = ParsedEmail::from_email(email);

        assert!(
            matches!(parsed, Err(EmailParseError::ParseError(_))),
            "Expected ParseError error but got: {:?}",
            parsed
        );
    }

    #[test]
    fn test_missing_from_to() {
        let email = "From: sender@example.com\n\nBody content.";
        let parsed = ParsedEmail::from_email(email);

        assert!(
            matches!(parsed, Err(EmailParseError::MissingField(_))),
            "Expected MissingField error but got: {:?}",
            parsed
        );
    }
}
