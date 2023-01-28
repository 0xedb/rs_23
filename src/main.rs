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

impl<D: std::fmt::Debug> Fine for D {
    fn fine(&self) {
        println!("magic here!!");
    }

}

#[derive(Debug)]
struct Single;

fn main() {
    let sng = Single;
    sng.fine();

    let mine = Mine::new("0123456789 Hello World News".to_string());

    for i in mine {
        println!("{i}");
    }
}
