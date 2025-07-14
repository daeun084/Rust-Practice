fn main() {
    let number = "T-H-R-E-E"; 
    println!("Spell a number: {}", number);

    // variable shadowing을 통해 동일한 이름의 변수를 타입을 바꿔 재사용
    let number: i32 = 3;
    println!("Number plus two is: {}", number + 2);
}