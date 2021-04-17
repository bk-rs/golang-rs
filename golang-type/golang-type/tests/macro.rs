use std::{
    any::{Any as _, TypeId},
    collections::HashMap,
};

macro_rules! assert_macro_type {
    ($type_lit:literal, $value:expr, $type:ty) => {
        let v: golang_type::r#type!($type_lit);
        v = $value;
        assert_eq!(v.type_id(), TypeId::of::<$type>());
    };
}

#[test]
fn test_array_type() {
    assert_macro_type!("[32]byte", vec![8_u8], Vec<u8>);
    assert_macro_type!("[1000]*float64", vec![0.0_f64], Vec<f64>);
    assert_macro_type!("[3][5]int", vec![vec![-1_isize]], Vec<Vec<isize>>);
    assert_macro_type!(
        "[2][2][2]float64",
        vec![vec![vec![0.0_f64]]],
        Vec<Vec<Vec<f64>>>
    );
}

#[test]
fn test_slice_type() {
    assert_macro_type!("[]int", vec![-1_isize], Vec<isize>);
    assert_macro_type!("[][]uint", vec![vec![1_usize]], Vec<Vec<usize>>);
}

#[test]
fn test_map_type() {
    assert_macro_type!("map[string]int", vec![("".to_owned(), -1_isize)].into_iter().collect(), HashMap<String, isize>);
    assert_macro_type!(
        "map[string][]string",
        vec![("".to_owned(), vec!["".to_owned()])]
            .into_iter()
            .collect(),
        HashMap<String, Vec<String>>
    );
}

#[test]
fn test_parenthesized_type() {
    assert_macro_type!("(int)", -1_isize, isize);
}
