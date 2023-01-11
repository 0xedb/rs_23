fn main() {
    let mirror = |x| x;

    println!("{}", mirror(200));

    let one_to_ten = 1..=10;

    let l: Vec<_> = one_to_ten.map(|x| x * x).collect();

    let a = [54; 10];

    println!("{l:?} {a:?}");

    let play = |x| println!("{x} -- {a:?}");

    play(342);
    println!("{a:?}");

    let c: char = Default::default();
    println!("{c:?}");

    let  n = &mut 42342;
    *n = 0xedb;

    println!("{n}")
}

/// just a function called a
/// # Example
/// ```
/// let res = a("hello");
/// assert_eq!("hello", res);
/// ```
fn a(p: &str) -> &str {
    p
}
