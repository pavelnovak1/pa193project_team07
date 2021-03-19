struct Certificate {
    title : String,
    versions : Versions,
    bibliography : Vec<Vec<String>>,
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