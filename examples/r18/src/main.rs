

fn main() {
    // 18.1

    drink("water");
    // drink("lemonade");

    drink_v2("lemonade");
}
// 18.3 
fn give_adult(drink:Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck! Too sugary"),
        Some(inner) => println!("{}? How nice.", inner),
        None => println!("No drink ? oh, well."),
    }
}

fn drink_v3(drink:Option<&str>) {
    let inside = drink.unwrap();
    if inside == "lemonade" {
        panic!("AAAaaa");
    }

    println!("I love {}s!!!", inside);
}

// 18.2
// rustc  lemonade.rs -C panic=abort

#[cfg(panic = "unwind")]
fn ah(){ println!("Spit it out!!!!");}

#[cfg(not(panic="unwind"))]
fn ah(){ println!("This is not your party. Run!!!!");}

fn drink_v2(beverage: &str){
    if beverage == "lemonade"{ ah();}
    else{println!("Some refreshing {} is all I need.", beverage);}
}

// 18.1
fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}