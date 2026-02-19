use std::process::Command;

fn main() {
    let child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("failed to execute process");
    println!("Spawned child process with PID: {}", child.id());

    let child2 = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("failed to execute process");
    println!("Spawned child process with PID: {}", child2.id());

    let child3 = Command::new("echo")
        .arg("Hello from child")
        .spawn()
        .expect("failed to execute process");
    println!("Spawned child process with PID: {}", child3.id());

    std::thread::sleep(std::time::Duration::from_secs(60));

    
}
