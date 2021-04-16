pub use tree_sitter;

use tree_sitter::{Node, Tree};

pub const NODE_KIND_COMMENT: &str = "comment";

pub struct Parser {
    code: String,
    tree: Tree,
}
impl Parser {
    pub fn new(code: impl AsRef<str>) -> Result<Self, Error> {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_go::language())
            .map_err(|err| Error::TreeSitterLanguageError(err.to_string()))?;

        let code = code.as_ref();
        let code = if code.ends_with(';') {
            code.to_owned()
        } else {
            format!("{};", code)
        };

        let tree = parser
            .parse(&code, None)
            .ok_or(Error::TreeSitterParseCodeFailed)?;

        debug_assert!(tree.root_node().kind() == "source_file");

        Ok(Self { code, tree })
    }

    pub fn get_source(&self) -> &[u8] {
        &self.code.as_bytes()
    }

    pub fn get_root_node(&self) -> Node<'_> {
        self.tree.root_node()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("TreeSitterLanguageError {0}")]
    TreeSitterLanguageError(String),
    #[error("TreeSitterParseCodeFailed")]
    TreeSitterParseCodeFailed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        match Parser::new("var _ int") {
            Ok(parser) => {
                assert_eq!(parser.get_source(), b"var _ int;");
                assert_eq!(
                    parser.get_root_node().utf8_text(parser.get_source()),
                    Ok("var _ int;")
                );
            }
            Err(err) => assert!(false, "{:?}", err),
        }

        match Parser::new("var _ int;") {
            Ok(parser) => {
                assert_eq!(parser.get_source(), b"var _ int;");
            }
            Err(err) => assert!(false, "{:?}", err),
        }
    }
}
