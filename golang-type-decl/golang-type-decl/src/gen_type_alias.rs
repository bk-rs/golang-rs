#[macro_export]
macro_rules! gen_type_alias {
    //
    ($code:literal) => {
        golang_type_decl_macro::gen_type_alias!(code = $code,);
    };
    //
    (
        $code:literal,
        $type_:ty
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            code = $code,
            type_ = $type_,
        );
    };
    (
        $code:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            code = $code,
            $( $opt_k = $opt_v ,)*
        );
    };
    //
    (
        $code:literal,
        $type_:ty,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            code = $code,
            type_ = $type_,
            $( $opt_k = $opt_v ,)*
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
        $type_:ty
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            path = $path,
            type_ = $type_,
        );
    };
    (
        $path:literal,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            path = $path,
            $( $opt_k = $opt_v ,)*
        );
    };
    //
    (
        $path:literal,
        $type_:ty,
        $( $opt_k:ident = $opt_v:literal ),+ $(,)?
    ) => {
        golang_type_decl_macro::gen_type_alias!(
            path = $path,
            type_ = $type_,
            $( $opt_k = $opt_v ,)*
        );
    };
}
