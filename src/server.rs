use tiny_http::{Response, Server};

use crate::definition::Definitions;

pub fn serve(address: &str, definitions: &Definitions) {
    println!("INFO: Launching server: {address}");
    let server = Server::http(address).expect("Server could not start.");

    println!("INFO: {:?}", definitions);

    loop {
        let request = match server.recv() {
            Ok(req) => {
                println!("INFO: Received a request {} {}", req.method(), req.url());
                req
            }
            Err(err) => {
                eprintln!("ERROR: Could not process request {err}");
                continue;
            }
        };

        let endpoint = definitions.find_endpoint(request.method(), request.url());
        match endpoint {
            Some(endpoint) => {
                println!("INFO: Matched {:?} {}", endpoint.method, endpoint.path);
                match request.respond(Response::from_string("OK").with_status_code(200)) {
                    Ok(_) => {
                        println!("INFO: Finished request");
                    }
                    Err(err) => {
                        eprintln!("ERROR: Could not respond {err}");
                    }
                };
            }
            _ => {
                eprintln!("WARN: Could not match {} {}", request.method(), request.url());
                match request
                    .respond(Response::from_string("").with_status_code(404))
                {
                    Ok(_) => {
                        println!("INFO: Finished request");
                    }
                    Err(err) => {
                        eprintln!("ERROR: Could not respond {err}");
                    }
                };
            }
        }
    }
}