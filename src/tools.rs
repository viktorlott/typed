use ::rustfmt::{format_input, Input};
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, Type, Field, parse_quote, DeriveInput};
use std::{io::Sink, marker::PhantomData};

pub struct TypeDecl<'a> {
    original: &'a str,
    ident: Ident,
    ty: Type,
    _marker: PhantomData<fn(&'a ()) -> &'a ()>
}

impl<'a> TypeDecl<'a> {
    pub fn new(original: &'a str, ident: Ident, ty: &'a Type) -> Self {
        Self {
            original,
            ident,
            ty: ty.clone(),
            _marker: PhantomData
        }
    }
}

impl<'a> ToTokens for TypeDecl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let original = self.original;
        let ident = &self.ident;
        let ty = &self.ty;

        let docu = doc_type(ident, ty, original);
        let ty_decl = quote!(#[doc = #docu] pub type #ident = #ty;);
        tokens.append_all(ty_decl);
    }
}

pub fn doc_type(name: &Ident, ty: &Type, original: &str) -> String {
    let name = name.to_string();
    let ty = ty.to_token_stream().to_string().replace(" ", "");

    format!(
        include_str!("docs/type.md"),
        original = original,
        name = name,
        ty = ty
    )
}
pub fn doc_field(name: &Ident, ty: &Type, parent: &str) -> String {
    let name = name.to_string();
    let ty = ty.to_token_stream().to_string().replace(" ", "");

    format!(
        include_str!("docs/field.md"),
        parent = parent,
        name = name,
        ty = ty
    )
}

pub fn doc_struct(name: &str, original: &str) -> String {
    format!(
        include_str!("docs/struct.md"),
        name = name,
        original = original
    )
}



pub fn format_code(orig: String) -> String {
    format_input(Input::Text(orig), &<_>::default(), None::<&mut Sink>)
        .map(|res| res.1.into_iter().next())
        .ok()
        .flatten()
        .map(|m| m.1.to_string())
        .expect("Original input should be formatted")
}

pub fn create_ident(i: usize) -> impl Fn() -> Ident  {
    move || Ident::new(format!("ty_{i}").as_str(), proc_macro2::Span::call_site())
}

pub fn modify(field: &mut Field, parent: &str, i: usize) -> Ident {
    let ident = field.ident.clone().unwrap_or_else(create_ident(i));
    let docu = doc_field(
        &ident,
        &field.ty,
        parent,
    );

    field.vis = parse_quote!(pub);
    field.attrs.push(parse_quote!(#[doc = #docu]));

    ident
}

pub fn publicify(ast: &mut DeriveInput) {
    ast.ident = parse_quote!(ty);
    ast.vis = parse_quote!(pub);
}



