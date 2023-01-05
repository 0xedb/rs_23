const FEE: f64 = 390.837;

fn main() {
    let a = 239;
    println!("Hello, world!");

    println!("{FEE}");

    let mut i = 0;

    let ans = loop {
        i += 1;

        if i == 4 {
            break 200;
        }
    };

    println!("{ans}");

    let name = "some long name";

    let part = &name[0..3];

    println!("{part}");
}
