struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let s = "Hello,".to_string();
    let x = "world!";
    let mut ints = vec![1, 2, 8, 9];
    ints.push(11);
    println!("{} {} {:?}", s, x, ints);
    let p = Point { x: 10, y: 20 };
    println!("point: {}x{}", p.x, p.y);
}
