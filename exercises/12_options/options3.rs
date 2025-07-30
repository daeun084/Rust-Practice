#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // 포인트 이동시 소유권 문제에 의해 아래 출력문에서 참조 불가
    if let Some(ref p) = optional_point {
        // 값 이동 없이 참조만 사용
        println!("Co-ordinates are {},{}", p.x, p.y);
    } else {
        panic!("No match!");
    }

    println!("{optional_point:?}");
}
