const FEE: f64 = 390.837;

struct Feed {
    id: i32,
    tag: String,
}

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

    let my_feed = Feed {
        id: 03,
        tag: String::from("t"),
    };

    println!("{} {}", my_feed.id, my_feed.tag)
}
