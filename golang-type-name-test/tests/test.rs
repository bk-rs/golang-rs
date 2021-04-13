use std::any::{Any as _, TypeId};

use golang_type_name_test::expand_let_type_name;
use num_complex::{Complex32, Complex64};
use proc_macro2::{Ident, Span};

macro_rules! let_type_name {
    ($name:ident, $type_name:literal) => {
        expand_let_type_name!($name, $type_name)
    };
}

macro_rules! assert_builtin_type_name {
    ($type_name:literal, $value:expr, $type:ty) => {
        let_type_name!(v, $type_name);
        v = $value;
        assert_eq!(v.type_id(), TypeId::of::<$type>());
    };
}

#[test]
fn test_builtin() {
    assert_builtin_type_name!("bool", true, bool);

    assert_builtin_type_name!("uint8", 8_u8, u8);
    assert_builtin_type_name!("uint16", 16_u16, u16);
    assert_builtin_type_name!("uint32", 32_u32, u32);
    assert_builtin_type_name!("uint64", 64_u64, u64);
    assert_builtin_type_name!("int8", -8_i8, i8);
    assert_builtin_type_name!("int16", -16_i16, i16);
    assert_builtin_type_name!("int32", -32_i32, i32);
    assert_builtin_type_name!("int64", -64_i64, i64);
    assert_builtin_type_name!("float32", 0.0_f32, f32);
    assert_builtin_type_name!("float64", 0.0_f64, f64);
    assert_builtin_type_name!("complex64", Complex32::new(0.0, 0.0), Complex32);
    assert_builtin_type_name!("complex128", Complex64::new(0.0, 0.0), Complex64);
    assert_builtin_type_name!("byte", 8_u8, u8);
    assert_builtin_type_name!("rune", -32_i32, i32);
    assert_builtin_type_name!("uint", 1_usize, usize);
    assert_builtin_type_name!("int", -1_isize, isize);
    assert_builtin_type_name!("uintptr", 1_usize, usize);

    assert_builtin_type_name!("string", "".to_string(), String);
}

#[test]
fn test_qualified_identifier() {
    let_type_name!(v, "syn.Ident");
    v = Ident::new("foo", Span::call_site());
    assert_eq!(v.type_id(), TypeId::of::<syn::Ident>());
}

#[test]
fn test_identifier() {
    type Foo = ();

    let_type_name!(v, "Foo");
    v = ();
    assert_eq!(v.type_id(), TypeId::of::<Foo>());
}
