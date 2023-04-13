mod server;
mod definition;
mod parser;

use crate::parser::parse_dir;
use crate::server::serve;

fn main() {
    let definitions_folder = "definitions";
    let address = "127.0.0.1:5000";
    let definitions = parse_dir(definitions_folder);
    serve(address, &definitions);
}
