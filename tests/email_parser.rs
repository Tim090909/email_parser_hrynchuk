use hrynchuk_email_parser::parse_email;

#[test]
fn valid_email() {
    let email = "user.name-123@example.com";
    assert!(parse_email(email).is_ok());
}

#[test]
fn invalid_email_missing_at() {
    let email = "username.example.com";
    assert!(parse_email(email).is_err());
}

#[test]
fn invalid_email_invalid_domain() {
    let email = "username@com";
    assert!(parse_email(email).is_err());
}

#[test]
fn valid_email_with_subdomain() {
    let email = "username@mail.example.com";
    assert!(parse_email(email).is_ok());
}

#[test]
fn invalid_email_with_invalid_characters() {
    let email = "user!name@example.com";
    assert!(parse_email(email).is_err());
}
