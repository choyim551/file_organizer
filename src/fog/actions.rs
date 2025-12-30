use std::{fs, path::Path};

use crate::{engine::rule_engine::Action, fog::scanner::FileInfo};

pub fn execute_action(file: &FileInfo, action: &Action, dry_run: bool) {
    match action {
        Action::MoveTo(target_dir) => {
            let file_name = match file.path.file_name() {
                Some(name) => name,
                None => return
            };

            let target_path = Path::new(target_dir).join(file_name);

            if dry_run {
                println!("[DRY-RUN Move {:?} -> {:?}]", file.path, target_path);
            } else {
                fs::create_dir_all(target_dir).unwrap();
                fs::rename(&file.path, &target_path).unwrap();
                println!("Move {:?} -> {:?}", file.path, target_path);
            }

        }
    }
}
