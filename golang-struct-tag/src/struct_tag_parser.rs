use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/struct_tag.pest"]
pub(crate) struct StructTagParser;
