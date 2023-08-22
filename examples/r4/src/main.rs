fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    let _unused_variable = 3u32;
    let noisy_unused_variable = 2u32;

    // ***Mutability***
    let _immutable_binding = 1;
    let mut mutble_binding = 1;

    println!("Before mutation: {}", mutble_binding);

    mutble_binding += 1;

    println!("After mutation: {}", mutble_binding);

    // Scope and Shadowing
    let a = 1;
    {
        println!("before being shadowed: {}", a);
        let a = 100;
        println!("after being shadowed: {}", a);
    }

    println!("outside inner block: {}", a);

    let a = 200;
    println!("shadowed in outer block: {}", a);

    // Declare first
    let b;
    {
        let c = 500;
        b = c * c;
    }
    println!("b: {}", b);

    // Freezing
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        // _mutable_integer = 50;
    }
    _mutable_integer = 500;
}
