use std::{env::{self, join_paths}, path::{Path, PathBuf}, process::Command};

pub fn get_current_working_directory() -> String {
    env::current_dir().expect("failed to fetch working directory: pwd").into_os_string().into_string().unwrap()
}


pub fn change_working_directory(path : &str) {


    let working_directory = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    let new_working_directory = format!("{}/{}", working_directory, Path::new(path).to_str().unwrap());

    env::set_current_dir(&new_working_directory).expect(format!("failed to switch directory: {}", new_working_directory).as_str())
}