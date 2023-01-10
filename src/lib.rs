/*!
just something to thing about here!!!!
*/


pub fn t() -> i32 {
    0xEDB
}

#[cfg(test)]
mod tests {
    #[test]
    fn if_it_works() {
        assert!(super::t() == 3803);
    }
}


/// just a function called a
/// # Example
/// ```
/// let res = rs_23::a("hello");
/// assert_eq!("hello", res);
/// ```
pub fn a(p: &str) -> &str {
    p
}
