use crate::convention::ConventionStructTag;

pub enum StructTag {
    RawStringLiteral(String),
    InterpretedStringLiteral(String),
    Convention(Vec<ConventionStructTag>),
}
