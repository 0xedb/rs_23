use futures::executor::block_on;

async fn hello_world() {
    println!("hello world");
}

fn main() {
    let hw = hello_world();

    block_on(hw);
}
