#[macro_export]
macro_rules! gen_json_struct {
    //
    ($code:literal) => {
        golang_type_decl_macro::gen_json_struct!(code = $code,);
    };
    //
    (
        $code:literal,
        $( $opt_k:ident = $opt_v:literal),+ $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            code = $code,
            $( $opt_k = $opt_v ,)*
        );
    };
    (
        $code:literal;
        $( $field_type_name:literal => $field_type:ty ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            code = $code,
            field_types = $( $field_type_name => $field_type ,)* ,
        );
    };
    (
        $code:literal;
        $( $field_opt_name:literal => { $( $field_opt_k:literal : $field_opt_v:tt ),* $(,)? } ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            code = $code,
            field_opts = $( $field_opt_name => $( $field_opt_k -> $field_opt_v ,)* ,)* ,
        );
    };
    //
    (
        $code:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?;
        $( $field_type_name:literal => $field_type:ty ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            code = $code,
            $( $opt_k = $opt_v ,)*
            field_types = $( $field_type_name => $field_type ,)* ,
        );
    };
    (
        $code:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?;
        $( $field_opt_name:literal => { $( $field_opt_k:literal : $field_opt_v:tt ),* $(,)? } ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            code = $code,
            $( $opt_k = $opt_v ,)*
            field_opts = $( $field_opt_name => $( $field_opt_k -> $field_opt_v ,)* ,)* ,
        );
    };
    (
        $code:literal;
        $( $field_type_name:literal => $field_type:ty ),* $(,)?;
        $( $field_opt_name:literal => { $( $field_opt_k:literal : $field_opt_v:tt ),* $(,)? } ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            code = $code,
            field_types = $( $field_type_name => $field_type ,)* ,
            field_opts = $( $field_opt_name => $( $field_opt_k -> $field_opt_v ,)* ,)* ,
        );
    };
    //
    (
        $code:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?;
        $( $field_type_name:literal => $field_type:ty ),* $(,)?;
        $( $field_opt_name:literal => { $( $field_opt_k:literal : $field_opt_v:tt ),* $(,)? } ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            code = $code,
            $( $opt_k = $opt_v ,)*
            field_types = $( $field_type_name => $field_type ,)* ,
            field_opts = $( $field_opt_name => $( $field_opt_k -> $field_opt_v ,)* ,)* ,
        );
    };
}

#[macro_export]
macro_rules! gen_json_struct_from_file {
    //
    ($path:literal) => {
        golang_type_decl_macro::gen_json_struct!(path = $path,);
    };
    //
    (
        $path:literal,
        $( $opt_k:ident = $opt_v:literal),+ $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            path = $path,
            $( $opt_k = $opt_v ,)*
        );
    };
    (
        $path:literal;
        $( $field_type_name:literal => $field_type:ty ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            path = $path,
            field_types = $( $field_type_name => $field_type ,)* ,
        );
    };
    (
        $path:literal;
        $( $field_opt_name:literal => { $( $field_opt_k:literal : $field_opt_v:tt ),* $(,)? } ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            path = $path,
            field_opts = $( $field_opt_name => $( $field_opt_k -> $field_opt_v ,)* ,)* ,
        );
    };
    //
    (
        $path:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?;
        $( $field_type_name:literal => $field_type:ty ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            path = $path,
            $( $opt_k = $opt_v ,)*
            field_types = $( $field_type_name => $field_type ,)* ,
        );
    };
    (
        $path:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?;
        $( $field_opt_name:literal => { $( $field_opt_k:literal : $field_opt_v:tt ),* $(,)? } ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            path = $path,
            $( $opt_k = $opt_v ,)*
            field_opts = $( $field_opt_name => $( $field_opt_k -> $field_opt_v ,)* ,)* ,
        );
    };
    (
        $path:literal;
        $( $field_type_name:literal => $field_type:ty ),* $(,)?;
        $( $field_opt_name:literal => { $( $field_opt_k:literal : $field_opt_v:tt ),* $(,)? } ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            path = $path,
            field_types = $( $field_type_name => $field_type ,)* ,
            field_opts = $( $field_opt_name => $( $field_opt_k -> $field_opt_v ,)* ,)* ,
        );
    };
    //
    (
        $path:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?;
        $( $field_type_name:literal => $field_type:ty ),* $(,)?;
        $( $field_opt_name:literal => { $( $field_opt_k:literal : $field_opt_v:tt ),* $(,)? } ),* $(,)?
    ) => {
        golang_type_decl_macro::gen_json_struct!(
            path = $path,
            $( $opt_k = $opt_v ,)*
            field_types = $( $field_type_name => $field_type ,)* ,
            field_opts = $( $field_opt_name => $( $field_opt_k -> $field_opt_v ,)* ,)* ,
        );
    };
}
