fn main() {
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        // tuples.0, tuples.1 등으로 튜플의 각 요소에 접근
        let numbers = (1, 2, 3);
        let second = numbers.1;
        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
