use std::str::{self, FromStr};

use golang_parser::{tree_sitter::Node, Parser};

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
    Byte,
    Rune,
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
    #[error("GolangParserError {0:?}")]
    GolangParserError(#[from] golang_parser::Error),
    #[error("NodeMissing {0}")]
    NodeMissing(String),
    #[error("NodeKindUnknown {0}")]
    NodeKindUnknown(String),
    #[error("Utf8Error {0:?}")]
    Utf8Error(str::Utf8Error),
    #[error("IdentifierMissing")]
    IdentifierMissing,
}

impl FromStr for TypeName {
    type Err = TypeNameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = Parser::new(format!("var _ {}", s))?;
        let source = parser.get_source();
        let root_node = parser.get_root_node();

        let mut cursor = root_node.walk();

        let node_var_declaration = root_node
            .named_children(&mut cursor)
            .find(|node| node.kind() == "var_declaration")
            .ok_or_else(|| TypeNameParseError::NodeMissing("var_declaration".to_string()))?;
        let node_var_spec = node_var_declaration
            .named_children(&mut cursor)
            .find(|node| node.kind() == "var_spec")
            .ok_or_else(|| TypeNameParseError::NodeMissing("var_spec".to_string()))?;

        let _ = node_var_spec
            .named_child(0)
            .ok_or_else(|| TypeNameParseError::NodeMissing("var_spec name".to_string()))?;
        let node_var_spec_type = node_var_spec
            .named_child(1)
            .ok_or_else(|| TypeNameParseError::NodeMissing("var_spec type".to_string()))?;

        match node_var_spec_type.kind() {
            "qualified_type" => Self::from_qualified_type_node(node_var_spec_type, source),
            "type_identifier" => Self::from_type_identifier_node(node_var_spec_type, source),
            _ => Err(TypeNameParseError::NodeKindUnknown(
                node_var_spec_type.kind().to_owned(),
            )),
        }
    }
}

impl TypeName {
    pub fn from_qualified_type_node(node: Node, source: &[u8]) -> Result<Self, TypeNameParseError> {
        let node_qualified_type_package = node
            .named_child(0)
            .ok_or_else(|| TypeNameParseError::NodeMissing("qualified_type package".to_string()))?;
        let node_qualified_type_name = node
            .named_child(1)
            .ok_or_else(|| TypeNameParseError::NodeMissing("qualified_type name".to_string()))?;

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

        if s.is_empty() {
            return Err(TypeNameParseError::IdentifierMissing);
        }

        match s {
            //
            "bool" => Ok(Self::Bool),
            //
            "uint8" => Ok(Self::Uint8),
            "uint16" => Ok(Self::Uint16),
            "uint32" => Ok(Self::Uint32),
            "uint64" => Ok(Self::Uint64),
            "int8" => Ok(Self::Int8),
            "int16" => Ok(Self::Int16),
            "int32" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            "float32" => Ok(Self::Float32),
            "float64" => Ok(Self::Float64),
            "complex64" => Ok(Self::Complex64),
            "complex128" => Ok(Self::Complex128),
            "byte" => Ok(Self::Byte),
            "rune" => Ok(Self::Rune),
            "uint" => Ok(Self::Uint),
            "int" => Ok(Self::Int),
            "uintptr" => Ok(Self::Uintptr),
            //
            "string" => Ok(Self::String),
            //
            _ => Ok(Self::Identifier(s.to_owned())),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::Bool => "bool".to_owned(),
            Self::Uint8 => "uint8".to_owned(),
            Self::Uint16 => "uint16".to_owned(),
            Self::Uint32 => "uint32".to_owned(),
            Self::Uint64 => "uint64".to_owned(),
            Self::Int8 => "int8".to_owned(),
            Self::Int16 => "int16".to_owned(),
            Self::Int32 => "int32".to_owned(),
            Self::Int64 => "int64".to_owned(),
            Self::Float32 => "float32".to_owned(),
            Self::Float64 => "float64".to_owned(),
            Self::Complex64 => "complex64".to_owned(),
            Self::Complex128 => "complex128".to_owned(),
            Self::Byte => "byte".to_owned(),
            Self::Rune => "rune".to_owned(),
            Self::Uint => "uint".to_owned(),
            Self::Int => "int".to_owned(),
            Self::Uintptr => "uintptr".to_owned(),
            Self::String => "string".to_owned(),
            Self::QualifiedIdent(package_str, identifier_str) => {
                format!("{}.{}", package_str, identifier_str)
            }
            Self::Identifier(identifier_str) => identifier_str.to_owned(),
        }
    }
}

#[cfg(feature = "enable-quote-to_tokens")]
mod enable_quote_to_tokens {
    use super::TypeName;

