use std::sync::Mutex;

struct Mine;

impl !Send for Mine{};

fn main() {
    let mine = Mutex::new(0xedb);

    {
        let mut v = mine.lock().unwrap();
        *v = 3932;

        println!("{v}");
    }
}
