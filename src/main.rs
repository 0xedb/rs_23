use std::sync::Mutex;

fn main() {
    let mine = Mutex::new(0xedb);

    {
        let mut v = mine.lock().unwrap();
        *v = 3932;

        println!("{v}");
    }
}
