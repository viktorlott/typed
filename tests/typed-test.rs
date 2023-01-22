#![allow(unused_variables)]

extern crate dismantler;
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
    let c: Container::c = c;
    let c: <Container::core<i64> as Container::protocol>::c = c;
    let d: <Container::core<i64> as Container::protocol>::d = 10;
    let container: Container::core<i64> = Container::core { a, b, c, d, e: 10 };



    assert!(container.a == a);
}
