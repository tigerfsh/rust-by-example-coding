struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked ...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep {
            naked: false,
            name: name,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "babbbbb?"
        } else {
            "baaaaaaa!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly ... {}", self.name, self.noise());
    }
}

// 16.1
/*

    Comparison traits: Eq, PartialEq, Ord, PartialOrd.
    Clone, to create T from &T via a copy.
    Copy, to give a type 'copy semantics' instead of 'move semantics'.
    Hash, to compute a hash from &T.
    Default, to create an empty instance of a data type.
    Debug, to format a value using the {:?} formatter.

*/

#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

#[derive(Debug, PartialEq)]
struct Seconds(i32);

fn main() {
    // let mut dolly = Sheep::new("Dolly");
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();

    // 16.1
    let _one_second = Seconds(1);
    println!("One second looks like: {:?}", _one_second);

    let _this_is_true = _one_second == _one_second;

    let foot = Inches(12);
    println!("One foot equals: {:?}", foot);

    let meter = Centimeters(100.0);
    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("one foot is {} than another one.", cmp);

    // Default trait
    // https://doc.rust-lang.org/std/default/trait.Default.html

    #[derive(Default)]
    struct SomeOptions {
        foo: i32,
        bar: f32,
    }

    let _options: SomeOptions = Default::default();

    // Hash trait
    // https://doc.rust-lang.org/std/hash/trait.Hash.html
    #[derive(Hash)]
    struct Rustacean {
        name: String,
        country: String,
    }

    use std::hash::{Hash, Hasher};
    struct Person {
        id: u32,
        name: String,
        phone: u64,
    }

    impl Hash for Person {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
            self.phone.hash(state);
        }
    }

    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    let p = Person {
        id: 100,
        name: String::from("xiaoming"),
        phone: 16619921943,
    };
    p.hash(&mut hasher);
    println!("Hash is {:?}", hasher.finish());

    // Eq, PartialEq
    // pub trait Eq: PartialEq<Self> {},  Eq implies PartialEq, and has no extra methods.
    // Note that the derive strategy requires all fields are Eq, which isn’t always desired.
    #[derive(Clone, Copy)]
    enum BookFormat {
        PaperBack,
        Hardback,
        Ebook,
    }

    struct Book {
        isbn: i32,
        format: BookFormat,
    }

    impl PartialEq for Book {
        fn eq(&self, other: &Self) -> bool {
            self.isbn == other.isbn
        }
    }

    impl Ord for Book {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.isbn.cmp(&other.isbn)
        }
    }

    impl PartialOrd for Book {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Eq for Book {}
    let book1 = Book {
        isbn: 100,
        format: BookFormat::PaperBack,
    };

    let book2 = Book {
        isbn: 200,
        format: BookFormat::Ebook,
    };

    println!("book1 == book2: {:?}", book1 == book2);

    // PartialOrd, Ord
    println!("book1 < book2: {}", book1 < book2);

    // Clone vs Copy
    // https://doc.rust-lang.org/core/marker/trait.Copy.html

    // 不使用#[derive(Copy, Clone)]，手动实现Copy, Clone

    impl Copy for Book {}

    impl Clone for Book {
        fn clone(&self) -> Self {
            *self
        }
    }

    struct MyOtherObject {
        test: u8,
    }

    impl Copy for MyOtherObject {}

    impl Clone for MyOtherObject {
        fn clone(&self) -> Self {
            *self
        }
    }

    // 16.2
    let my_fruit = random_fruit(1.0);
    println!("fruit: {}", my_fruit.color());

    // 16.3 operator overloading
    println!("Foo + Bar = {:?}", Foo + Bar);

    // 16.4
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // drop(_a);
    println!("end of the main func");

    // 16.5 Iterator 
    let mut sequence = 0..3;
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    for i in 0..3 {
        println!("> {}", i);
    }


    println!("the first four terms: ");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    println!("skip four terms: ");
    for i in fibonacci().skip(4).take(4) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];
    println!("array: {:?}", &array);
    for i in array.iter() {
        println!("> {}", i);
    }
}
// 16.5 
struct Fibonacci {
    curr: u32, 
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

// 16.4
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

// 16.3
use std::ops::Add;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

impl Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar> was called");
        FooBar
    }
}

impl Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called.");
        BarFoo
    }
}

// 16.2
struct Apple {}
struct Orange {}

trait Fruit {
    fn color(&self) -> &'static str;
}

impl Fruit for Apple {
    fn color(&self) -> &'static str {
        "the color of apple is red."
    }
}

impl Fruit for Orange {
    fn color(&self) -> &'static str {
        "the color of orange is yellow."
    }
}

fn random_fruit(random_num: f64) -> Box<dyn Fruit> {
    if random_num < 0.5 {
        Box::new(Apple {})
    } else {
        Box::new(Orange {})
    }
}

// supertrait
trait MySuperTrait1 {
    fn my_super_method1(&self);
}

trait MySuperTrait2 {
    fn my_super_method2(&self);
}

trait MyTrait: MySuperTrait1 + MySuperTrait2 {
    fn my_method(&self);
}
