use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    #![warn(box_pointers)]

    let b = Box::new(5);
    println!("b={}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let m = MyBox::new(x);
    assert_eq!(5, *m);

    let c = MyBox::new(String::from("Rust"));
    hello(&c);
    hello(&(*c)[..]);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello {}", name);
}
