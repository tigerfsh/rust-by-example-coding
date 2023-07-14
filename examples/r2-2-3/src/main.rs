use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    let xs = [1, 2, 3, 4, 5];
    let ys = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the aray: {}", xs[1]);

    println!("Number of elements in array: {}", xs.len());

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    println!("Borrow a section of the array as a slice");
    analyze_slice(&xs[1..]);

    analyze_slice(&ys[1..4]);

    // let empty_array: [u32; 0] = [];
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // Same but more verbose

    let s_xs = &xs;

    for i in 0..s_xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(val) => println!("{}: {}", i, val),
            None => println!("{} is too far!", i),
        }
    }
}
