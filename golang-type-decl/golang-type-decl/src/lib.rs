pub use golang_type_decl_core::*;
pub use golang_type_decl_macro;

#[macro_export]
macro_rules! gen_json_struct {
    ($code:literal) => {
        golang_type_decl_macro::gen_json_struct!(code = $code)
    };
}

#[macro_export]
macro_rules! gen_json_struct_from_file {
    ($path:literal) => {
        golang_type_decl_macro::gen_json_struct!(path = $path)
    };
}
