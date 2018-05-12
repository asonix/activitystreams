/*
 * This file is part of ActivityStreams Derive.
 *
 * Copyright © 2018 Riley Trautman
 *
 * ActivityStreams Derive is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams Derive is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams Derive.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::Tokens;
use syn::{Attribute, Data, DeriveInput, Fields, Ident, Type};

#[proc_macro_derive(UnitString, attributes(activitystreams))]
pub fn unit_string(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;

    let attr = input
        .attrs
        .iter()
        .find(|attribute| {
            attribute
                .path
                .segments
                .last()
                .map(|segment| {
                    let segment = segment.into_value();
                    segment.ident == Ident::new("activitystreams", segment.ident.span())
                })
                .unwrap_or(false)
        })
        .unwrap()
        .clone();

    let value = get_value(attr);

    let visitor_name = Ident::from(format!("{}Visitor", value));

    let serialize = quote! {
        impl Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(#value)
            }
        }
    };

    let expecting = quote! {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "The string '{}'", #value)
        }
    };

    let visit = quote! {
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v == #value {
                Ok(#name)
            } else {
                Err(de::Error::custom("Invalid type"))
            }
        }
    };

    let visitor = quote! {
        pub struct #visitor_name;

        impl<'de> Visitor<'de> for #visitor_name {
            type Value = #name;

            #expecting

            #visit
        }
    };

    let deserialize = quote! {
        impl<'de> Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<#name, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str(#visitor_name)
            }
        }
    };

    let c = quote! {
        #serialize
        #visitor
        #deserialize
    };

    c.into()
}

fn get_value(attr: Attribute) -> String {
    let group = attr.tts
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Group(group) => Some(group),
            _ => None,
        })
        .next()
        .unwrap();

    group
        .stream()
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Term(term) => Some(term.as_str().to_owned()),
            _ => None,
        })
        .next()
        .unwrap()
}

#[proc_macro_derive(Properties, attributes(activitystreams))]
pub fn properties_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let name = input.ident;

    let data = match input.data {
        Data::Struct(s) => s,
        _ => panic!("Can only derive for structs"),
    };

    let fields = match data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Can only derive for named fields"),
    };

    let getters = fields
        .named
        .iter()
        .filter_map(|field| {
            let our_attr = field.attrs.iter().find(|attribute| {
                attribute
                    .path
                    .segments
                    .last()
                    .map(|segment| {
                        let segment = segment.into_value();
                        segment.ident == Ident::new("activitystreams", segment.ident.span())
                    })
                    .unwrap_or(false)
            });

            if our_attr.is_some() {
                let our_attr = our_attr.unwrap();

                let is_option = match field.ty {
                    Type::Path(ref path) => path.path
                        .segments
                        .last()
                        .map(|seg| {
                            let seg = seg.into_value();
                            seg.ident == Ident::new("Option", seg.ident.span())
                        })
                        .unwrap_or(false),
                    _ => false,
                };

                let is_vec = match field.ty {
                    Type::Path(ref path) => path.path
                        .segments
                        .last()
                        .map(|seg| {
                            let seg = seg.into_value();
                            seg.ident == Ident::new("Vec", seg.ident.span())
                        })
                        .unwrap_or(false),
                    _ => false,
                };

                Some((
                    field.ident.clone().unwrap(),
                    is_option,
                    is_vec,
                    is_functional(our_attr.clone()),
                    our_attr.clone(),
                ))
            } else {
                None
            }
        })
        .flat_map(|(ident, is_option, is_vec, is_functional, attr)| {
            variants(attr)
                .into_iter()
                .map(move |(variant, is_concrete)| {
                    let lower_variant = variant.to_lowercase();
                    let fn_name = Ident::from(format!("{}_{}", ident, lower_variant));
                    let fn_plural = Ident::from(format!("{}_{}_vec", ident, lower_variant));
                    let variant = Ident::from(variant);

                    if is_concrete && is_option && is_functional {
                        quote! {
                            pub fn #fn_name(&self) -> Result<#variant> {
                                self.get_item(|t| &t.#ident)
                            }
                        }
                    } else if is_concrete && is_option {
                        quote! {
                            pub fn #fn_name(&self) -> Result<#variant> {
                                self.get_item(|t| &t.#ident)
                            }

                            pub fn #fn_plural(&self) -> Result<Vec<#variant>> {
                                self.get_item(|t| &t.#ident)
                            }
                        }
                    } else if is_concrete && is_vec {
                        quote! {
                            pub fn #fn_name(&self) -> Result<Vec<#variant>> {
                                self.get_vec(|t| &t.#ident)
                            }
                        }
                    } else if is_concrete && is_functional {
                        quote! {
                            pub fn #fn_name(&self) -> Result<#variant> {
                                self.get_value(|t| &t.#ident)
                            }
                        }
                    } else if is_concrete {
                        quote! {
                            pub fn #fn_name(&self) -> Result<#variant> {
                                self.get_value(|t| &t.#ident)
                            }

                            pub fn #fn_plural(&self) -> Result<Vec<#variant>> {
                                self.get_value(|t| &t.#ident)
                            }
                        }
                    } else if is_option && is_functional {
                        quote! {
                            pub fn #fn_name<T: #variant>(&self) -> Result<T> {
                                self.get_item(|t| &t.#ident)
                            }
                        }
                    } else if is_option {
                        quote! {
                            pub fn #fn_name<T: #variant>(&self) -> Result<T> {
                                self.get_item(|t| &t.#ident)
                            }

                            pub fn #fn_plural<T: #variant>(&self) -> Result<Vec<T>> {
                                self.get_item(|t| &t.#ident)
                            }
                        }
                    } else if is_vec {
                        quote! {
                            pub fn #fn_name<T: #variant>(&self) -> Result<Vec<T>> {
                                self.get_vec(|t| &t.#ident)
                            }
                        }
                    } else if is_functional {
                        quote! {
                            pub fn #fn_name<T: #variant>(&self) -> Result<T> {
                                self.get_value(|t| &t.#ident)
                            }
                        }
                    } else {
                        quote! {
                            pub fn #fn_name<T: #variant>(&self) -> Result<T> {
                                self.get_value(|t| &t.#ident)
                            }

                            pub fn #fn_plural<T: #variant>(&self) -> Result<Vec<T>> {
                                self.get_value(|t| &t.#ident)
                            }
                        }
                    }
                })
        });

    let mut tokens = Tokens::new();
    tokens.append_all(getters);

    let full = quote!{
        impl Properties for #name {}

        impl #name {
            #tokens
        }
    };

    full.into()
}

fn variants(attr: Attribute) -> Vec<(String, bool)> {
    let group = attr.tts
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Group(group) => Some(group),
            _ => None,
        })
        .next()
        .unwrap();

    let mut is_concrete = false;

    group
        .stream()
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Term(term) => {
                is_concrete = term.as_str() == "concrete";
                None
            }
            TokenTree::Group(group) => Some(group.stream().into_iter().filter_map(
                move |token_tree| match token_tree {
                    TokenTree::Term(term) => Some((term.as_str().to_owned(), is_concrete)),
                    _ => None,
                },
            )),
            _ => None,
        })
        .flat_map(|i| i)
        .collect()
}

fn is_functional(attr: Attribute) -> bool {
    let group = attr.tts
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Group(group) => Some(group),
            _ => None,
        })
        .next()
        .unwrap();

    group
        .stream()
        .clone()
        .into_iter()
        .any(|token_tree| match token_tree {
            TokenTree::Term(term) => term.as_str() == "functional",
            _ => false,
        })
}
