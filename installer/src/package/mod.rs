use crate::dependency::Dependency;
use crate::manager::PACKAGE_MANAGER;
use crate::package::Package::{Git, Npm, OpenSSL, PostgreSQL};

#[allow(non_camel_case_types)]
pub enum Package {
    Rust,
    OpenSSL,
    Git,
    PostgreSQL,
    Npm,
}

impl Package {
    pub(crate) fn details(&self) -> Dependency {
        match self {
            Package::Rust => Dependency {
                name: "Rust",
                check_command: "rustc",
                check_arg: "--version",
                install_command:
                "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y >/dev/null 2>&1"
                    .parse()
                    .unwrap(),
            },
            Package::OpenSSL => Dependency {
                name: "OpenSSL",
                check_command: "openssl",
                check_arg: "version",
                install_command: PACKAGE_MANAGER.get_install_command(OpenSSL),
            },
            Package::Git => Dependency {
                name: "Git",
                check_command: "git",
                check_arg: "--version",
                install_command: PACKAGE_MANAGER.get_install_command(Git),
            },
            Package::PostgreSQL => Dependency {
                name: "PostgreSQL",
                check_command: "pg_config",
                check_arg: "--version",
                install_command: PACKAGE_MANAGER.get_install_command(PostgreSQL),
            },
            Package::Npm => Dependency {
                name: "Npm",
                check_command: "npm",
                check_arg: "--version",
                install_command: PACKAGE_MANAGER.get_install_command(Npm),
            },
        }
    }
}
