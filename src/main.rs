use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;

use tiny_http::{Response, Server};
use walkdir::WalkDir;
use yaml_rust::{Yaml, YamlLoader};

fn serve(address: &str) {
    eprintln!("Launching server: {address}");
    let server = Server::http(address).unwrap();

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


        match request.respond(Response::from_string("Test").with_status_code(200)) {
            Ok(_) => {
                eprintln!("INFO: Finished request");
            }
            Err(err) => {
                eprintln!("ERROR: Could not respond {err}");
            }
        };
    }
}

fn parse_definition(yaml: &Yaml) {
    let paths = &yaml["paths"];
    if paths.is_badvalue() {
        return;
    }
    let map = paths.as_hash().unwrap();

    for (path, path_description) in map.iter() {
        let path_map = path_description.as_hash().unwrap();
        for (method, method_description) in path_map.iter() {
            let parameters = &method_description["parameters"];
            todo!("not implemented yet");
        }
    }
}

fn parse_dir(path: &str) -> Result<(), Error> {
    for description in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
        let path = description.path();
        if path.is_dir() {
            continue;
        }
        eprintln!("INFO: Parsing {}", path.display());

        let data = load_file(path);

        for item in data {
            parse_definition(&item);
        }
    }

    Ok(())
}

fn load_file(path: &Path) -> Vec<Yaml> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    YamlLoader::load_from_str(&contents).unwrap()
}

fn main() {
    let a = parse_dir("definitions");
    let address = "127.0.0.1:5000";
    serve(address);
}
