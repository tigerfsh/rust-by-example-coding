mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `you::function()`");
}

fn private_function() {
    println!("called `you::private_function()`");
}

pub fn indirect_access() {
    println!("called `you::indirect_access()`, that\n");
    private_function();
}

