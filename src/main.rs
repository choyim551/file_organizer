use log::info;
use crate::{engine::rule_engine::apply_rules, fog::{actions::execute_action, scanner::scan_dir}};

mod fog;
mod engine;
mod logger;

fn main() {
    logger::init();

    let target_dir = "./dummy_test/";
    
    let dry_run = false;

    let files = scan_dir(target_dir);

    info!("Program Started");

    for file in &files {
        if let Some(action) = apply_rules(file) {
            execute_action(file, &action, dry_run);
        }
    }

    println!("Total files {}", files.len());
}
