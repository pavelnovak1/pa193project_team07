use regex::Regex;
use crate::cert_info::LineOfContents;


const CHAPTER_MAX_CHAR: usize = 100;
const CHAPTER_MIN_CHAR: usize = 5;
const CERT_MAX_PAGE: i32 = 200;

pub fn find_table_of_content(text: &String) -> Vec<LineOfContents> {
    let mut res: Vec<LineOfContents> = Vec::new();

    let mut table_section = find_section(text);

    let simple_line_regex =
        Regex::new(r"(?P<section>\d{1,2}(\.\d)*|[A-Z]\.|\d{1,2}.)\s*(?P<title>[A-Z](\w|\s|[“”\-\(\)\-:,/]|\w\.)*)(\s|\.)+(?P<page>\d+)")
            .unwrap();
    let dots_regex = Regex::new(r"(\.|\.\s){10,}").unwrap();
    let no_dots_line_regex =
        Regex::new(r"(?P<section>\d{1,2}(\.\d)*|[A-Z]\.|\d{1,2}.)\s*(?P<title>[A-Z]((\w|[“”\-\(\)\-:,/]|\w\.)+\s?)+)(\s|\.)+(?P<page>\d+)")
            .unwrap();

    let mut section;

    // println!("Section head: {}", table_section);
    // println!("######### STOP ###########");
    // println!("##### Now content lines #####");

    if dots_regex.is_match(&table_section) {
        section = find_lines(&mut table_section, simple_line_regex.clone());
        parse_lines(&mut res, simple_line_regex, &mut section);
    } else {
        section = find_lines(&mut table_section, no_dots_line_regex);
        parse_lines(&mut res, simple_line_regex, &mut section);
    }
    res
}

fn parse_lines(res: &mut Vec<LineOfContents>, regex: Regex, section: &mut Vec<String>) {
    let mut last_page: i32 = 0;
    for line in section.iter() {
        // println!("Content line: {}", line);
        let line_info: LineOfContents = extract_line_info(line, regex.clone(), last_page);
        if !line_info.title.is_empty() {
            res.push(line_info);
        }
        if !res.is_empty() {
            last_page = res[res.len() - 1].page;
        }
    }
}

