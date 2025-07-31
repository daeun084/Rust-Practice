fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(!is_even(3), "Expected 3 to be even");
        assert!(is_even(8), "Expected 8 to be even");
    }
}
