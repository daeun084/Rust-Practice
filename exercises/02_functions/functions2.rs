// fn argument는 : 뒤에 타입 명시 필요
fn call_me(num: i32){
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
