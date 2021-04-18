use golang_type_decl::json_struct;

#[test]
fn simple() {
    json_struct!(
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
