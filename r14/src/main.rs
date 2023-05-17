use std::{fmt::{Debug, Display}, marker::PhantomData};

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

struct AA;
struct BB;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {}
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
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

// empty bounds
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

// trait 继承，冒号后面的是父trait，如果是多个trait则用加号相连
trait MyPrint: Debug {}

// usage of `impl trait`
fn ff(obj: impl Red) -> impl Blue {
    // let b = BlueJay;
    // b
    BlueJay
}

// multiple bounds
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

// where
trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

// new type idiom
struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}

// associated types 
// struct Container(i32, i32);

// trait Contains<A, B> {
//     fn contains(&self, _: &A, _: &B) -> bool;
//     fn first(&self) -> i32;
//     fn last(&self) -> i32;
// }

// impl Contains<i32, i32> for Container {
//     fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
//         (&self.0 == number_1) && (&self.1 == number_2) 
//     }

//     fn first(&self) -> i32 {
//         self.0 
//     }

//     fn last(&self) -> i32 {
//         self.1
//     }
// }

// fn difference<A, B, C>(container: &C) -> i32 
// where
// C: Contains<A, B>
// {
//     container.last() - container.first()
// }

struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

// phantom type parameters 
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {first: A, phantom: PhantomData<B>}


// unit clarification
// TODO

// RAII
fn create_box() {
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
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
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };
    let z = GenVal { gen_val: 6f32 };

    println!("{}, {}, {}", x.value(), y.value(), z.value());

    // traits
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    // 二者的所有权转移到内部，函数调用结束，这两个变量都不能再使用
    let aa = AA;
    let bb = BB;

    aa.double_drop(bb);

    // bounds
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);

    println!("Area: {}", area(&rectangle));

    // empty bounds
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A trukey is {}", red(&_turkey));

    // multiple bounds
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);

    compare_types(&array, &vec);

    // where 
    let vec = vec![1, 2, 3];
    vec.print_in_option();
    let array = [100, 200, 300];
    array.print_in_option();

    // new type idiom
    let age = Years(5);
    let age_days = age.to_days();

    println!("{}, {}", old_enough(&age), old_enough(&age_days.to_years()));

    let years = Years(42);
    let years_as_primitive_1 = years.0;
    println!("years_as_primitive_1: {}", years_as_primitive_1);

    let Years(val) = years;
    println!("val: {}", val);

    // associated types 
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!("{}", container.contains(&number_1, &number_2));

    println!("first: {}", container.first());
    println!("last: {}", container.last());
    println!("the diffdrence is {}", difference(&container));

    // phantom type parameters 
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct { first: 'Q', phantom: PhantomData };
    let _struct2: PhantomStruct<char, f64> = PhantomStruct { first: 'Q', phantom: PhantomData };

    // RAII
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested scope:
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);

        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }

    // `_box2` is destroyed here, and memory gets freed
}
