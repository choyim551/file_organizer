use log4rs::{config::Deserializers, init_file};

pub fn init() {
    let config_path = "config/logers.yaml";

    let mut deserializers = Deserializers::new();

    init_file(config_path, deserializers).expect("Failed to init logers");
}
