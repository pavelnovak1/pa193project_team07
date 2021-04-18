use regex::Match;
use regex::Regex;
use crate::cert_info::Revision;


pub fn find_version_control(text: &str) -> Option<Vec<Revision>> {
    let regex_version_start =
        Regex::new(r"Version Control\s+Version\s+Date\s+Author\s+Changes to Previous Version\s+")
            .unwrap();
    let regex_version_end =
        Regex::new(r"\s*\n\s*\n\s*\n")
            .unwrap();
    let regex_version_entry = Regex::new(r"(?P<rev>[\w\.]+) +(?P<date>[\d\-]+) +([\w ]+)   +(?P<info>[\w\- \.]+)").unwrap();

    let mut version_start = regex_version_start.find(&text)?;
    println!("{}, {}\n", version_start.start(), version_start.end());
    let (_, version_start_text) = text.split_at(version_start.end());
    println!("{:?}\n", version_start_text);
    let version_end = regex_version_end.find(version_start_text)?;
    let (version_to_parse, _) = version_start_text.split_at(version_end.start());

    println!("{:?}\n", version_to_parse);
    let mut revisions = Vec::new();
    for cap in regex_version_entry.captures_iter(version_to_parse) {
        //println!("Rev: {:?},\n Date: {:?},\n  Info: {:?}\n\n", cap.name("rev"), cap.name("date"), cap.name("info"));
        println!("Rev: {:?},\n Date: {:?},\n  Info: {:?}\n\n", &cap[1], &cap[2], &cap[4]);
        revisions.push(Revision{ version: cap[1].to_string(), date: cap[2].to_string(), description: cap[4].to_string() })
    }
    Some(revisions)
}


pub fn find_revision_history_end(text: &str) -> Option<Vec<Revision>> {
    let regex_version_start =
        Regex::new(r"Revision History(\s*\n\s*.*\s*\n)?\s*\n\s*Version\s\s\s*.*\s")
            .unwrap();
    let regex_version_end =
        Regex::new(r"\s*\n\s*\n\s*\n")
            .unwrap();
    let regex_version_entry = Regex::new(r"(?P<rev>(?:(?:\w[\w ]+\w)|\d+\.\d+))  +(?P<info>[\w\- \.()]+)").unwrap();

    let mut version_start = regex_version_start.find(&text)?;
    println!("....\n");
    println!("{}, {}\n", version_start.start(), version_start.end());
    let (_, version_start_text) = text.split_at(version_start.end());
    println!("{:?}\n", version_start_text);
    let version_end = regex_version_end.find(version_start_text)?;
    let (version_to_parse, _) = version_start_text.split_at(version_end.start());

    println!("{:?}\n", version_to_parse);
    let mut revisions = Vec::new();
    for cap in regex_version_entry.captures_iter(version_to_parse) {
        //println!("Rev: {:?},\n Date: {:?},\n  Info: {:?}\n\n", cap.name("rev"), cap.name("date"), cap.name("info"));
        println!("Rev: {:?},\n  Info: {:?}\n", &cap[1], &cap[2]);
        revisions.push(Revision{ version: cap[1].to_string(), date: "".to_string(), description: cap[2].to_string() })
    }
    Some(revisions)
}


pub fn find_revision(text: &str) -> String {
/*    let mut result = find_title_certification_report(&text);
    if result.is_none() {
        result = find_title_for_from(&text);
    }
    if result.is_none() {
        result = find_title_security_target_lite_before(&text);
    }
    if result.is_none() {
        result = find_title_security_target_after(&text);
    }
    result.unwrap()*/
    "".to_string()
}



