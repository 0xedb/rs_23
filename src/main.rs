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
            Some(self.message.chars().nth(p).unwrap())
        } else {
            None
        }
    }
}

fn main() {
    let mine = Mine::new("2023 Hello World News".to_string());

    for i in mine {
        println!("{i}");
    }
}
