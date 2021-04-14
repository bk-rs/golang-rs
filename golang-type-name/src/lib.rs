use std::str::{self, FromStr};

use proc_macro2::{Punct, Spacing, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};
use tree_sitter::Node;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TypeName {
    // https://golang.org/ref/spec#Boolean_types
    // https://github.com/golang/go/blob/go1.16.3/src/builtin/builtin.go#L13-L14
    Bool,
    // https://golang.org/ref/spec#Numeric_types
    // https://github.com/golang/go/blob/go1.16.3/src/builtin/builtin.go#L22-L66
    // https://github.com/golang/go/blob/go1.16.3/src/builtin/builtin.go#L73-L92
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64,
    Complex64,
    Complex128,
    Uint,
    Int,
    Uintptr,
    // https://golang.org/ref/spec#String_types
    // https://github.com/golang/go/blob/go1.16.3/src/builtin/builtin.go#L68-L71
    String,
    // https://golang.org/ref/spec#QualifiedIdent
    QualifiedIdent(PackageName, String),
    //
    Identifier(String),
}
pub type PackageName = String;

#[derive(thiserror::Error, Debug)]
pub enum TypeNameParseError {
    #[error("TreeSitterLanguageError {0}")]
    TreeSitterLanguageError(String),
    #[error("TreeSitterParseFailed {0}")]
    TreeSitterParseFailed(String),
    #[error("Utf8Error {0:?}")]
    Utf8Error(str::Utf8Error),
    #[error("UnsupportedType {0}")]
    UnsupportedType(String),
}

impl FromStr for TypeName {
    type Err = TypeNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_go::language())
            .map_err(|err| TypeNameParseError::TreeSitterLanguageError(err.to_string()))?;

        let code = format!("var _ {};", s);

        let tree = parser.parse(&code, None).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed("Not found tree".to_string())
        })?;
        let mut tree_cursor = tree.walk();
        let source = code.as_bytes();
        let node_source_file = tree.root_node();

        let node_var_declaration = node_source_file
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "var_declaration")
            .ok_or_else(|| {
                TypeNameParseError::TreeSitterParseFailed("Not found var_declaration".to_string())
            })?;
        let node_var_spec = node_var_declaration
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "var_spec")
            .ok_or_else(|| {
                TypeNameParseError::TreeSitterParseFailed("Not found var_spec".to_string())
            })?;

        let _ = node_var_spec.named_child(0).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed("Not found var_spec name".to_string())
        })?;
        let node_var_spec_type = node_var_spec.named_child(1).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed("Not found var_spec type".to_string())
        })?;

        match node_var_spec_type.kind() {
            "qualified_type" => Self::from_qualified_type_node(node_var_spec_type, source),
            "type_identifier" => Self::from_type_identifier_node(node_var_spec_type, source),
            _ => Err(TypeNameParseError::UnsupportedType(
                node_var_spec_type.kind().to_owned(),
            )),
        }
    }
}

impl TypeName {
    pub fn from_qualified_type_node(node: Node, source: &[u8]) -> Result<Self, TypeNameParseError> {
        let node_qualified_type_package = node.named_child(0).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed(
                "Not found qualified_type package".to_string(),
            )
        })?;
        let node_qualified_type_name = node.named_child(1).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed("Not found qualified_type name".to_string())
        })?;

        let package_str = node_qualified_type_package
            .utf8_text(source)
            .map_err(TypeNameParseError::Utf8Error)?;
        let name_str = node_qualified_type_name
            .utf8_text(source)
            .map_err(TypeNameParseError::Utf8Error)?;

        Ok(Self::QualifiedIdent(
            package_str.to_owned(),
            name_str.to_owned(),
        ))
    }

    pub fn from_type_identifier_node(
        node: Node,
        source: &[u8],
    ) -> Result<Self, TypeNameParseError> {
        let s = node
            .utf8_text(source)
            .map_err(TypeNameParseError::Utf8Error)?;

        match s {
            //
            "bool" => Ok(Self::Bool),
            //
            "uint8" | "byte" => Ok(Self::Uint8),
            "uint16" => Ok(Self::Uint16),
            "uint32" => Ok(Self::Uint32),
            "uint64" => Ok(Self::Uint64),
            "int8" => Ok(Self::Int8),
            "int16" => Ok(Self::Int16),
            "int32" | "rune" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            "float32" => Ok(Self::Float32),
            "float64" => Ok(Self::Float64),
            "complex64" => Ok(Self::Complex64),
            "complex128" => Ok(Self::Complex128),
            "uint" => Ok(Self::Uint),
            "int" => Ok(Self::Int),
            "uintptr" => Ok(Self::Uintptr),
            //
            "string" => Ok(Self::String),
            //
            _ => Ok(Self::Identifier(s.to_owned())),
        }
    }
}

