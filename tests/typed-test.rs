#![allow(unused_variables)]

extern crate typer;
use typer::type_it;

#[type_it]
struct Container<T> {
    a: i32,
    b: Vec<i32>,
    c: Vec<T>,
    d: T,
}

#[type_it]
struct Tuple(i32, i32);

#[type_it]
struct Tuple2<T>(i32, T);

fn main() {
    let a: Container::a = 10;
    let b: Container::b = vec![a];
    let c: <Container::core<i64> as Container::protocol>::c = vec![10];
    let d: <Container::core<i64> as Container::protocol>::d = 10;
    let container: Container::core<i64> = Container::core { a, b, c, d };

    assert!(container.a == a);
}
