use regex::Regex;
use crate::cert_info::LineOfContents;


const CHAPTER_MAX_CHAR: usize = 100;
const CHAPTER_MIN_CHAR: usize = 5;
const CERT_MAX_PAGE: i32 = 200;

pub fn find_table_of_content(text: &String) -> Vec<LineOfContents> {
    let mut res: Vec<LineOfContents> = Vec::new();

    let table_section_regex =
        Regex::new(r"(?m)(Table of Contents$|TABLE OF CONTENTS|Contents$|Content$|INDEX$|CONTENTS:$)\n([^\n]*\n){1,150}")
            .unwrap();
    // this regex works on base cases
    // (\d{1,2}(\.\d)*|[A-Z]\.|\d{1,2}.)\s*([A-Z](\w|\s|[“”\-\(\)\-:,/]|\w\.)*)(\s|\.)+(\d+)
    let simple_line_regex =
        Regex::new(r"(\d{1,2}(\.\d)*|[A-Z]\.|\d{1,2}.)\s*([A-Z](\w|\s|[“”\-\(\)\-:,/]|\w\.)*)(\s|\.)+(\d+)")
            .unwrap();
    let dots_regex = Regex::new(r"(\.|\.\s){10,}").unwrap();
    let no_dots_line_regex =
        Regex::new(r"(\d{1,2}(\.\d)*|[A-Z]\.|\d{1,2}.)\s*([A-Z]((\w|[“”\-\(\)\-:,/]|\w\.)+\s?)+)(\s|\.)+(\d+)")
            .unwrap();

    let mut table_section = find_section(text, table_section_regex);
    let mut section;

    // println!("Section head: {}", table_section);
    // println!("######### STOP ###########");
    // println!("##### Now content lines #####");

    if dots_regex.is_match(&table_section) {
        section = find_lines(&mut table_section, simple_line_regex.clone());
        parse_lines(&mut res, simple_line_regex, &mut section);
    } else {
        section = find_lines(&mut table_section, no_dots_line_regex.clone());
        parse_lines(&mut res, simple_line_regex, &mut section);
    }
    return res;
}

fn parse_lines(res: &mut Vec<LineOfContents>, regex: Regex, section: &mut Vec<String>) {
    let mut last_page: i32 = 0;
    for line in section.iter() {
        // println!("Content line: {}", line);
        let line_info: LineOfContents = extract_line_info(line, regex.clone(), last_page);
        if !line_info.title.is_empty() {
            res.push(line_info);
        }
        if res.len() > 0 {
            last_page = res[res.len() - 1].page;
        }
    }
}

fn find_section(text: &String, table_regex: regex::Regex) -> String {
    let mut text_clone = text.clone();

    erase_wrong_beginning(&mut text_clone);

    let mut head = table_regex.find(&text_clone).unwrap();

    let second_wrong_header_regex = Regex::new(r"Content\s+Manager").unwrap();
    if second_wrong_header_regex.is_match(head.as_str()) {
        let offset = second_wrong_header_regex.find(&*text_clone).unwrap().end();
        crop_letters(&mut text_clone, offset);
        head = table_regex.find(&text_clone).unwrap();
    }

    let third_wrong_header_regex = Regex::new(r"Default\sPersonalisation\sContent|Content\s+Tab").unwrap();
    while third_wrong_header_regex.is_match(head.as_str()) {
        let offset = third_wrong_header_regex.find(&*text_clone).unwrap().end();
        crop_letters(&mut text_clone, offset);
        head = table_regex.find(&text_clone).unwrap();
    }

    head.as_str().to_string()
}

fn erase_wrong_beginning(mut text_clone: &mut String) {
    let wrong_header_regex = Regex::new(r"Info(rmation)?\s+Content\s+Keywords").unwrap();
    if wrong_header_regex.is_match(&text_clone) {
        let offset = wrong_header_regex.find(&*text_clone).unwrap().end();
        crop_letters(&mut text_clone, offset);
    }
}

fn find_lines(text: &mut String, line_regex: regex::Regex) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    // use regex, because they can be different from lines in the .txt file
    let mut offset = 0;
    while offset < text.len() {
        if line_regex.is_match(text) {
            let line = line_regex.find(text).unwrap();
            offset = line.end();
            result.push(line.as_str().to_string());
            crop_letters(text, offset);
        }
        // sometimes the regex does not find anything by find (with find_iter does) and this helps
        else {
            offset += 1;
        }
    }
    result
}

// from here
// https://stackoverflow.com/questions/38447780/how-to-crop-characters-off-the-beginning-of-a-string-in-rust
fn crop_letters(s: &mut String, pos: usize) {
    match s.char_indices().nth(pos) {
        Some((pos, _)) => {
            s.drain(..pos);
        }
        None => {
            s.clear();
        }
    }
}

fn extract_line_info(line: &String, regex: regex::Regex, last_page: i32) -> LineOfContents {
    let mut result = LineOfContents::new();
    let caps = regex.captures(line).unwrap();

    let mut section_number = caps.get(1).unwrap().as_str().to_string();
    if section_number.chars().last().unwrap().eq(&'.') {
        section_number.pop();
    }

    let mut section_title = caps.get(3).unwrap().as_str().to_string();
    while section_title.chars().last().unwrap().eq(&' ') {
        section_title.pop();
    }
    //this is not safe, should be OK/Err options
    let page = caps.get(6).unwrap().as_str().parse::<i32>().unwrap();


    if section_title.len() > CHAPTER_MAX_CHAR || section_title.len() < CHAPTER_MIN_CHAR {
        return result;
    }
    // last_page > page ||
    if last_page > page || page > CERT_MAX_PAGE {
        return result;
    }

    // println!("Number: {}   Title: {}  chars: {} Page: {} Last page: {}",
    //          section_number, section_title, section_title.len(), page, last_page);

    result.section = section_number;
    result.title = section_title;
    result.page = page;

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simplest_case_test() {
        let simplest_case: String = String::from(
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