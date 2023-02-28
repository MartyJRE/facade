use std::fs::File;
use std::io::{Read};
use std::path::Path;

use tiny_http::{Method, Response, Server};
use walkdir::WalkDir;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug)]
struct Endpoint {
    method: Method,
    path: String,
}

impl Endpoint {
    fn new(method: Method, path: &str) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
        }
    }

    fn parsed_path(&self) -> &str {
        return self.path.as_str()
    }
}

#[derive(Default, Debug)]
struct Definition {
    endpoints: Vec<Endpoint>,
}

impl Definition {
    fn add_endpoint(&mut self, method: Method, path: &str) {
        self.endpoints.push(Endpoint::new(method, path));
    }

    fn find_endpoint(&self, method: Method, path: &str) -> Option<&Endpoint> {
        self.endpoints.iter().find(|&endpoint| endpoint.method == method && endpoint.parsed_path() == path)
    }
}

#[derive(Debug)]
struct Definitions {
    definitions: Vec<Definition>
}

impl Definitions {
    fn find_endpoint(&self, method: &Method, path: &str) -> Option<&Endpoint> {
        let mut endpoint:Option<&Endpoint> = None;
        for definition in &self.definitions {
            match definition.find_endpoint(method.clone(), path) {
                Some(definition) => {
                    endpoint = Some(definition);
                    break
                }
                _ => {}
            }
        }
        endpoint
    }
}

fn serve(address: &str, definitions: &Definitions) {
    eprintln!("Launching server: {address}");
    let server = Server::http(address).unwrap();

    eprintln!("{:?}", definitions);

    loop {
        let request = match server.recv() {
            Ok(req) => {
                eprintln!("INFO: Received a request {} {}", req.method(), req.url());
                req
            }
            Err(err) => {
                eprintln!("ERROR: Could not process request {err}");
                continue;
            }
        };

        let endpoint = definitions.find_endpoint(request.method(),request.url());
        match endpoint {
            Some(endpoint) => {
                eprintln!("INFO: Matched {:?} {}", endpoint.method, endpoint.path);
                match request.respond(Response::from_string("OK").with_status_code(200)) {
                    Ok(_) => {
                        eprintln!("INFO: Finished request");
                    }
                    Err(err) => {
                        eprintln!("ERROR: Could not respond {err}");
                    }
                };
            }
            _ => {
                eprintln!("WARN: Could not match {} {}",request.method(), request.url());
                match request
                    .respond(Response::from_string("").with_status_code(404))
                {
                    Ok(_) => {
                        eprintln!("INFO: Finished request");
                    }
                    Err(err) => {
                        eprintln!("ERROR: Could not respond {err}");
                    }
                };
            }
        }
    }
}

fn parse_definition(yaml: &Yaml) -> Definition {
    let paths = &yaml["paths"];
    let map = paths.as_hash().unwrap();
    let mut definition = Definition::default();

    for (path, path_description) in map.iter() {
        let path_map = path_description.as_hash().unwrap();
        for (method, method_description) in path_map.iter() {
            let parameters = &method_description["parameters"];
            match path.as_str() {
                Some(path) => match method.as_str() {
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
                },
                _ => {}
            }
        }
    }

    definition
}

fn parse_dir(path: &str) -> Definitions {
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

fn main() {
    let definitions_folder = "definitions";
    let address = "127.0.0.1:5000";
    let definitions = parse_dir(definitions_folder);
    serve(address, &definitions);
}
