#![allow(clippy::ptr_arg)]

fn get_char(data: &String) -> char {
    // String : 소유권(ownership)을 받음
    data.chars().last().unwrap()
}

fn string_uppercase(mut data: String) {
    // &String은 불변 참조 -> 소유권 받음 X
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);
    string_uppercase(data);
}
