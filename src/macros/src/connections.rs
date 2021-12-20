use std::collections::btree_map::BTreeMap;

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, quote_spanned};
use syn::Token;
use syn::parse::{Parse, Parser, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Dot;

use crate::way::Way;

struct Connection {
    module: Ident,
    #[allow(dead_code)]
    dot_token: Dot,
    port: Ident,
    way: Way,
    signal: Ident,
}

impl Parse for Connection {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Connection {
            module: input.parse()?,
            dot_token: input.parse()?,
            port: input.parse()?,
            way: input.parse()?,
            signal: input.parse()?,
        })
    }
}

pub fn connections(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let parser = Punctuated::<Connection, Token![;]>::parse_terminated;
    let connections = parser.parse2(input).unwrap();

    let mut module_ports: BTreeMap<&Ident, TokenStream> = BTreeMap::new();
    let terminals = connections.iter().map(|connection| {
        let Connection {
            module,
            dot_token: _,
            port,
            way,
            signal
        } = connection;
        let box_ident = format_ident!("{}__{}__{}",  module, port, signal);

        let (terminal, port_connection) = match way {
            Way::In => (
                quote_spanned! {signal.span()=>
                    #signal.subscribe()
                },
                quote! {
                    #port: system_rust::port::In {
                        signal: #box_ident,
                        value: None
                    },
                },
            ),
            Way::Out => (
                quote_spanned! {signal.span()=>
                    #signal.clone()
                },
                quote! {
                    #port: system_rust::port::Out {
                        signal: #box_ident
                    },
                },
            )
        };

        module_ports.entry(module)
            .or_default()
            .extend(port_connection);

        quote!(
            let #box_ident = Box::new(#terminal);
        )
    }).collect::<TokenStream>();

    let modules = module_ports.iter().map(|(module, port_connections)| {
        let module_ports_ident = format_ident!("{}_ports", module);
        quote!(
            tokio::task::spawn( async move {
                let mut #module_ports_ident = #module::Ports {
                    #port_connections
                };
                #module::process(&mut #module_ports_ident).await;
            }),
        )
    }).collect::<TokenStream>();

    quote!(
        #terminals
        let children = vec![
            #modules
        ];
        join_all(children).await;
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connections_test() {
        use std::{fs::File, io::Write};
        let generated = connections(
            quote!(
            mod1.first_out -> mod1_to_mod2;
            mod2.second_out -> mod2_to_mod1;
            mod1.third_in <- mod2_to_mod1;
            mod2.fourth_in <- mod1_to_mod2;
        )
                .into(),
        );

        let mut file = File::create("test_connections.rs").unwrap();
        file.write_all(format!("{}", generated).as_bytes()).unwrap();

        assert_eq!(generated.to_string(), quote!(
            let mod1__first_out__mod1_to_mod2 = Box::new(mod1_to_mod2.clone());

            let mod2__second_out__mod2_to_mod1 = Box::new(mod2_to_mod1.clone());

            let mod1__third_in__mod2_to_mod1 = Box::new(mod2_to_mod1.subscribe());

            let mod2__fourth_in__mod1_to_mod2 = Box::new(mod1_to_mod2.subscribe());

            let children = vec![
                tokio::task::spawn( async move {
                    let mut mod1_ports = mod1::Ports {
                        first_out: system_rust::ports::Out {
                            signal: mod1__first_out__mod1_to_mod2
                        },
                        third_in: system_rust::ports::In {
                            signal: mod1__third_in__mod2_to_mod1,
                            value: None
                        },
                    };
                    mod1::process(&mut mod1_ports).await;
                }),
                tokio::task::spawn( async move {
                    let mut mod2_ports = mod2::Ports {
                        second_out: system_rust::ports::Out {
                            signal: mod2__second_out__mod2_to_mod1
                        },
                        fourth_in: system_rust::ports::In {
                            signal: mod2__fourth_in__mod1_to_mod2,
                            value: None
                        },
                    };
                    mod2::process(&mut mod2_ports).await;
                }),
            ];
            join_all(children).await;
        ).to_string());
    }
}
