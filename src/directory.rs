use std::process::Command;

pub fn get_current_working_directory() -> String {
    let output = Command::new("pwd")
        .output()
        .expect("failed to execute process: pwd");

    String::from_utf8_lossy(&output.stdout).clone().to_string()
}
