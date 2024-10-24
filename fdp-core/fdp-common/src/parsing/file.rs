use crate::parsing::definition::AppDefinitionModule;
use quote::quote;
use syn::parse::Parse;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn parse_file(contents: &str) {
    let file = syn::parse_file(&contents).unwrap();
    for item in file.items {
        match item {
            syn::Item::Mod(item_mod) => {
                if item_mod.ident == "definition" {
                    // We have the pub mod definition {} module

                    let app_definition: AppDefinitionModule =
                        syn::parse2(quote! { #item_mod }).unwrap();
                    p!("{}", quote! { #app_definition });
                }
            }
            _ => {}
        }
    }
}

pub struct AppDefinitionFile {}

impl Parse for AppDefinitionFile {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _file: syn::File = input.parse()?;

        todo!()
    }
}

/*
let app1_file_contents = fs::read_to_string("src/apps/app_1.rs").unwrap();
    let app1_syntax = parse_file(&app1_file_contents).unwrap();

    let definition_module = app1_syntax
        .items
        .iter()
        .find_map(|item| {
            if let Item::Mod(module) = item {
                if module.ident == "definition" {
                    return Some(module);
                }
            }
            None
        })
        .expect("Failed to find definition module in app_1.rs");

    // Now you have the `definition` module from app_1.rs in the `definition_module` variable
    // You can further process it as needed
*/
