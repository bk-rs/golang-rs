pub use golang_type_decl_core::*;
pub use golang_type_decl_macro;

#[macro_export]
macro_rules! gen_json_struct {
    ($code:literal) => {
        golang_type_decl_macro::gen_json_struct!(code = $code,);
    };
    ($code:literal, nth = $nth:literal) => {
        golang_type_decl_macro::gen_json_struct!(code = $code, nth = $nth,);
    };
    ($code:literal; $( $field:literal => { $( $field_opt_k:literal : $field_opt_v:tt $(,)? ),* }$(,)? ),*) => {
        golang_type_decl_macro::gen_json_struct!(code = $code, field_opts = $( $field => $( $field_opt_k -> $field_opt_v ,)* ,)* ,);
    };
}

#[macro_export]
macro_rules! gen_json_struct_from_file {
    ($path:literal) => {
        golang_type_decl_macro::gen_json_struct!(path = $path,);
    };
    ($path:literal, nth = $nth:literal) => {
        golang_type_decl_macro::gen_json_struct!(path = $path, nth = $nth,);
    };
}
