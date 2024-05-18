use std::process::{exit, Command};
use std::time::Duration;
use std::{env, thread};

use clap::Arg;
use dialoguer::Confirm;
use zenity::spinner;
use zenity::spinner::Frames;
use zenity::style::{Color, StyledString, Stylize};

use crate::package::Package;

mod dependency;
mod installer;
mod manager;
mod package;

pub const LOGO: &str = r#"
 __   __  _______  __   __  _______  ______    _______
|  |_|  ||       ||  |_|  ||  _    ||    _ |  |       |
|       ||    ___||       || |_|   ||   | ||  |  _____|
|       ||   |___ |       ||       ||   |_||_ | |_____
|       ||    ___||       ||  _   | |    __  ||_____  |
| ||_|| ||   |___ | ||_|| || |_|   ||   |  | | _____| |
|_|   |_||_______||_|   |_||_______||___|  |_||_______|
"#;

fn install_docker() {
    spinner::MultiSpinner::default();
    let sp = spinner::MultiSpinner::default();
    sp.set_text(&sp.get_last(), "Installing Docker...".to_string());
    match Command::new("sh").arg("-c").arg("curl -fsSL https://get.docker.com -o get-docker.sh && sudo sh get-docker.sh && sudo systemctl enable docker").status() {
        Ok(status) if status.success() => {
            sp.set_styled_text(&sp.get_last(), StyledString::simple("Docker installation completed successfully.", Some(Color::Green), None, None));
            sp.stop(&sp.get_last());
            check_docker_daemon();
        }
        Ok(_) | Err(_) => {
            sp.set_styled_text(&sp.get_last(), StyledString::simple("Failed to install Docker.", Some(Color::Red), None, None));
            sp.stop(&sp.get_last());
            exit(1);
        }
    }
}

fn check_docker_daemon() {
    let sp = spinner::MultiSpinner::default();
    sp.set_text(&sp.get_last(), "Checking Docker daemon...".to_string());
    for _ in 0..10 {
        if let Ok(output) = Command::new("docker").arg("info").output() {
            if output.status.success() {
                sp.set_styled_text(
                    &sp.get_last(),
                    StyledString::simple(
                        "Docker daemon is running.",
                        Some(Color::Green),
                        None,
                        None,
                    ),
                );
                sp.stop(&sp.get_last());
                return;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
    sp.set_styled_text(
        &sp.get_last(),
        StyledString::simple(
            "Failed to start Docker daemon.",
            Some(Color::Red),
            None,
            None,
        ),
    );
    sp.stop(&sp.get_last());
}

fn main() {
    let matches = clap::Command::new("membrs")
        .author("Arteii <ben.arteii@proton.me>")
        .version("1.0.1")
        .about("base for membrs server and frontend")
        .arg(
            Arg::new("start")
                .required(false)
                .short('s')
                .help("executes a installed instance of membrs"),
        )
        .get_matches();

    if matches.get_one::<String>("start").is_some() {
        start_all();
    }

    println!("{}\n\n", LOGO.magenta());

    if Confirm::new()
        .with_prompt("Do you want to use Docker?")
        .interact()
        .unwrap_or(false)
    {
        println!("\n");
        if let Err(err) = Command::new("docker").arg("--version").output() {
            eprintln!("Error checking Docker version: {}", err);
            install_docker();
        } else {
            println!("Docker is already installed.");
            check_docker_daemon();
        }
    }

    install_dependencies();

    // Prompt the user for confirmation
    if Confirm::new()
        .with_prompt("Do you want to install the application here?")
        .interact()
        .unwrap_or(false)
    {
        // Get the path of the executable file
        let exe_path = env::current_exe().expect("Failed to get current executable path");

        // Call the installation function with the path of the executable file
        installer::install(exe_path);
    }
}

#[inline]
fn install_dependencies() {
    let sp = spinner::MultiSpinner::new();
    sp.show_line_number();
    sp.run_all();

    let rust_sp = sp.add(Frames::dot_spinner1());
    sp.set_text(&rust_sp, "Checking Rust...".to_string());

    let openssl_sp = sp.add(Frames::dot_spinner1());
    sp.set_text(&openssl_sp, "Checking OpenSSL...".to_string());

    let git_sp = sp.add(Frames::dot_spinner1());
    sp.set_text(&git_sp, "Checking Git...".to_string());
    thread::sleep(Duration::from_secs(2));

    let dependency = Package::Rust.details();
    dependency.check_and_install(&sp, &rust_sp);
    thread::sleep(Duration::from_secs(2));

    let dependency = Package::OpenSSL.details();
    dependency.check_and_install(&sp, &openssl_sp);
    thread::sleep(Duration::from_secs(2));

    let dependency = Package::Git.details();
    dependency.check_and_install(&sp, &git_sp);
    thread::sleep(Duration::from_secs(2));
}

fn start_all() {
    println!("starting applications...");

    todo!();
}
