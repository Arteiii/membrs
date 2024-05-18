use std::process::{exit, Command};

use zenity::spinner::MultiSpinner;
use zenity::style::{Color, StyledString};

pub struct Dependency {
    pub name: &'static str,
    pub check_command: &'static str,
    pub check_arg: &'static str,
    pub install_command: String,
}

impl Dependency {
    pub(crate) fn check_and_install(&self, sp: &MultiSpinner, sp_id: &usize) {
        match Command::new(self.check_command)
            .arg(self.check_arg)
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    sp.set_styled_text(
                        sp_id,
                        StyledString::simple(
                            &format!(
                                "{} is already installed: {}",
                                self.name,
                                String::from_utf8_lossy(&output.stdout)
                            ),
                            Some(Color::Green),
                            None,
                            None,
                        ),
                    );
                    sp.stop(sp_id);
                } else {
                    self.install(sp, sp_id);
                }
            }
            Err(_) => {
                self.install(sp, sp_id);
            }
        }
    }

    fn install(&self, sp: &MultiSpinner, sp_id: &usize) {
        sp.set_styled_text(
            sp_id,
            StyledString::simple(
                &format!(
                    "{} is not installed. Installing {}...",
                    self.name, self.name
                ),
                Some(Color::Grey),
                None,
                None,
            ),
        );
        match Command::new("sh")
            .arg("-c")
            .arg(&self.install_command)
            .status()
        {
            Ok(status) if status.success() => {
                sp.set_styled_text(
                    sp_id,
                    StyledString::simple(
                        &format!("{} installation completed successfully.", self.name),
                        Some(Color::Green),
                        None,
                        None,
                    ),
                );
                sp.stop(sp_id);
            }
            Ok(_) | Err(_) => {
                sp.set_styled_text(
                    sp_id,
                    StyledString::simple(
                        &format!("Failed to install {}.", self.name),
                        Some(Color::Red),
                        None,
                        None,
                    ),
                );
                sp.stop(sp_id);
                exit(1);
            }
        }
    }
}
