use std::collections::{HashMap, HashSet};
use regex::Regex;

pub fn find_biblio(text : &String) -> HashMap<String, String>{
    let mut biblio = HashMap::<String, String>::new();
    let mut results = HashSet::<String>::new();
    let regexBiblio = Regex::new(r"\d{1,2}\.{0,1}\s+Bibliography\s+[^\.](?s:.)*").unwrap();
    let regexLiterature = Regex::new(r"\d{1,2}\.{0,1}\s+(Referenced){0,1}[ ]{0,1}Literature\s+[^\.](?s:.)*").unwrap();
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
        let regexCaptures = Regex::new(r"(\[[^\]]+\])\s+([[:alpha:]]([^\[\n]+\n)+)").unwrap();
        for name in regexEntry.find_iter(&i) {
            
            let caps = match regexCaptures.captures(name.as_str()) { 
                Some(T) => T,
                None => continue,
            };
            let name = remove_whitespaces(String::from(caps.get(1).unwrap().as_str()));
            let content = remove_whitespaces(String::from(caps.get(2).unwrap().as_str()));
            biblio.insert( name.to_string(), content.to_string());
        }
    }
    biblio
}

fn remove_whitespaces(text: String) -> String {
    let regexMultipleSpaces = Regex::new(r"\s+").unwrap();
    let regexDashNewLine = Regex::new(r"-\s+").unwrap();
    let mut t = regexDashNewLine.replace_all(&text, "-").to_string();
    t = regexMultipleSpaces.replace_all(&t, " ").to_string().trim().to_string();
    t
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

            for (k, v) in &result {
                assert!(json.contains(&format!("\"{}\": \"{}\"", k.as_str(), v.as_str()).to_string()), "Json file {} does not contain: \"{}\": \"{}\". {} files were successfully tested.", &p, k.as_str(), v.as_str(), i);
            }
            i = i+1;
        }
    }
}
 
