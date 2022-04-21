use crate::directory::get_current_working_directory;
use std::borrow::Borrow;

pub fn print_prompt() {
    print!(
        "{} R$> ",
        replace_home(get_current_working_directory().borrow(), "~")
    );
}

pub fn replace_home(pwd: &str, replacement: &str) -> String {
    pwd.to_string()
        .replace(env!("HOME"), replacement)
        .trim_end()
        .to_string()
}
