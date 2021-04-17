pub use golang_type_core::*;
pub use golang_type_macro;

#[macro_export]
macro_rules! r#type {
    ($type_lit:literal) => {
        golang_type_macro::r#type!($type_lit)
    };
}
