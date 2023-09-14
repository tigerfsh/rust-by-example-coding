use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;

// static NTHREADS: i32 = 3;

const NTHREADS: u32 = 10;

const PI: f32 = 3.1415926;

static mut GLOBAL_COUNTER: i32 = 0;

fn main() {
    // 20.1
    let mut children = vec![];
    for i in 0..NTHREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        let _ = child.join();
    }

    // 20.1.1
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";

    let mut children = vec![];

    let chunked_data = data.split_whitespace();
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();
            println!("processed segment {}, result={}", i, result);

            result
        }));
    }

    // we use the "turbofish" ::<> to provide sum() with a type hint.
    // let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    // try without the turbofish, by instead explicitly
    // specifying the type of final_result
    let final_result: u32 = children.into_iter().map(|c| c.join().unwrap()).sum();

    println!("(A) Final sum result: {}", final_result);

    // 20.1.1
    // Limit thread nums
    // split input
    // 20.1.1

    /*
    Note that, although we're passing references across thread boundaries,
    Rust understands that we're only passing read-only references, and
    that thus no unsafety or data races can occur. Also because the references
    we're passing have 'static lifetimes<how to understand this?>, Rust understands
    that our data won't be destroyed while these threads are still running.
    (When you need to share non-static data between threads, you can use a smart
    pointer like Arc to keep the data alive and avoid non-static lifetimes.)
    */
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
    const NUM: usize = 2;

    let chunked_data = data.split_whitespace();
    let chunked_list: Vec<&str> = chunked_data.collect();
    let mut final_result = 0;

    for i in (0..chunked_list.len()).step_by(NUM) {
        let mut children = vec![];

        let chunked_list = &chunked_list[i..(i + NUM)];
        for &data_segment in chunked_list.iter() {
            children.push(thread::spawn(move || -> u32 {
                let result = data_segment.chars().map(|c| c.to_digit(10).unwrap()).sum();
                result
            }));
        }
        let tmp_result: u32 = children.into_iter().map(|h| h.join().unwrap()).sum();

        final_result += tmp_result;
    }
    println!("(B) Final sum result: {}", final_result);

    // const vs static ?
    // const: 不可变， 常量
    // static:可变或者不可变，全局变量
    println!("PI: {PI}");

    unsafe {
        GLOBAL_COUNTER += 1;
        println!("GLOBAL_COUNTER: {GLOBAL_COUNTER}");
    }

    // 20.2 channel
}
