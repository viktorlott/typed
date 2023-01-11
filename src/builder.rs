use proc_macro::TokenStream;
use quote::{quote};
use syn::{parse, Data, DeriveInput};
use tools::{modify, format_code, doc_struct, TypeDecl, publicify};

#[path = "tools.rs"]
mod tools;


pub fn codegen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse(item.clone()).unwrap();

    let mut struct_entry = ast.clone();

    publicify(&mut struct_entry);

    let Data::Struct(ref mut data_struct) = struct_entry.data else {
        panic!("Cannot destruct Struct");
    };

    let parent = ast.ident.to_string();
    let original = format_code(item.to_string());

    let mut ty_decls: Vec<TypeDecl> = Vec::new();

    for (index, field) in data_struct.fields.iter_mut().enumerate() { 
        let ident = modify(field, parent.as_str(), index);
        ty_decls.push(TypeDecl::new(original.as_str(), ident, &field.ty));
    }

    let name = ast.ident;
    let docs = doc_struct(parent.as_str(), original.as_str());

    let output = quote!(
        #[allow(non_snake_case)]
        #[doc = #docs]
        pub mod #name {
            #![allow(non_camel_case_types)]

            #(#ty_decls)*

            #[doc = #docs]
            #struct_entry
        }
    );

    output.into()
}
