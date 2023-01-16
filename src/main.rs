fn main() {
    let mut a = [30, 3420, 329, 623];

    let r: Vec<_> = a.as_mut_slice().iter().map(|n| n * n).collect();

    println!("{r:?}");

    let p = true.then(||0xC0De);

    println!("{p:?}");

    println!("{}", 1i32.abs());

    let mut a = [-930, 823, 1, 0, 12, 4, 5];

    a.sort_by(|a,b |  a.cmp(&b));

    println!("{a:?}");
}
