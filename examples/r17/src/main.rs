/*
So why are macros useful?

    Don't repeat yourself. There are many cases where you may need similar functionality in multiple places but with different types. Often, writing a macro is a useful way to avoid repeating code. (More on this later)

    Domain-specific languages. Macros allow you to define special syntax for a specific purpose. (More on this later)

    Variadic interfaces. Sometimes you want to define an interface that takes a variable number of arguments. An example is println! which could take any number of arguments, depending on the format string. (More on this later)

*/

/*
These are some of the available designators:

    block
    expr is used for expressions
    ident is used for variable/function names
    item
    literal is used for literal constants
    pat (pattern)
    path
    stmt (statement)
    tt (token tree)
    ty (type)
    vis (visibility qualifier)

*/

/*
https://doc.rust-lang.org/reference/macros-by-example.html
*/

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

// 17.1.1

macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("You called {:?}()",
                    stringify!($func_name))
        }
    };
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}",
            stringify!($expression),
            $expression)
    };
}
fn main() {

    say_hello!();

    // 17.1.1
    
    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!(
        {

            let x = 1u32;
            x * x + 2 * x -1
        }
        
    );
}
