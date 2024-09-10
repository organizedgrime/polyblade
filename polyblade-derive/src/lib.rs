use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SimpleGraph, attributes(internal))]
pub fn derive_simple(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
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
                    return quote! {
                        use std::collections::hash_set::Iter;
                        impl #ident {
                            pub fn connect(&mut self, e: impl Into<Edge>) {
                                self.#internal_ident.connect(e)
                            }
                            pub fn disconnect(&mut self, e: impl Into<Edge>) {
                                self.#internal_ident.disconnect(e)
                            }
                            pub fn insert(&mut self) -> VertexId {
                                self.#internal_ident.insert()
                            }
                            pub fn delete(&mut self, v: VertexId) {
                                self.#internal_ident.delete(v)
                            }
                            pub fn vertex_count(&self) -> usize {
                                self.#internal_ident.vertex_count()
                            }
                            pub fn edge_count(&self) -> usize {
                                self.#internal_ident.edge_count()
                            }
                            pub fn vertices(&self) -> Iter<VertexId> {
                                self.#internal_ident.vertices()
                            }
                            pub fn edges(&self) -> Iter<Edge> {
                                self.#internal_ident.edges()
                            }
                            pub fn vertex_connections(&self, v: VertexId) -> HashSet<VertexId> {
                                self.#internal_ident.vertex_connections(v)
                            }
                            pub fn edge_connections(&self, v: VertexId) -> Vec<Edge> {
                                self.#internal_ident.edge_connections(v)
                            }
                            pub fn face_count(&self) -> i64 {
                                self.#internal_ident.face_count()
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

#[proc_macro_derive(MetaGraph, attributes(internal))]
pub fn derive_meta(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
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
                    return quote! {
                        impl #ident {
                            pub fn find_cycles(&mut self) {
                                self.#internal_ident.pst()
                            }

                            pub fn pst(&mut self) {
                                self.#internal_ident.pst()
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
