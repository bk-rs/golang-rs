use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/convention_struct_tag.pest"]
pub(crate) struct ConventionStructTagParser;
