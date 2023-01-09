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
