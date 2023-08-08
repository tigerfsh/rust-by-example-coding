// RAII

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