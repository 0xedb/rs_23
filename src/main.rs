mod another;

use another::MESSAGE;

const FEE: f64 = 390.837;

#[derive(Debug)]
struct App;

struct Feed {
    id: i32,
    tag: String,
}

impl Feed {
    fn notify(&self) {
        println!("notifying you of this feed!")
    }

    fn to_app(self) -> App {
        return App {};
    }

    fn get_id() -> &'static str {
        "FEED"
    }
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

    println!("{} {}", my_feed.id, my_feed.tag);
    my_feed.notify();

    let app = my_feed.to_app();
    println!("{app:?}");
    // my_feed.notify();

    let id = Feed::get_id();
    println!("{id}!! {MESSAGE}");
}
