use std::cell::{Cell};

struct Something {
    name: Cell<i32>,
}

fn main() {
  let sm = Something{name: Cell::new(0xedb)};

  sm.name.replace(0xdead);

  println!("{:?}", sm.name);
}
