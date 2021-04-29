use std::error;

use golang_type_decl::gen_json_struct;
use serde_aux::field_attributes::deserialize_bool_from_anything;

#[test]
fn simple() -> Result<(), Box<dyn error::Error>> {
    gen_json_struct!(
        r#"
    type User struct {
        Name   string
        Age    uint   `json:",string"`
        Gender string `json:",omitempty"`
        Addr   string `json:"Address"`
    }
    "#
    );

    let user: User = serde_json::from_str(
        r#"
    {
        "Name": "foo",
        "Age": "20",
        "Address": "bar"
    }
    "#,
    )?;

    assert_eq!(user.name, "foo");
    assert_eq!(user.age, "20");
    assert_eq!(user.gender, None);
    assert_eq!(user.addr, "bar");

    Ok(())
}

#[test]
fn with_nth() {
    gen_json_struct!(
        r#"
    type (
        Bar = int
        Foo struct {
            bar uint
        }
    )
    "#,
        nth = 1
    );
    Foo { bar: 0 };
}

#[test]
fn with_field_types_and_field_opts() -> Result<(), Box<dyn error::Error>> {
    gen_json_struct!(
        r#"
    type User struct {
        Age     int
        Actived string
    }
    "#;
        "Age" => u8,
        "Actived" => bool
    ;
        "Actived" => {
            "attr_serde_deserialize_with": "deserialize_bool_from_anything"
        }
    );
    User {
        age: 18_u8,
        actived: true,
    };

    let user: User = serde_json::from_str(
        r#"
    {
        "Age": 18,
        "Actived": "1"
    }
    "#,
    )?;

    assert_eq!(user.age, 18);
    assert_eq!(user.actived, true);

    Ok(())
}

#[test]
fn with_nth_and_field_types() {
    gen_json_struct!(
        r#"
    type (
        Bar = int
        Foo struct {
            bar uint
        }
    )
    "#,
        nth = 1
    ;
        "bar" => Option<bool>
    );
    Foo { bar: None };
}

#[test]
fn with_other_opts() {
    gen_json_struct!(
        r#"
    type Foo struct {
        bar uint
    }
    "#,
        disable_derive_serde_ser = true,
        disable_derive_serde_de = true,
        disable_derive_debug = true,
        disable_derive_clone = true,
        alias_name = "Bar"
    );

    Bar { bar: 1_usize };
}
