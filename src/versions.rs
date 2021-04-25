use std::collections::HashSet;
use regex::Regex;
use crate::cert_info::Versions;
use crate::tools::replace_whitespace_with_space;
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
pub fn find_versions(text: &str) -> Versions {
    let mut result = Versions::new();

    result.eal = find_eal(&text);
    result.global_platform = find_gp(&text);
    result.java_card = find_java_card(&text);
    result.sha = find_sha(&text);
    result.rsa = find_rsa(&text);
    result.ecc = find_ecc(&text);
    result.des = find_des(&text);
    result
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
fn find(regex: regex::Regex, text: &str) -> Vec<String> {
    let results: HashSet<String> = regex
        .find_iter(&text)
        // .map(|m| (String::from(m.as_str())).trim().to_string())
        .map(|m| (replace_whitespace_with_space(m.as_str())).trim().to_string())
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
fn find_eal(text: &str) -> Vec<String> {
    let mut result = find(
        Regex::new(r"(^|\s|\()EAL\s?\d\s?\+?").unwrap(),
        &text,
    );
    for s in result.clone() {
        if s.starts_with("("){
            // from here https://stackoverflow.com/questions/26243025/remove-an-element-from-a-vector
            let index = result.iter().position(|x| x.starts_with("(")).unwrap();
            let new_s = s.strip_prefix("(").unwrap().to_string();
            result.remove(index);
            result.push(new_s);
        }
    }
    result.sort();
    result.dedup();
    return result
}

/// Wrapper - Return all occurences of global platform versions in text
///
/// # Arguments
///
/// * `text` - the text in which the regular expression will be searched = certificate
///
/// # Return
/// Vector of strings containing all unique global platform versions in certificate

fn find_gp(text: &str) -> Vec<String> {
    find(Regex::new(r"[Gg]lobal\s*[Pp]latform\s*\d(\.\d)*").unwrap(), &text)
}

fn find_java_card(text: &str) -> Vec<String> {
    find(Regex::new(r"[Jj]ava\s*[Cc]ard\s*\d(\.\d)*").unwrap(), &text)
}


fn find_sha(text: &str) -> Vec<String>{
    let mut result =
        // old regex r"(SHA|sha)(\s*|-|_)?\d(\d\d)?(/\d\d\d)?"
        // did not match the manually created templates but is actually more accurate
        find(Regex::new(r"(SHA|sha)(\s*|-|_)?\d(\d\d)?").unwrap(), &text);

    return result;
}

fn find_rsa(text: &str) -> Vec<String>{
    // old regex r"(RSA|rsa)(\s*|-|_)?(\d\d\d\d|CRT|SignaturePKCS1|PSS|SSA-PSS)(/\d\d\d\d)?"
    find(Regex::new(r"(RSA|rsa)(\s*|-|_)?(\d\d\d\d|SignaturePKCS1|PSS|SSA-PSS)").unwrap(), &text)
}

fn find_ecc(text: &str) -> Vec<String> {
    find(Regex::new(r"(ECC|ecc|ECDSA|ecdsa)(\s*|-|_)?\d\d\d\d?").unwrap(), &text)
}

fn find_des(text: &str) -> Vec<String> {
    find(Regex::new(r"(([Tt]riple|T|3|[Ss]ingle|[Ss]imple)(\s*|-|_)?(DES|des))|((des|DES)3)").unwrap(), &text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_eal_ok() {
        let right_eals = vec![
            "EAL1", "EAL 1", "EAL5+", "EAL 5+", "EAL5 +", "EAL 6 +", " EAL1 ", "EAL4+"
        ];

        for eal_ok in right_eals {
            assert!(
                find_eal(&eal_ok.to_string()).contains(&eal_ok.to_string().trim().to_string()),
                "Value {} was expected to be parsed but parsing failed!",
                eal_ok
            );
        }
    }

    #[test]
    fn find_eal_bad() {
        let wrong_eals = vec!["EAL", "EAL+", "eal 1"];

        for eal_nok in wrong_eals {
            assert_eq!(
                find_eal(&eal_nok.to_string()).len() == 0,
                "Value {} was not expected to be successfully parsed but it was!",
                eal_nok
            );
        }
    }

    #[test]
    fn find_gp_ok() {
        let right_gps = vec!["GlobalPlatform 2.2.1", "GlobalPlatform 2.3"];


        for gp_ok in right_gps {
            assert!(find_gp(&gp_ok.to_string()).contains(&gp_ok.to_string().trim().to_string()),
                    "Value {} was expected to be parsed but parsing failed!",
                    gp_ok);
        }
    }

    #[test]
    fn find_java_card_ok() {
        let right_java_cards = vec!["Java Card 3.0.4", "Java Card 3", "Java Card 3.0.5"];


        for java_card_ok in right_java_cards {
            assert!(find_java_card(&java_card_ok.to_string()).contains(&java_card_ok.to_string().trim().to_string()),
                    "Value {} was expected to be parsed but parsing failed!",
                    java_card_ok);
        }
    }

    #[test]
    fn find_sha_ok() {
        let right_shas = vec!["SHA-256", "SHA-1", "SHA224", "SHA1",
                              "SHA-3/224", "SHA-3/256", "SHA-3/384", "SHA-3/512", "SHA-3",
                              "SHA_224", "SHA_256", "SHA_384", "SHA_512"];


        for sha_ok in right_shas {
            assert!(find_sha(&sha_ok.to_string()).contains(&sha_ok.to_string().trim().to_string()),
                    "Value {} was expected to be parsed but parsing failed!",
                    sha_ok);
        }
    }

    #[test]
    fn find_rsa_ok() {
        let right_rsas = vec!["RSA2048/4096", "RSA2048", "RSA2048", "RSA 2048", "RSA 4096",
                              "RSA 1024", "RSA_1024", "RSA-CRT", "RSASignaturePKCS1", "RSASSA-PSS"];


        for rsa_ok in right_rsas {
            assert!(find_rsa(&rsa_ok.to_string()).contains(&rsa_ok.to_string().trim().to_string()),
                    "Value {} was expected to be parsed but parsing failed!",
                    rsa_ok);
        }
    }

    #[test]
    fn find_ecc_ok() {
        let right_eccs = vec!["ECC 224", "ECC 256"];


        for ecc_ok in right_eccs {
            assert!(find_ecc(&ecc_ok.to_string()).contains(&ecc_ok.to_string().trim().to_string()),
                    "Value {} was expected to be parsed but parsing failed!",
                    ecc_ok);
        }
    }

    #[test]
    fn find_des_ok() {
        let right_deses = vec!["Triple-DES", "TDES", "Triple DES", "single-des", "3DES",
                               "TripleDES", "DES3", "triple-DES"];


        for des_ok in right_deses {
            assert!(find_des(&des_ok.to_string()).contains(&des_ok.to_string().trim().to_string()),
                    "Value {} was expected to be parsed but parsing failed!",
                    des_ok);
        }
    }

    #[test]
    fn find_versions_test() {
        let right_eals = vec!["EAL1", "EAL 1", "EAL5+", "EAL 5+", "EAL5 +", "EAL 6 +"];
        let eals = "EAL1, EAL 1, EAL5+, EAL 5+, EAL5 +, EAL 6 +";
        let result = find_versions(&eals.to_string());

        for eal in right_eals {
            assert!(result.eal.contains(&eal.to_string()), "{} is missing in the result!", eal);
        }
    }
}
