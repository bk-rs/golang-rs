use std::error;

use golang_type_decl::gen_json_struct_from_file;

#[test]
fn simple() -> Result<(), Box<dyn error::Error>> {
    gen_json_struct_from_file!("../golang-type-decl-core/tests/files/simple.go#L21-L23");
    Foo { bar: 0 };

    Ok(())
}

#[test]
fn with_nth() -> Result<(), Box<dyn error::Error>> {
    gen_json_struct_from_file!("tests/files/simple.go#L25-L30", nth = 1);
    Foo { bar: 0 };

    Ok(())
}
