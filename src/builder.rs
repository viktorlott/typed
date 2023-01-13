use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    self,
    parse::Parse,
    parse_quote, token,
    visit::{visit_type, visit_type_path, Visit},
    Attribute, Fields, Generics, Ident, Token, Type, Visibility,
};
use std::{collections::HashSet};

use tools::{doc_struct, doc_type, format_code, modify};

#[path ="tools.rs"]
mod tools;

struct TypeModule {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    inner: TypeModuleInner,
}

struct TypeModuleInner {
    type_decls: Vec<TypeDecl>,
    struct_decl: TypeStructure,
}

struct TypeStructure {
    attrs: Vec<Attribute>,
    vis: Visibility,
    struct_token: Token![struct],
    ident: Ident,
    generics: Generics,
    fields: Fields,
    semi_colon: Option<Token![;]>,
}

struct TypeDecl {
    docs: Attribute,
    ident: Ident,
    ty: Type,
}

struct Source {
    name: String,
    code: String,
}


struct FieldTypeGenerics(HashSet<Ident>);

impl FieldTypeGenerics {
    fn get_idents(ty: &Type) -> Self {
        let mut field_type_generics = FieldTypeGenerics(<_>::default());
        visit_type(&mut field_type_generics, ty);
        field_type_generics
    }
}
impl<'ast> Visit<'ast> for FieldTypeGenerics {
    fn visit_type_path(&mut self, i: &'ast syn::TypePath) {
        if let Some(p) = i.path.segments.first() {
            self.0.insert(p.ident.clone());
        }
        visit_type_path(self, i);
    }
}


impl Parse for TypeModule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let code = format_code(input.to_string());
        println!("{}", code);

        let mut attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let struct_token: Token![struct] = input.parse()?;
        let ident: Ident = input.parse()?;
        let generics: Generics = input.parse()?;
        let mut semi_colon: Option<Token![;]> = None;

        let source = Source {
            name: ident.to_string(),
            code,
        };

        let mut type_decls: Vec<TypeDecl> = Vec::new();
        let mut fields: Fields;

        if input.peek(token::Brace) {
            fields = Fields::Named(input.parse()?);
            type_decls = parse_type_decls(&mut fields, &generics, &source);
        } else if input.peek(token::Paren) {
            fields = Fields::Unnamed(input.parse()?);
        } else {
            fields = Fields::Unit;
        }

        let docs = doc_struct(
            source.name.as_str(), 
            source.code.as_str()
        );

        attrs.push(parse_quote!(#[doc = #docs]));


        if input.peek(Token![;]) {
            semi_colon = input.parse().ok();
        }

        let struct_decl = TypeStructure {
            attrs: attrs.clone(),
            vis: parse_quote!(pub),
            struct_token,
            ident: format_ident!("ty", span = proc_macro2::Span::call_site()),
            generics,
            fields: fields.clone(),
            semi_colon,
        };


        let inner = TypeModuleInner {
            type_decls,
            struct_decl,
        };

        Ok(Self {
            attrs,
            vis,
            ident,
            inner,
        })
    }
}


impl ToTokens for TypeModule {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.attrs;
        let vis = &self.vis;
        let ident = &self.ident;
        let module_inner = &self.inner;

        
        let type_module = quote!(
            #[allow(non_snake_case)]
            #(#attrs)*
            #vis mod #ident {
                #![allow(non_camel_case_types)]

                #module_inner

            }
        );

        tokens.append_all(type_module);
    }
}

impl ToTokens for TypeModuleInner {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let type_delcs = &self.type_decls;
        let struct_decl = &self.struct_decl;

        let inner_decls = quote!(
            #(#type_delcs)*

            #struct_decl
        );

        tokens.append_all(inner_decls)
    }
}

impl ToTokens for TypeStructure {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let attrs = &self.attrs;
        let visibility = &self.vis;
        let struct_token = &self.struct_token;
        let ident = &self.ident;
        let generics = &self.generics;
        let fields = &self.fields;
        let semi_colon = &self.semi_colon;

        let struct_decl = quote!(
            #(#attrs)*
            #visibility #struct_token #ident #generics #fields #semi_colon
        );

        tokens.append_all(struct_decl);
    }
}

impl ToTokens for TypeDecl {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let docs = &self.docs;
        let ident = &self.ident;
        let ty = &self.ty;
        
        tokens.append_all(quote!(#docs pub type #ident = #ty;));
    }
}



impl TypeDecl {
    pub fn new<'a>(source_code: &'a str, ident: &'a Ident, ty: &'a Type) -> Self {
        let docs = doc_type(ident, ty, source_code);
        let docs: Attribute = parse_quote!(#[doc = #docs]);

        Self {
            docs,
            ident: ident.clone(),
            ty: ty.clone(),
        }
    }
}

fn parse_type_decls(fields: &mut Fields, generics: &Generics, source: &Source) -> Vec<TypeDecl> {
    let mut type_decls: Vec<TypeDecl> = Vec::new();

    let param_generics = generics
        .type_params()
        .map(|tp| tp.ident.clone())
        .collect::<HashSet<Ident>>();

    for (index, field) in fields.iter_mut().enumerate() {
        let field_type_generics = FieldTypeGenerics::get_idents(&field.ty);
        if param_generics.intersection(&field_type_generics.0).count() == 0 {
            let field_ident = modify(field, source.name.as_str(), index);
            type_decls.push(TypeDecl::new(source.code.as_str(), &field_ident, &field.ty));
        }
    }

    type_decls
}

pub fn codegen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let type_module = syn::parse::<TypeModule>(item).unwrap();
    let output = quote!(#type_module);
    output.into()
}
