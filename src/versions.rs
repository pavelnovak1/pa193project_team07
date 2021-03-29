use regex::Regex;
use std::collections::HashSet;
use crate::cert_info::Versions;

/// Main function used to extract version information from given text.
/// This function return a `Version` struct containing all extracted information
///
/// # Arguments
///
/// * `text` - Whole text of certificate in text form
///
/// # Return
/// 
/// `Version` struct, filled with all version information extracted from certificate
///
pub fn find_versions(text : &String) -> Versions{

    let mut result = Versions::new();

    result.eal = find_eal(&text);
    result.global_platform = find_gp(&text);
    result.java_card = find_java_card(&text);
    result.sha = find_sha(&text);
    result.rsa = find_rsa(&text);
    result.ecc = find_ecc(&text);
    result.des = find_des(&text);
}

/// Returns vector of strings containing all _unique_ and both sides trimmed pieces of text that fits to given regular expression in
/// the text. 
///
/// # Arguments
///
/// * `regex` - arbitrary regular expression
/// * `text` - the text in which the regular expression will be searched
///
/// # Return
///
/// Vector of strings containing all unique and trimmed occurences of regular expression in text
///
fn find(regex: regex::Regex, text: &String ) -> Vec<String> {
    let results: HashSet<String> = regex.find_iter(&text)
        .map(|eal| (String::from(eal.as_str())).trim().to_string())
        .collect();
    results.into_iter().collect()
}

/// Wrapper - Return all occurences of EAL versions in text
///
/// # Arguments
///
/// * `text` - the text in which the regular expression will be searched = certificate
///
/// # Return 
/// Vector of strings containing all unique EAL versions in certificate
fn find_eal(text : &String) -> Vec<String>{
    find(Regex::new(r"(^|\s)EAL\s{0,1}\d{1}\s{0,1}\+{0,1}").unwrap(), &text)
}

/// Wrapper - Return all occurences of global platform versions in text
///
/// # Arguments
///
/// * `text` - the text in which the regular expression will be searched = certificate
///
/// # Return 
/// Vector of strings containing all unique global platform versions in certificate
fn find_gp(text: &String) -> Vec<String>{
    find(Regex::new(r"[Gg]lobal\s*[Pp]latform\s*\d(\.\d)*").unwrap(), &text)
}

fn find_java_card(text: &String) -> Vec<String>{
    find(Regex::new(r"[Jj]ava\s*[Cc]ard\s*\d(\.\d)*").unwrap(), &text)
}

fn find_sha(text: &String) -> Vec<String>{
    find(Regex::new(r"(SHA|sha)(\s*|-|_)?\d\d?\d?(\\\d\d\d)?").unwrap(), &text)
}

fn find_rsa(text: &String) -> Vec<String>{
    find(Regex::new(r"(RSA|rsa)(\s*|-|_)?(\d\d\d\d|CRT|SignaturePKCS1|PSS)").unwrap(), &text)
}

fn find_ecc(text: &String) -> Vec<String>{
    find(Regex::new(r"(ECC|ecc|ECDSA|ecdsa)(\s*|-|_)?\d\d\d\d?").unwrap(), &text)
}

fn find_des(text: &String) -> Vec<String>{
    find(Regex::new(r"(([Tt]riple|T|3|[Ss]ingle|[Ss]imple)(\s*|-|_)?(DES|des))|((des|DES)3)").unwrap(), &text)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_eal_ok(){
        let right_eals = vec!["EAL1", "EAL 1", "EAL5+", "EAL 5+", "EAL5 +", 
                                "EAL 6 +", " EAL1 "];
        

        for eal_ok in right_eals {

            assert!(find_eal(&eal_ok.to_string()).contains(&eal_ok.to_string().trim().to_string()),
                        "Value {} was expected to be parsed but parsing failed!",
                        eal_ok);
        }
    }

    #[test]
    fn find_eal_bad(){

        let wrong_eals = vec!["EAL", "EAL+", "eal 1"];

        for eal_nok in wrong_eals {
            assert!(find_eal(&eal_nok.to_string()).len()==0,
                    "Value {} was not expected to be successfully parsed but it was!", 
                    eal_nok);
        }
    }

    #[test]
    fn find_versions_test(){
         let right_eals = vec!["EAL1", "EAL 1", "EAL5+", "EAL 5+", "EAL5 +", 
                                "EAL 6 +"];
         let eals = "EAL1, EAL 1, EAL5+, EAL 5+, EAL5 +, EAL 6 +";
         let result = find_versions(&eals.to_string());

         for eal in right_eals {
             assert!(result.eal.contains(&eal.to_string()), "{} is missing in the result!", eal);
         }
    }
}
