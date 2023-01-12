#[derive(Debug)]
struct Mine<T>(T);

impl<T> std::ops::Deref for Mine<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for Mine<T> {
    fn drop(&mut self) {
        println!("you got serve!!!");
    }
}

fn main() {
    // let mine = Mine(300);

    use std::rc::Rc;

    let r = Rc::new(Mine(2000));

    let p = vec![Rc::clone(&r)];
    println!("count {}", Rc::strong_count(&r));
    drop(p);
    let w = vec![Rc::clone(&r)];

    println!("count {}", Rc::strong_count(&r));

    // drop(mine);

    // println!("yours {mine:?}");
}
