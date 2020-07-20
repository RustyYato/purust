use proc_macro2::TokenStream;
use quote::quote;
use syn::visit::Visit;

pub fn encode(raw_items: &[syn::Item]) -> TokenStream {
    let mut items = Items {
        items: Vec::new(),
        errors: TokenStream::new(),
    };

    raw_items.iter().for_each(|raw_item| items.visit_item(&raw_item));

    if items.errors.is_empty() {
        quote!(compile_error!("success");)
    } else {
        items.errors
    }
}

struct EnumVariant {
    name: String,
    generics: Vec<syn::Type>,
}

struct Function {

}

enum Item {
    Enum(Vec<EnumVariant>),
    Func(Vec<Function>)
}

struct Items {
    items: Vec<Item>,
    errors: TokenStream
}

impl syn::visit::Visit<'_> for Items {
    fn visit_item_enum(&mut self, item: &syn::ItemEnum) {
        if !self.errors.is_empty() {
            return
        }

        let mut variants = Vec::new();

        for variant in item.variants.iter() {
            let name = &variant.ident;
            match variant.fields {
                syn::Fields::Unnamed(ref fields) => {
                    variants.push(EnumVariant {
                        name: name.to_string(),
                        generics: fields.unnamed.iter()
                            .map(|field| field.ty.clone())
                            .collect(),
                    });
                },
                syn::Fields::Unit => {
                    variants.push(EnumVariant {
                        name: name.to_string(),
                        generics: Vec::new(),
                    });
                }
                syn::Fields::Named(_) => {
                    let errors = &self.errors;
                    self.errors = quote!(#errors compile_error!("variants cannot have named fields"););
                    return
                }
            }
        }

        self.items.push(Item::Enum(variants));
    }
}
