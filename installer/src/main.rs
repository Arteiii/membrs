use std::io::stdin;
use std::process::exit;
use std::process::Command;
use std::thread;
use std::time::Duration;

use lazy_static::lazy_static;
use zenity::style::Stylize;

pub const LOGO: &str = r#"
 __   __  _______  __   __  _______  ______    _______
|  |_|  ||       ||  |_|  ||  _    ||    _ |  |       |
|       ||    ___||       || |_|   ||   | ||  |  _____|
|       ||   |___ |       ||       ||   |_||_ | |_____
|       ||    ___||       ||  _   | |    __  ||_____  |
| ||_|| ||   |___ | ||_|| || |_|   ||   |  | | _____| |
|_|   |_||_______||_|   |_||_______||___|  |_||_______|
"#;

lazy_static! {
    static ref PACKAGE_MANAGER: Manager = Manager::get_active().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });
}

enum Manager {
    APT,
    DNF,
    YUM,
}

impl Manager {
    pub fn get_active() -> Result<Self, String> {
        if Command::new("apt").arg("--version").output().is_ok() {
            Ok(Self::APT)
        } else if Command::new("dnf").arg("--version").output().is_ok() {
            Ok(Self::DNF)
        } else if Command::new("yum").arg("--version").output().is_ok() {
            Ok(Self::YUM)
        } else {
            Err("No package manager found".to_string())
        }
    }

    pub fn get_install_command(&self, package: &str) -> String {
        match self {
            Self::DNF => match package {
                "openssl" => "sudo dnf install -y openssl".to_string(),
                "git" => "sudo dnf install -y git".to_string(),
                _ => String::new(),
            },
            Self::YUM => match package {
                "openssl" => "sudo yum install -y openssl".to_string(),
                "git" => "sudo yum install -y git".to_string(),
                _ => String::new(),
            },
            Self::APT => match package {
                "openssl" => "sudo apt-get install -y openssl".to_string(),
                "git" => "sudo apt-get install -y git".to_string(),
                _ => String::new(),
            },
        }
    }
}

enum Package {
    Rust,
    OpenSSL,
    Git,
}

impl Package {
    fn details(&self) -> Dependency {
        match self {
            Package::Rust => Dependency {
                name: "Rust",
                check_command: "rustc",
                check_arg: "--version",
                install_command:
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
                        .parse()
                        .unwrap(),
            },
            Package::OpenSSL => Dependency {
                name: "OpenSSL",
                check_command: "openssl",
                check_arg: "version",
                install_command: PACKAGE_MANAGER.get_install_command("openssl"),
            },
            Package::Git => Dependency {
                name: "Git",
                check_command: "git",
                check_arg: "--version",
                install_command: PACKAGE_MANAGER.get_install_command("git"),
            },
        }
    }
}

struct Dependency {
    name: &'static str,
    check_command: &'static str,
    check_arg: &'static str,
    install_command: String,
}

impl Dependency {
    fn check_and_install(&self) {
        match Command::new(self.check_command)
            .arg(self.check_arg)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    println!(
                        "{} is already installed: {}",
                        self.name,
                        String::from_utf8_lossy(&output.stdout)
                    );
                } else {
                    self.install();
                }
            }
            Err(_) => {
                self.install();
            }
        }
    }

    fn install(&self) {
        println!(
            "{} is not installed. Installing {}...",
            self.name, self.name
        );
        match Command::new("sh")
            .arg("-c")
            .arg(&self.install_command)
            .status()
        {
            Ok(status) if status.success() => {
                println!("{} installation completed successfully.", self.name);
            }
            Ok(_) | Err(_) => {
                eprintln!("Failed to install {}.", self.name);
                exit(1);
            }
        }
    }
}

fn use_docker() -> bool {
    println!("Would you like to use Docker? (yes/no)");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().eq_ignore_ascii_case("yes")
}

fn install_docker() {
    println!("Installing Docker...");
    match Command::new("sh").arg("-c").arg("curl -fsSL https://get.docker.com -o get-docker.sh && sudo sh get-docker.sh && sudo systemctl enable docker").status() {
        Ok(status) if status.success() => {
            println!("Docker installation completed successfully.");
            check_docker_daemon();
        }
        Ok(_) | Err(_) => {
            eprintln!("Failed to install Docker.");
            exit(1);
        }
    }
}

fn check_docker_daemon() {
    println!("Checking Docker daemon...");
    for _ in 0..10 {
        if let Ok(output) = Command::new("docker").arg("info").output() {
            if output.status.success() {
                println!("Docker daemon is running.");
                return;
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
    eprintln!("Failed to start Docker daemon.");
    exit(1);
}

fn main() {
    println!("{}\n\n", LOGO.magenta());

    if use_docker() {
        if let Err(err) = Command::new("docker").arg("--version").output() {
            eprintln!("Error checking Docker version: {}", err);
            install_docker();
        } else {
            println!("Docker is already installed.");
            check_docker_daemon();
        }
    }

    let dependencies = vec![Package::Rust, Package::OpenSSL, Package::Git];

    for package in dependencies {
        let dependency = package.details();
        dependency.check_and_install();
    }
}
