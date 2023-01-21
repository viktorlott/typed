use proc_macro::TokenStream;

mod tools;
mod builder;

/// Use `type_it` as a proc-macro.
/// - *Every field should be documented so it's easier to know what is what.*
/// 
/// # Example
/// ```no_run
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
/// ```no_run
/// let current: Container::current = 10;
/// let buffer: Container::buffer = vec![current];
/// let another: <Container::core<u8> as Container::protocol>::another = 20;
/// let container: Container::core<u8> = 
///     Container::core {
///         current,
///         buffer,
///         another
///     };
/// ```
/// - It's also possible to use it as following:
/// ```no_run
/// trait Trait: Container::protocol {
///     fn retrieve(&self) -> Container::protocol::buffer;
///     fn extend(&mut self, val: Container::protocol::another); 
/// }
/// ```
/// 
#[proc_macro_attribute]
pub fn type_it(_attr: TokenStream, item: TokenStream) -> TokenStream {
    builder::codegen(_attr, item)
}


