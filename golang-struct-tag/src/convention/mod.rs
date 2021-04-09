pub mod json;

use self::json::JsonStructTag;

pub enum ConventionStructTag {
    Json(JsonStructTag),
    Unknown(String),
}
