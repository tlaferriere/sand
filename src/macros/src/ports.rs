use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{Token, Visibility};
use syn::parse::{Parse, Parser, ParseStream};
use syn::punctuated::Punctuated;

use crate::way::Way;

struct Port {
    visibility: Visibility,
    name: Ident,
    ty: Ident,
    way: Way,
}

impl Parse for Port {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        let name: Ident = input.parse()?;
        let way: Way = input.parse()?;
        let ty: Ident = input.parse()?;
        Ok(Port {
            visibility,
            name,
            ty,
            way,
        })
    }
}

pub fn ports(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parser = Punctuated::<Port, Token![,]>::parse_terminated;
    let ports = parser.parse2(input).unwrap();
    let fields = ports.iter().map(|port| {
        let visibility = match port.visibility {
            Visibility::Inherited => quote! { pub(crate) },
            _ => port.visibility.to_token_stream()
        };
        let id = &port.name;
        let ty = &port.ty;
        let port_type = match port.way {
            Way::In => quote_spanned! {ty.span()=>
                system_rust::port::In<#ty>
            },
            Way::Out => quote_spanned! {ty.span()=>
                system_rust::port::Out<#ty>
            }
        };
        quote!(
            #visibility #id: #port_type,
        )
    }).collect::<TokenStream>();
    quote! {
        pub(crate) struct Ports {
            #fields
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ports_test() {
        use std::{fs::File, io::Write};
        let generated = ports(
            quote!(
            first_out -> i32,
            second_out -> u64,
            third_in <- u8,
            fourth_in <- i16
        )
                .into(),
        );
        assert_eq!(generated.to_string(), quote!(
        pub(crate) struct Ports {
            pub(crate) first_out: system_rust::port::Out<i32>,
            pub(crate) second_out: system_rust::port::Out<u64>,
            pub(crate) third_in: system_rust::port::In<u8>,
            pub(crate) fourth_in: system_rust::port::In<i16>,
        }
    ).to_string());
        let mut file = File::create("test_ports.rs").unwrap();
        file.write_all(format!("{}", generated).as_bytes()).unwrap();
    }
}
