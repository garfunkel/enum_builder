//! Simple macros that allow building enum types from variants that can be defined in multiple dispersed files in the crate.
//! ## Example
//! #### main.rs
//! ```
//! mod animals;
//!
//! use animals::*;
//!
//! #[enum_builder]
//! enum Animal {}
//!
//! // expanded result
//! // enum Animal {
//! //     Dog(Dog),
//! //     Cow(Cow),
//! //     Fish(Fish),
//! // }
//! ```
//!
//! #### animals.rs
//! ```
//! #[enum_builder_variant(Animal)]
//! struct Dog {}
//!
//! #[enum_builder_variant(Animal)]
//! struct Cow {}
//!
//! #[enum_builder_variant(Animal)]
//! struct Fish {}
//! ```
//!
//! ## Tips
//! It can be very useful to combine this crate with the
//! [enum_dispatch](https://docs.rs/enum_dispatch/latest/enum_dispatch) crate, to allow for a simple
//! "plugin" architecture without the overhead of dynamic dispatch. When doing this, take care to note
//! that the order of macros is important, as [macro@enum_builder] must be used before
//! [enum_dispatch](https://docs.rs/enum_dispatch/latest/enum_dispatch/attr.enum_dispatch.html).
//!
//! #### Example
//! ```
//! #[enum_builder]
//! #[enum_dispatch]
//! enum Animal {}
//! ```

use std::{ffi::OsStr, fs};

use proc_macro::{Span, TokenStream};
use quote::ToTokens;
use syn::{
	Item::{Enum, Struct, Type, Union},
	MetaNameValue, parse_macro_input,
	punctuated::Punctuated,
};
use walkdir::WalkDir;

fn valid_variant(enum_name: &syn::Ident, attrs: Vec<syn::Attribute>) -> bool {
	for attr in attrs {
		if let syn::Meta::List(list) = attr.meta {
			if list.path.to_token_stream().to_string() != "enum_builder_variant" {
				continue;
			}

			if list.tokens.to_token_stream().to_string() == enum_name.to_string() {
				return true;
			}
		}
	}

	false
}

/// Creates enum variants by discovering types annotated with [macro@enum_builder_variant].
/// Variants are searched recursively in source files located in the same directory as the macro.
///
/// ## Optional Parameters
/// #### path = [str]
/// Override the variant scan location with the provided file/directory path.
///
/// ## Examples:
/// ```
/// #[enum_builder]
/// enum Animal {}
/// ```
/// ```
/// #[enum_builder(path = "animals/")]
/// enum Animal {}
/// ```
/// ```
/// #[enum_builder(path = "animals.rs")]
/// enum Animal {}
/// ```
#[proc_macro_attribute]
pub fn enum_builder(attrs: TokenStream, item: TokenStream) -> TokenStream {
	let Some(dir) = Span::call_site().local_file() else {
		return item;
	};

	let mut dir = dir.parent().unwrap().to_owned();
	let parsed_item = parse_macro_input!(item);
	let mut enum_variants: Vec<String> = vec![];
	let attrs = parse_macro_input!(attrs with Punctuated::<MetaNameValue, syn::Token![,]>::parse_terminated);

	for attr in attrs {
		let name = attr.path.to_token_stream().to_string();

		if name != "path" {
			continue;
		}

		dir = dir.join(
			attr.value
				.to_token_stream()
				.to_string()
				.trim_matches('"')
				.to_owned(),
		);
	}

	let Enum(item_enum) = parsed_item else {
		return parsed_item.to_token_stream().into();
	};

	let enum_name = item_enum.ident;

	for entry in WalkDir::new(dir) {
		let Ok(entry) = entry else { continue };
		let path = entry.path();

		if path.is_dir() {
			continue;
		}

		if path.extension() != Some(OsStr::new("rs")) {
			continue;
		};

		let src = fs::read_to_string(path)
			.expect(format!("unable to read file {}", path.to_string_lossy()).as_str());
		let syntax = syn::parse_file(&src)
			.expect(format!("unable to parse file {}", path.to_string_lossy()).as_str());

		for item in syntax.items {
			let ident;
			let generics;

			match item {
				Struct(item) => {
					if !valid_variant(&enum_name, item.attrs) {
						continue;
					}

					ident = item.ident;
					generics = item.generics.to_token_stream().to_string();
				}
				Type(item) => {
					if !valid_variant(&enum_name, item.attrs) {
						continue;
					}

					ident = item.ident;
					generics = item.generics.to_token_stream().to_string();
				}
				Enum(item) => {
					if !valid_variant(&enum_name, item.attrs) {
						continue;
					}

					ident = item.ident;
					generics = item.generics.to_token_stream().to_string();
				}
				Union(item) => {
					if !valid_variant(&enum_name, item.attrs) {
						continue;
					}

					ident = item.ident;
					generics = item.generics.to_token_stream().to_string();
				}
				_ => continue,
			}

			enum_variants.push(format!("{}({}{})", ident, ident, generics));
		}
	}

	format!(
		"#[enum_dispatch]\nenum {enum_name}<'a> {{ {} }}",
		enum_variants.join(",\n")
	)
	.parse()
	.unwrap()
}

/// Creates a variant for the provided enum type.
///
/// ## Required Parameters
/// #### enum
/// Sets the enum type the variant is registered for.
///
/// ## Examples
/// ```
/// #[enum_builder_variant(Animal)]
/// struct Fish {}
/// ```
/// ```
/// #[enum_builder_variant(Animal)]
/// type Snake<'a> = ();
/// ```
/// ```
/// #[enum_builder_variant(Animal)]
/// enum Dog {
/// 	Kelpy,
/// 	BorderCollie,
/// 	Terrier,
/// }
/// ```
/// ```
/// #[enum_builder_variant(Animal)]
/// union BarnAnimal {
///		Horse: u32,
///		Donkey: f32,
///	}
/// ```
#[proc_macro_attribute]
pub fn enum_builder_variant(_: TokenStream, item: TokenStream) -> TokenStream {
	item
}
