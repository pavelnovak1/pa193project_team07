mod extract_info;
mod write_info;
mod cert_info;
mod versions;
mod title;
mod biblio;
mod revision;
mod tools;

use std::env;
use crate::extract_info::extract_info;
use crate::write_info::write;
use crate::cert_info::Certificate;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    //(pretty print) queries zatim nebudou

    let certificate = extract_info(&config.input_filename);
    write(&certificate, &config.output_filename);
}
struct Config {
    queries : Vec<String>,
    input_filename : String,
    output_filename : String,
}
impl Config {
    fn new(args: &[String]) -> Config{
        let input_filename = args.last().unwrap().clone();
        let n = args.len();
        let queries = args[0..(n-2)].to_vec(); //toto se mozna v Rustu nema delat
        let output_filename = args[n-2].clone();

        Config { queries, input_filename, output_filename }
    }
}


