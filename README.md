# Email Pest Parser

*Link:* https://crates.io/crates/email_pest_parser  
*Docs:* https://docs.rs/email_pest_parser/latest/email_pest_parser/

A Rust-based email parser using the `pest` parsing library, designed to extract and validate various components such as headers, email addresses, and the body content of an email.

## Parsing Logic

The grammar is defined using `pest` and covers the following components:

- **Headers**: Key-value pairs that represent the metadata of an email, such as "From," "To," etc.
- **Email Addresses**: Extracted from specific headers like "From" and "To." Supports standard formats for usernames and domains.
- **Body**: The main content of the email, supporting multiple lines and any type of character.

### How It Works

The parser processes emails by breaking them into components using the predefined grammar rules. Then, it encapsulates the results in a structured `ParsedEmail` object for easy access to headers, email addresses, and body content.

## Example

```text
From: sender@example.com
To: recipient@example.com
Subject: Meeting Update

Hello,

This is a reminder for our meeting scheduled tomorrow at 10 AM.
Please let us know if you have any questions.

Best regards,
Sender
```

## Result

```text
From: sender@example.com
To: recipient@example.com
Subject: Meeting Update

Hello,

This is a reminder for our meeting scheduled tomorrow at 10 AM.
Please let us know if you have any questions.

Best regards,
Sender
```

### Grammar

```text
ParsedEmail {
    headers: [
        (
            "From",
            "sender@example.com",
        ),
        (
            "To",
            "recipient@example.com",
        ),
        (
            "Subject",
            "Meeting Update",
        ),
    ],
    body: "Hello,\r\n\r\nThis is a reminder for our meeting scheduled tomorrow at 10 AM.\r\nPlease let us know if you have any questions.\r\n\r\nBest regards,\r\nSender",
    email_addresses: [
        "sender@example.com",
        "recipient@example.com",
    ],
}
```

### email

```
email = { headers ~ NEWLINE ~ body }
```

The email rule consists of two main parts: the `headers` and the `body`. These are separated by a `NEWLINE`. The `headers` are a series of header lines, and the `body` contains the actual message content.

### headers

```
headers = { (header_line ~ NEWLINE)* }
```

The `headers` rule consists of one or more `header_line` rules, each followed by a `NEWLINE`.

### header_line

```
header_line = { field_name ~ ": " ~ field_value }
```

A `header_line` consists of a `field_name`, followed by a colon and a space (`": "`), and then a `field_value`. The `field_name` represents the name of the header (e.g., `Subject`, `From`), and the `field_value` represents the value of the header.

### field_name

```
field_name = { (ASCII_ALPHANUMERIC | "-" | "_")+ }
```

A `field_name` can consist of one or more characters, which can be ASCII alphanumeric characters (letters and numbers), or the special characters `"-"` (hyphen) or `"_"` (underscore).

### field_value

```
field_value = { (!NEWLINE ~ ANY)+ }
```

A `field_value` consists of any characters except a `NEWLINE`. The `ANY` rule matches any character, and the value can have one or more characters.

### body

```
body = { (!EOI ~ ANY)* }
```

The `body` rule matches any character (`ANY`) except the end of input (`EOI`), repeated zero or more times. This rule defines the actual content of the email after the headers section.

### email_address

```
email_address = { username ~ "@" ~ domain }
```

An `email_address` consists of a `username`, followed by the "@" symbol, and then a `domain`. The domain is further broken down into subdomains.

### username

```
username = { (ASCII_ALPHANUMERIC | "_" | "." | "-")+ }
```

A `username` can consist of one or more characters, which can be ASCII alphanumeric characters, or the special characters `"_", ".", and "-"`.

### domain

```
domain = { subdomain ~ ("." ~ subdomain)+ }
```

A `domain` consists of one or more `subdomain` rules, separated by periods (`"."`). Each subdomain is defined by the `subdomain` rule.

### subdomain

```
subdomain = { ASCII_ALPHANUMERIC+ }
```

A `subdomain` consists of one or more ASCII alphanumeric characters. Subdomains are typically used in the domain name (e.g., `gmail` in `gmail.com`).
