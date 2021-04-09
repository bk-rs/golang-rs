use std::collections::HashSet;

pub enum JsonStructTag {
    // https://github.com/golang/go/blob/go1.16.3/src/encoding/json/encode.go#L1259
    Ignored,
    // https://github.com/golang/go/blob/go1.16.3/src/encoding/json/encode.go#L1262
    Normal(JsonStructTagName, HashSet<JsonStructTagOption>),
}

pub type JsonStructTagName = Option<String>;
pub enum JsonStructTagOption {
    // https://github.com/golang/go/blob/go1.16.3/src/encoding/json/encode.go#L1278
    String,
    // https://github.com/golang/go/blob/go1.16.3/src/encoding/json/encode.go#L1300
    Omitempty,
}
