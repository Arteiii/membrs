use crate::dependency::Dependency;
use crate::manager::PACKAGE_MANAGER;

pub enum Package {
    Rust,
    OpenSSL,
    Git,
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
