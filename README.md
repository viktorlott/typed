# rust-typed

`Typed` is a procedural macro that is used for disassembling `structs` and `fns` into their inner `type` components that are then accompanied with documentation and examples. The `Typed` structures will be wrapped into a module and reassigned with a name (default `core`), this also goes for the `static` and `generic` fields.


#### *Project is still under development*

# Struct example
```rust
#[type_it]
struct Containter<T> {
    current: u8,
    buffer: Vec<u8>,
    another: T,
}
#[type_it]
struct Area(i32);
```
- Will let you access the struct types as followed:
```rust
let current: Container::current = 10;
let buffer: Container::buffer = vec![current];
let another: <Container::core<u8> as Container::protocol>::another = 20;
let container: Container::core<u8> = 
    Container::core {
        current,
        buffer,
        another
    };
```
- It's also possible to use it as following:
```rust
trait Trait: Container::protocol {
    fn retrieve(&self) -> Container::protocol::buffer;
    fn extend(&mut self, val: Container::protocol::another); 
}
```

# Disassembler
```rust
#[type_it]
struct #name {
    #(#ident: #ty)*
}

// Turns into

#[allow(non_snake_case)]
// Docs (/w examples) describing the original `item` and also what `types` are available to use.
#[doc = #docs] 
pub mod #name {
    #![allow(non_camel_case_types)]
    
    // The static fields of the `item` as type aliases.
    #(#ty_decls)* // Access through `#name::#field`
    
    // A trait where all `ìtem` fields are associated types
    #struct_generic // Access through `#name::gen`
    
    // Docs (/w examples) describing the original `item`.
    #[doc = #docs]
    // The original `ìtem`.
    #struct_original // Access through `#name::core`
}
```

# Future plans 
#### Renaming disassembler
```rust
#[type_it = "MContainer"]
struct Containter<T> {
    current: u8,
    buffer: Vec<u8>,
    another: T,
}

fn main() {
    let x: Container<i32> = {
        current: 10,
        buffer: Vec::default(),
        another: 20,
    }
    
    let y: MContainer::core<i32> = x;
}
```

