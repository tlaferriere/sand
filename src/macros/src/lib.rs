mod ports;
mod connections;
mod way;

#[proc_macro]
pub fn ports(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ports::ports(input.into()).into()
}

#[proc_macro]
pub fn connections(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    connections::connections(input.into()).into()
}
