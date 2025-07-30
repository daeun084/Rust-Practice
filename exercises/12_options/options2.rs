fn main() {
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);
        
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;
        while let Some(integer) = optional_integers.pop() {
            // ignore None value
            if let Some(value) = integer {
                assert_eq!(value, cursor);
                cursor -= 1;
            }
        }

        assert_eq!(cursor, 0);
    }
}
