// vecs & slices

struct Mine {
    pos: usize,
    message: String,
}

impl Mine {
    fn new(message: String) -> Self {
        Self {
            pos: Default::default(),
            message,
        }
    }
}

impl Iterator for Mine {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < 10 {
            let p = self.pos;
            self.pos += 1;
            return Some(self.message.chars().nth(p).unwrap());
        }

        None
    }
}

trait Fine {
    fn fine(&self);
}

impl<D> Fine for D
where
    D: std::fmt::Debug + Clone,
{
    fn fine(&self) {
        println!("magic here!!");
    }
}

#[derive(Debug, Clone)]
struct Single;

fn main() {
    let sng = Single;
    sng.fine();

    let items = [0, 10, 20, 40];

    let ans = items.iter().fold(0, |acc, x| acc + x);
    let ans2 = items.into_iter().reduce(|acc, x| acc + x).unwrap();

    println!("{ans} {ans2} {items:?}");
}
