# Dismantler

[<img alt="github" src="https://img.shields.io/github/languages/code-size/viktorlott/typed?style=flat-square&logo=github" height="20">](https://github.com/viktorlott/typed)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dismantler?style=flat-square&logo=rust" height="20">](https://crates.io/crates/dismantle)


`Dismantler` is a procedural macro that is used for disassembling `structs` and `fns` into their inner `type` components that are then accompanied with documentation and examples. The `Dismantler` structures will be wrapped into a module and reassigned with a name (default `core`), this also goes for the `static` and `generic` fields.


#### **Project is still under development**

## Examples
```toml
[dependencies]
dismantler = "0.0.1"
```

```rust
use dismantler::dismantle;

#[dismantle]
struct Container<C: Clone, T = i64> {
    a: i32,
    b: Vec<i32>,
    c: Vec<T>,
    d: C,
    e: T
}

#[dismantle]
struct Tuple(i32, i32);

#[dismantle]
struct Tuple2<T>(i32, T);

fn main() {
    let a: Container::a = 10;
    let b: Container::b = vec![a];
    let c: Container::c<i64> = vec![10];
    let c: <Container::core<i64> as Container::protocol>::c = c;
    let d: <Container::core<i64> as Container::protocol>::d = 10;
    let container: Container::core<i64> = Container::core { a, b, c, d, e: 10 };

    assert!(container.a == a);
}
```

### More examples

```rust
use dismantler::dismantle;

#[dismantle]
struct Containter<T> {
    current: u8,
    buffer: Vec<u8>,
    another: T,
}
#[dismantle]
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


### Current support:
  - [x] **Struct** — `static` and `generic` types
  - [ ] **Function** — `static` and `generic` types


### Disassembler
```rust
#[dismantle]
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

### Future plans 
#### Renaming disassembler
```rust
use dismantler::dismantle;

#[dismantle = "MContainer"]
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

