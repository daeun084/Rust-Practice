// ;은 값을 반환하지 않고 종료한다는 의미
// -> {} 을 이용한 암시적 반환을 이용하려면, 반환하려는 값 뒤에 ;을 제거해야 함 
fn square(num: i32) -> i32 {
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}