    use proc_macro2::{Punct, Spacing, TokenStream};
    use quote::{format_ident, quote, ToTokens, TokenStreamExt as _};

    impl ToTokens for TypeName {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                Self::Bool => tokens.append_all(quote!(::core::primitive::bool)),
                Self::Uint8 | Self::Byte => tokens.append_all(quote!(::core::primitive::u8)),
                Self::Uint16 => tokens.append_all(quote!(::core::primitive::u16)),
                Self::Uint32 => tokens.append_all(quote!(::core::primitive::u32)),
                Self::Uint64 => tokens.append_all(quote!(::core::primitive::u64)),
                Self::Int8 => tokens.append_all(quote!(::core::primitive::i8)),
                Self::Int16 => tokens.append_all(quote!(::core::primitive::i16)),
                Self::Int32 | Self::Rune => tokens.append_all(quote!(::core::primitive::i32)),
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
                1 => {
                    assert_eq!(TypeName::Bool, str.parse()?);
                    assert_eq!(TypeName::Bool.name(), str)
                }
                //
                2 => {
                    assert_eq!(TypeName::Uint8, str.parse()?);
                    assert_eq!(TypeName::Uint8.name(), str)
                }
                3 => {
                    assert_eq!(TypeName::Uint16, str.parse()?);
                    assert_eq!(TypeName::Uint16.name(), str)
                }
                4 => {
                    assert_eq!(TypeName::Uint32, str.parse()?);
                    assert_eq!(TypeName::Uint32.name(), str)
                }
                5 => {
                    assert_eq!(TypeName::Uint64, str.parse()?);
                    assert_eq!(TypeName::Uint64.name(), str)
                }
                6 => {
                    assert_eq!(TypeName::Int8, str.parse()?);
                    assert_eq!(TypeName::Int8.name(), str)
                }
                7 => {
                    assert_eq!(TypeName::Int16, str.parse()?);
                    assert_eq!(TypeName::Int16.name(), str)
                }
                8 => {
                    assert_eq!(TypeName::Int32, str.parse()?);
                    assert_eq!(TypeName::Int32.name(), str)
                }
                9 => {
                    assert_eq!(TypeName::Int64, str.parse()?);
                    assert_eq!(TypeName::Int64.name(), str)
                }
                10 => {
                    assert_eq!(TypeName::Float32, str.parse()?);
                    assert_eq!(TypeName::Float32.name(), str)
                }
                11 => {
                    assert_eq!(TypeName::Float64, str.parse()?);
                    assert_eq!(TypeName::Float64.name(), str)
                }
                12 => {
                    assert_eq!(TypeName::Complex64, str.parse()?);
                    assert_eq!(TypeName::Complex64.name(), str)
                }
                13 => {
                    assert_eq!(TypeName::Complex128, str.parse()?);
                    assert_eq!(TypeName::Complex128.name(), str)
                }
                14 => {
                    assert_eq!(TypeName::Byte, str.parse()?);
                    assert_eq!(TypeName::Byte.name(), str)
                }
                15 => {
                    assert_eq!(TypeName::Rune, str.parse()?);
                    assert_eq!(TypeName::Rune.name(), str)
                }
                16 => {
                    assert_eq!(TypeName::Uint, str.parse()?);
                    assert_eq!(TypeName::Uint.name(), str)
                }
                17 => {
                    assert_eq!(TypeName::Int, str.parse()?);
                    assert_eq!(TypeName::Int.name(), str)
                }
                18 => {
                    assert_eq!(TypeName::Uintptr, str.parse()?);
                    assert_eq!(TypeName::Uintptr.name(), str)
                }
                //
                19 => {
                    assert_eq!(TypeName::String, str.parse()?);
                    assert_eq!(TypeName::String.name(), str)
                }
                //
                20 => {
                    assert_eq!(
                        TypeName::QualifiedIdent("time".to_owned(), "Duration".to_owned()),
                        str.parse()?
                    );
                    assert_eq!(
                        TypeName::QualifiedIdent("time".to_owned(), "Duration".to_owned()).name(),
                        str
                    )
                }
                //
                21 => {
                    assert_eq!(TypeName::Identifier("foo".to_owned()), str.parse()?);
                    assert_eq!(TypeName::Identifier("foo".to_owned()).name(), str)
                }
                _ => assert!(false),
            }
        }

        Ok(())
    }

    #[test]
    fn test_parse_with_identifier_missing() {
        match "".parse::<TypeName>() {
            Ok(_) => assert!(false),
            Err(TypeNameParseError::IdentifierMissing) => {}
            Err(err) => assert!(false, "{:?}", err),
        }
    }
}
