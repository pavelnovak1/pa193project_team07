use regex::Regex;

pub fn find_versions(text : String){
    find_eal(text);
    // find_gp();
    // find_java_card();
    // find_sha();
    // find_rsa();
    // find_ecc();
    // find_des;
}

fn find_eal(text : String){
    let re = Regex::new(r"EAL+\d+[+]*");
    for mat in re.unwrap().find_iter(&text)
    {
        println!("{:?}", mat.as_str());
    }

}