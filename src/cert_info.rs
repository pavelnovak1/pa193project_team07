use std::collections::HashMap;

pub(crate) struct Certificate {    
    title : String,
    versions : Versions,
    bibliography : Vec<(String, String)>,
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
            bibliography: vec![]
        };
    }
}

pub struct Versions {
    pub eal : Vec<String>,
    pub global_platform : Vec<String>,
    pub java_card : Vec<String>,
    pub sha : Vec<i32>,
    pub rsa : Vec<i32>,
    pub ecc : Vec<i32>,
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
