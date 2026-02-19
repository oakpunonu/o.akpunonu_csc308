use std::process::{Command, Child};
use std::thread;
use std::time::Duration;

fn main() {
    let mut child: Child = Command::new("ping")
        .arg("google.com")
        .spawn()
        .expect("Failed to start ping process");

    println!("Ping process started with PID: {}", child.id());
    thread::sleep(Duration::from_secs(5));


    match child.kill() {
        Ok(_) => println!("Ping process killed successfully."),
        Err(e) => eprintln!("Failed to kill ping process: {}", e),
    }

    match child.wait() {
        Ok(status) => {
            if let Some(code) = status.code() {
                println!("Ping process exited with code: {}", code);
            } else {
                println!("Ping process terminated by signal.");
            }
        }
        Err(e) => eprintln!("Failed to wait for ping process: {}", e),
    }
}
