use std::sync::Mutex;
use std::thread;

fn main() {
    let value_mtx = Mutex::new(0xed);

    thread::scope(|s| {
        let t = s.spawn(|| {
            let mut v = value_mtx.lock().unwrap();
            *v = 0xc0de;
            println!("value {:?}", v);
        });

        let b = s.spawn(|| {
            let mut v = value_mtx.lock().unwrap();
            *v = 0xedb;
            print!("another value {:?}", v);
        });
    });
}
