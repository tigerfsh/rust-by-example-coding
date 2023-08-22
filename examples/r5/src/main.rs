#![allow(overflowing_literals)]
fn main() {
    // Casting
    // using the as keyword
    let decimal = 65.1234_f32;

    // let integer: u8 = decimal;

    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);

    println!("-1 as a u8 is {}", (-1i32) as u8);

    println!("1000 mod 256 is {}", 1000 % 256);

    // Literals
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    let i = 1;
    let f = 1.0;

    println!("size of 'x' in bytes: {}", std::mem::size_of_val(&x));

    println!("size of i in bytes: {}", std::mem::size_of_val(&i));

    // interence
    let elem = 5u8;

    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    // Aliasing
    // aliases are *not* new types
    // The main use of aliases is to reduce boilerplate; for example the io::Result<T> type is an alias for the Result<T, io::Error> type.
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;