impl ToTokens for TypeName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Bool => tokens.append_all(quote!(::core::primitive::bool)),
            Self::Uint8 => tokens.append_all(quote!(::core::primitive::u8)),
            Self::Uint16 => tokens.append_all(quote!(::core::primitive::u16)),
            Self::Uint32 => tokens.append_all(quote!(::core::primitive::u32)),
            Self::Uint64 => tokens.append_all(quote!(::core::primitive::u64)),
            Self::Int8 => tokens.append_all(quote!(::core::primitive::i8)),
            Self::Int16 => tokens.append_all(quote!(::core::primitive::i16)),
            Self::Int32 => tokens.append_all(quote!(::core::primitive::i32)),
            Self::Int64 => tokens.append_all(quote!(::core::primitive::i64)),
            Self::Float32 => tokens.append_all(quote!(::core::primitive::f32)),
            Self::Float64 => tokens.append_all(quote!(::core::primitive::f64)),
            Self::Complex64 => tokens.append_all(quote!(::num_complex::Complex32)),
            Self::Complex128 => tokens.append_all(quote!(::num_complex::Complex64)),
            Self::Uint => tokens.append_all(quote!(::core::primitive::usize)),
            Self::Int => tokens.append_all(quote!(::core::primitive::isize)),
            Self::Uintptr => tokens.append_all(quote!(::core::primitive::usize)),
            Self::String => tokens.append_all(quote!(::std::string::String)),
            Self::QualifiedIdent(package_str, identifier_str) => {
                let package_ident = format_ident!("{}", package_str);
                let identifier_ident = format_ident!("{}", identifier_str);

                tokens.append(Punct::new(':', Spacing::Joint));
                tokens.append(Punct::new(':', Spacing::Alone));
                tokens.append_all(quote!(#package_ident));
                tokens.append(Punct::new(':', Spacing::Joint));
                tokens.append(Punct::new(':', Spacing::Alone));
                tokens.append_all(quote!(#identifier_ident));
            }
            Self::Identifier(identifier_str) => {
                let identifier_ident = format_ident!("{}", identifier_str);

                tokens.append_all(quote!(#identifier_ident));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    #[test]
    fn test_parse() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/type_names.txt"))?;
        for (i, str) in content.lines().enumerate() {
            match i + 1 {
                //
                1 => assert_eq!(TypeName::Bool, str.parse()?),
                //
                2 => assert_eq!(TypeName::Uint8, str.parse()?),
                3 => assert_eq!(TypeName::Uint16, str.parse()?),
                4 => assert_eq!(TypeName::Uint32, str.parse()?),
                5 => assert_eq!(TypeName::Uint64, str.parse()?),
                6 => assert_eq!(TypeName::Int8, str.parse()?),
                7 => assert_eq!(TypeName::Int16, str.parse()?),
                8 => assert_eq!(TypeName::Int32, str.parse()?),
                9 => assert_eq!(TypeName::Int64, str.parse()?),
                10 => assert_eq!(TypeName::Float32, str.parse()?),
                11 => assert_eq!(TypeName::Float64, str.parse()?),
                12 => assert_eq!(TypeName::Complex64, str.parse()?),
                13 => assert_eq!(TypeName::Complex128, str.parse()?),
                14 => assert_eq!(TypeName::Uint8, str.parse()?),
                15 => assert_eq!(TypeName::Int32, str.parse()?),
                16 => assert_eq!(TypeName::Uint, str.parse()?),
                17 => assert_eq!(TypeName::Int, str.parse()?),
                18 => assert_eq!(TypeName::Uintptr, str.parse()?),
                //
                19 => assert_eq!(TypeName::String, str.parse()?),
                //
                20 => assert_eq!(
                    TypeName::QualifiedIdent("time".to_owned(), "Duration".to_owned()),
                    str.parse()?
                ),
                //
                21 => assert_eq!(TypeName::Identifier("foo".to_owned()), str.parse()?),
                _ => assert!(false),
            }
        }

        Ok(())
    }
}
