use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::biblio::{find_biblio, remove_page_ends};
use crate::cert_info::*;
use crate::revision::find_revision;
use crate::title::find_title;
use crate::table_of_contents::find_table_of_content;
use crate::versions::find_versions;

pub(crate) fn extract_info(filename: &str, conf: &config::Config) -> Certificate {
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
    if !conf.pretty || conf.pretty_title {
        certificate.title = find_title(&cert_text);
    }
    if !conf.pretty || conf.pretty_versions {
        certificate.versions = find_versions(&cert_text);
    }
    if !conf.pretty || conf.pretty_biblio {
        certificate.bibliography = find_biblio(&cert_text);
    }
    if !conf.pretty || conf.pretty_revisions {
        certificate.revisions = find_revision(&cert_text);
    }
    if !conf.pretty || conf.pretty_content {
        certificate.table_of_contents = find_table_of_content(&cert_text);
    }
    certificate
}