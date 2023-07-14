use std::fmt::{self, Display, Formatter};

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(
            f,
            "{}: {:.3} {} {:.3} {}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let p = format!("{self:X}");
        let p = format!("0x{p:0>6}");
        write!(f, "RGB ({}, {}, {}) {}", self.red, self.green, self.blue, p)
    }
}

impl fmt::UpperHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for c in [self.red, self.green] {
            let val = c;
            fmt::UpperHex::fmt(&val, f)?;
        }
        let val = self.blue;
        fmt::UpperHex::fmt(&val, f)
    }
}

fn main() {
    for city in [
        City {
            name: "Dublin",
            lat: 53.123,
            lon: -6.259,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("{}", *city);
    }

    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ]
    .iter()
    {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        // println!("{:?}", *color);
        println!("{}", color);
    }

    for i in 0..10 {
        println!("{}", i);
    }

    let colors = ["red", "green", "blue"];
    for i in 0..colors.len() {
        println!("the color name is {}", colors[i]);
    }

    let countries = vec!["India", "China", "USA", "Russia"];
    for conuntry in countries.iter() {
        println!("the country name is {}", conuntry);
    }

    for (index, country) in countries.iter().enumerate() {
        println!("the country at index {} is {}", index, country);
    }

    for (index, country) in countries.iter().enumerate().step_by(2) {
        println!("the country at index {} is {} (step 2)", index, country);
    }
}
