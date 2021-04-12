use std::any::{Any as _, TypeId};

use golang_type_name_test::expand_let_type_name;

macro_rules! let_type_name {
    ($name:ident, $type_name:literal) => {
        expand_let_type_name!($name, $type_name)
    };
}

#[test]
fn test_builtin() {
    let_type_name!(v_bool, "bool");
    v_bool = true;
    assert_eq!(v_bool.type_id(), TypeId::of::<bool>());

    let_type_name!(v_u8, "uint8");
    v_u8 = 1;
    assert_eq!(v_u8.type_id(), TypeId::of::<u8>());
}
