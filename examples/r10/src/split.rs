mod you;

fn function() {
    println!("called `function()`");
}

fn main() {
    you::function();
    function();
    you::indirect_access();
    you::nested::function();
    
}