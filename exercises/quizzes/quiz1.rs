fn calculate_price_of_apples(cnt: i8) -> i8 { 
    let cost = 2;

    if cnt <= 40 {
        cost * cnt
    } else {
        cnt
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
