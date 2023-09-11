use std::rc::Rc;

use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;
use std::str;

use std::time::Duration;
use std::sync::Arc;
use std::thread;

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

    // 19.3 part-2
    let byte_escape = "I'm writing \x52\x75\x73\x74";
    println!("What are you dong\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"apple\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "AAAAA
                            BBBBBB.
                            The linebreakand indentation here ->\
                            <- can be escaped too!";
    println!("{}", long_string);

    // raw string
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    // Note that this is not actually a `&str`
    let bytestring: &[u8; 21] = b"this is a byte string";

    // Byte arrays don't have the `Display` trait, so printing them is a bit limited
    println!("A byte string: {:?}", bytestring);

    // Byte strings can have byte escapes...
    let escaped = b"\x52\x75\x73\x74 as bytes";
    // ...but no unicode escapes
    // let escaped = b"\u{211D} is not allowed";
    println!("Some escaped bytes: {:?}", escaped);

    // Raw byte strings work just like raw strings
    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    // Converting a byte array to `str` can fail
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let _quotes = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    // Byte strings don't have to be UTF-8
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb"; // "ようこそ" in SHIFT-JIS

    // But then they can't always be converted to `str`
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };

    // 19.7 HashMap
    let mut contacts = HashMap::new();
    contacts.insert("A", "100");
    contacts.insert("B", "200");
    contacts.insert("C", "300");

    match contacts.get(&"100") {
        Some(&number) => println!("Calling 100: {}", call(number)),
        _ => println!("not 100."),
    }

    contacts.remove(&"A");

    for (&k, &v) in contacts.iter() {
        println!("k: {:?}, v: {:?}", k, v);
    }
    // 19.7.2
    let mut a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // assert!(b.insert(4), "Value 4 is already in set B");
    println!("A: {:?}", a);
    println!("B: {:?}", b);

    let union_collection = a.union(&b).collect::<Vec<&i32>>();
    println!("Union: {:?}", union_collection);

    let difference_collection = a.difference(&b).collect::<Vec<&i32>>();
    println!("Difference: {:?}", difference_collection);

    // Print [2, 3, 4] in arbitrary order.
    println!(
        "Intersection: {:?}",
        a.intersection(&b).collect::<Vec<&i32>>()
    );

    // Print [1, 5]
    println!(
        "Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>()
    );

    // 19.8 Rc
    let rc_example = "Rc example".to_string();
    {
        println!("rc_a is created.");

        let rc_a: Rc<String> = Rc::new(rc_example);
        println!("Ref count of rc_a: {}", Rc::strong_count(&rc_a));
        {
            println!("rc_a is cloned to rc_b");

            let rc_b: Rc<String> = Rc::clone(&rc_a);

            println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));
            println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

            // Two `Rc`s are equal if their inner values are equal
            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            // We can use methods of a value directly
            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }
        println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

        println!("--- rc_a is dropped out of scope ---");
    }

    // 19.9 Arc(Atomically Reference Counted)
    let apple = Arc::new("the same apple");
    for _ in 0..10 {
        let apple = Arc::clone(&apple);
        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }

    thread::sleep(Duration::from_secs(5));

}

// 19.7.2 HashSet
// in actuality, just a wrapper around HashMap<T, ()>).

// 19.7.1
/*
Any type that implements the Eq and Hash traits can be a key in HashMap.
Note that f32 and f64 do not implement Hash, likely because floating-point precision errors would make using them as hashmap keys horribly error-prone.

*/
// 19.7 HashMap
fn call(number: &str) -> &str {
    match number {
        "100" => "100",
        "200" => "200",
        _ => "ignore ...",
    }
}
// 19.6 panic! (省略)
// 19.5.1 ? (省略)
// 19.5 Result (省略)
// 19.4 Option (省略)

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
