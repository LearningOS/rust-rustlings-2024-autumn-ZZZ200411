// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point { //一个点x y
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    //let y = Some(Point{x:100 , y:200});
    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _None => panic!("no match!"),
    }
    //y; // Fix without deleting this line.
}
