use std::process::{Command, exit};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref PACKAGE_MANAGER: Manager = Manager::get_active().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        exit(1);
    });
}

pub enum Manager {
    Apt,
    Dnf,
    Yum,
}

impl Manager {
    pub fn get_active() -> Result<Self, String> {
        if Command::new("apt").arg("--version").output().is_ok() {
            Ok(Self::Apt)
        } else if Command::new("dnf").arg("--version").output().is_ok() {
            Ok(Self::Dnf)
        } else if Command::new("yum").arg("--version").output().is_ok() {
            Ok(Self::Yum)
        } else {
            Err("No package manager found".to_string())
        }
    }

    pub fn get_install_command(&self, package: &str) -> String {
        match self {
            Self::Dnf => match package {
                "openssl" => "sudo dnf install -y openssl >/dev/null 2>&1".to_string(),
                "git" => "sudo dnf install -y git >/dev/null 2>&1".to_string(),
                _ => String::new(),
            },
            Self::Yum => match package {
                "openssl" => "sudo yum install -y openssl >/dev/null 2>&1".to_string(),
                "git" => "sudo yum install -y git >/dev/null 2>&1".to_string(),
                _ => String::new(),
            },
            Self::Apt => match package {
                "openssl" => "sudo apt-get install -y openssl >/dev/null 2>&1".to_string(),
                "git" => "sudo apt-get install -y git >/dev/null 2>&1".to_string(),
                _ => String::new(),
            },
        }
    }
}