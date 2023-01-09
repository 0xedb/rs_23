trait Empty {
    fn say(&self) {
        println!("Hi!")
    }
}

#[derive(Debug)]
struct Something;

impl Empty for Something {}

fn f(t: &impl Empty) {
    t.say()
}

fn shout() -> &'static str {
    "shouting!"
}

fn exp<'a>(f: &'a str) {
    println!("{f}")
}

fn main() {
    let r#strict = "so strict";
    let s = Something;

    exp("dkafj");

    f(&s);
    s.say();
    println!("{strict}");

    let b = b'H';
    let bb = b"hello world ";
    let aa = r#"hello!"#;

    println!("{bb:?} {aa} {b:?}");

    let a = "foo";
    let b = "fo\
    o";
    let c = "fo\


    o";

    println!("{} {} {}", a == b, b == c, c == a);

    // let l16 = 390348u8;


}
