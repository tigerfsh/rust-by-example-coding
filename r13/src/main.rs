// decode_code 
fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

#[allow(dead_code)]
fn noisy_unused_function() {}

// cfg
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are not running linux");
}

fn main() {
    // dead_code 
    used_function();

    // cfg
    are_you_on_linux();
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's linux.");
    } else {
        println!("Yes. It's not linux.");
    }

}
