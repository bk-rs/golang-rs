use golang_type_decl::gen_json_struct_from_file;

#[test]
fn simple() {
    gen_json_struct_from_file!("../golang-type-decl-core/tests/files/simple.go#L21-L23");
    Foo { bar: 0 };
}

#[test]
fn with_nth() {
    gen_json_struct_from_file!("tests/files/simple.go#L25-L30", nth = 1);
    Foo { bar: 0 };
}

#[test]
fn with_field_types() {
    gen_json_struct_from_file!("tests/files/simple.go#L21-L23"; "bar" => bool);
    Foo { bar: true };
}

#[test]
fn with_nth_and_field_types() {
    gen_json_struct_from_file!("tests/files/simple.go#L25-L30", nth = 1; "bar" => bool);
    Foo { bar: true };
}

#[test]
fn with_box_type() {
    type Comparable = isize;

    gen_json_struct_from_file!(
        "tests/files/simple.go#L16-L19";
        "left" => { "box_type": true },
        "right" => { "box_type": true }
    );
}
