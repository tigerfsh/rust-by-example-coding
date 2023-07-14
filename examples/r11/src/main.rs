// extern crate rary; // May be required for Rust 2015 edition or earlier

// rustc main.rs --extern rary=library.rlib && ./main 

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}