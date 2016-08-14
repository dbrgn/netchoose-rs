//! Everything interfacing with netctl.
//!
//! Communication with netctl is done through a subprocess.

use std::process::Command;

#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub active: bool,
}

/// Return list of netctl profiles.
pub fn get_profiles() -> Vec<Profile> {
    let output = Command::new("netctl")
                         .arg("list")
                         .output()
                         .expect("Could not fetch netctl profiles");
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines()
          .map(|line| Profile {
              name: line.trim_left_matches(|c| c == '*' || c == ' ').to_string(),
              active: line.starts_with('*'),
          }).collect::<Vec<_>>()
}

/// Switch to the profile with the specified name.
///
/// Note: This command is blocking and might take a while!
pub fn switch_to_profile(name: &str) {
    let status = Command::new("gksudo")
                         .arg("netctl")
                         .arg("switch-to").arg(name)
                         .status()
                         .expect("Could not switch netctl profile");
    if status.success() {
        println!("Switched to {}!", name);
    } else {
        println!("Switching to {} failed", name);
    }
}
