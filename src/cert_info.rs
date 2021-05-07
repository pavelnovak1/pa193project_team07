use std::collections::HashMap;

use regex::Captures;
use serde::{Serialize, Serializer};
use serde::ser::SerializeSeq;

use crate::tools;

#[derive(Serialize)]
pub struct Certificate {
    pub title: String,
    pub versions: Versions,
    pub bibliography: HashMap<String, String>,
    pub table_of_contents: Vec<LineOfContents>,
    pub revisions: Vec<Revision>,
}

impl Certificate {
    pub fn new() -> Certificate {
        Certificate {
            title: "".to_string(),
            versions: Versions::new(),
            bibliography: HashMap::new(),
            table_of_contents: vec![],
            revisions: vec![],
        }
    }
}

#[derive(Serialize)]
pub struct Versions {
    pub eal: Vec<String>,
    pub global_platform: Vec<String>,
    pub java_card: Vec<String>,
    pub sha: Vec<String>,
    pub rsa: Vec<String>,
    pub ecc: Vec<String>,
    pub des: Vec<String>,
}

impl Versions {
    pub fn new() -> Versions {
        Versions {
            eal: vec![],
            global_platform: vec![],
            java_card: vec![],
            sha: vec![],
            rsa: vec![],
            ecc: vec![],
            des: vec![],
        }
    }
}


pub struct LineOfContents {
    pub section: String,
    pub title: String,
    pub page: i32,
}

impl LineOfContents {
    pub fn new() -> LineOfContents {
        LineOfContents {
            section: String::new(),
            title: String::new(),
            page: 0,
        }
    }
}

impl Serialize for LineOfContents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut seq = serializer.serialize_seq(Option::from(3_usize))?;
        seq.serialize_element(&self.section)?;
        seq.serialize_element(&self.title)?;
        seq.serialize_element(&self.page)?;
        seq.end()
    }
}

#[derive(Serialize)]
pub struct Revision {
    pub version: String,
    pub date: String,
    pub description: String,
}

impl Revision {
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