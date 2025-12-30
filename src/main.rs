use log::info;

mod fog;
mod engine;
mod logger;

fn main() {
    logger::init();

    let target_dir = "./";

    let files = fog::scanner::scan_dir(target_dir);

    info!("Program Started");

    for file in &files {
        let action = engine::rule_engine::apply_rules(file);
        println!("{:?} => {:?}", file.path, action);
    }

    println!("Total files {}", files.len());
}
