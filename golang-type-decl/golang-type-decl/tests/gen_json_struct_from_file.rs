use std::error;

use golang_type_decl::gen_json_struct_from_file;

#[test]
fn simple() -> Result<(), Box<dyn error::Error>> {
    // gen_json_struct_from_file!("files/simple.go");

    Ok(())
}

#[test]
fn from() -> Result<(), Box<dyn error::Error>> {
    // gen_json_struct_from_file!("../golang-type-decl-core/tests/files/simple.go");

    Ok(())
}
