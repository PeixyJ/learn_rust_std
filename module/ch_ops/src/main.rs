use std::ops::{Add, Sub};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: u32,
    y: u32,
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

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[test]
fn fn1() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 5, y: 15 };
    let p3 = p1 + p2;
    assert_eq!(p3, Point { x: 15, y: 35 });
}
