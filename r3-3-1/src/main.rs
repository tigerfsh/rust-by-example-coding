#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// a unit struct
struct Unit;

// a tuple struct
struct Pair(i32, f32);

// a struct with two fields
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(r: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: tx, y: ty },
        bottom_right: Point { x: bx, y: by },
    } = r;
    (*bx - *tx) * (*ty - by)
}
fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point = Point { x: 10.3, y: 0.4 };

    println!("point coordinates: ({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point {
        x: left_edge,
        y: right_edge,
    } = point;
    let _rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: right_edge,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;
    let pair = Pair(1, 0.1);
    println!("pair contains {:?}, {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let r1 = Rectangle {
        top_left: Point { x: 1.0, y: 2.0 },
        bottom_right: Point { x: 2.0, y: 1.0 },
    };

    let area = rect_area(&r1);
    println!("area: {}", area);
}
