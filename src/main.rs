// // // use futures::executor::block_on;

// // // async fn say_my_name() {
// // //     println!("yes sir, saying your name");
// // // }

// // // async fn hello_world() {
// // //     say_my_name().await;
// // //     println!("hello world");
// // // }

// // // fn main() {
// // //     let hw = hello_world();

// // //     block_on(hw);
// // // }

// // use std::ops::Not;

// // #[derive(Debug, Default)]
// // struct Mine {
// //     something: Option<i32>,
// // }

// // impl Not for Mine {
// //     type Output = Self;
// //     fn not(self) -> Self::Output {
// //         Mine {
// //             ..Default::default()
// //         }
// //     }
// // }

// // use tokio::task;

// // #[inline]
// // async fn for_tokio() -> &'static str {
// //     "funny right?"
// // }

// // #[tokio::main]
// // async fn main() {
// //     let ans = for_tokio();

// //     println!("{}", ans.await);

// //     let a = 300;

// //     task::spawn(async move {
// //         println!("inside tokio thread {a}");
// //     })
// //     .await
// //     .unwrap();

// //     println!("from main thread");

// //     let mine = Mine{something: Some(0xedb)};
// //     print!("{mine:?}");
// //     let n_mine = !mine;

// //     print!("{n_mine:?}");
// // }

// macro_rules! t_vec {
//     ($t: expr, $e: expr) => {{
//         [$t; $e].to_vec()
//     }};
// }

// macro_rules! magic_answer {
//     {} => {println!("INCORRECT")};
//     ("open") => {
//         println!("CORRECT")
//     }
// }

// fn main() {
//     magic_answer!();
//     magic_answer!("open");

//     let one = vec![1, 2, 3, 4];
//     let two = vec![1, 2, 3, 4];
//     let three = vec![1, 2, 3, 4];

//     println!("{one:?} {two:?} {three:?}");

//     // let e = Vec::with_capacity(200);

//     let fs = t_vec!(32, 10);

//     // let a = Vec::<i32>::with_capacity(200);
//     // let tt = [32; 10].to_vec();

//     println!("{fs:?}");
// }

macro_rules! p {
    () => {
        compile_error!("terrible situation");
    };
}

fn main() {
    // let nothing = Something{};

    // println!("{nothing:?}");
    p!();
}
