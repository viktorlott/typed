#![allow(unused_variables)]

extern crate typer;
use typer::type_it;

#[type_it]
struct Container {
    current: i32,
    buffer: Vec<i32>,
}

#[type_it]
struct Tuple(i32, i32);

#[type_it]
struct Tuple2<T>(i32, T);

fn main() {
    let current: Container::current = 10;
    let buffer: Container::buffer = vec![current];
    let container: Container::ty = Container::ty { current, buffer };

    assert!(container.current == current);
}
