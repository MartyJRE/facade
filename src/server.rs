use tiny_http::{Response, Server};
use crate::definition::Definitions;

pub fn serve(address: &str, definitions: &Definitions) {
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

        let endpoint = definitions.find_endpoint(request.method(), request.url());
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
                eprintln!("WARN: Could not match {} {}", request.method(), request.url());
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