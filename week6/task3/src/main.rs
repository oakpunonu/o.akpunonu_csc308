use std::process::Command;

fn main() {
    // Spawn a child process to run the echo command
    let output = Command::new("echo")
        .arg("Hello from child process!")
        .output()  // Executes the command and captures output
        .expect("Failed to execute command");

    // Convert the output bytes to a string and print
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

