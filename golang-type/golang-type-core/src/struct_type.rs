use std::str;

use golang_parser::tree_sitter::Node;
use golang_struct_tag::{StructTag, StructTagParseError};

use crate::{golang_type_name_core::TypeName, PointerType, Type, TypeParseError};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StructType {
    pub field_decls: Vec<FieldDecl>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FieldDecl {
    pub struct_field: StructField,
    pub tag: Option<StructTag>,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum StructField {
    IdentifierListType(Vec<String>, Box<Type>),
    EmbeddedField(EmbeddedField),
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum EmbeddedField {
    TypeName(TypeName),
    PointerType(TypeName),
}
impl EmbeddedField {
    pub fn name(&self) -> String {
        match self {
            Self::TypeName(type_name) | Self::PointerType(type_name) => {
                TypeNameWrapper(type_name).struct_field_name()
            }
        }
    }
    pub fn r#type(&self) -> Type {
        match self {
            Self::TypeName(type_name) | Self::PointerType(type_name) => {
                Type::TypeName(type_name.to_owned())
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum StructTypeParseError {
    #[error("NodeMissing {0}")]
    NodeMissing(String),
    #[error("NodeKindUnknown {0}")]
    NodeKindUnknown(String),
    #[error("Utf8Error {0:?}")]
    Utf8Error(str::Utf8Error),
    #[error("UnexpectedType {0}")]
    UnexpectedType(String),
    #[error("DuplicateField {0}")]
    DuplicateField(String),

    #[error("StructTagParseError {0:?}")]
    StructTagParseError(#[from] StructTagParseError),
}
impl StructType {
    pub(crate) fn from_struct_type_node(node: Node, source: &[u8]) -> Result<Self, TypeParseError> {
        let node_field_declaration_list = node.named_child(0).ok_or_else(|| {
            StructTypeParseError::NodeMissing("field_declaration_list".to_string())
        })?;
        if node_field_declaration_list.kind() != "field_declaration_list" {
            return Err(
                StructTypeParseError::NodeMissing("field_declaration_list".to_string()).into(),
            );
        }
        let mut tree_cursor = node_field_declaration_list.walk();

        let mut field_decls = vec![];
        let mut non_blank_field_names = vec![];

        for node_field_declaration in node_field_declaration_list.named_children(&mut tree_cursor) {
            match node_field_declaration.kind() {
                "field_declaration" => {}
                "comment" => continue,
                _ => {
                    return Err(StructTypeParseError::NodeKindUnknown(
                        node_field_declaration.kind().to_owned(),
                    )
                    .into())
                }
            }

            // TODO, try parse
            // func()
            // map[string]int
            //

            let mut i = 0;
            let mut node_field_declaration_names = vec![];
            let node_field_declaration_type = loop {
                let node_field_declaration_name_or_type =
                    node_field_declaration.named_child(i).ok_or_else(|| {
                        StructTypeParseError::NodeMissing(
                            "field_declaration name or type".to_string(),
                        )
                    })?;
                i += 1;

                match node_field_declaration_name_or_type.kind() {
                    "field_identifier" => {
                        node_field_declaration_names.push(node_field_declaration_name_or_type);
                    }
                    _ => break node_field_declaration_name_or_type,
                }
            };

            let r#type = Type::from_node(node_field_declaration_type, source)?;

            let tag = if let Some(node_field_declaration_tag) =
                node_field_declaration.named_child(i)
            {
                match node_field_declaration_tag.kind() {
                    "raw_string_literal" => Some(
                        StructTag::from_raw_string_literal_node(node_field_declaration_tag, source)
                            .map_err(StructTypeParseError::StructTagParseError)?,
                    ),
                    "interpreted_string_literal" => Some(
                        StructTag::from_interpreted_string_literal_node(
                            node_field_declaration_tag,
                            source,
                        )
                        .map_err(StructTypeParseError::StructTagParseError)?,
                    ),
                    _ => {
                        return Err(StructTypeParseError::NodeKindUnknown(
                            node_field_declaration_tag.kind().to_owned(),
                        )
                        .into())
                    }
                }
            } else {
                None
            };

            if node_field_declaration_names.is_empty() {
                let embedded_field = match &r#type {
                    Type::TypeName(type_name) => EmbeddedField::TypeName(type_name.to_owned()),
                    Type::PointerType(PointerType(pointer_type_element)) => {
                        match **pointer_type_element {
                            Type::TypeName(ref type_name) => {
                                EmbeddedField::PointerType(type_name.to_owned())
                            }
                            _ => {
                                return Err(StructTypeParseError::UnexpectedType(format!(
                                    "unexpected type {:?}",
                                    pointer_type_element
                                ))
                                .into())
                            }
                        }
                    }
                    _ => {
                        return Err(StructTypeParseError::UnexpectedType(format!(
                            "unexpected type {:?}",
                            &r#type
                        ))
                        .into())
                    }
                };

                if non_blank_field_names.contains(&embedded_field.name()) {
                    return Err(StructTypeParseError::DuplicateField(format!(
                        "duplicate field {}",
                        &embedded_field.name()
                    ))
                    .into());
                }
                non_blank_field_names.push(embedded_field.name().to_owned());

                let field_decl = FieldDecl {
                    struct_field: StructField::EmbeddedField(embedded_field),
                    tag,
                };
                field_decls.push(field_decl);
            } else {
                let mut names = vec![];
                for node_field_declaration_name in node_field_declaration_names {
                    let name = node_field_declaration_name
                        .utf8_text(source)
                        .map_err(StructTypeParseError::Utf8Error)?
                        .to_owned();

                    if name != "_" {
                        if non_blank_field_names.contains(&name) {
                            return Err(StructTypeParseError::DuplicateField(format!(
                                "duplicate field {}",
                                name
                            ))
                            .into());
                        }
                        non_blank_field_names.push(name.to_owned());
                    }

                    names.push(name);
                }

                let field_decl = FieldDecl {
                    struct_field: StructField::IdentifierListType(names, r#type.into()),
                    tag: tag.to_owned(),
                };
                field_decls.push(field_decl);
            }
        }

        Ok(Self { field_decls })
    }
}

//
//
//
trait StructFieldName {
    fn struct_field_name(&self) -> String;
}
struct TypeNameWrapper<'a>(&'a TypeName);
impl StructFieldName for TypeNameWrapper<'_> {
    fn struct_field_name(&self) -> String {
        match self.0 {
            TypeName::QualifiedIdent(_, identifier_str) => identifier_str.to_owned(),
            _ => self.0.name(),
        }
    }
}
