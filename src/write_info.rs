use std::fs;

use regex::Regex;

use crate::Certificate;
use crate::config::Config;

pub fn write_parts_only(cert: &Certificate, filename: &str, conf: &Config) {
    println!("\n{}", filename);
    if conf.pretty_title {
        print!("title: ");
        println!("{}", serde_json::to_string_pretty(&cert.title).unwrap());
    }
    if conf.pretty_versions {
        print!("versions: ");
        println!("{}", serde_json::to_string_pretty(&cert.versions).unwrap());
    }
    if conf.pretty_biblio {
        print!("bibliography: ");
        println!("{}", serde_json::to_string_pretty(&cert.bibliography).unwrap());
    }
    if conf.pretty_revisions {
        print!("revisions: ");
        println!("{}", serde_json::to_string_pretty(&cert.revisions).unwrap());
    }
    if conf.pretty_content {
        print!("table of content: ");
        println!("{}", serde_json::to_string_pretty(&cert.table_of_contents).unwrap());
    }
}

pub fn write(cert: &Certificate, filename: &str) {
    let mut res = serde_json::to_string_pretty(cert).unwrap();
    let patterns = ["\n\"eal\": \\[\\],\n", "\"global_platform\": \\[\\],\n", "\"java_card\": \\[\\],\n", "\"sha\": \\[\\],\n", "\"rsa\": \\[\\],\n", "\"ecc\": \\[\\],\n", "\"des\": \\[\\],"];
    for pattern in &patterns {
        let r = Regex::new(pattern).unwrap();
        res = String::from(r.replace_all(&res, ""));
    }
    fs::write(filename, res);
}