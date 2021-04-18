use syn::{
    parse::{Parse, ParseStream},
    Error as SynError, Ident, LitStr, Token,
};

pub struct Input {
    pub code: String,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self, SynError> {
        let mut code = String::new();

        let mut expect_comma = false;

        while !input.is_empty() {
            if expect_comma {
                let _ = input.parse::<Token![,]>()?;
            }

            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            if key == "code" {
                code = input
                    .parse::<LitStr>()?
                    .value()
                    .trim_start()
                    .trim_end()
                    .to_owned();
            } else {
                let message = format!("unexpected input key: {}", key);
                return Err(SynError::new_spanned(key, message));
            }

            expect_comma = true;
        }

        Ok(Self { code })
    }
}
