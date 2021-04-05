use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::cert_info::*;
use crate::versions::find_versions;
use crate::title::find_title;
use crate::biblio::find_biblio;

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

    let versions = find_versions(&cert_text);
    let biblio = find_biblio(&cert_text);
    let certificate = Certificate::new();
    certificate
}
