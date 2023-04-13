use tiny_http::Method;

#[derive(Debug)]
pub struct Endpoint {
    pub method: Method,
    pub path: String,
}

impl Endpoint {
    fn new(method: Method, path: &str) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
        }
    }

    fn parsed_path(&self) -> &str {
        return self.path.as_str();
    }
}

#[derive(Default, Debug)]
pub struct Definition {
    endpoints: Vec<Endpoint>,
}

impl Definition {
    pub fn add_endpoint(&mut self, method: Method, path: &str) {
        self.endpoints.push(Endpoint::new(method, path));
    }

    pub fn find_endpoint(&self, method: Method, path: &str) -> Option<&Endpoint> {
        self.endpoints.iter().find(|&endpoint| endpoint.method == method && endpoint.parsed_path() == path)
    }
}

#[derive(Debug)]
pub struct Definitions {
    pub definitions: Vec<Definition>,
}

impl Definitions {
    pub fn find_endpoint(&self, method: &Method, path: &str) -> Option<&Endpoint> {
        let mut endpoint: Option<&Endpoint> = None;
        for definition in &self.definitions {
            if let Some(definition) = definition.find_endpoint(method.clone(), path) {
                endpoint = Some(definition);
                break;
            }
        }
        endpoint
    }
}