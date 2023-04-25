#![allow(dead_code)]

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// creates a type alias 
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// using Self alias 
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        // self 是指枚举成员
        // Self 是指枚举类型
        match self {
            Self::Add => x+y,
            Self::Subtract => x-y,
        }
    }
}
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("pressed {}", c),
        WebEvent::Paste(s) => println!("pasted \"{} \".", s),
        WebEvent::Click { x, y } => println!("clicked at x={}, y={}", x, y),
    }
}

// ***use***
enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

// ***C-like***
enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    // Green = 0x00ff00,
    // Blue = 0x0000ff,
    Green,
    Blue,
}

// Testcase: linked-list 
// List here is Node
enum List {
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list 
    Nil,
}

impl List {
    fn new() -> List {
        Self::Nil
    }

    fn prepend(self, elem: u32) -> List {
        Self::Cons(elem, Box::new(self))
    }

    // after Rust 2018 you can use self here and tail (with no ref) below as well,
    // rust will infer &s and ref tail. 
    // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
    fn len(&self) -> u32 {
        match self {
            Self::Cons(_, tail) => 1 + tail.len(),
            Self::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match self {
            Self::Cons(head, tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Self::Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let x = Operations::Add;

    let res = x.run(10, 20);
    println!("res: {}", res);

    // ***use***
    use crate::Status::{Poor, Rich};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    // ***C-like***
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    // ***linked-list***
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    
    //临时写了一道编程题
    let mut nums: Vec<i32> = vec![1, 1, 2, 3];
    let new_length = remove_duplicates(&mut nums);
    println!("new length: {}", new_length);


}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = 0;

    let n = nums.len();

    while i < n && j < n {
        if nums.get(i).unwrap() < nums.get(j).unwrap() {
            let j_val = *nums.get(j).unwrap();
            // val: value of i+1
            if let Some(val) = nums.get_mut(i+1) {
                *val = j_val;
                i += 1;
            }

        }
        j += 1;
    }

    (i+1) as i32
}