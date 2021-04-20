pub use golang_type_core::*;
pub use golang_type_macro;

#[macro_export]
macro_rules! gen_type {
    ($type_lit:literal) => {
        golang_type_macro::gen_type!($type_lit)
    };
}
