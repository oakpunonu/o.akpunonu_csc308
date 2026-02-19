use std::fs::File;
use std::io::Write;
use std::process::Command;  

fn main() {

    let output = Command::new("echo")
        .arg("Rust Process Management")
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(stdout.as_bytes()).expect("Failed to write to file");
}
