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
    let y = x;
    println!("{}, {}", x, y);

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
    println!("man's age: {}", age);
    
}

#[derive(Debug)]
struct Man {
    name: String,
    age: Box<u8>,
}

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}