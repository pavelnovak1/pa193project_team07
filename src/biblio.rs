use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn find_biblio(text : &String) -> HashMap<String, String>{
    let mut biblio = HashMap::<String, String>::new();
    let mut results = HashSet::<String>::new();
    let regexBiblio = Regex::new(r"\d{1,2}\.{0,1}\s+Bibliography\s+(?s:.)*").unwrap();
    let regexLiterature = Regex::new(r"\d{1,2}\.{0,1}\s+Literature\s+(?s:.)*").unwrap();
    let regexEntry = Regex::new(r"\[[^\]]+\]\s+([^\[\n]+\n)+").unwrap();

    if regexBiblio.is_match(&text) {
        results = regexBiblio.find_iter(&text)
            .map(|txt| (String::from(txt.as_str())).to_string()) 
            .collect();
    }

    else if regexLiterature.is_match(&text) {
        results = regexLiterature.find_iter(&text)
            .map(|txt| (String::from(txt.as_str())).to_string()) 
            .collect();

    }

    for i in results {
        let regexCaptures = Regex::new(r"(\[[^\]]+\])\s+(([^\[\n]+\n)+)").unwrap();
        for name in regexEntry.find_iter(&i) {
            let caps = regexCaptures.captures(name.as_str()).unwrap();
            biblio.insert(
                String::from(caps.get(1).unwrap().as_str()), 
                String::from(caps.get(2).unwrap().as_str())
            );
        }
    }
    biblio
}

