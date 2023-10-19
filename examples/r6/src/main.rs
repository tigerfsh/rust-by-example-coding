use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
use std::str::FromStr;

// From and Into
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// TryFrom and TryInto
#[derive(Debug, PartialEq)]
struct EventNumber(i32);

impl TryFrom<i32> for EventNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EventNumber(value))
        } else {
            Err(())
        }
    }
}

// To and from Strings
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// FromStr
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        let x_fromstr = x.parse::<i32>().map_err(|_| ParsePointError)?;
        let y_fromstr = y.parse::<i32>().map_err(|_| ParsePointError)?;
        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

fn transformer(a: impl Into<Number>) -> Number{
    a.into()
}

fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);

    // From trait
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // Into trait
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    let my_num = transformer(int);
    println!("Hi, my num is {:?}", my_num);

    // TryFrom and TryInto

    // TryFrom
    assert_eq!(EventNumber::try_from(8), Ok(EventNumber(8)));
    assert_eq!(EventNumber::try_from(5), Err(()));

    // TryInto
    let result: Result<EventNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EventNumber(8)));
    let result: Result<EventNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // To and from Strings
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parsing a String
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum);

    // FromStr
    let expected = Ok(Point { x: 1, y: 2 });
    assert_eq!(Point::from_str("(1,2)"), expected);
    assert_eq!("(1,2)".parse(), expected);
    assert_eq!("(1,2)".parse::<Point>(), expected);

    // Option: map vs map_or vs map_or_else
    // map
    let maybe_some_string = Some(String::from("Hello, world!"));
    let maybe_some_len = maybe_some_string.map(|s| s.len());
    assert_eq!(maybe_some_len, Some(13));

    // map_or
    let x = Some("foo");
    assert_eq!(x.map_or(42, |v| v.len()), 3);
    let x: Option<&str> = None;
    assert_eq!(x.map_or(42, |v| v.len()), 42);

    // map_or_else
    let k = 21;
    let x = Some("foo");
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);

    let x: Option<&str> = None;
    assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);

    // Option: ok_or vs ok_or_else
    // Option<T> => Result<T,E>
    // Some(v) -> Ok(v)
    // None -> Err(err)
    let x = Some("foo");
    assert_eq!(x.ok_or(0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or(0), Err(0));

    // ok_or_else
    let x = Some("foo");
    assert_eq!(x.ok_or_else(|| 0), Ok("foo"));

    let x: Option<&str> = None;
    assert_eq!(x.ok_or_else(|| 0), Err(0));

    // Result: map_err
    let x: Result<u32, u32> = Ok(2);
    assert_eq!(x.map_err(stringify), Ok(2));

    let x: Result<u32, u32> = Err(13);
    assert_eq!(x.map_err(stringify), Err("error code: 13".to_string()));

    // unwrap vs unwrap_or vs unwrap_or_default vs unwrap_or_else
    // 如果调用者是Some，则返回其中的值，否则执行后续的逻辑
    let x = Some("air");
    assert_eq!(x.unwrap(), "air");

    let x: Option<&str> = None;
    // will panic
    // assert_eq!(x.unwrap(), "air")

    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    assert_eq!(Some(12).unwrap_or_default(), 12);
    let x: Option<i32> = None;
    assert_eq!(x.unwrap_or_default(), 0i32);

    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
}

fn stringify(x: u32) -> String {
    format!("error code: {x}")
}
