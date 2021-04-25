use std::error;

use golang_type_decl::gen_json_struct;

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
fn with_nth() -> Result<(), Box<dyn error::Error>> {
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

    Ok(())
}
