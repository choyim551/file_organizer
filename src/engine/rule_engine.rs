use crate::fog::scanner::FileInfo;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::engine::rule_engine::{apply_rules, Action};
    use crate::fog::scanner::FileInfo;

    fn fake_file(path: &str, ext: Option<&str>) -> FileInfo {
        FileInfo {
            path: PathBuf::from(path),
            extension: ext.map(|s| s.to_string())
        }
    }

    #[test]
    fn jpg_and_png_goes_to_picture() {
        let file = fake_file("test.jpg", Some("jpg"));
        let action = apply_rules(&file);

        assert_eq!(action, Some(Action::MoveTo("Picture/".to_string())));
    }

    #[test]
    fn pdf_to_documents() {
        let file = fake_file("test.pdf", Some("pdf"));
        let action = apply_rules(&file);

        assert_eq!(action, Some(Action::MoveTo("Documents/".to_string())));
    }

    #[test]
    fn no_extension_no_action() {
        let file = fake_file("test", None);
        let action = apply_rules(&file);

        assert_eq!(action, None);
    }

    #[test]
    fn unknown_extension_no_action() {
        let file = fake_file("test.rs", Some("rs"));
        let action = apply_rules(&file);

        assert_eq!(action, None);
    }
}