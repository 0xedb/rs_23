use std::{cell::RefCell, rc::Rc};

struct A {
    next: Rc<RefCell<A>>,
}

// struct B {
//     data: Vec<impl std::fmt::Display>
// }

fn main() {
    let mut gen = 32;
    let p = &mut 323;
    let v: &mut i32 = &mut gen;
}
