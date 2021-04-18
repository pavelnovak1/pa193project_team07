use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn find_biblio(text : &String) -> Vec<(String, String)>{
    let mut bibliography_result = HashMap::<String, String>::new();
    let mut bibliography_section = HashSet::<String>::new();
    let regex_biblio = Regex::new(r"\d{1,2}\.{0,1}\s+Bibliography\s*[^\.\d\s](?s:.)*").unwrap();
    let regex_literature = Regex::new(r"\d{1,2}\.{0,1}\s+(Referenced){0,1}[ ]{0,1}Literature\s+[^\.](?s:.)*").unwrap();
    let regex_biblio_entry = Regex::new(r"(?m)(^|\s)\[[^\]]+\]\s+([^\[\n]+\n)+").unwrap();

    if regex_biblio_entry.is_match(&text) {
        bibliography_section = regex_biblio.find_iter(&text)
            .map(|txt| (String::from(txt.as_str())).to_string()) 
            .collect();
    }

    else if regex_literature.is_match(&text) {
        bibliography_section = regex_literature.find_iter(&text)
            .map(|txt| (String::from(txt.as_str())).to_string()) 
            .collect();
    }

    for i in bibliography_section {
        let regex_entry_cap = Regex::new(r"(\[[^\]]+\])\s+(([^\[\n]+\n)+)").unwrap();
        for entry in regex_biblio_entry.find_iter(&i) {

            // Remove some elements that are not part of the bibliography
            if String::from(entry.as_str()).contains("..") || String::from(entry.as_str()).contains("|") {
                continue;
            }
            
            let caps = match regex_entry_cap.captures(entry.as_str()) { 
                Some(T) => T,
                None => continue,
            };
            let name = remove_whitespaces(String::from(caps.get(1).unwrap().as_str()));
            let content = remove_whitespaces(String::from(caps.get(2).unwrap().as_str()));
            if !name.to_string().is_empty() {
                bibliography_result.insert( name.to_string(), content.to_string());
            }
        }
    }
    bibliography_result.into_iter().collect()
}

fn remove_whitespaces(text: String) -> String {
    let regex_mul_spaces = Regex::new(r"\s+").unwrap();
    let regex_dash_nl = Regex::new(r"-\n\s+").unwrap();
    let mut t = regex_dash_nl.replace_all(&text, "-").to_string();
    regex_mul_spaces.replace_all(&t, " ").to_string().trim().to_string()
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::fs;

    #[test]
    fn check_test_dataset(){
        let paths = fs::read_dir("test_dataset/").unwrap();
        let mut i = 0;

        let filenames = paths.filter_map(|entry| {
            entry.ok().and_then(|e|
            e.path().file_name()
            .and_then(|n| n.to_str().map(|s| String::from(s)))
            )
            }).collect::<Vec<String>>();
        for path in filenames {
            let mut p = path;
            p.insert_str(0,"test_dataset/");

            if p.contains(".json"){
                p = p.replace("json", "txt");
            }
            
            let txt = std::fs::read_to_string(&p).unwrap();
            p = p.replace("txt", "json");
            let json = std::fs::read_to_string(&p).unwrap();

            let result = find_biblio(&txt);
            println!("Problems in {} file", &p);

            for (k, v) in &result {
                if(!json.contains(&format!("\"{}\": \"{}\"", k.as_str(), v.as_str()).to_string())){
                    println!("\"{}\": \"{}\"", k.as_str(), v.as_str());
                }
            }
            i = i+1;
        }
        panic!("");
    }
}
 
