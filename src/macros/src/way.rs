use syn::{custom_punctuation};
use syn::parse::{Parse, ParseStream};

custom_punctuation!(In, <-);
custom_punctuation!(Out, ->);

#[derive(Debug)]
/// Way of a port connection.
pub(crate) enum Way {
    /// `<-`
    In,
    /// `->`
    Out,
}

impl Parse for Way {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<In>() {
            Ok(_) => {
                Ok(Way::In)
            }
            Err(_) => {
                input.parse::<Out>()?;
                Ok(Way::Out)
            }
        }
    }
}
