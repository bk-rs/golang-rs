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
pub enum TypeNameParseError {}
