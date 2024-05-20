use std::fmt;
use std::process::{Command, exit};

use lazy_static::lazy_static;

use crate::package::Package;

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

impl fmt::Debug for Manager {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let name = match self {
			Manager::Apt => "APT",
			Manager::Dnf => "DNF",
			Manager::Yum => "YUM",
		};
		write!(f, "{}", name)
	}
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

	pub fn get_install_command(&self, package: Package) -> String {
		match self {
			Self::Dnf => match package {
				Package::OpenSSL => "sudo dnf install -y openssl >/dev/null 2>&1".to_string(),
				Package::Git => "sudo dnf install -y git >/dev/null 2>&1".to_string(),
				Package::PostgreSQL => "sudo dnf install -y postgresql postgresql-contrib && sudo systemctl enable postgresql.service >/dev/null 2>&1".to_string(),
				Package::Npm => "sudo dnf install -y npm >/dev/null 2>&1".to_string(),
				_ => String::new(),
			},
			Self::Yum => match package {
				Package::OpenSSL => "sudo yum install -y openssl >/dev/null 2>&1".to_string(),
				Package::Git => "sudo yum install -y git >/dev/null 2>&1".to_string(),
				Package::PostgreSQL => "sudo yum install -y postgresql postgresql-contrib && sudo systemctl enable postgresql.service >/dev/null 2>&1".to_string(),
				Package::Npm => "sudo yum install -y npm >/dev/null 2>&1".to_string(),
				_ => String::new(),
			},
			Self::Apt => match package {
				Package::OpenSSL => "sudo apt-get install -y openssl >/dev/null 2>&1".to_string(),
				Package::Git => "sudo apt-get install -y git >/dev/null 2>&1".to_string(),
				Package::PostgreSQL => "sudo apt-get install -y postgresql postgresql-contrib && sudo systemctl enable postgresql.service >/dev/null 2>&1".to_string(),
				Package::Npm => "sudo apt-get install -y npm >/dev/null 2>&1".to_string(),
				_ => String::new(),
			},
		}
	}
}
