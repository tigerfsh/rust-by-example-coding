/*
 Unsafe annotations in Rust are used to bypass protections put in place by the compiler; specifically, there are four primary things that unsafe is used for:

    dereferencing raw pointers
    calling functions or methods which are unsafe (including calling a function over FFI, see a previous chapter of the book)
    accessing or modifying static mutable variables
    implementing unsafe traits


*/
use std::slice;

fn main() {
    let raw_p: *const u32 = &10;

    unsafe {
        assert!(*raw_p == 10);
    }

    let some_vector = vec![1, 2, 3, 4];
    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);
        assert_eq!(some_vector.as_slice(), my_slice);
    }

    // 22.1 
    // pass 
}
