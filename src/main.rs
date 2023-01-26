use std::thread;

fn main() {
    let name = "carb";
    let fee = 30.239;

    thread::scope(|s| {
        s.spawn(|| println!("scope 1 {name}"));

        s.spawn(|| {
            println!("hey {fee}");
        });
    });
}
