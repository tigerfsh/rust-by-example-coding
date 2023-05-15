use std::fmt::{Display, Debug};

struct A;

struct Single(A);

struct SingleGen<T>(T);

// functions
fn reg_fn(_s: Single) {}

fn gen_spec_t(_s: SingleGen<A>) {}

fn gen_spec_i32(_s: SingleGen<i32>) {}

fn generic<T>(_s: SingleGen<T>) {
    println!("this is function generic.");
}

// implementation
struct S;
struct GenericVal<T>(T);

impl GenericVal<f32> {}

impl GenericVal<S> {}

impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

// traits 
struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {
        
    }
}

fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct SS<T: Display>(T);

// bounds
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {self.length * self.height}
}

#[derive(Debug)]
struct Rectangle {length: f64, height: f64}

#[allow(dead_code)]
struct Triangle {length: f64, height: f64}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    // Generics
    let _s = Single(A);

    let _char = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('B');

    // functions
    reg_fn(Single(A));
    gen_spec_t(SingleGen(A));
    gen_spec_i32(SingleGen(6));
    generic(SingleGen('a'));
    generic::<char>(SingleGen('A'));
    generic(SingleGen("hello"));

    // implementation
    let x = Val {val: 3.0};
    let y = GenVal { gen_val: 3i32};
    let z = GenVal{gen_val: 6f32};

    println!("{}, {}, {}", x.value(), y.value(), z.value());

    // traits
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    
    // bounds
    let rectangle = Rectangle {length: 3.0, height: 4.0};
    let _triangle = Triangle {length: 3.0, height: 4.0};

    print_debug(&rectangle);

    println!("Area: {}", area(&rectangle));
    
}

