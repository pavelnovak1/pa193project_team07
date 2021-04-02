use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn find_biblio(text : &String) -> HashMap<String, String>{
    let mut biblio = HashMap::<String, String>::new();
    let mut results = HashSet::<String>::new();
    let regexBiblio = Regex::new(r"\d{1,2}\.{0,1}\s+Bibliography\s+[^\.\n](?s:.)*").unwrap();
    let regexLiterature = Regex::new(r"\d{1,2}\.{0,1}\s+Literature\s+[^\.](?s:.)*").unwrap();
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
            let name = remove_whitespaces(String::from(caps.get(1).unwrap().as_str()));
            let content = remove_whitespaces(String::from(caps.get(2).unwrap().as_str()));
            println!("{} {}\n", name.to_string(), content.to_string());
            biblio.insert( name.to_string(), content.to_string());
        }
    }
    biblio
}

fn remove_whitespaces(text: String) -> String {
    let re = Regex::new(r"\s+").unwrap();
    let t = re.replace_all(&text, " ").to_string();
    t
}

