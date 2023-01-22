#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;

mod builder;
mod tools;

/// Use `dismantle` as a proc-macro.
/// - *Every field should be documented so it's easier to know what is what.*
///
/// # Example
/// ```no_run
/// #[dismantle]
/// struct Containter<T> {
///     current: u8,
///     buffer: Vec<u8>,
///     another: T,
/// }
/// #[dismantle]
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
pub fn dismantle(_attr: TokenStream, item: TokenStream) -> TokenStream {
    builder::codegen(_attr, item)
}
