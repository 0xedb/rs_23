use std::borrow::Cow;

#[derive(Debug)]
struct Something;

#[derive(Debug)]
struct Nothing;

impl From<Nothing> for Something {
    fn from(_: Nothing) -> Self {
        Something
    }
}

impl From<Something> for &'static str {
    fn from(_: Something) -> &'static str {
        "something really!!!"
    }
}

fn p<'a, T>(t: T)
where
    T: Into<&'static str> + std::fmt::Debug,
{
    println!("{}", t.into())
}

fn main() {
    let something = Something;
    let nothing = Nothing;

    let another: Something = Something::from(nothing);

    println!("{:?}", another);

    let st = String::from("Jokes On You!");

    let ss = st.to_owned();

    p(something);

    

    let j = "adfd".as_mut();
}
