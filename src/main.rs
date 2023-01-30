// use futures::executor::block_on;

// async fn say_my_name() {
//     println!("yes sir, saying your name");
// }

// async fn hello_world() {
//     say_my_name().await;
//     println!("hello world");
// }

// fn main() {
//     let hw = hello_world();

//     block_on(hw);
// }

async fn for_tokio() {
    println!("Welcome to Tokio");
}

#[tokio::main]
async fn main() {
    let ans = for_tokio();

    ans.await;
}
