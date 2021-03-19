pub(crate) struct Certificate {
    title : String,
    versions : Versions,
    bibliography : Vec<Vec<String>>,
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

struct Versions {
    eal : Vec<String>,
    global_platform : Vec<String>,
    java_card : Vec<String>,
    sha : Vec<i32>,
    rsa : Vec<i32>,
    ecc : Vec<i32>,
    des : Vec<String>,
}

impl Versions{
    fn new()->Versions{
        return Versions{ eal: vec![],
            global_platform: vec![],
            java_card: vec![],
            sha: vec![],
            rsa: vec![],
            ecc: vec![],
            des: vec![]
        };
    }
}