enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    fn upper(input: String) -> String {
        input.to_uppercase().to_string()
    }

    fn trim(input: String) -> String {
        input.trim().to_string()
    }

    fn append(input: String, size: usize) -> String {
        let mut result = input;
        for _i in 0..size {
            result = format!("{}bar", result);
        }
        result
    }

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> { 
        // 가변 배열 선언
        let mut results : Vec<String> = Vec::new();

        // into_iter() : 소유권을 mov하며 순환
        for (i, command) in input.into_iter() {
           let mod_result = match command {
                    Command::Uppercase => upper(i),
                    Command::Trim => trim(i),
                    Command::Append(size) => append(i, size),
                };
            results.push(mod_result);
        }
        results
    }
}

fn main() {
}

#[cfg(test)]
mod tests {
    
    // 모듈 시스템 인식
    use crate::my_module::transformer as transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
