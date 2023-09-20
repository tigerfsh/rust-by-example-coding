use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;

use std::env;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;

use std::io::{self};

use std::process::Command;
use std::process::Stdio;
use std::os::unix;

static PANGRAM: &'static str = "Welcome to Beijing\n";

// static NTHREADS: i32 = 3;

const NTHREADS: u32 = 10;
const MY_NTHREADS: i32 = 5;

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
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let mut children = Vec::new();

    for id in 0..MY_NTHREADS {
        let thread_tx = tx.clone();
        let child = thread::spawn(move || {
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
        children.push(child);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..MY_NTHREADS {
        // recv will block current thread.
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread painced");
    }

    println!("{:?}", ids);

    // 20.3
    let path = Path::new(".");

    let _display = path.display();
    println!("display: {:?}", _display);

    let mut new_path = path.join("a").join("b");
    println!("new path: {:?}", new_path);

    new_path.push("c");
    new_path.push("myfile.tar.gz");

    println!("new path: {:?}", new_path);

    new_path.set_file_name("package.tgz");
    println!("set_file_name: {:?}", new_path);

    match new_path.to_str() {
        None => panic!("new path is not a valid UTF-8 sequence."),
        Some(s) => println!("new path is {}", s),
    }

    // 20.4.1 open file
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("can not open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("can not read {}: {}", display, why),
        Ok(_) => println!("{} contains: \n{}", display, s),
    }

    // 20.4.2 create file
    let path = Path::new("lorem_ipsum.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Ok(file) => file,
        Err(why) => panic!("can not create {}:{}", display, why),
    };

    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => panic!("can not write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("lorem_ipsum.txt")
        .unwrap();
    file.write_all("AAABBBCCC\n".as_bytes()).unwrap();

    // 20.4.3

    if let Ok(lines) = read_lines_v3("./hello.txt") {
        for line in lines {
            if let Ok(ip) = line {
                println!("{ip}");
            }
        }
    }

    // 20.5
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e);
        });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("rustc succeed and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("rustc failed and stderr was:\n{}", s);
    }

    // run wc command, and you will understand.
    let process = match Command::new("wc")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(why) => panic!("couldn't spawn wc: {}", why),
        Ok(process) => process,
    };

    match process.stdin.unwrap().write_all(PANGRAM.as_bytes()) {
        Ok(_) => println!("sent pangram to wc"),
        Err(why) => panic!("couldn't write to wc stdio: {}", why),
    }

    let mut s = String::new();
    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(why) => panic!("couldn't read wc stdout: {}", why),
        Ok(_) => print!("wc responded with:\n{}", s),
    }

    // 20.5.2 wait
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("done.");

    // 20.6
    println!("`mkdir a`");
    // Create a directory, returns `io::Result<()>`
    match fs::create_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(_) => {},
    }

    println!("`echo hello > a/b.txt`");
    // The previous match can be simplified using the `unwrap_or_else` method
    echo("hello", &Path::new("a/b.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`mkdir -p a/c/d`");
    // Recursively create a directory, returns `io::Result<()>`
    fs::create_dir_all("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`touch a/c/e.txt`");
    touch(&Path::new("a/c/e.txt")).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`ln -s ../b.txt a/c/b.txt`");
    // Create a symbolic link, returns `io::Result<()>`
    if cfg!(target_family = "unix") {
        unix::fs::symlink("../b.txt", "a/c/b.txt").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    println!("`cat a/c/b.txt`");
    match cat(&Path::new("a/c/b.txt")) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(s) => println!("> {}", s),
    }

    println!("`ls a`");
    // Read the contents of a directory, returns `io::Result<Vec<Path>>`
    match fs::read_dir("a") {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {:?}", path.unwrap().path());
        },
    }

    println!("`rm a/c/e.txt`");
    // Remove a file, returns `io::Result<()>`
    fs::remove_file("a/c/e.txt").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    println!("`rmdir a/c/d`");
    // Remove an empty directory, returns `io::Result<()>`
    fs::remove_dir("a/c/d").unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    // 20.7
    let args: Vec<String> = env::args().collect();
    println!("My path is {}", args[0]);
    println!("I got {:?} arguments: {:?}", args.len()-1, &args[1..]);

    // 20.8
    // FFI pass 
    
}

// 20.4.3
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }
    result
}
// 20.6
fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn echo(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    f.write_all(s.as_bytes())
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn read_lines_v2(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn read_lines_v3<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let res = Ok(io::BufReader::new(file).lines());
    res
}

static LOREM_IPSUM: &str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
";
