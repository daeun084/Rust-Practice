fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        assert_eq!(power_of_2(2), 4);
        assert_eq!(power_of_2(3), 8);
        assert_eq!(power_of_2(1), 2);
        assert_eq!(power_of_2(5), 32);
    }
}
