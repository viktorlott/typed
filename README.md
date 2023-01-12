
`Typed` is a procedural macro that is used for disassembling `structs`, `enums` and `fns` into their inner `type` components that are then accompanied with documentation and examples. The `Typed` structures will be wrapped into a module and reassigned with a name (default `ty`), this also goes for the `static` and `generic` fields.
<br />
<br />
#### *Project is still under development*
<br />
<br />

#### Current support:
- `struct`
  - [x] static types
  - [ ] generic types
- `enum`
  - [ ] static types
  - [ ] generic types
- `fn`
  - [ ] static types
  - [ ] generic types


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
let another: <Container::ty<u8> as Container::proto>::another = 20;
let container: Container::ty<u8> = 
    Container::ty {
        current,
        buffer,
        another
    };
```
- It's also possible to use it as following:
```rust
trait Trait: Container::proto {
    fn retrieve(&self) -> Container::proto::buffer;
    fn extend(&mut self, val: Container::proto::another); 
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
    
    let y: MContainer::ty<i32> = x;
}
```

