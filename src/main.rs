 
macro_rules! plenty {
    ($($i: literal),*) => {
    {let mut m = Vec::new();

    $(
      m.push($i);
    )*

    m}

    };
}

fn main() {
    let name = "carb";
    let fee = 30.239;

    let res = plenty!(1, 2, 3, 4, 5, 0xedb);

    println!("{res:?}");

    // thread::scope(|s| {
    //     s.spawn(|| println!("scope 1 {name}"));

    //     s.spawn(|| {
    //         println!("hey {fee}");
    //     });
    // });
}
