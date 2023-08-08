// RAII

use std::fmt::Debug;

// raii.rs
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped!");
    }
}

struct Person(i32);

impl Drop for Person {
    fn drop(&mut self) {
        println!("this person's num is {}", self.0);
    }
}
fn main() {
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

    // so whenever an object goes out of scope, its destructor is called and its owned resources are freed.
    let _x = ToDrop;
    println!("Made a ToDrop x !");

    let p1: Person = Person(100);
    println!("num of p1: {}", p1.0);

    let p2 = Person(200);
    println!("num of p2: {}", p2.0);

    // FILO
    let a = vec![("apple", "orange")];
    let s: String = "apple".into();

    // ownership-and-moves
    let x = 5u32;
    // *Copy* `x` into `y` - no resources are moved
    let y = x;
    println!("{}, {}", x, y);

    let a = "hello";
    // copy a to b 
    let b = a;
    println!("{}, {}", a, b);

    let a = Box::new(5i32);
    println!("a contains {}", a);

    let b = a;
    // println!("a contains {}", a);

    destroy_box(b);

    // mutability
    let immutable_box = Box::new(5u32);
    println!("immutable box contains {}", immutable_box);

    let mut mutable_box = immutable_box;
    *mutable_box = 4;
    println!("mutable_box now contains {}", mutable_box);

    // partial-moves
    let man = Man {name: String::from("Alice"), age: Box::new(20)};
    let Man {name, ref age} = man;
    // `name` is moved out of person, but `age` is referenced
    println!("man's age: {}", age);
    println!("man's name: {name}");

    // println!("{:?}", man);
    println!("man's age: {}", man.age);
    
    // 15.3 
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // eat_box_i32(boxed_i32);
        borrow_i32(_ref_to_i32);
    }
    eat_box_i32(boxed_i32);

    // 15.3.1 
    let immutabook = Book {
        author: "xiao ming",
        title: "Food",
        year: 1979,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);
    borrow_book(&mutabook);

    new_edition(&mut mutabook);
    // new_edition(&mut immutabook); // not work 

    // 15.3.2
    let mut point = Point {x: 0, y: 0, z: 0};

    let borrowed_point = &point;
    let another_borrwo = &point;

    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrwo.y, point.z);
    
    // let mutable_borrow = &mut point;

    println!("Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrwo.y, point.z);
    
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // no other refs (point) can be used here.

    println!("Point has coordinates: ({}, {}, {})", 
    mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    // The mutable reference is no longer used for the rest of the code so it
    // is possible to reborrow

    let new_borrowd_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
    new_borrowd_point.x, new_borrowd_point.y, new_borrowd_point.z);

    // 15.3.3

    let c = 'Q';

    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 == ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point{x: 0, y: 0, z: 0};
    let _copy_of_x = {
        let Point { x: ref ref_to_x, y: _, z: _ } = point;
        *ref_to_x

    };

    // A mutable copy of `point`
    let mut mutable_point = point;
    {
        let Point { x: _, y: ref mut mut_ref_to_y, z: _ } = mutable_point;
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {}, {})", point.x, point.y, point.z);
    println!("mutable point is ({}, {}, {})",
    mutable_point.x, mutable_point.y, mutable_point.z);

    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);

    // 15.4 
    // 15.4.3 
    let mut owner = Owner(18);
    owner.add_one();
    owner.print();

    // 15.4.5 
    let b: BorrowedX = Default::default();
    println!("b is {:?}", b);

    // 15.4.6 
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print_bounds(ref_x);

    // 15.4.7
    let first = 2;
    {
        let second = 3;
        println!("{}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));

    }

    // 15.4.8
    // static 
    // Reference lifetime 
    // As a reference lifetime 'static indicates that the data pointed to by the reference lives for the entire lifetime of the running program. It can still be coerced to a shorter lifetime.
    {
        let static_string = "I am in read-only memory";
        println!("static_string: {}", static_string);
    }

    {
        let lifetime_num = 9;
        let coerced_static = coerce_static(&lifetime_num);
        println!("coerced_static: {}", coerced_static);

    }

    println!("NUM: {}", NUM);

    // Trait bound 
    /*
    As a trait bound, it means the type does not contain any non-static references. Eg. the receiver can hold on to the type for as long as they want and it will never become invalid until they drop it.

    It's important to understand this means that any owned data always passes a 'static lifetime bound, but a reference to that owned data generally does not:

    */

    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    // print_it(&i); // error 

    print_it(&NUM);

}
static NUM: i32 = 18;

// 15.4.1 
// fn failed_borrow<'a>() {
//     let _x = 12;
//     let y: &'a i32 = &_x;
//     // Attempting to use the lifetime `'a` as an explicit type annotation 
//     // inside the function will fail because the lifetime of `&_x` is shorter
//     // than that of `y`. A short lifetime cannot be coerced into a longer one.

// }
fn longer_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        return a
    }
    b 
}

// 15.4.2 
// One input reference with lifetime `'a` which must live
// at least as long as the function.
fn print_one<'a>(x: &'a i32) {
    println!("print_one: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it
// would be fine for both to have the same lifetime `'a`, but
// in more complex cases, different lifetimes may be required.
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {x}

// 15.4.3 
struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a >(&'a self) {
        println!("print: {}", self.0);
    }
}

// 15.4.4 
#[derive(Debug)]
struct BorrowedItem<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

// 15.4.5
#[derive(Debug)]
struct BorrowedX<'a> {
    x: &'a i32,
}

impl<'a> Default for BorrowedX<'a> {
    fn default() -> Self {
        Self { x: &10 }
    }
}

// 15.4.6 
/*
    T: 'a: All references in T must outlive lifetime 'a.
    T: Trait + 'a: Type T must implement trait Trait and all references in T must outlive 'a.
    说明：`all references in T`，这个T可能是结构体，所以出现了这样的说法。
*/
#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);

fn print_bounds<T>(t: T)
where
T: Debug 
{
    println!("print_bounds: t is {:?}", t);
}

fn print_ref<'a, T>(t: &'a T) 
where 
T: Debug + 'a 
{
    println!("print_ref: t is {:?}", t);
}

fn print_ref_v2<T>(t: &T) 
where 
T: Debug
{
    println!("print_ref_v2: t is {:?}", t);
}

fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first 
}

// 15.4.8 
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn print_it(input: impl Debug + 'static) {
    // impl: 'static, this means the type does not contain any non-static references.
    println!("static value passed in is: {:?}", input);
}

// 15.3 
// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

// 15.3.1 
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} -{} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I immutably borrowed {} -{} edition", book.title, book.year);
}

// 15.3.2 and 15.3.3 
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Man {
    name: String,
    age: Box<u8>,
}

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}