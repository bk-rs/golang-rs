pub mod convention;
pub mod error;
pub mod struct_tag;
// https://github.com/pest-parser/pest/issues/490#issuecomment-808942497
#[allow(clippy::upper_case_acronyms)]
pub(crate) mod struct_tag_parser;

pub use self::convention::json::{JsonStructTag, JsonStructTagOption};
pub use self::convention::ConventionStructTag;
pub use self::error::ParseError;
pub use self::struct_tag::StructTag;
