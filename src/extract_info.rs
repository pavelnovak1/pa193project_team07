use std::process;

use crate::biblio::find_biblio;
use crate::cert_info::*;
use crate::revision::find_revision;
use crate::title::find_title;
use crate::table_of_contents::find_table_of_content;
use crate::versions::find_versions;

pub(crate) fn extract_info(filename: &String) -> Certificate {
    let cert = std::fs::read_to_string(filename);
    let cert_text: String;
    match cert {
        Ok(txt) => cert_text = txt,
        Err(e) => {
            println!("Error reading file {}: {}", filename, e);
            process::exit(1);          
        }
    }
    let mut certificate = Certificate::new();
    certificate.title = find_title(&cert_text);
    certificate.versions = find_versions(&cert_text);
    certificate.bibliography = find_biblio(&cert_text);
    certificate.revisions = find_revision(&cert_text);
    certificate.table_of_contents = find_table_of_content(&cert_text);
    certificate
}