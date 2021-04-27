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
fn with_field_opts() {
    gen_json_struct_from_file!("tests/files/simple.go#L21-L23"; "bar" => { "special_type": bool });
    Foo { bar: true };
}

#[test]
fn with_nth_and_field_opts() {
    gen_json_struct_from_file!("tests/files/simple.go#L25-L30", nth = 1; "bar" => { "special_type": bool });
    Foo { bar: true };
}
