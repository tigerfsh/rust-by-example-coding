use std::mem;

fn main() {
    // 19.1

    let point = origin();
    let rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    };

    let boxed_rectangle = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // The output of functions can be boxed
    let boxed_point: Box<Point> = Box::new(origin());

    // Double indirection
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!(
        "Point occupies {} bytes on the stack",
        mem::size_of_val(&point)
    );
    println!(
        "Rectangle occupies {} bytes on the stack",
        mem::size_of_val(&rectangle)
    );

    // box size == pointer size
    println!(
        "Boxed point occupies {} bytes on the stack",
        mem::size_of_val(&boxed_point)
    );
    println!(
        "Boxed rectangle occupies {} bytes on the stack",
        mem::size_of_val(&boxed_rectangle)
    );
    println!(
        "Boxed box occupies {} bytes on the stack",
        mem::size_of_val(&box_in_a_box)
    );

    // Copy the data contained in `boxed_point` into `unboxed_point`
    let unboxed_point: Point = *boxed_point;
    println!(
        "Unboxed point occupies {} bytes on the stack",
        mem::size_of_val(&unboxed_point)
    );

    // 19.2
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("collected_iterator: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 4];
    println!("xs: {:?}", xs);

    xs.push(4);
    println!("xs: {:?}", xs);

    println!("length of xs: {:?}", xs.len());

    println!("Second element: {}", xs[1]);

    println!("Pop: {:?}", xs.pop());

    // println!("Fourth element: {}", xs[3]);

    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("index: {}, value: {}", i, x);
    }

    for x in xs.iter_mut() {
        *x = *x + 3;
    }

    println!("Updated vector: {:?}", xs);

    // xs can not be used after this.
    for x in xs.into_iter() {
        println!("> {}", x);
    }

    // 19.3 strings
    let a = "apple";

    let pangram: &'static str = "我 aaa bbb ccc ddd eee fff";
    for word in pangram.split_whitespace() {
        println!("> {}", word);
    }

    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    // duplication: 重复
    chars.dedup();

    println!("chars: {:?}", chars);

    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("string: {:?}", string);

    let chars_to_trim: &[char] = &[' ', ','];
    let trimed_str = string.trim_matches(chars_to_trim);
    println!("trimed_str: {:?}", trimed_str);

    let alice = String::from("I like dogs");
    let bob = alice.replace("dog", "cat");

    println!("Alice: {}", alice);
    println!("Bob: {}", bob);
}
// 19.3

// 19.2 Vector

// 19.1 Box

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}
