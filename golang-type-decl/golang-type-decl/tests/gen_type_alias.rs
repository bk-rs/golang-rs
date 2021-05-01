use golang_type_decl::gen_type_alias;

#[test]
fn simple() {
    gen_type_alias!(
        r#"
    type Bar = int
    "#
    );
}

#[test]
fn with_nth() {
    gen_type_alias!(
        r#"
    type (
        Bar = int
        Foo struct {
            bar uint
        }
    )
    "#,
        nth = 0
    );
}

#[test]
fn with_nth_and_type() {
    gen_type_alias!(
        r#"
    type (
        Bar = int
        Foo struct {
            bar uint
        }
    )
    "#,
        Option<bool>,
        nth = 0
    );

    let _: Bar = None;
}

#[test]
fn with_other_opts() {
    gen_type_alias!(
        r#"
    type Foo = int
    "#,
        alias_name = "Bar"
    );

    let _: Bar = 1_isize;
}
