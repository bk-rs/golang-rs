use golang_type_decl::gen_type_alias_from_file;

#[test]
fn simple() {
    gen_type_alias_from_file!("../golang-type-decl-core/tests/files/simple.go#L3");
    let _: Node = 0_isize;
}

#[test]
fn with_nth() {
    gen_type_alias_from_file!("tests/files/simple.go#L25-L30", nth = 0);
    let _: Bar = 0_isize;
}

#[test]
fn with_type() {
    gen_type_alias_from_file!("tests/files/simple.go#L3", bool);
    let _: Node = true;
}

#[test]
fn with_nth_and_type() {
    gen_type_alias_from_file!("tests/files/simple.go#L25-L30", bool, nth = 0);
    let _: Bar = true;
}
