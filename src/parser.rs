use std::fs::File;
use std::io::Read;
use std::path::Path;
use tiny_http::Method;
use walkdir::WalkDir;
use yaml_rust::{Yaml, YamlLoader};
use crate::definition::{Definition, Definitions};

fn parse_definition(yaml: &Yaml) -> Definition {
    let paths = &yaml["paths"];
    let map = paths.as_hash().unwrap();
    let mut definition = Definition::default();

    for (path, path_description) in map.iter() {
        let path_map = path_description.as_hash().unwrap();
        for (method, method_description) in path_map.iter() {
            let parameters = &method_description["parameters"];
            if let Some(path) = path.as_str() {
                match method.as_str() {
                    Some("get") => {
                        definition.add_endpoint(Method::Get, path);
                    }
                    Some("post") => {
                        definition.add_endpoint(Method::Post, path);
                    }
                    Some("patch") => {
                        definition.add_endpoint(Method::Patch, path);
                    }
                    _ => {}
                }
            }
        }
    }

    definition
}

pub fn parse_dir(path: &str) -> Definitions {
    let mut definitions: Vec<Definition> = Vec::new();
    for description in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        let path = description.path();
        if path.is_dir() {
            continue;
        }
        eprintln!("INFO: Parsing {}", path.display());

        let data = load_file(path);

        for item in data {
            definitions.push(parse_definition(&item));
        }
    }

    Definitions { definitions }
}

fn load_file(path: &Path) -> Vec<Yaml> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    YamlLoader::load_from_str(&contents).unwrap()
}