// // use std::collections::{BTreeMap, BinaryHeap, HashMap, LinkedList};

// // #[derive(PartialEq, PartialOrd, Debug, Eq, Clone)]
// // struct Point {
// //     x: i32,
// //     y: i32,
// //     degree: usize,
// // }

// // impl Ord for Point {
// //     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
// //         self.degree.cmp(&other.degree)
// //     }
// // }

// // fn main() {
// //     let mut btm = BTreeMap::new();

// //     btm.insert(11, "one");
// //     btm.insert(22, "two");
// //     btm.insert(3, "three");
// //     btm.insert(54, "four");

// //     let r = btm
// //         .entry(111)
// //         .and_modify(|v| *v = "____///")
// //         .or_insert("3000");
// //     println!("{r}");

// //     let mut bh = BinaryHeap::new();

// //     bh.push(Point {
// //         x: 1,
// //         y: 2,
// //         degree: 200,
// //     });
// //     bh.push(Point {
// //         x: 1,
// //         y: 2,
// //         degree: 0,
// //     });
// //     bh.push(Point {
// //         x: 1,
// //         y: 2,
// //         degree: 600,
// //     });

// //     let ans: Vec<_> = bh.clone().into_sorted_vec();
// //     println!("{ans:?}");

// //     let ans: Vec<_> = bh.iter().rev().collect();

// //     println!("{ans:?}");

// //     let mut ll = LinkedList::new();
// //     ll.push_front(1);
// //     ll.push_front(2);
// //     ll.push_front(3);
// //     ll.push_front(4);

// //     let s = ll.split_off(2);

// //     println!("{:?}\t{:?}", ll, s);
// // }

// #[derive(Debug)]
// struct Mine<T> {
//     data: T,
// }
// // impl AsRef<usize> for Mine<usize> {
// //     fn as_ref(&self) -> &usize {
// //         &2000
// //     }
// // }

// impl<T> AsRef<T> for Mine<T> {
//     fn as_ref(&self) -> &T {
//         &self.data
//     }
// }

// fn main() {
//     let mut m = Mine { data: 0xedbi64 };

//     let a = m.as_ref();

//     println!("{a:?}")
// }

// use std::{fs::File, io::Write};

// fn main() {
//     let mut f = File::options().read(true).open("mine.txt").unwrap();
//     println!("{f:?}");
// // }

// use std::{fs, path::Path};

// fn fail() {
//     todo!("FIASCO!!!")
// }

// #[derive(Debug, Clone, Copy)]
// struct Mine;

// impl AsRef<i32> for Mine {
//     fn as_ref(&self) -> &i32 {
//         &32
//     }
// }

// fn taker(i: impl AsRef<i32>) {
//     println!("{}", i.as_ref())
// }

// fn main()   {
//     let m = Mine;
//     taker(m);

//     let a = String::from("ekefdajf");

//     let p = Path::new(&a);
// }

// use std::{
//     fs::File,
//     io::{BufRead, BufReader},
//     ops::Deref,
// };

#[derive(Debug)]
struct Mine;

// impl Deref for Mine {
//     type Target = i32;
//     fn deref(&self) -> &Self::Target {
//         &-32
//     }
// }

// fn main() {
//     let a = Mine;

//     println!("{:#?}", a.pow(2));
// }

#[cfg(target_os = "windows")]
fn one_only(i: &i32) {
    if i != &1 {
        compile_error!("function only accepts 1 as argument")
    }
}

fn main() {
    for i in [1, 2, 3, 4, 5].chunks(2) {
        println!("{i:?}")
    }

    macro_rules! give_me_foo_or_bar {
        (foo) => {};
        (bar) => {};
        ($x:ident) => {
            compile_error!("This macro only accepts `foo` or `bar`");
        };
    }

    give_me_foo_or_bar!(foo);
}
