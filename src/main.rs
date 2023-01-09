

trait Empty {
    fn say(&self){println!("Hi!")}
}

#[derive(Debug)]
struct Something;

impl Empty for Something {}

fn f(t: &impl Empty) {
    t.say()
}

fn main() {
    let s = Something;

    f(&s);
    s.say();
}
