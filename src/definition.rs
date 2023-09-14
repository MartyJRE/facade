use regex::Regex;
use tiny_http::Method;

#[derive(Debug)]
pub struct Parameter {
    pub parameter_in: String,
    pub name: String,
}

impl Parameter {
    fn new(name: &str, parameter_in: &str) -> Parameter {
        Parameter {
            name: name.to_string(),
            parameter_in: parameter_in.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Endpoint {
    pub method: Method,
    pub path: String,
    pub parameters: Vec<Parameter>,
}

impl Endpoint {
    pub fn new(method: Method, path: &str) -> Endpoint {
        Endpoint {
            method,
            path: path.to_string(),
            parameters: vec![],
        }
    }

    pub fn regex(&mut self) -> Regex {
        for parameter in self.parameters.iter().filter(|&item| item.parameter_in == "path") {
            self.path = self.path.replace(format!("{{{}}}", parameter.name).as_str(), r"[\w]+");
        }
        Regex::new(format!("^{}$", self.path).as_str()).expect("Could not parse regex.")
    }


    pub fn add_parameter(&mut self, name: &str, parameter_in: &str) {
        self.parameters.push(Parameter::new(name, parameter_in));
    }
}

#[derive(Default, Debug)]
pub struct Definition {
    endpoints: Vec<Endpoint>,
}

impl Definition {
    pub fn add_endpoint(&mut self, endpoint: Endpoint) {
        self.endpoints.push(endpoint);
    }

    pub fn find_endpoint(&self, method: Method, path: &str) -> Option<&Endpoint> {
        self.endpoints.iter().find(|&endpoint| endpoint.method == method && endpoint.regex().is_match(path))
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