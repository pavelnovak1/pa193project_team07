use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::cert_info::Certificate;
use crate::versions::find_versions;

pub(crate) fn extract_info(filename : &String) -> Certificate{
    // let cert_text = fs::read_to_string(filename)
    //     .expect("Something went wrong reading the file");
    if let Ok(lines) = read_lines(filename) {
        // TODO
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                find_versions(ip);
            }
        }
        // let title = find_title(lines);
        // let versions = find_versions(lines);
        // let bibliography = find_biblio(lines);
    }
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
