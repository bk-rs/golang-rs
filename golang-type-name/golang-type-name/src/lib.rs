pub use golang_type_name_core::*;
pub use golang_type_name_macro;

#[macro_export]
macro_rules! gen_type_name {
    ($type_name_lit:literal) => {
        golang_type_name_macro::gen_type_name!($type_name_lit)
    };
}
