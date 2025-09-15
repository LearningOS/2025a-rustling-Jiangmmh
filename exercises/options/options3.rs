// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // 当你在 match 模式中使用 Some(Point { x, y }) 或类似的模式时，
        // Rust 会尝试将 Point 结构体中的 x 和 y 字段从 p 中移动出来，并绑定到新的局部变量 x 和 y
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
