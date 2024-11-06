use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct EmailParser;

pub fn parse_email(input: &str) -> anyhow::Result<()> {
    EmailParser::parse(Rule::email, input).map(|_| ())?;
    Ok(())
}
