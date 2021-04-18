mod extract_info;
mod write_info;
mod cert_info;
mod versions;
mod title;
mod biblio;
mod revision;

use std::env;
use crate::extract_info::extract_info;

fn main() {
    println!("Arguments are");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);
    println!("{}", config.input_filename);

    //(pretty print) queries zatim nebudou

    let certificate = extract_info(&config.input_filename);
    //write_info(config.output_filename);
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


