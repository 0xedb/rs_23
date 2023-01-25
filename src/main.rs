use serde::{Deserialize, Serialize};

/// Pointy
#[derive(Debug, Serialize, Deserialize)]
struct Point(i32, i32, i32);

fn main() {
  
  macro_rules! quest {
      ($a: ident, $b:ident) => {
        println!("({}::{})", $a, $b)
      };
  }

  let a = 20;
  let b = 340;

  quest!(a, b);
}
