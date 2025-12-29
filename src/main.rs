mod fog;
mod engine;

fn main() {
    let target_dir = "./";

    let files = fog::scanner::scan_dir(target_dir);

    for file in &files {
        let action = engine::rule_engine::apply_rules(file);
        println!("{:?} => {:?}", file.path, action);
    }

    println!("Total files {}", files.len());
}