fn find_section(text: &String) -> String {
    let mut text_clone = text.clone();
    let table_section_regex =
        Regex::new(r"(?m)(Table of Contents$|TABLE OF CONTENTS|Contents$|Content$|INDEX$|CONTENTS:$)\n([^\n]*\n){1,150}")
            .unwrap();

    erase_wrong_beginning(&mut text_clone);

    let mut head = table_section_regex.find(&text_clone).unwrap();

    let second_wrong_header_regex = Regex::new(r"Content\s+Manager").unwrap();
    if second_wrong_header_regex.is_match(head.as_str()) {
        let offset = second_wrong_header_regex.find(&*text_clone).unwrap().end();
        crop_letters(&mut text_clone, offset);
        head = table_section_regex.find(&text_clone).unwrap();
    }

    let third_wrong_header_regex = Regex::new(r"Default\sPersonalisation\sContent|Content\s+Tab").unwrap();
    while third_wrong_header_regex.is_match(head.as_str()) {
        let offset = third_wrong_header_regex.find(&*text_clone).unwrap().end();
        crop_letters(&mut text_clone, offset);
        head = table_section_regex.find(&text_clone).unwrap();
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

    let mut section_number : String = caps.name("section").unwrap().as_str().to_string();
    if section_number.chars().last().unwrap().eq(&'.') {
        section_number.pop();
    }

    let mut section_title : String = caps.name("title").unwrap().as_str().to_string();
    while section_title.chars().last().unwrap().eq(&' ') {
        section_title.pop();
    }
    //this is not safe, should be OK/Err options
    let page : i32 = caps.name("page").unwrap().as_str().parse::<i32>().unwrap();

    if section_title.len() > CHAPTER_MAX_CHAR || section_title.len() < CHAPTER_MIN_CHAR {
        return result;
    }

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
    fn simplest_table_test() {
        let simplest_table_case = String::from("Here I have some text that is before the table of contents \
        Blablablabla
        Tydyda
        Contents
               1. The Very First Section .............. 21
               2. The Second Section .................. 22
               3. The Third Section ....................23
               ");
        assert_eq!(find_section(&simplest_table_case),
        String::from("Contents
               1. The Very First Section .............. 21
               2. The Second Section .................. 22
               3. The Third Section ....................23\n"));
    }
    #[test]
    fn wrong_header_table_test() {
        let wrong_header_table_case: String = String::from("Here I have some text that is before the table of contents\n
                Document information
        Info                 Content
        Keywords            CC, Security Target Lite, P60D024/016/012yVB(Y/Z/A)/yVF
                Blablablabla
                Tydyda
                Contents
                       1. The Very First Section .............. 21
                       2. The Second Section .................. 22
                       3. The Third Section ....................23
                       ");
        assert_eq!(find_section(&wrong_header_table_case),
                   String::from("Contents
                       1. The Very First Section .............. 21
                       2. The Second Section .................. 22
                       3. The Third Section ....................23\n"));
    }
    #[test]
    fn wrong_middle_table_test() {
        let wrong_middle_table_case: String = String::from("Here I have some text that is before the table of contents\n
        Document information
Info                 Content
Keywords            CC, Security Target Lite, P60D024/016/012yVB(Y/Z/A)/yVF
        Blablablabla
        Here I have some text in which the word Content occur.
        Also, it can occur at the end of line like Content
        Tab 1.3 or
        Content
        Manager
        Really terrible.
        Tydyda
        Contents
               1. The Very First Section .............. 21
               2. The Second Section .................. 22
               3. The Third Section ....................23
               ");
        assert_eq!(find_section(&wrong_middle_table_case),
                   String::from("Contents
               1. The Very First Section .............. 21
               2. The Second Section .................. 22
               3. The Third Section ....................23\n"));
    }
    #[test]
    fn simple_line_test(){
        let simple_line = String::from("1.1 The Very First Section .............. 21");
        let simple_line_regex =
            Regex::new(r"(?P<section>\d{1,2}(\.\d)*|[A-Z]\.|\d{1,2}.)\s*(?P<title>[A-Z](\w|\s|[“”\-\(\)\-:,/]|\w\.)*)(\s|\.)+(?P<page>\d+)")
                .unwrap();
        let expected_section = String::from("1.1");
        let expected_title = String::from("The Very First Section");
        let expected_page = 21;
        let result = extract_line_info(&simple_line, simple_line_regex , 0);
        assert_eq!(result.section, expected_section);
        assert_eq!(result.title, expected_title);
        assert_eq!(result.page, expected_page);
    }
    #[test]
    fn no_dots_line_test(){
        let no_dots_line = String::from("1.1 The Very First Section             21");
        let no_dots_line_regex =
            Regex::new(r"(?P<section>\d{1,2}(\.\d)*|[A-Z]\.|\d{1,2}.)\s*(?P<title>[A-Z]((\w|[“”\-\(\)\-:,/]|\w\.)+\s?)+)(\s|\.)+(?P<page>\d+)")
                .unwrap();
        assert!(no_dots_line_regex.is_match(&no_dots_line));

        let expected_section = String::from("1.1");
        let expected_title = String::from("The Very First Section");
        let expected_page = 21;
        let result = extract_line_info(&no_dots_line, no_dots_line_regex , 0);
        assert_eq!(result.section, expected_section);
        assert_eq!(result.title, expected_title);
        assert_eq!(result.page, expected_page);
    }
    #[test]
    fn erase_dot_test(){
        let no_dots_line = String::from("A. The Very First Section             21");
        let no_dots_line_regex =
            Regex::new(r"(?P<section>\d{1,2}(\.\d)*|[A-Z]\.|\d{1,2}.)\s*(?P<title>[A-Z]((\w|[“”\-\(\)\-:,/]|\w\.)+\s?)+)(\s|\.)+(?P<page>\d+)")
                .unwrap();
        assert!(no_dots_line_regex.is_match(&no_dots_line));

        let expected_section = String::from("A");
        let expected_title = String::from("The Very First Section");
        let expected_page = 21;
        let result = extract_line_info(&no_dots_line, no_dots_line_regex , 0);
        assert_eq!(result.section, expected_section);
        assert_eq!(result.title, expected_title);
        assert_eq!(result.page, expected_page);
    }

}
