use crate::fog::scanner::FileInfo;

#[derive(Debug)]
pub enum Action {
    MoveTo(String)
}

pub fn apply_rules(file: &FileInfo) -> Option<Action> {
    match file.extension.as_deref() {
        Some("jpg") | Some("png") => {
            Some(Action::MoveTo("Picture/".to_string()))
        }
        Some("pdf") => {
            Some(Action::MoveTo("Documents/".to_string()))
        }
        Some("txt") => {
            Some(Action::MoveTo("Text/".to_string()))
        }
        Some("toml") => {
            Some(Action::MoveTo("Config/".to_string()))
        }
        _ => None
    }
}