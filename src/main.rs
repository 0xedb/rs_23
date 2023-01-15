trait Serial {
    fn serialize(&self);
}

#[derive(Debug)]
struct HTML;

#[derive(Debug)]
struct Svelte;

#[derive(Debug)]
struct React;

fn serilize_it(s: Vec<Box<dyn Serial>>) {
    s.iter().for_each(|a| a.serialize())
}

fn ss(s: &dyn Serial) {}

// fn bb(s: Serial) {}

unsafe fn i_can_byte() {
    println!("byting you!");
}

static 

fn main() {
    for i in 0..10 {
        println!("{i}")
    }

    let mut p = 3493;

    let mut v = &mut p as *mut i32;

    unsafe {
        *v = 0xedb;
        println!("{}", *v);
    }

    unsafe {
        i_can_byte();
    }
}
