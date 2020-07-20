use syn::parse;

mod encode;

struct Items(Vec<syn::Item>);

impl parse::Parse for Items {
    fn parse(input: parse::ParseStream) -> parse::Result<Self> {
        let mut items = Vec::new();
        while !input.cursor().eof() {
            let item = input.parse::<syn::Item>()?;
            items.push(item);
        }
        Ok(Items(items))
    }
}

#[proc_macro]
pub fn purust(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Items(items) = syn::parse_macro_input!(input as Items);
    encode::encode(&items).into()
}
