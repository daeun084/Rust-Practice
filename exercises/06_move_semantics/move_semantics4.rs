fn main() {
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);

        // 동일한 ref에 대해 두 가지 변수로 동시에 접근 불가
        // 차례대로 변수 선언 후 접근해야 함
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
