use std::process::{Command, Stdio};
use std::thread;

fn main() {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    if cfg!(windows) {
        eprintln!("Windows is currently not supported");
        // todo!:
        // add windows support (for what ever reason cmd can not find npm)
    }

    let mut child = if cfg!(debug_assertions) {
        Command::new("npm")
            .arg("run")
            .arg("dev")
            .current_dir("frontend")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("Failed to start Next.js development server")
    } else {
        println!("Running npm build in production mode");

        let npm_build = Command::new("npm")
            .arg("run")
            .arg("build")
            .current_dir("frontend")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Failed to run npm build");

        if npm_build.status.success() {
            println!("Next.js build completed successfully");

            println!("Running npm start to serve built files");

            Command::new("npm")
                .arg("run")
                .arg("start")
                .current_dir("frontend")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to start server for serving built files")
        } else {
            eprintln!(
                "Next.js build failed with exit code: {}",
                npm_build.status.code().unwrap_or_default()
            );
            return;
        }
    };

    // Capture child process ID for later use
    let _child_id = child.id();

    // Spawn a thread to wait for the child process to finish
    thread::spawn(move || {
        let _ = child.wait();
    });

    // Wait for the main thread to exit
    // This will ensure that the child process also exits
    thread::park();
}
