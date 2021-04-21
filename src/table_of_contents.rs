use regex::Regex;
use std::collections::HashSet;
use crate::cert_info::LineOfContents;

pub fn find_table_of_content(text : &String)->Vec<LineOfContents>{
    let mut res : Vec<LineOfContents> = Vec::new();

    let mut section = find_lines(text);
    for line in section.iter() {
        println!("Content line: {}", line);
        // line = remove_whitespaces(line);
        // line_info : LineOfContents = extract_line_info(line);
        // res.push(line_info);
    }
    return res;
}

fn find_lines(text : &String)->Vec<String>{
    let basic_line_regex = Regex::new(r"(\d(\.\d)*|[A-Z]\.|\d+.)\s*(\w|\s|[“”\-\(\)\-:,/]|\w\.)*(\s|\.)+\d+")
                            .unwrap();
    let results: HashSet<String> = basic_line_regex
        .find_iter(&text)
        .map(|line| (String::from(line.as_str())).trim().to_string())
        .collect();
    results.into_iter().collect()
}

// copy od Pavla
// fn remove_whitespaces(line : String)->String{
//     let regex_mul_spaces = Regex::new(r"\s+").unwrap();
//     let regex_dash_nl = Regex::new(r"-\n\s+").unwrap();
//     let mut t = regex_dash_nl.replace_all(&text, "-").to_string();
//     regex_mul_spaces.replace_all(&t, " ").to_string().trim().to_string()
// }

// fn extract_line_info(line : String)->LineOfContents{
//
// }



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest_case_test(){
    let simplest_case : String = String::from(
    "Contents
A. Certification......................................................................................................................6
   1.   Preliminary Remarks....................................................................................................6
   2.   Specifications of the Certification Procedure...............................................................6
   3.   Recognition Agreements..............................................................................................7
   4.   Performance of Evaluation and Certification................................................................8
   5.   Validity of the Certification Result.................................................................................8
   6.   Publication....................................................................................................................9
B. Certification Results.......................................................................................................10
   1. Executive Summary....................................................................................................11
   2. Identification of the TOE.............................................................................................12
   3. Security Policy............................................................................................................15
   4. Assumptions and Clarification of Scope.....................................................................15
   5. Architectural Information.............................................................................................16
   6. Documentation...........................................................................................................16
   7. IT Product Testing.......................................................................................................16
   8. Evaluated Configuration.............................................................................................17
   9. Results of the Evaluation............................................................................................18
   10. Obligations and Notes for the Usage of the TOE.....................................................19
   11. Security Target..........................................................................................................20
   12. Regulation specific aspects (eIDAS, QES)..............................................................20
   13. Definitions.................................................................................................................20
   14. Bibliography..............................................................................................................23
C. Excerpts from the Criteria..............................................................................................26

D. Annexes.........................................................................................................................27"
    );
    find_table_of_content(&simplest_case);
}

}