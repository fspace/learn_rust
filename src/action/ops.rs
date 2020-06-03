use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);
// 没有使用默认类型参数
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

pub fn action_op_overwrite() {
    assert_eq!(
        Point { x: 1, y: 1 } + Point { x: 2, y: 2 },
        Point { x: 3, y: 3 }
    );

    let mi = Millimeters(1);
    let m = Meters(1);
    let r = mi + m;
    println!("r: {:?}", r);
}
