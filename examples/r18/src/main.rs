fn main() {
    // 18.1

    drink("water");
    // drink("lemonade");

    drink_v2("lemonade");

    // 18.3
    let res = next_birthday(None);
    println!("res: {:?}", res);

    // 18.3.3
    // Combinators: map vs and_then
    let _a = my_func().map(|a| Some(a));
    let _b = Some(100).map(|a| Some(a));
    let _c = Some(100).and_then(|a| Some(a));

    // 18.3.4
    // or vs or_else, get_or_insert vs get_or_insert_with

    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("{:?}", first_available_fruit);

    let apple = Some(Fruit::Apple);
    let no_fruit: Option<Fruit> = None;

    let get_kiwi_as_fallback = || {
        println!("Prividing kiwi as fallback");
        Some(Fruit::Kiwi)
    };

    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first avaiable fruit: {:?}", first_available_fruit);

    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);
    println!("my_fruit: {:?}", my_fruit);

    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback_v2 = || Fruit::Lemon;

    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback_v2);
    println!("first_available_fruit: {:?}", first_available_fruit);

    let twenty = multiply("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);
}

// 18.4.2  
// alias for Result, test in 18.4.1 

// 18.4.1
use std::num::ParseIntError;

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    match first_number_str.parse::<i32>() {
        Ok(first_number) => match second_number_str.parse::<i32>() {
            Ok(second_number) => Ok(first_number * second_number),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

fn multiply_v2(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_number| {
        second_number_str
            .parse::<i32>()
            .map(|second_number| first_number * second_number)
    })
}

fn multiply_v3(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
    first_number_str.parse::<i32>().and_then(|first_num| {
        second_number_str
            .parse::<i32>()
            .map(|second_num| first_num * second_num)
    })
}
fn print(result: AliasedResult<i32>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
// 18.3.4
#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon,
}
fn my_func() -> Option<i32> {
    Some(100)
}

// 18.3.3
#[derive(Debug)]
enum MyFood {
    CordonBleu,
    Steak,
    Sushi,
}

fn have_ingredients(food: MyFood) -> Option<MyFood> {
    match food {
        MyFood::Sushi => None,
        _ => Some(food),
    }
}

fn have_recipe(food: MyFood) -> Option<MyFood> {
    match food {
        MyFood::CordonBleu => None,
        _ => Some(food),
    }
}

fn cookable_v1(food: MyFood) -> Option<MyFood> {
    match have_recipe(food) {
        None => None,
        Some(food) => have_ingredients(food),
    }
}

fn cookable_v2(food: MyFood) -> Option<MyFood> {
    have_recipe(food).and_then(have_ingredients)
}
// 18.3
fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink ? oh, well."),
    }
}

fn drink_v3(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaa");
    }

    println!("I love {}s!!!", inside);
}

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

// when using ?, return value must be Option or Result.
fn next_birthday_v2(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

// 18.3.2

#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}

#[derive(Debug)]
struct Peeled(Food);

#[derive(Debug)]
struct Chopped(Food);

#[derive(Debug)]
struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)| Chopped(f))
        .map(|Chopped(f)| Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("Mmm. I love {:?}", food),
        None => println!("Oh, no! It wasn't edible."),
    }
}

// 18.2
// rustc  lemonade.rs -C panic=abort

#[cfg(panic = "unwind")]
fn ah() {
    println!("Spit it out!!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("This is not your party. Run!!!!");
}

fn drink_v2(beverage: &str) {
    if beverage == "lemonade" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}

// 18.1
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" {
        panic!("AAAaaaaa!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}
