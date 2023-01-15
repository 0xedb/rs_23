// #[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
    }
}

fn main() {
    let one = Point { x: 0, y: 1 };
    let two = Point { x: 0, y: 2 };

    println!("{}", one == two);
}
