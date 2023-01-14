#![allow(unused_variables)]

extern crate typer;
use typer::type_it;

#[type_it]
struct Container<T> {
    a: i32,
    b: Vec<i32>,
    c: Vec<T>,
    d: T
}

#[type_it]
struct Tuple(i32, i32);

#[type_it]
struct Tuple2<T>(i32, T);



fn main() {
    let a: Container::a = 10;
    let b: Container::b = vec![a];
    let c: <Container::ty<i64> as Container::gen>::c = vec![10];
    let d: <Container::ty<i64> as Container::gen>::d = 10;
    let container: Container::ty<i64> = Container::ty { a, b, c, d };

    assert!(container.a == a);
}
