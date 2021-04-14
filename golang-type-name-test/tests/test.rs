use std::any::{Any as _, TypeId};

use num_complex::{Complex32, Complex64};
use proc_macro2::{Ident, Span};

macro_rules! assert_let_type_name {
    ($type_name_lit:literal, $value:expr, $type:ty) => {
        golang_type_name_test::let_type_name!(v, $type_name_lit);
        v = $value;
        assert_eq!(v.type_id(), TypeId::of::<$type>());
    };
}

#[test]
fn test_builtin() {
    assert_let_type_name!("bool", true, bool);

    assert_let_type_name!("uint8", 8_u8, u8);
    assert_let_type_name!("uint16", 16_u16, u16);
    assert_let_type_name!("uint32", 32_u32, u32);
    assert_let_type_name!("uint64", 64_u64, u64);
    assert_let_type_name!("int8", -8_i8, i8);
    assert_let_type_name!("int16", -16_i16, i16);
    assert_let_type_name!("int32", -32_i32, i32);
    assert_let_type_name!("int64", -64_i64, i64);
    assert_let_type_name!("float32", 0.0_f32, f32);
    assert_let_type_name!("float64", 0.0_f64, f64);
    assert_let_type_name!("complex64", Complex32::new(0.0, 0.0), Complex32);
    assert_let_type_name!("complex128", Complex64::new(0.0, 0.0), Complex64);
    assert_let_type_name!("byte", 8_u8, u8);
    assert_let_type_name!("rune", -32_i32, i32);
    assert_let_type_name!("uint", 1_usize, usize);
    assert_let_type_name!("int", -1_isize, isize);
    assert_let_type_name!("uintptr", 1_usize, usize);

    assert_let_type_name!("string", "".to_string(), String);
}

#[test]
fn test_qualified_identifier() {
    assert_let_type_name!(
        "proc_macro2.Ident",
        Ident::new("foo", Span::call_site()),
        proc_macro2::Ident
    );
}

#[test]
fn test_identifier() {
    type Foo = ();

    assert_let_type_name!("Foo", (), Foo);
}
