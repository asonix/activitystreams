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

//! Derive macros for Activity Streams
//!
//! ## Examples
//!
//! ```rust
//! #[macro_use]
//! extern crate activitystreams_derive;
//! extern crate activitystreams_traits;
//! extern crate serde;
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_json;
//!
//! use activitystreams_traits::{Link, Object};
//!
//! /// Using the UnitString derive macro
//! ///
//! /// This macro implements Serialize and Deserialize for the given type, making this type
//! /// represent the string "SomeKind" in JSON.
//! #[derive(Clone, Debug, Default, UnitString)]
//! #[activitystreams(SomeKind)]
//! pub struct MyKind;
//!
//! /// Using the Properties derive macro
//! ///
//! /// This macro generates getters and setters for the associated fields.
//! #[derive(Clone, Debug, Default, Deserialize, Serialize, Properties)]
//! #[serde(rename_all = "camelCase")]
//! pub struct MyProperties {
//!     /// Derive getters and setters for @context with Link and Object traits.
//!     #[serde(rename = "@context")]
//!     #[activitystreams(ab(Object, Link))]
//!     pub context: Option<serde_json::Value>,
//!
//!     /// Use the UnitString MyKind to enforce the type of the object by "SomeKind"
//!     #[serde(rename = "type")]
//!     pub kind: MyKind,
//!
//!     /// Derive getters and setters for required_key with String type.
//!     ///
//!     /// In the Activity Streams spec, 'functional' means there can only be one item for this
//!     /// key. This means all fields not labeled 'functional' can also be serialized/deserialized
//!     /// as Vec<T>.
//!     #[activitystreams(concrete(String), functional)]
//!     pub required_key: serde_json::Value,
//! }
//! #
//! # fn main () {}
//! ```

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
                }).unwrap_or(false)
        }).unwrap()
        .clone();

    let value = from_value(attr);

    let visitor_name = Ident::from(format!("{}Visitor", value));

    let serialize = quote! {
        impl ::serde::ser::Serialize for #name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str(#value)
            }
        }
    };

    let expecting = quote! {
        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(formatter, "The string '{}'", #value)
        }
    };

    let visit = quote! {
        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: ::serde::de::Error,
        {
            if v == #value {
                Ok(#name)
            } else {
                Err(::serde::de::Error::custom("Invalid type"))
            }
        }
    };

    let visitor = quote! {
        struct #visitor_name;

        impl<'de> ::serde::de::Visitor<'de> for #visitor_name {
            type Value = #name;

            #expecting

            #visit
        }
    };

    let deserialize = quote! {
        impl<'de> ::serde::de::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<#name, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
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

fn from_value(attr: Attribute) -> String {
    let group = attr
        .tts
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Group(group) => Some(group),
            _ => None,
        }).next()
        .unwrap();

    group
        .stream()
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Term(term) => Some(term.as_str().to_owned()),
            _ => None,
        }).next()
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

    let impls = fields
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
                    let set_fn_name = Ident::from(format!("set_{}_{}", ident, lower_variant));
                    let set_fn_plural = Ident::from(format!("set_{}_{}_vec", ident, lower_variant));
                    let variant = Ident::from(variant);

                    if is_concrete {
                        if is_option {
                            let single_1 = quote! {
                                /// Retrieve a value from the given struct
                                ///
                                /// This method deserializes the item from JSON, so be wary of using
                                /// this a lot.
                                ///
                                /// Possible errors from this method are `Error::NotFound` and
                                /// `Error::Deserialize`
                                pub fn #fn_name(&self) -> ::activitystreams_traits::Result<#variant> {
                                    ::activitystreams_traits::properties::from_item(&self.#ident)
                                }
                            };

                            let single_2 = quote! {
                                /// Set a value in the given struct
                                ///
                                /// This method serializes the item to JSON, so be wary of using this a
                                /// lot.
                                ///
                                /// Possible errors from this method are `Error::Serialize`
                                pub fn #set_fn_name(&mut self, item: #variant) -> ::activitystreams_traits::Result<()> {
                                    self.#ident = ::activitystreams_traits::properties::to_item(item)?;
                                    Ok(())
                                }
                            };

                            let single = quote! {
                                #single_1
                                #single_2
                            };

                            if is_functional {
                                single
                            } else {
                                let plural_1 = quote! {
                                    /// Retrieve many values from the given struct
                                    ///
                                    /// This method deserializes the item from JSON, so be wary of using
                                    /// this a lot.
                                    ///
                                    /// Possible errors from this method are `Error::NotFound` and
                                    /// `Error::Deserialize`
                                    pub fn #fn_plural(&self) -> ::activitystreams_traits::Result<Vec<#variant>> {
                                        ::activitystreams_traits::properties::from_item(&self.#ident)
                                    }
                                };

                                let plural_2 = quote! {
                                    /// Set many values in the given struct
                                    ///
                                    /// This method serializes the item to JSON, so be wary of using
                                    /// this a lot.
                                    ///
                                    /// Possible errors from this method are `Error::Serialize`
                                    pub fn #set_fn_plural(&mut self, item: Vec<#variant>) -> ::activitystreams_traits::Result<()> {
                                        self.#ident = ::activitystreams_traits::properties::to_item(item)?;
                                        Ok(())
                                    }
                                };

                                quote! {
                                    #single
                                    #plural_1
                                    #plural_2
                                }
                            }
                        } else if is_vec {
                            let single_1 = quote! {
                                /// Retrieve many values from the given struct
                                ///
                                /// This method deserializes the item from JSON, so be wary of using
                                /// this a lot.
                                ///
                                /// Possible errors from this method are `Error::Deserialize`
                                pub fn #fn_name(&self) -> ::activitystreams_traits::Result<Vec<#variant>> {
                                    ::activitystreams_traits::properties::from_vec(&self.#ident)
                                }
                            };

                            let single_2 = quote! {
                                /// Set many values in the given struct
                                ///
                                /// This method serializes the item to JSON, so be wary of using
                                /// this a lot.
                                ///
                                /// Possible errors from this method are `Error::Serialize`
                                pub fn #set_fn_name(&mut self, item: Vec<#variant>>) -> ::activitystreams_traits::Result<()> {
                                    self.#ident = ::activitystreams_traits::properties::to_vec(item)?;
                                    Ok(())
                                }
                            };

                            quote! {
                                #single_1
                                #single_2
                            }
                        } else {
                            let single = quote! {
                                /// Retrieve a value from the given struct
                                ///
                                /// This method deserializes the item from JSON, so be wary of using
                                /// this a lot.
                                ///
                                /// Possible errors from this method are `Error::Deserialize`
                                pub fn #fn_name(&self) -> ::activitystreams_traits::Result<#variant> {
                                    ::activitystreams_traits::properties::from_value(&self.#ident)
                                }

                                /// Set a value in the given struct
                                ///
                                /// This method serializes the item to JSON, so be wary of using this a
                                /// lot.
                                ///
                                /// Possible errors from this method are `Error::Serialize`
                                pub fn #set_fn_name(&mut self, item: #variant) -> ::activitystreams_traits::Result<()> {
                                    self.#ident = ::activitystreams_traits::properties::to_value(item)?;
                                    Ok(())
                                }
                            };

                            if is_functional {
                                single
                            } else {
                                let plural_1 = quote! {
                                    /// Retrieve many values from the given struct
                                    ///
                                    /// This method deserializes the item from JSON, so be wary of using
                                    /// this a lot.
                                    ///
                                    /// Possible errors from this method are `Error::Deserialize`
                                    pub fn #fn_plural(&self) -> ::activitystreams_traits::Result<Vec<#variant>> {
                                        ::activitystreams_traits::properties::from_value(&self.#ident)
                                    }
                                };

                                let plural_2 = quote! {
                                    /// Set many values in the given struct
                                    ///
                                    /// This method serializes the item to JSON, so be wary of using this
                                    /// a lot.
                                    ///
                                    /// Possible errors from this method are `Error::Serialize`
                                    pub #set_fn_plural(&mut self, item: Vec<#variant>) -> ::activitystreams_traits::Result<()> {
                                        self.#ident = ::activitystreams_traits::properties::to_value(item)?;
                                        Ok(())
                                    }
                                };

                                quote! {
                                    #single
                                    #plural_1
                                    #plural_2
                                }
                            }
                        }
                    } else if is_option {
                        let single_1 = quote! {
                            /// Retrieve a value of type T from the given struct
                            ///
                            /// This method deserializes the item from JSON, so be wary of using
                            /// this a lot.
                            ///
                            /// Possible errors from this method are `Error::NotFound` and
                            /// `Error::Deserialize`
                            pub fn #fn_name<T: #variant>(&self) -> ::activitystreams_traits::Result<T> {
                                ::activitystreams_traits::properties::from_item(&self.#ident)
                            }
                        };

                        let single_2 = quote! {
                            /// Set a value of type T in the given struct
                            ///
                            /// This method serializes the item to JSON, so be wary of using this a
                            /// lot.
                            ///
                            /// Possible errors from this method are `Error::Serialize`
                            pub fn #set_fn_name<T: #variant>(&mut self, item: T) -> ::activitystreams_traits::Result<()> {
                                self.#ident = ::activitystreams_traits::properties::to_item(item)?;
                                Ok(())
                            }
                        };

                        let single = quote!{
                            #single_1
                            #single_2
                        };

                        if is_functional {
                            single
                        } else {
                            let plural_1 = quote! {
                                /// Retrieve many values of type T from the given struct
                                ///
                                /// This method deserializes the item from JSON, so be wary of using
                                /// this a lot.
                                ///
                                /// Possible errors from this method are `Error::NotFound` and
                                /// `Error::Deserialize`
                                pub fn #fn_plural<T: #variant>(&self) -> ::activitystreams_traits::Result<Vec<T>> {
                                    ::activitystreams_traits::properties::from_item(&self.#ident)
                                }
                            };

                            let plural_2 = quote! {
                                /// Set many values of type T in the given struct
                                ///
                                /// This method serializes the item to JSON, so be wary of using
                                /// this a lot.
                                ///
                                /// Possible errors from this method are `Error::Serialize`
                                pub fn #set_fn_plural<T: #variant>(&mut self, item: Vec<T>) -> ::activitystreams_traits::Result<()> {
                                    self.#ident = ::activitystreams_traits::properties::to_item(item)?;
                                    Ok(())
                                }
                            };

                            quote! {
                                #single
                                #plural_1
                                #plural_2
                            }
                        }
                    } else if is_vec {
                        let single_1 = quote! {
                            /// Retrieve many values of type T from the given struct
                            ///
                            /// This method deserializes the item from JSON, so be wary of using
                            /// this a lot.
                            ///
                            /// Possible errors from this method are `Error::Deserialize`
                            pub fn #fn_name<T: #variant>(&self) -> ::activitystreams_traits::Result<Vec<T>> {
                                ::activitystreams_traits::properties::from_vec(&self.#ident)
                            }
                        };

                        let single_2 = quote! {
                            /// Set many values of type T in the given struct
                            ///
                            /// This method serializes the item to JSON, so be wary of using
                            /// this a lot.
                            ///
                            /// Possible errors from this method are `Error::Serialize`
                            pub fn #set_fn_name<T: #variant>(&mut self, item: Vec<T>) -> ::activitystreams_traits::Result<()> {
                                self.#ident = ::activitystreams_traits::properties::to_vec(item)?;
                                Ok(())
                            }
                        };

                        quote! {
                            #single_1
                            #single_2
                        }
                    } else {
                        let single_1 = quote! {
                            /// Retrieve a value of type T from the given struct
                            ///
                            /// This method deserializes the item from JSON, so be wary of using
                            /// this a lot.
                            ///
                            /// Possible errors from this method are `Error::Deserialize`
                            pub fn #fn_name<T: #variant>(&self) -> ::activitystreams_traits::Result<T> {
                                ::activitystreams_traits::properties::from_value(&self.#ident)
                            }
                        };

                        let single_2 = quote! {
                            /// Set a value of type T in the given struct
                            ///
                            /// This method serializes the item to JSON, so be wary of using this a
                            /// lot.
                            ///
                            /// Possible errors from this method are `Error::Serialize`
                            pub fn #set_fn_name<T: #variant>(&mut self, item: T) -> ::activitystreams_traits::Result<()> {
                                self.#ident = ::activitystreams_traits::properties::to_value(item)?;
                                Ok(())
                            }
                        };

                        let single = quote! {
                            #single_1
                            #single_2
                        };

                        if is_functional {
                            single
                        } else {
                            let plural_1 = quote! {
                                /// Retrieve many values of type T from the given struct
                                ///
                                /// This method deserializes the item from JSON, so be wary of using
                                /// this a lot.
                                ///
                                /// Possible errors from this method are `Error::Deserialize`
                                pub fn #fn_plural<T: #variant>(&self) -> ::activitystreams_traits::Result<Vec<T>> {
                                    ::activitystreams_traits::properties::from_value(&self.#ident)
                                }
                            };

                            let plural_2 = quote! {
                                /// Set many values of type T in the given struct
                                ///
                                /// This method serializes the item to JSON, so be wary of using this
                                /// a lot.
                                ///
                                /// Possible errors from this method are `Error::Serialize`
                                pub fn #set_fn_plural<T: #variant>(&mut self, item: Vec<T>) -> ::activitystreams_traits::Result<()> {
                                    self.#ident = ::activitystreams_traits::properties::to_value(item)?;
                                    Ok(())
                                }
                            };

                            quote! {
                                #single
                                #plural_1
                                #plural_2
                            }
                        }
                    }
                })
        });

    let mut tokens = Tokens::new();
    tokens.append_all(impls);

    let full = quote!{
        impl #name {
            #tokens
        }
    };

    full.into()
}

fn variants(attr: Attribute) -> Vec<(String, bool)> {
    let group = attr
        .tts
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Group(group) => Some(group),
            _ => None,
        }).next()
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
        }).flat_map(|i| i)
        .collect()
}

fn is_functional(attr: Attribute) -> bool {
    let group = attr
        .tts
        .clone()
        .into_iter()
        .filter_map(|token_tree| match token_tree {
            TokenTree::Group(group) => Some(group),
            _ => None,
        }).next()
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
