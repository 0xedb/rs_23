fn main() {
    let items: Vec<_> = (0..=1000).into_iter().map(|x| x + 1).collect();

    let ans = items.binary_search(&1);

    if ans.is_ok() {
        println!("yaaaay {}", ans.unwrap());
    }

    println!("{}", u8::MAX);
}
