#[macro_export]
macro_rules! gen_type_alias {
    //
    ($code:literal) => {
        golang_type_decl_macro::gen_type_alias!(code = $code,);
    };
    //
    (
        $code:literal,
        $( $opt_k:ident = $opt_v:literal),+ $(,)?
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            code = $code,
            $( $opt_k = $opt_v ,)*
        );
    };
    (
        $code:literal;
        type = $type:ty
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            code = $code,
            ty = $type,
        );
    };
    //
    (
        $code:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?;
        type = $type:ty
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            code = $code,
            $( $opt_k = $opt_v ,)*
            ty = $type,
        );
    };
}

#[macro_export]
macro_rules! gen_type_alias_from_file {
    //
    ($path:literal) => {
        golang_type_decl_macro::gen_type_alias!(path = $path,);
    };
    //
    (
        $path:literal,
        $( $opt_k:ident = $opt_v:literal),+ $(,)?
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            path = $path,
            $( $opt_k = $opt_v ,)*
        );
    };
    (
        $path:literal;
        type = $type:ty
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            path = $path,
            ty = $type,
        );
    };
    //
    (
        $path:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?;
        type = $type:ty
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            path = $path,
            $( $opt_k = $opt_v ,)*
            ty = $type,
        );
    };
}
