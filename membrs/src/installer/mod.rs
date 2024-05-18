use std::fs;
use std::path::{Path, PathBuf};

use git2::Repository;
use zenity::spinner::{Frames, MultiSpinner};
use zenity::style::{Color, StyledString};

pub fn install(path: PathBuf) {
    let sp = MultiSpinner::new();
    sp.show_line_number();
    sp.run_all();

    create_dir(&sp, &path);
    clone_repository(&sp, &path);

    println!("Installation completed successfully!");
}

fn create_dir(sp: &MultiSpinner, path: &Path) {
    let frontend_sp = sp.add(Frames::dot_spinner1());
    sp.set_text(&frontend_sp, "Creating membrs directory...".to_string());

    let frontend_dir = path.join("membrs");
    if let Err(err) = fs::create_dir_all(frontend_dir) {
        sp.set_styled_text(
            &frontend_sp,
            StyledString::simple(
                &format!("Error creating membrs directory: {}", err),
                Some(Color::Red),
                None,
                None,
            ),
        );
        sp.stop(&frontend_sp);
    } else {
        sp.set_styled_text(
            &frontend_sp,
            StyledString::simple(
                "Successfully created the membrs directory",
                Some(Color::Green),
                None,
                None,
            ),
        );
        sp.stop(&frontend_sp);
    }
}

fn clone_repository(sp: &MultiSpinner, path: &Path) {
    let git_sp = sp.add(Frames::dot_spinner1());
    sp.set_text(&git_sp, "Cloning repository...".to_string());

    let repository_url = "https://github.com/Arteiii/membrs.git";

    match Repository::clone(repository_url, path.join("membrs")) {
        Ok(_) => {
            sp.set_styled_text(
                &git_sp,
                StyledString::simple(
                    "Repository cloned successfully",
                    Some(Color::Green),
                    None,
                    None,
                ),
            );
        }
        Err(err) => {
            sp.set_styled_text(
                &git_sp,
                StyledString::simple(
                    &format!("Error cloning repository: {}", err),
                    Some(Color::Red),
                    None,
                    None,
                ),
            );
        }
    }

    sp.stop(&git_sp);
}

pub fn build_projects() {}
