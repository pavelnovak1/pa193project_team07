use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::cert_info::*;
use crate::versions::find_versions;
use crate::title::find_title;

pub(crate) fn extract_info(filename : &String) -> Certificate{
    let cert = std::fs::read_to_string(filename);
    let cert_text: String;
    match cert {
        Ok(txt) => cert_text = txt,
        Err(e) => {
            println!("Error reading file {}: {}", filename, e);
            panic!();
        },
    }
    //let mut title = String::new();
    let versions: Versions;
    //let mut bibliography: Vec<String> = vec![];

    versions = find_versions(&cert_text);

    let certificate = Certificate::new();
    certificate

}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