#[test]
fn find_revision_history_end_test() {
    let mut text = String::from(
        "Common Criteria v3.1 - EAL5+
Revision History


10                   Revision History
Major changes since the last revision

Version       Description of change
0.2           Initial draft version
3.5           Final version




CC Document   ",
    );

    let mut rev = find_revision_history_end(&text).unwrap();
    assert_eq!(rev.len(), 2);
    assert_eq!(rev[0].version, "0.2");
    assert_eq!(rev[1].version, "3.5");
    assert_eq!(rev[0].date, "");
    assert_eq!(rev[1].date, "");
    assert_eq!(rev[0].description, "Initial draft version");
    assert_eq!(rev[1].description, "Final version");


    text = String::from(
        "14. QUALITY REQUIREMENTS

14.1   Revision History
                Version                                      Subject
                 Rev A         Initial Release (09-July-2020)
                 Rev B         29-July-2020 Review for AGD reference version
                 Rev C         27-October-2020 TOE version update
                                      Table 14 - Revision History


15. ENVIRONMENTAL/ECOLOGICAL REQUIREMENTS",
    );

    rev = find_revision_history_end(&text).unwrap();
    assert_eq!(rev.len(), 3);
    assert_eq!(rev[0].version, "Rev A");
    assert_eq!(rev[1].version, "Rev B");
    assert_eq!(rev[2].version, "Rev C");
    assert_eq!(rev[0].date, "");
    assert_eq!(rev[1].date, "");
    assert_eq!(rev[2].date, "");
    assert_eq!(rev[0].description, "Initial Release (09-July-2020)");
    assert_eq!(rev[1].description, "29-July-2020 Review for AGD reference version");
    assert_eq!(rev[2].description, "27-October-2020 TOE version update");


    text = String::from(
        "Common Criteria v3.1 - EAL5+
Revision History


14                     Revision History
Major changes since the last revision

Version       Description of change
0.2           Initial draft version
4.3           Final version




Security Target Lite                      51          4.3
                                               2019-07-24",
    );

    rev = find_revision_history_end(&text).unwrap();
    assert_eq!(rev.len(), 2);
    assert_eq!(rev[0].version, "0.2");
    assert_eq!(rev[1].version, "4.3");
    assert_eq!(rev[0].date, "");
    assert_eq!(rev[1].date, "");
    assert_eq!(rev[0].description, "Initial draft version");
    assert_eq!(rev[1].description, "Final version");


    text = String::from(
        "Security Target Lite
Common Criteria v3.1 - EAL5+
Revision History


16                   Revision History
Major changes since the last revision

Version       Description of change
v1.2          Initial draft version
v4.0          Final version




Security Target                         68         v4.0",
    );

    rev = find_revision_history_end(&text).unwrap();
    assert_eq!(rev.len(), 2);
    assert_eq!(rev[0].version, "1.2");
    assert_eq!(rev[1].version, "4.0");
    assert_eq!(rev[0].date, "");
    assert_eq!(rev[1].date, "");
    assert_eq!(rev[0].description, "Initial draft version");
    assert_eq!(rev[1].description, "Final version");
}












#[test]
fn find_version_control_test() {
    let mut text = String::from(
        "NXP eDoc Suite v3.5 on JCOP4 P71 / PP0056v2 based Security Target Lite




Version Control
Version     Date           Author                Changes to Previous Version
1.0         2020-12-09     Thomas Zeggel         ST-Lite based on ST version 1.0.

1.1         2020-12-15     Thomas Zeggel         ST-Lite based on ST version 1.1.
1.2         2020-12-17     Thomas Zeggel         ST-Lite based on ST version 1.2.




                                                   3 of 104
NXP eDoc Suite v3.5 on JCOP4 P71 / PP0056v2 based Security Target Lite


1 Introduction
",
    );

    let mut rev = find_version_control(&text).unwrap();
    assert_eq!(rev.len(), 3);
    assert_eq!(rev[0].version, "1.0");
    assert_eq!(rev[1].version, "1.1");
    assert_eq!(rev[2].version, "1.2");
    assert_eq!(rev[0].date, "2020-12-09");
    assert_eq!(rev[1].date, "2020-12-15");
    assert_eq!(rev[2].date, "2020-12-17");
    assert_eq!(rev[0].description, "ST-Lite based on ST version 1.0.");
    assert_eq!(rev[1].description, "ST-Lite based on ST version 1.1.");
    assert_eq!(rev[2].description, "ST-Lite based on ST version 1.2.");
}