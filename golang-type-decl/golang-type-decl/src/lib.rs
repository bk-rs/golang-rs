pub use golang_type_decl_core::*;
pub use golang_type_decl_macro;

#[macro_export]
macro_rules! json_struct {
    ($code:literal) => {
        golang_type_decl_macro::json_struct!(code = $code)
    };
}
