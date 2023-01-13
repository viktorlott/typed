use ::rustfmt::{format_input, Input};
use quote::{format_ident, ToTokens};
use std::{io::Sink};
use syn::{self, parse_quote, Field, Ident, Type};


pub fn doc_type(name: &Ident, ty: &Type, source_code: &str) -> String {
    let name = name.to_string();
    let ty = ty.to_token_stream().to_string().replace(' ', "");

    format!(
        include_str!("docs/type.md"),
        source_code = source_code,
        name = name,
        ty = ty
    )
}
fn doc_field(name: &Ident, ty: &Type, parent: &str) -> String {
    let name = name.to_string();
    let ty = ty.to_token_stream().to_string().replace(' ', "");

    format!(
        include_str!("docs/field.md"),
        parent = parent,
        name = name,
        ty = ty
    )
}

pub fn doc_struct(name: &str, source_code: &str) -> String {
    format!(
        include_str!("docs/struct.md"),
        name = name,
        source_code = source_code
    )
}

pub fn format_code(orig: String) -> String {
    format_input(Input::Text(orig), &<_>::default(), None::<&mut Sink>)
        .map(|res| res.1.into_iter().next())
        .ok()
        .flatten()
        .map(|m| m.1.to_string())
        .expect("source_code input should be formatted")
}

pub fn create_ident(i: usize) -> impl Fn() -> Ident {
    move || format_ident!("ty_{i}", span = proc_macro2::Span::call_site())
}

pub fn modify(field: &mut Field, parent: &str, i: usize) -> Ident {
    let ident = field.ident.clone().unwrap_or_else(create_ident(i));
    let docu = doc_field(&ident, &field.ty, parent);

    field.vis = parse_quote!(pub);
    field.attrs.push(parse_quote!(#[doc = #docu]));

    ident
}
