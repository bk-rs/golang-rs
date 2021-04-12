use std::str::{self, FromStr};

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

        let node_type_declaration = node_source_file
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "var_declaration")
            .ok_or_else(|| {
                TypeNameParseError::TreeSitterParseFailed("Not found var_declaration".to_string())
            })?;
        let node_var_spec = node_type_declaration
            .named_children(&mut tree_cursor)
            .find(|node| node.kind() == "var_spec")
            .ok_or_else(|| {
                TypeNameParseError::TreeSitterParseFailed("Not found var_spec".to_string())
            })?;

        let _ = node_var_spec.named_child(0).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed("Not found name".to_string())
        })?;
        let node_type = node_var_spec.named_child(1).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed("Not found type".to_string())
        })?;

        match node_type.kind() {
            "qualified_type" => Self::from_qualified_type_node(node_type, source),
            "type_identifier" => Self::from_type_identifier_node(node_type, source),
            _ => Err(TypeNameParseError::UnsupportedType(
                node_type.kind().to_owned(),
            )),
        }
    }
}

impl TypeName {
    pub fn from_qualified_type_node(node: Node, source: &[u8]) -> Result<Self, TypeNameParseError> {
        let node_package = node.named_child(0).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed("Not found qualified package".to_string())
        })?;
        let node_identifier = node.named_child(1).ok_or_else(|| {
            TypeNameParseError::TreeSitterParseFailed("Not found qualified identifier".to_string())
        })?;

        let package_name = node_package
            .utf8_text(source)
            .map_err(TypeNameParseError::Utf8Error)?;
        let identifier_name = node_identifier
            .utf8_text(source)
            .map_err(TypeNameParseError::Utf8Error)?;

        Ok(Self::QualifiedIdent(
            package_name.to_owned(),
            identifier_name.to_owned(),
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::{error, fs, path::PathBuf};

    #[test]
    fn simple() -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(PathBuf::new().join("tests/files/type_names.txt"))?;
        for (i, line) in content.lines().enumerate() {
            let mut split = line.split("\t");
            let str = split.next().unwrap();
            assert!(split.next().is_none());

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
