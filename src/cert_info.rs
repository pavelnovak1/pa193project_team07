use std::collections::HashMap;

pub(crate) struct Certificate {    
    title : String,
    versions : Versions,
    bibliography : Vec<(String, String)>,
    table_of_contents : Vec<LineOfContents>
}

impl Certificate{
    pub(crate) fn new() -> Certificate{
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
            bibliography: vec![],
            table_of_contents : vec![]
        };
    }
}

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

pub struct LineOfContents{
    pub section : String,
    pub title : String,
    pub page : i32
}

impl LineOfContents{
    pub fn new()->LineOfContents{
        return LineOfContents {
            section: String::new(),
            title: String::new(),
            page: 0
        }
    }
}
