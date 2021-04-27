use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn find_biblio(text: &str) -> HashMap<String, String> {
    let mut bibliography_result = HashMap::<String, String>::new();
    let mut bibliography_section = HashSet::<String>::new();
    let regex_biblio = Regex::new(r"\d{1,2}\.{0,1}\s+Bibliography[^\.\d\s]*(?s:.)*").unwrap();
    let regex_literature = Regex::new(r"\d{1,2}\.{0,1}\s+(Referenced){0,1}[ ]{0,1}Literature[^\.\d\s]*(?s:.)*").unwrap();
    let regex_references = Regex::new(r"\d{0,2}\.{0,1}\s+References[^\.\d\s]*(?s:.)*").unwrap();
    let regex_referenced_lit = Regex::new(r"\d{0,2}\.{0,1}\s+REFERENCE DOCUMENTS[^\.\d\s]*(?s:.)*").unwrap();
    let regex_biblio_entry = Regex::new(r"(?m)(^|\s)\[[^\]]+\]\s+([^\[\n]+\n)+").unwrap();

    if regex_biblio.is_match(&text) {
        bibliography_section = regex_biblio.find_iter(&text)
            .map(|txt| (String::from(txt.as_str())))
            .collect();
    } else if regex_literature.is_match(&text) {
        bibliography_section = regex_literature.find_iter(&text)
            .map(|txt| (String::from(txt.as_str())).to_string())
            .collect();
    } else if regex_references.is_match(&text) {
        bibliography_section = regex_references.find_iter(&text)
            .map(|txt| (String::from(txt.as_str())).to_string())
            .collect();
    } else if regex_referenced_lit.is_match(&text) {
        bibliography_section = regex_referenced_lit.find_iter(&text)
            .map(|txt| (String::from(txt.as_str())))
            .collect();
    }
    for i in bibliography_section {
        let text_without_padding = remove_page_ends(&i);
        let regex_entry_cap = Regex::new(r"(\[[A-Z\d][^\]]{0,20}\])\s{1,2}(([^a-z][^\[\n]+\n)+)").unwrap();
        for entry in regex_biblio_entry.find_iter(&text_without_padding) {

            // Remove some elements that are not part of the bibliography
            if String::from(entry.as_str()).contains("..") || String::from(entry.as_str()).contains('|') {
                continue;
            }

            let caps = match regex_entry_cap.captures(entry.as_str()) {
                Some(t) => t,
                None => continue,
            };
            let name = remove_whitespaces(String::from(caps.get(1).unwrap().as_str()));
            let content = remove_whitespaces(String::from(caps.get(2).unwrap().as_str()));
            if !name.to_string().is_empty() {
                bibliography_result.insert(name.to_string(), content.to_string());
            }
        }
    }
    bibliography_result
}

fn remove_whitespaces(text: String) -> String {
    let regex_mul_spaces = Regex::new(r"\s+").unwrap();
    let regex_dash_nl = Regex::new(r"-\n\s+").unwrap();
    let t = regex_dash_nl.replace_all(&text, "-").to_string();
    regex_mul_spaces.replace_all(&t, " ").to_string().trim().to_string()
}

pub fn remove_page_ends(text: &str) -> String {
    let line_end = Regex::new(r"(([ ]*\n)*(.*\n){2}[\f])").unwrap();
    String::from(line_end.replace_all(&text, "\n\n\n"))
}