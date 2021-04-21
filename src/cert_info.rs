use std::collections::HashMap;
use serde::Serialize;
use regex::Captures;
use crate::tools;


#[derive(Serialize)]
pub struct Certificate {
    pub title : String,
    pub versions : Versions,
    pub bibliography : HashMap<String, String>,
    pub table_of_contents: Vec<(String, String)>,
    pub revisions: Vec<(String, String)>


impl Certificate{
    pub fn new() -> Certificate{
        return Certificate{
            title: "".to_string(),
            versions: Versions {
                eal: vec![],
                global_platform: vec![],
                java_card: vec![],
                sha: vec![],
                rsa: vec![],
                ecc: vec![],
                des: vec![]
            },
            bibliography: HashMap::new(),
            table_of_contents: vec![],
            revisions: vec![]
        };
    }
}

#[derive(Serialize)]
pub struct Versions {
    pub eal : Vec<String>,
    pub global_platform : Vec<String>,
    pub java_card : Vec<String>,
    pub sha : Vec<String>,
    pub rsa : Vec<String>,
    pub ecc : Vec<String>,
    pub des : Vec<String>,
}

impl Versions{
    pub fn new()->Versions{
        return Versions{ 
            eal: vec![],
            global_platform: vec![],
            java_card: vec![],
            sha: vec![],
            rsa: vec![],
            ecc: vec![],
            des: vec![]
        };
    }
}


pub struct Revision {
    pub version: String,
    pub date: String,
    pub description: String,
}

impl Revision {
    //TODO prejmenovat
    pub fn new(capture: &Captures) -> Revision {
        let version = match capture.name("rev") {
            Some(_) => capture["rev"].to_string(),
            None => "".to_string()
        };
        let date = match capture.name("date") {
            Some(_) => tools::format_date(&capture["date"]),
            None => "".to_string()
        };
        let description = match capture.name("info") {
            Some(_) => tools::replace_whitespace_with_space(&capture["info"].to_string()),
            None => "".to_string()
        };
        Revision { version, date, description }
    }
}