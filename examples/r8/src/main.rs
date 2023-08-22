#![allow(unreachable_code)]

use std::str::FromStr;
use std::{option, str::ParseBoolError};

fn main() {
    // if/else
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n = if n < 10 && n > -10 { 10 * n } else { n / 2 };

    println!("{} -> {}", n, big_n);

    //loop
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("ok, that's enough");

            break;
        }
    }

    //Nesting and labels
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never  be reached");
    }

    println!("Exited the outer loop");

    // returning from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            if counter == 10 {
                // put it after the break, and it will be returned by the loop expression.
                break counter * 2;
            }
        }
    };

    assert_eq!(result, 20);

    // while
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        n += 1;
    }

    // for loops
    println!("for and range");

    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
    println!("for and range end.");

    for n in 0..10 {
        println!("n: {}", n);
    }

    // for and iterators
    // iter
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("This is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);

    // into_iter
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("This is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names);

    // iter_mut
    let mut names = vec!["Bob", "Frank", "Ferris"];
    let name = names.get_mut(0).unwrap();
    *name = "good";
    println!("reset item at index 0: {:?}", names);

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "This is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);

    // match
    let number = 13;
    println!("Tell me about {}", number);

    match number {
        1 => print!("one"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    // Destructuring
    // tuples
    let triple = (3, -2, 4);

    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is 0, y: {}, z: {}", y, z),
        (1, ..) => println!("First is 1 and the rest doesn't matter."),
        (.., 2) => println!("Last is 2 and the rest doesn't matter"),
        (3, .., 4) => println!("First is 3, last is 4 , and the rest doesn't matter"),

        _ => println!("It doesn't matter what they are"),
    }

    // arrays/slices
    let array = [10, -2, 6];

    match array {
        [0, b, c] => println!("a=0, b={}, c={}", b, c),
        [1, _, c] => println!("a=1, c={}", c),
        [-1, b, ..] => println!("a=-1, b={}", b),
        [3, b, cc @ ..] => println!("cc is {:?}", cc),
        [a, middle @ .., c] => println!("middle is {:?}", middle),
    }

    // enums
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color {
        Color::Red => println!("The color is red"),
        Color::RGB(r, g, b) => println!("r: {}, g: {}, b: {}", r, g, b),
        _ => println!("the color doesn't matter"),
    }

    // pointers/ref
    let reference = &4;
    match reference {
        &val => println!("Got a value: {:?}", val),
    }

    match *reference {
        val => println!("Got a value: {:?}", val),
    }

    let _not_a_reference = 3;
    // the ref pattern
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_vlaue = 6;
    match value {
        ref r => println!("Got a ref to a value:{:?}", r),
    }

    match mut_vlaue {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    // structs
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (3, 2), y: 2 };
    match foo {
        Foo { x: (1, b), y } => println!("b={}, y={}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i={:?}", i),
        // Foo {y, ..} => println!("y = {}, we don't care about x", y),
        Foo { y: 100, x: value } => println!("value: {:?}", value),
        _ => (),
    }

    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }

    let number: u8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("doesn't matter"),
        // _ => println!("Less than zero"),
    }

    // binding
    match age() {
        0 => println!("age is zero"),
        n @ 1..=12 => println!("I am a child of age {:?}", n),
        n @ 13..=19 => println!("I am a teen of age {:?}", n),
        n => println!("I am an old person of age {:?}", n),
    }

    match some_number() {
        Some(n @ 43) => println!("The answer is {}", n),
        Some(n) => println!("{} ignored", n),
        _ => (),
    }

    // if let
    // if let 本质上就是match的一个缩略版
    let optional = Some(7);
    match optional {
        Some(i) => {
            println!("i: {}", i);
        }
        _ => (),
    }

    if let Some(i) = optional {
        println!("i is {}", i);
    }

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("xxx");
    } else {
        println!("yyy");
    }

    let a = Fpp::Bar;
    let b = Fpp::Baz;
    let c = Fpp::Qux(100);

    if let Fpp::Bar = a {
        println!("a is foobar");
    }

    if let Fpp::Bar = b {
        print!("b is foobar");
    }

    if let Fpp::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Fpp::Qux(value @ 100) = c {
        println!("c is {}", value);
    }

    // let-else
    // The scope of name bindings is the main thing that makes this different from match or if let-else expressions.

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // while let
    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("gt than 9, quit!");
                    optional = None;
                } else {
                    println!("i is {:?}. try again", i);
                    optional = Some(i + 1);
                }
            }
            _ => {
                break;
            }
        }
    }

    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("gt than 9, quit");
            optional = None;
        } else {
            println!("i is {}, try again", i);
            optional = Some(i + 1);
        }
    }
}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: {s}")
    };
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: {count_str}");
    };
    (count, item)
}
#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(43)
}

enum Fpp {
    Bar,
    Baz,
    Qux(u32),
}
