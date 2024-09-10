use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Meow, attributes(internal))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    println!("ast: {:#?}", ast);

    let ident = ast.ident.clone();
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    for field in fields {
        for attr in &field.attrs {
            if let Some(first_segment) = attr.meta.path().segments.first() {
                if first_segment.ident == "internal" {
                    let internal_ident = field.ident.clone();
                    println!("internal_ident: {:?}", internal_ident);
                    return quote! {
                        impl SimpleGraph for #ident {
                            fn insert(&mut self) -> usize {
                                self.#internal_ident.insert()
                            }
                        }
                    }
                    .into();
                }
            }
        }
    }

    unimplemented!();
}
