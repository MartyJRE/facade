use std::fs::File;
use std::io::Read;
use std::path::Path;

use tiny_http::Method;
use walkdir::WalkDir;
use yaml_rust::{Yaml, YamlLoader};

use crate::definition::{Definition, Definitions, Endpoint};

fn parse_path(yaml: &Yaml) -> Vec<Definition> {
    let paths = &yaml["paths"];
    let map = paths.as_hash().expect("Paths did not exist.");

    let mut definitions = Vec::default();

    for (path, path_description) in map.iter() {
        let mut definition = Definition::default();
        let path_map = path_description.as_hash().expect("Path did not exist.");
        for (method, method_description) in path_map.iter() {
            let mut endpoint = {
                match method.as_str() {
                    Some("get") => {
                        Endpoint::new(Method::Get, path.as_str().expect("Path was not string."))
                    }
                    Some("post") => {
                        Endpoint::new(Method::Post, path.as_str().expect("Path was not string."))
                    }
                    Some("patch") => {
                        Endpoint::new(Method::Patch, path.as_str().expect("Path was not string."))
                    }
                    _ => {
                        panic!("Unknown method.")
                    }
                }
            };
            let parameters = &method_description["parameters"].as_vec().expect("Could not parse parameters.");
            for parameter in parameters.iter() {
                let name = parameter["name"].as_str().expect("Could not parse parameter name.");
                let parameter_in = parameter["in"].as_str().expect("Parameter is missing the `in` field.");
                endpoint.add_parameter(name, parameter_in);
            }
            let m = endpoint.regex().is_match("/test/emotions");
            println!("{:?}", m);
            definition.add_endpoint(endpoint);
        }
        definitions.push(definition);
    }

    definitions
}

pub fn parse_dir(path: &str) -> Definitions {
    let mut definitions: Vec<Definition> = Vec::new();
    for description in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        let path = description.path();
        if path.is_dir() {
            continue;
        }
        println!("INFO: Parsing {}", path.display());

        let data = load_file(path);

        for item in data {
            let parsed_paths = parse_path(&item);
            for parsed_path in parsed_paths {
                definitions.push(parsed_path)
            }
        }
    }

    Definitions { definitions }
}

fn load_file(path: &Path) -> Vec<Yaml> {
    let mut file = File::open(path).expect("File could not be opened.");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("File could not be read to string.");

    YamlLoader::load_from_str(&contents).expect("File could not be parsed as YAML.")
}