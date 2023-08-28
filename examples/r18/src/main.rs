fn main() {
    // 18.1 

    drink("bbb");
    // drink("aaa");
    
    // 18.2 
    drink_v2("aaa");
    drink_v2("bbb");

    drink_v3("aaa");
    drink_v3("bbb");
}   
// 18.2 

#[cfg(panic="unwind")]
fn ah() {println!("Split it out !!!");}

#[cfg(not(panic="unwind"))]
fn ah() {println!("This  is not your party. Run !!!");}

fn drink_v3(beverage: &str) {
    if beverage == "aaa" {
        ah();
    } else {
        println!("Some refreshing {} is all I need.", beverage);
    }
}
fn drink_v2(beverage: &str) {
    if beverage == "aaa" {
        if cfg!(panic="abort") {
            println!("This is not your party. Run !!!");
        } else {
            println!("Spit it out !!!");
        }
    } else { println!("Some refreshing {} is all I need.", beverage); }
}

// 18.1 
fn drink(beverage: &str) {
    if beverage == "aaa" {
        panic!("aaa");
    }

    println!("Some refreshing {} is all I need.", beverage);
}