use my_mod::public_function_in_create;

mod my_mod {
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    pub fn function() {
        println!("called `my_mod::function()`");
    }

    pub fn indirect_access() {
        println!("called my_mod::indirect_access()");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("my_mod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("my_mod::nested::private_function()");
        }

        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("my_mod::nested::public_function_in_my_mod()");
            public_function_in_nested();
            
        }
        
        // <=> private 
        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested()");
        }

        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod()");
        }
    }

    pub fn call_public_function_in_my_mod() {
        nested::public_function_in_my_mod();
        nested::public_function_in_super_mod();
        
    }

    pub(crate) fn public_function_in_create() {
        println!("called my_mod::public_function_in_create()");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function(){
            println!("my_mod::private_nested::function()");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function(){
            println!("called my_mod::private_nested::restricted_function()");
        }
    }
}

fn function() {
    println!("called function()");
}

// struct visibility 
mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

mod deeply {
    pub mod nested {
        pub fn function(){
            println!("called deeply::nested::function()");
        }
    }
}

use deeply::nested::function as other_function;


mod cool {
    pub fn function() {
        println!("called `coll::function()`");
    }
}

mod mymod {
    fn function() {
        println!("called `mymod::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `mymod::cool::function()`");
        }
    }

    pub fn indirect_call() {
        println!("mymod::indirect_call()");

        self::function();
        function();

        self::cool::function();

        // The `super` keyword refers to the parent scope (outside the `mymod` module).
        use super::cool::function as root_function;
        root_function();
    }
}
fn main() {
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    my_mod::public_function_in_create();

    // struct visibility 
    let open_box = my::OpenBox {contents: "public information"};
    println!("{}", open_box.contents);

    let _closed_box = my::ClosedBox::new("classified information");

    // the `use` declaration
    other_function();

    println!("Entering block");
    {
        use crate::deeply::nested::function;

        function();

        println!("Leaving block");
    }

    function();

    // super and self
    mymod::indirect_call();
}
