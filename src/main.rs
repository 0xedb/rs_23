#[derive(Debug)]
struct Toffee;

fn main() {
    let items = vec![Toffee, Toffee, Toffee];

    let choice = &items[0];

    println!("{choice:?}");
    println!("{items:?}");

    panic!("Sad situation");
}