use ::rustfmt::{format_input, Input};
use quote::{format_ident, ToTokens, quote};
use std::{io::Sink, collections::HashSet};
use syn::{self, parse_quote, Field, Ident, Type, Generics};


/// Use `new!(..)` to construct `structs`. 
/// - `clone[$ident,*]`  -> `$ident.clone(),*`
/// - `string[$ident,*]` -> `$ident.to_string(),*`
/// - `into[$ident,*]`   -> `$ident.into(),*`
/// - `$ident($tt)`      -> `$ident: $tt`
/// 
/// # Example
/// ```no_run
/// struct TypeStructure {
///     attrs: Vec<Attribute>,
///     vis: Visibility,
///     struct_token: Token![struct],
///     ident: Ident,
///     generics: Generics,
///     fields: Fields,
///     semi_colon: Option<Token![;]>,
/// }
/// 
/// let attrs = ...;
/// let generics ...;
/// 
/// let struct_decl = new!({
///     clone[attrs, generics], 
///     vis(parse_quote!(pub)),
///     ident(format_ident!("core", span = proc_macro2::Span::call_site())),
///     struct_token,
///     fields,
///     semi_colon,
/// }: TypeStructure);
/// ```
/// 
macro_rules! new {
    // Invoke syntax
    ({ $($tail:tt)* } = $name:ident                                                  ) => { new!($name [] $($tail)*) };
    ({ $($tail:tt)* }: $name:ident                                                   ) => { new!($name [] $($tail)*) };
    ({ $($tail:tt)* } => $name:ident                                                 ) => { new!($name [] $($tail)*) };
    ({ $($tail:tt)* } $name:ident                                                    ) => { new!($name [] $($tail)*) };

    ($name:ident    { $($tail:tt)* }                                                 ) => { new!($name [] $($tail)*) };
    ($name:ident => { $($tail:tt)* }                                                 ) => { new!($name [] $($tail)*) };
    ($name:ident:   { $($tail:tt)* }                                                 ) => { new!($name [] $($tail)*) };
    ($name:ident =  { $($tail:tt)* }                                                 ) => { new!($name [] $($tail)*) };

    ($name:ident => $($tail:tt)*                                                     ) => { new!($name [] $($tail)*) };
    ($name:ident:   $($tail:tt)*                                                     ) => { new!($name [] $($tail)*) };
    ($name:ident =  $($tail:tt)*                                                     ) => { new!($name [] $($tail)*) };


    // Grammer
    ($name:ident [$($stored:tt)*] $(.)? into $(.)? [$($field:ident),*]     $(, $($tail:tt)*)?) 
        => { new!($name [$(, $stored)* $($field: $field.into()),*] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*] $(.)? string $(.)? [$($field:ident),*]   $(, $($tail:tt)*)?) 
        => { new!($name [$(, $stored)* $($field: $field.to_string()),*] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*] $(.)? str $(.)? [$($field:ident),*]      $(, $($tail:tt)*)?) 
        => { new!($name [$(, $stored)* $($field: $field.as_str()),*] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*] $(.)? clone $(.)? [$($field:ident),*]    $(, $($tail:tt)*)?) 
        => { new!($name [$(, $stored)* $($field: $field.clone()),*] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*] $(.)? clone $(.)? [$($field:ident as $alias:ident),*]    $(, $($tail:tt)*)?) 
        => { new!($name [$(, $stored)* $($alias: $field.clone()),*] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*] $field:ident: $field2:ident              $(, $($tail:tt)*)?) 
        => { new!($name [$($stored)*, $field: $field2] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*] $field:ident ($($sym:tt)*)               $(, $($tail:tt)*)?) 
        => { new!($name [$field: $($sym)*, $($stored)*] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*] $field:ident                             $(, $($tail:tt)*)?) 
        => { new!($name [$field, $($stored)*] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*] ..                                       $(, $($tail:tt)*)?) 
        => { new!([.., $($stored)*] $($($tail)*)? ) };

    ($name:ident [$($stored:tt)*]                                                            ) 
        => { $name { $($stored)* } };
}

macro_rules! ident {
    ($name:tt) => { format_ident!($name, span = proc_macro2::Span::call_site()) };
}

pub(crate) use new;
pub(crate) use ident;


pub fn doc_type(name: &Ident, ty: &Type, generics: &Option<Generics>, source_code: &str) -> String {
    let name = name.to_string();
    let ty = ty.to_token_stream().to_string().replace(' ', "");
    let generics = quote!(#generics).to_string();

    format!(
        include_str!("docs/type.md"),
        source_code = source_code,
        name = name,
        ty = ty,
        generics = generics
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
        .map(|(_, v, _)| v.into_iter().next())
        .ok()
        .flatten()
        .map(|(_, m)| m.to_string())
        .expect("source_code input should be formatted")
}

pub fn create_ident(i: usize) -> impl Fn() -> Ident {
    move || format_ident!("ty_{i}", span = proc_macro2::Span::call_site())
}

pub fn publicify_and_docify(field: &mut Field, parent: &str, i: usize) -> Ident {
    let ident = field.ident.clone().unwrap_or_else(create_ident(i));
    let docu = doc_field(&ident, &field.ty, parent);

    field.vis = parse_quote!(pub);
    field.attrs.push(parse_quote!(#[doc = #docu]));

    ident
}

pub fn get_generic_idents(generics: &Generics) -> HashSet<Ident> {
    generics
        .type_params()
        .map(|tp| tp.ident.clone())
        .collect::<HashSet<Ident>>()
}
