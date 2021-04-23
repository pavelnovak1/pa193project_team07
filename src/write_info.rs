use crate::Certificate;
// use serde_json::Result;
use regex::Regex;
use std::fs;

pub fn write(cert: &Certificate, filename: &String){
    let mut res = serde_json::to_string_pretty(cert).unwrap();
    let patterns = ["\n\"eal\": \\[\\],\n", "\"global_platform\": \\[\\],\n", "\"java_card\": \\[\\],\n", "\"sha\": \\[\\],\n", "\"rsa\": \\[\\],\n", "\"ecc\": \\[\\],\n", "\"des\": \\[\\],"];
    for pattern in &patterns {
        let r = Regex::new(pattern).unwrap();
        res = String::from(r.replace_all(&res, ""));
    }
    fs::write(filename, res);
}