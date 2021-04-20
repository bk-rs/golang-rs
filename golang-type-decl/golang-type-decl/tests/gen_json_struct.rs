use golang_type_decl::gen_json_struct;

#[test]
fn simple() {
    gen_json_struct!(
        r#"
    type User struct {
        Name   string
        Age    uint   `json:",string"`
        Gender string `json:",omitempty"`
        Addr   string `json:"address"`
    }
    "#
    );
}
