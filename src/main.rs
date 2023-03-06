#[derive(Debug, Clone, Copy)]
struct Mine;

enum Value {
    One,
    Two,
}

impl From<i32> for Value {
    fn from(_: i32) -> Self {
        Value::One
    }
}

impl From<i128> for Value {
    fn from(_: i128) -> Self {
        Value::Two
    }
}

fn get_value<T>(v: T)
where
    T: Into<Value>,
{
    match &v.into() {
        Value::One => println!("got one!!!"),
        Value::Two => println!("got two!!!"),
    }
}

fn echo<T>(_: T) {}

fn main() {
    get_value(203i32);
    get_value(23023i128);

    let items = [Mine; 20];

    println!("{items:?}");

    let m = Mine;
    echo(m);

    println!("--{m:?}");
    let num = 0xedb;

    echo(num);

    println!("{num}");

    // let m =
}
