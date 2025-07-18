fn vec_loop(input: &[i32]) -> Vec<i32> {
    // 값이 없는 빈 벡터 생성
    // 값을 추가할 시 mut 명시
    let mut output = Vec::new();

    for element in input {
        // mutable vec에는 [] 대신 push로 접근
        output.push(element * 2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    input
        .iter()
        .map(|element| {
            element * 2
        })
        .collect()
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
