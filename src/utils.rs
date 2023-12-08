use std::{fs::File, io::Read};
use yaml_rust::{YamlLoader, Yaml};

pub fn get_config() -> Yaml {
    let mut config = File::open("dev.yaml").expect("Unable to open file");
    let mut content = String::new();

    config.read_to_string(&mut content).expect("Unable to read file");

    let docs = YamlLoader::load_from_str(&content).unwrap();
    let doc = docs[0].clone();

    doc
}