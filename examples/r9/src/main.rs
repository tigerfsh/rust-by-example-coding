struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // associated functions (not method)
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

// call associated function, for example:
// let res = Point::origin();

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        // This is a method
        // `&self` is sugar for `self: &Self`, where `Self` is the type of the
        // caller object. In this case `Self` = `Rectangle`
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // &mut self -> `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

fn main() {
    // Associated functions & Methods
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("{}", rectangle.perimeter());
    println!("{}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    // Closures
    // the ability to capture the outer environment variables.
    let out_var = 43;
    let closure_annotated = |i: i32| -> i32 { i + out_var };
    let closure_inferred = |i| i + out_var;

    print!("{}", closure_annotated(1));
    println!("{}", closure_inferred(1));

    let one = || 1;
    println!("{}", one());

    // capturing
    use std::mem;

    let color = String::from("green");

    let print = || println!("color: {}", color);

    print();

    let _reborrow = &color;
    print();

    let _color_moved = color;

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    inc();

    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // move key
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("{}", haystack.len());

    // As input parameters
    // 1. Fn -> reference(&T)
    // 2. FnMut -> mutable reference(&mut T)
    // 3. FnOnce -> value(T)
    // 如果类型注解为2，则1和2都是可行的，编译器会选择最小范围适用

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // requires Fn
        println!("I said {}", greeting);

        // requires FnMut
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`
        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubld: {}", apply_to_3(double));

    let x = 7;
    // Capture `x` into an anonymous type and implement
    // `Fn` for it. Store it in `print`.
    let print = || println!("{}", x);
    apply_v2(print);

    // Input functions
    let closure = || println!("I am a closure");
    call_me(closure);
    call_me(function);

    // As output parameters
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();

    // Iterator::any
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("{}", vec1.iter().any(|&x| x == 2));
    println!("{}", vec2.into_iter().any(|x| x == 2));

    println!("{}", vec1.len());
    println!("{}", vec1[0]);

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("{}", array1.iter().any(|&x| x == 2));
    println!("{}", array2.into_iter().any(|x| x == 2));

    // Searching through iterators
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("{:?}", iter.find(|&&x| x == 2));
    println!("{:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("{:?}", array1.iter().find(|&&x| x == 2));
    println!("{:?}", array2.into_iter().find(|&x| x == 2));

    let vec = vec![1, 9, 3, 3, 13, 2];
    let index_of_first_even_number = vec.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    let index_of_first_negative_number = vec.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);

    // higher order functions
    let upper = 1000;
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }

    println!("acc: {}", acc);

    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n_squared| n_squared < upper)
        .filter(|&n_squared| is_odd(n_squared))
        .sum();
    println!("{}", sum_of_squared_odd_numbers);

    for n in (0..).take_while(|&x| x < 11) {
        println!("{}", n);
    }

    // diverging functions 分叉函数，（流程控制）
    // return nothing
    let _a: () = some_fn(); // return unit
    println!("This function returns and you can see this line.");

    // panic!
    // loop {}
    // exit()
    // ...
    // let _a = std::process::exit(1);
    // let _b = loop {
    // };
    // let _c = panic!("this is a panic");
}

fn some_fn() {
    ()
}
fn foo() -> ! {
    panic!("This call never returns.");
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}
fn call_me<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn function() {
    println!("I'm a function!");
}

fn apply<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn apply_to_3<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn apply_v2<F>(f: F)
where
    F: Fn(),
{
    f();
}

// https://huonw.github.io/blog/2015/05/finding-closure-in-rust/
