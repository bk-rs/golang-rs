pub use golang_type;

use std::str::FromStr;

use golang_parser::Parser;

pub mod alias_decl;
pub mod type_def;

pub use self::alias_decl::{AliasDecl, AliasDeclParseError};
pub use self::type_def::{TypeDef, TypeDefParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct TypeDecl {
    pub type_specs: Vec<TypeSpec>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeSpec {
    AliasDecl(AliasDecl),
    TypeDef(TypeDef),
}

#[derive(thiserror::Error, Debug)]
pub enum TypeDeclParseError {
    #[error("GolangParserError {0:?}")]
    GolangParserError(#[from] golang_parser::Error),
    #[error("NodeMissing {0}")]
    NodeMissing(&'static str),
    #[error("NodeKindUnknown {0}")]
    NodeKindUnknown(String),
    //
    #[error("AliasDeclParseError {0:?}")]
    AliasDeclParseError(#[from] AliasDeclParseError),
    #[error("TypeDefParseError {0:?}")]
    TypeDefParseError(#[from] TypeDefParseError),
}

impl FromStr for TypeDecl {
    type Err = TypeDeclParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = Parser::new(s)?;

        let source = parser.get_source();
        let node_source_file = parser.get_root_node();
        let mut tree_cursor = node_source_file.walk();

        let node_type_declaration = node_source_file
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "type_declaration")
            .ok_or(TypeDeclParseError::NodeMissing("type_declaration"))?;

        let mut type_specs = vec![];
        for node in node_type_declaration.named_children(&mut tree_cursor) {
            match node.kind() {
                "type_alias" => {
                    let type_spec =
                        TypeSpec::AliasDecl(AliasDecl::from_type_alias_node(node, source)?);
                    type_specs.push(type_spec);
                }
                "type_spec" => {
                    let type_spec = TypeSpec::TypeDef(TypeDef::from_type_spec_node(node, source)?);
                    type_specs.push(type_spec);
                }
                "comment" => continue,
                _ => return Err(TypeDeclParseError::NodeKindUnknown(node.kind().to_owned())),
            }
        }

        Ok(Self { type_specs })
    }
}
