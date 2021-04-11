use std::str::FromStr;

use pest::Parser as _;

use crate::{
    convention::ConventionStructTag,
    error::ParseError,
    struct_tag_parser::{Rule, StructTagParser},
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum StructTag {
    RawStringLiteral(String),
    InterpretedStringLiteral(String),
    Convention(Vec<ConventionStructTag>),
}

impl FromStr for StructTag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match StructTagParser::parse(Rule::convention, s) {
            Ok(mut pair) => {
                let pair = pair.next().ok_or(ParseError::Unknown)?;

                match pair.as_rule() {
                    Rule::convention => {
                        ConventionStructTag::from_convention_pairs(pair.into_inner())
                            .map(Self::Convention)
                    }
                    _ => Err(ParseError::Unknown),
                }
            }
            Err(_) if s.starts_with('`') && s.ends_with('`') => {
                Ok(Self::RawStringLiteral(s.to_owned()))
            }
            Err(_) if s.starts_with('"') && s.ends_with('"') => {
                Ok(Self::InterpretedStringLiteral(s.to_owned()))
            }
            Err(err) => Err(ParseError::FormatMismatch(err.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error;

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        assert_eq!(
            StructTag::RawStringLiteral(r#"`foo"bar`"#.to_owned()),
            r#"`foo"bar`"#.parse()?
        );

        assert_eq!(
            StructTag::InterpretedStringLiteral(r#""foo`bar""#.to_owned()),
            r#""foo`bar""#.parse()?
        );

        assert_eq!(
            StructTag::Convention(vec![ConventionStructTag::Unknown(
                "foo".to_owned(),
                "bar".to_owned()
            )]),
            r#"`foo:"bar"`"#.parse()?
        );

        Ok(())
    }
}
