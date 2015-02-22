fn main() {
    are_you_on_linux();
}

#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}
