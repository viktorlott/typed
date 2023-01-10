use proc_macro::TokenStream;

mod builder;

/// Use `type_it` as a proc-macro.
/// - *Every field should be documented so it's easier to know what is what.*
/// 
/// # Example
/// ```
/// #[type_it]
/// struct Containter<T> {
///     current: u8,
///     buffer: Vec<u8>,
///     another: T,
/// }
/// #[type_it]
/// struct Area(i32);
/// ```
/// - Will let you access the struct types as followed:
/// ```
/// let current: Container::current = 10;
/// let buffer: Container::buffer = vec![current];
/// let another: <Container::ty<u8> as Container::proto>::another = 20;
/// let container: Container::ty<u8> = 
///     Container::ty {
///         current,
///         buffer,
///         another
///     };
/// ```
/// - It's also possible to use it as following:
/// ```
/// trait Trait: Container::proto {
///     fn retrieve(&self) -> Container::proto::buffer;
///     fn extend(&mut self, val: Container::proto::another); 
/// }
/// ```
/// 
#[proc_macro_attribute]
pub fn type_it(_attr: TokenStream, item: TokenStream) -> TokenStream {
    builder::codegen(_attr, item)
}


