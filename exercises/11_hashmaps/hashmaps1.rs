use std::{collections::HashMap, hash::Hash};

fn fruit_basket() -> HashMap<String, u32> {
    // hash map 선언부
    let mut basket = HashMap::new();
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 1);
    
    basket
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        // key의 개수
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        // value들의 전체 개수
        assert!(basket.values().sum::<u32>() >= 5);
    }
}