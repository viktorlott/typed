use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use std::collections::HashSet;
use syn::{
    self,
    parse::{Parse, ParseStream},
    parse_quote, token,
    visit::{visit_type, visit_type_path, Visit},
    Attribute, Fields, Generics, Ident, Token, Type, Visibility,
};

use tools::{doc_struct, doc_type, format_code, publicify_and_docify};

#[path = "tools.rs"]
mod tools;

struct TypeModule {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    inner: TypeModuleInner,
}

struct TypeModuleInner {
    type_decls: Vec<TypeAlias>,
    generic_decl: TypeGeneric,
    struct_decl: TypeStructure,
}

#[derive(Clone)]
struct TypeAlias {
    docs: Attribute,
    ident: Ident,
    ty: Type,
    has_gen: bool,
}

struct TypeGeneric {
    attrs: Vec<Attribute>,
    ident: Ident,
    generics: Generics,
    assoc_decls: Vec<TypeAlias>,
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
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let code = format_code(input.to_string());

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

        let mut type_decls: Vec<TypeAlias> = Vec::new();

        let fields: Fields = parse_fields(input, |fields| {
            type_decls = parse_type_decls(fields, &generics, &source)
        })?;

        let struct_doc = doc_struct(source.name.as_str(), source.code.as_str());

        attrs.push(parse_quote!(#[doc = #struct_doc]));

        if input.peek(Token![;]) {
            semi_colon = input.parse().ok();
        }

        let ty_ident = format_ident!("ty", span = proc_macro2::Span::call_site());
        let struct_decl = TypeStructure {
            attrs: attrs.clone(),
            vis: parse_quote!(pub),
            struct_token,
            ident: ty_ident,
            generics: generics.clone(),
            fields,
            semi_colon,
        };

        let gen_ident = format_ident!("gen", span = proc_macro2::Span::call_site());
        let assoc_decls: Vec<TypeAlias> = type_decls.to_vec();
        let generic_decl = TypeGeneric {
            attrs: attrs.clone(),
            ident: gen_ident,
            generics,
            assoc_decls,
        };

        let inner = TypeModuleInner {
            type_decls,
            generic_decl,
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
        let TypeModule {
            attrs,
            vis,
            ident,
            inner: module_inner,
        } = self;

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
        let type_delcs: Vec<TypeAlias> = self
            .type_decls
            .iter()
            .filter_map(|t| t.has_gen.then(|| t.clone()))
            .collect();

        let struct_decl = &self.struct_decl;
        let generic_decl= &self.generic_decl;

        let inner_decls = quote!(
            #(#type_delcs)*

            #struct_decl
            
            #generic_decl
        );

        tokens.append_all(inner_decls)
    }
}

impl ToTokens for TypeAlias {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let TypeAlias {
            docs, ident, ty, ..
        } = self;

        tokens.append_all(quote!(#docs pub type #ident = #ty;));
    }
}

impl ToTokens for TypeGeneric {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let trait_ident = &self.ident;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let mut assoc_decls = Vec::<TokenStream2>::new();
        let mut assoc_impl_decls = Vec::<TokenStream2>::new();
        let mut assoc_binds_decls = Vec::<TokenStream2>::new();

        for type_alias in self.assoc_decls.iter() {
            let TypeAlias {
                docs, ident, ty, ..
            } = type_alias;

            assoc_decls.push(quote!(#docs type #ident;));
            assoc_impl_decls.push(quote!(#docs type #ident = #ty;));
            assoc_binds_decls.push(quote!(#ident = Self::#ident))
        }

        let mut bind_generic: Option<TokenStream2> = None;

        if !assoc_binds_decls.is_empty() {
            bind_generic = Some(quote!(<#(#assoc_binds_decls,)*>));
        }

        tokens.append_all(quote!(
            pub trait #trait_ident {
                type __Bind: #trait_ident #bind_generic;

                #(#assoc_decls)*
            }

            impl #impl_generics #trait_ident for ty #ty_generics #where_clause {
                type __Bind = Self;
                #(#assoc_impl_decls)*
            }
        ));
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

impl TypeAlias {
    pub fn new<'a>(source_code: &'a str, ident: &'a Ident, ty: &'a Type, has_gen: bool) -> Self {
        let type_doc = doc_type(ident, ty, source_code);

        let docs: Attribute = parse_quote!(#[doc = #type_doc]);
        let ident = ident.clone();
        let ty = ty.clone();

        Self {
            docs,
            ident,
            ty,
            has_gen,
        }
    }
}

fn parse_fields<F>(input: ParseStream, mut cb: F) -> syn::Result<Fields>
where
    F: FnMut(&mut Fields),
{
    if input.peek(token::Brace) {
        let mut fields = Fields::Named(input.parse()?);
        cb(&mut fields);
        Ok(fields)
    } else if input.peek(token::Paren) {
        let mut fields = Fields::Unnamed(input.parse()?);
        cb(&mut fields);
        Ok(fields)
    } else {
        Ok(Fields::Unit)
    }
}

fn parse_type_decls(fields: &mut Fields, generics: &Generics, source: &Source) -> Vec<TypeAlias> {
    let mut type_decls: Vec<TypeAlias> = Vec::new();

    let param_generics = generics
        .type_params()
        .map(|tp| tp.ident.clone())
        .collect::<HashSet<Ident>>();

    for (index, field) in fields.iter_mut().enumerate() {
        let field_type_generics = FieldTypeGenerics::get_idents(&field.ty);
        let has_gen = param_generics.intersection(&field_type_generics.0).count() == 0;

        let field_ident = publicify_and_docify(field, source.name.as_str(), index);
        let type_decl = TypeAlias::new(source.code.as_str(), &field_ident, &field.ty, has_gen);

        type_decls.push(type_decl);
    }

    type_decls
}

pub fn codegen(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let type_module = syn::parse::<TypeModule>(item).unwrap();
    let output = quote!(#type_module);
    output.into()
}
