fn main() {
    // 0값의 요소를 100개 가지는 배열 초기화
    // let a: [i32;100] = [0; 100]; -> i32타입이 100개 있음을 명시
    let a = [0; 100];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
