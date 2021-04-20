use regex::{Captures, Match};
use regex::Regex;

use crate::cert_info::Revision;

fn replace_whitespace_with_space(text: &str) -> String {
    let mut re = Regex::new(r"[--]\s*\n\s*").unwrap();
    let result = String::from(re.replace_all(text, "-"));
    re = Regex::new(r"\s+").unwrap();
    let result2 = re.replace_all(&result, " ");
    re = Regex::new(r"\s+$").unwrap();
    let result3 = re.replace_all(&result2, "");
    let result4 = String::from(result3);
    result4
}


// TODO create function to change date to other format and use it everywhere

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
        println!("Rev: {:?},\n Date: {:?},\n  Info: {:?}\n\n", &cap["rev"], &cap["date"], &cap["info"]);
        revisions.push(Revision{ version: cap["rev"].to_string(), date: cap["date"].to_string(), description: cap["info"].to_string() })
    }
    Some(revisions)
}


fn find_revision_history_end(text: &str) -> Option<Vec<Revision>> {
    let regex_version_start =
        Regex::new(r"Revision History(\s*\n\s*.*\s*\n)?\s*\n\s*Version\s\s\s*.*\s")
            .unwrap();
    let regex_version_end =
        Regex::new(r"(Table.*)?\s*\n\s*\n\s*\n")
            .unwrap();
    let regex_version_entry = Regex::new(r"(?P<rev>(?:(?:\w[\w ]+\w)|\d+\.\d+))  +(?P<info>.+)").unwrap();
    let regex_version_entry_multiline = Regex::new(r"(?P<rev>(?:(?:\w[\w ]+\w)|\d+\.\d+))  +(?P<info>.+(\n {10,25}.+)*)").unwrap();

    let mut version_start = regex_version_start.find(&text)?;
    let (_, version_start_text) = text.split_at(version_start.end());

    let version_end = regex_version_end.find(version_start_text)?;
    let (version_to_parse, _) = version_start_text.split_at(version_end.start());

    let revisions = find_and_get_revision_entries(regex_version_entry, regex_version_entry_multiline, &version_to_parse);
    revisions
}

fn find_and_get_revision_entries(regex_version_entry: Regex, regex_version_entry_multiline: Regex, version_to_parse: &&str) -> Option<Vec<Revision>> {
    let mut res1 = regex_version_entry.find(version_to_parse)?;
    let mut res2 = regex_version_entry.find_at(version_to_parse, res1.end());

    let mut revisions = Vec::new();
    while res2 != None {
        let unwrapped = res2.unwrap();
        let substring_with_revision_entry = &version_to_parse[res1.start()..unwrapped.start() - 1];
        res1 = unwrapped;
        res2 = regex_version_entry.find_at(version_to_parse, res1.end());

        let cap = regex_version_entry_multiline.captures(substring_with_revision_entry)?;
        //revisions.push(Revision { version: cap["rev"].to_string(), date: cap.name("date").to_string(), description: replace_whitespace_with_space(&cap["info"].to_string()) });
        revisions.push(Revision::new(&cap));
    }

    let substring_with_revision_entry = &version_to_parse[res1.start()..];
    let cap = regex_version_entry_multiline.captures(substring_with_revision_entry)?;
    revisions.push(Revision::new(&cap));
    Some(revisions)
}



pub fn find_revision_history_date_version_info(text: &str) -> Option<Vec<Revision>> {
    let regex_version_start =
        Regex::new(r"Revision [Hh]istory\s*(\s*.*\s*)?\n\s*((Date)|(Release date))\s+((:?Rev(:?ision)?)|(Version))\s+((Description)|(Change ((notice)|(Description))))\s*\n\s*")
            .unwrap();
    let regex_version_end =
        Regex::new(r"\s*\n\s*\n\s*\n")
            .unwrap(); //TODO oddelat tecku v datumu
    let regex_version_entry = Regex::new(r"(?P<date>[\w\d][\w\d\-\. ]+[\w\d])   +(?P<rev>[\w\.]+)  +(?P<info>[\w\- \.]+)").unwrap();
    let regex_version_entry_multiline = Regex::new(r"(?P<date>[\w\d][\w\d\-\. ]+[\w\d])   +(?P<rev>[\w\.]+)  +(?P<info>.+(\n {10,25}.+)*)").unwrap();

    let mut version_start = regex_version_start.find(&text)?;
    println!("HERE!\n");
    println!("....\n");
    println!("{}, {}\n", version_start.start(), version_start.end());
    let (_, version_start_text) = text.split_at(version_start.end());
    println!("{:?}\n", version_start_text);
    let version_end = regex_version_end.find(version_start_text)?;
    let (version_to_parse, _) = version_start_text.split_at(version_end.start());

    let revisions = find_and_get_revision_entries(regex_version_entry, regex_version_entry_multiline, &version_to_parse);

    /*    println!("{:?}\n", version_to_parse);
        let mut revisions = Vec::new();
        for cap in regex_version_entry.captures_iter(version_to_parse) {
            //println!("Rev: {:?},\n Date: {:?},\n  Info: {:?}\n\n", cap.name("rev"), cap.name("date"), cap.name("info"));
            println!("Rev: {:?},\n Date: {:?},\n  Info: {:?}\n\n", &cap["rev"], &cap["date"], &cap["info"]);
            revisions.push(Revision{ version: cap["rev"].to_string(), date: cap["date"].to_string(), description: cap["info"].to_string() })
        }*/
    revisions
}



pub fn find_revision_history_version_date_info(text: &str) -> Option<Vec<Revision>> {
    let regex_version_start_both =
        Regex::new(r"Revision history\s*\n\s*((:?Rev(:?ision)?)|(Version))\s+((Date)|(Release date))\s+((Description)|(Change notice))\s*\n\s*")
            .unwrap();
    let regex_version_start_version_date_info =
        Regex::new(r"\s*\n\s*((:?Rev(:?ision)?)|(Version))\s+((Date)|(Release date))\s+((Description)|(Change notice))\s*\n\s*")
            .unwrap();
    let regex_version_end =
        Regex::new(r"\s*\n\s*\n\s*\n")
            .unwrap(); //TODO oddelat tecku v datumu
    //let regex_version_entry = Regex::new(r"(?P<rev>[\w\.]+)  +(?P<date>[\w\d][\w\d\-\. ]+[\w\d])   +(?P<info>[\w\- \./()]+(                       +[\w\- \.]+)*)").unwrap();
    let regex_version_entry = Regex::new(r"(?P<rev>[\w\.]+)  +(?P<date>[\w\d][\w\d\-\. ]+[\w\d])   +(?P<info>.+(\s+                      +.+)*)").unwrap();
    let mut version_start_find = regex_version_start_both.find(&text);
    if version_start_find == None {
        version_start_find = regex_version_start_version_date_info.find(&text);
    }
    let version_start = version_start_find?;
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
        println!("Rev: {:?},\n Date: {:?},\n  Info: {:?}\n\n", &cap[1], &cap[2], &cap[3]);
        revisions.push(Revision{ version: cap[1].to_string(), date: cap[2].to_string(), description: replace_whitespace_with_space(&cap[3].to_string()) })
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
fn find_revision_history_date_version_info_test() {
    let mut text = String::from(
        "                                                                                                 Security Target Lite
Public


Revision History

Major changes since previous revision
Date           Version     Change Description

2017-10-25     1.0         Initial version
2017-11-16     1.1         Update references in Chapter 9.
2017-11-21     1.2         Update references in Chapter 9.
2019-10-29     2.0         Updated user guidance. Introduced NRG terminology. Signatures for MAE lib is updated.
2020-03-30     2.1         Accepted final remarks and provided minor editorial update.




CC Developer Document                                        4                                               2020-04-23
                                                                                                                                                                         M7892 P11
                                                                                                                                                                 Security Target Lite
Public",
    );
    println!("FERE!\n");
    let mut rev = find_revision_history_date_version_info(&text).unwrap();
    assert_eq!(rev.len(), 5);
    assert_eq!(rev[0].version, "1.0");
    assert_eq!(rev[0].date, "2017-10-25");
    assert_eq!(rev[0].description, "Initial version");
    assert_eq!(rev[1].version, "1.1");
    assert_eq!(rev[1].date, "2017-11-16");
    assert_eq!(rev[1].description, "Update references in Chapter 9.");
    assert_eq!(rev[2].version, "1.2");
    assert_eq!(rev[2].date, "2017-11-21");
    assert_eq!(rev[2].description, "Update references in Chapter 9.");
    assert_eq!(rev[3].version, "2.0");
    assert_eq!(rev[3].date, "2019-10-29");
    assert_eq!(rev[3].description, "Updated user guidance. Introduced NRG terminology. Signatures for MAE lib is updated.");
    assert_eq!(rev[4].version, "2.1");
    assert_eq!(rev[4].date, "2020-03-30");
    assert_eq!(rev[4].description, "Accepted final remarks and provided minor editorial update.");



    let mut text = String::from(
        "public


Revision History

     Date          Version                          Change Description

  2017-01-27           1.0   Initial version.

  2020-10-21           4.1   final version




Security Target Lite                            4        ",
    );
    println!("FERE!\n");
    let mut rev = find_revision_history_date_version_info(&text).unwrap();
    assert_eq!(rev.len(), 2);
    assert_eq!(rev[0].version, "1.0");
    assert_eq!(rev[0].date, "2017-01-27");
    assert_eq!(rev[0].description, "Initial version.");
    assert_eq!(rev[1].version, "4.1");
    assert_eq!(rev[1].date, "2020-10-21");
    assert_eq!(rev[1].description, "final version");


}













#[test]
fn find_revision_history_test() {
    let mut text = String::from(
        "                       Security Target Lite


Revision history
Rev          Date           Description
1.3          30 June 2020   Initial version.




JCOP 4.7 SE051                        All information provided in this document is subject to legal disclaimers.          © NXP B.V. 2020. All rights reserved.",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 1);
    assert_eq!(rev[0].version, "1.3");
    assert_eq!(rev[0].date, "30 June 2020"); //TODO 2020-06-30
    assert_eq!(rev[0].description, "Initial version.");


    let mut text = String::from(
        "                   Security Target Lite



Revision history
Rev         Date            Description
1.0         2020-02-12      Initial Version of this Security Target Lite




Contact information",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 1);
    assert_eq!(rev[0].version, "1.0");
    assert_eq!(rev[0].date, "2020-02-12");
    assert_eq!(rev[0].description, "Initial Version of this Security Target Lite");


    let mut text = String::from(
        "Security Target Lite


Revision history
Revision     Date         Description
number
2.1          06.04.2018   Derived from P6022y VB Security Target v2.1




P6022y VB  ",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 1);
    assert_eq!(rev[0].version, "2.1");
    assert_eq!(rev[0].date, "06.04.2018");  // TODO 2018-04-06
    assert_eq!(rev[0].description, "Derived from P6022y VB Security Target v2.1");


    let mut text = String::from(
        "Table 1. Revision history
Version                   Release date   Change notice
1.0                       2018-11-30     Initial version based on full Security Target v1.4
1.1                       2019-05-31     Updated version based on full Security Target v1.5




NXP Secure Smart Card Controller N7121 ",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 2);
    assert_eq!(rev[0].version, "1.0");
    assert_eq!(rev[0].date, "2018-11-30");
    assert_eq!(rev[0].description, "Initial version based on full Security Target v1.4");
    assert_eq!(rev[1].version, "1.1");
    assert_eq!(rev[1].date, "2019-05-31");
    assert_eq!(rev[1].description, "Updated version based on full Security Target v1.5");


    let mut text = String::from(
        "                          Security Target Lite



Revision history
Revision                Date         Description
number
2.3                     2019-06-05   Derived from full Security Target v2.3




Crypto Library Cobalt on N7021 VA  ",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 1);
    assert_eq!(rev[0].version, "2.3");
    assert_eq!(rev[0].date, "2019-06-05");
    assert_eq!(rev[0].description, "Derived from full Security Target v2.3");


    let mut text = String::from(
        "Semiconductors                             P60D024/016/012yVB(Y/Z/A)/yVF
                                                                                                                   Security Target Lite



Revision history
Rev         Date                 Description
Rev. 4.1    23-November-2015     Derived from P60D024/016/012yVB(Y/Z/A)/yVF Security Target
Rev. 4.2    29-May-2018          Revise wording for access control policy SFRs to be consistent with datasheet MMU
                                 errata update
Rev. 4.3    21-June-2018         This ST now claims compliance to CC v3.1 Rev. 5
                                 Improve consistency in SFR wording
                                 Clarify read/write access permission
                                 FCS.COP.1[HW_DES] now claims compliance to NIST SP800-67
Rev. 4.4    29-October-2018      Update access control SFRs to reflect three access permissions enforeced by MMU

Latest version is: Rev. 4.4 (29 October 2018)


",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 4);
    assert_eq!(rev[0].version, "4.1");
    assert_eq!(rev[0].date, "23-November-2015"); //TODO 2015-11-23
    assert_eq!(rev[0].description, "Derived from P60D024/016/012yVB(Y/Z/A)/yVF Security Target");
    assert_eq!(rev[1].version, "4.2");
    assert_eq!(rev[1].date, "29-May-2018");  //TODO 2018-05-29
    assert_eq!(rev[1].description, "Revise wording for access control policy SFRs to be consistent with datasheet MMU errata update");
    assert_eq!(rev[2].version, "4.3");
    assert_eq!(rev[2].date, "21-June-2018");  //TODO 2018-06-21
    assert_eq!(rev[2].description, "This ST now claims compliance to CC v3.1 Rev. 5 Improve consistency in SFR wording Clarify read/write access permission FCS.COP.1[HW_DES] now claims compliance to NIST SP800-67");
    assert_eq!(rev[3].version, "4.4");
    assert_eq!(rev[3].date, "29-October-2018");  //TODO 2018-10-29
    assert_eq!(rev[3].description, "Update access control SFRs to reflect three access permissions enforeced by MMU");

    text = String::from(
        "Public




Rev     Date                Description
1.0     03-April-2017       First version
1.1     31-May-2017         Minor update after certifier feedback.
2.0     06-September-2018   Updated document version numbers in Tab. 1.1. Updated CC conformance to v3.1
                            rev5.
2.1     15-November-2018    Updated SP 800-67 reference. Updated delivery information in section 1.4.1.1.
2.2     09-May-2019         Removed single-DES and 2-key TDES references.
2.3     06-June-2019        Updated Guidance and Operation Manual reference.




Final                ",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 6);
    assert_eq!(rev[0].version, "1.0");
    assert_eq!(rev[0].date, "03-April-2017"); //TODO 2017-04-03
    assert_eq!(rev[0].description, "First version");

    assert_eq!(rev[1].version, "1.1");
    assert_eq!(rev[1].date, "31-May-2017");  //TODO 2017-05-31
    assert_eq!(rev[1].description, "Minor update after certifier feedback.");

    assert_eq!(rev[2].version, "2.0");
    assert_eq!(rev[2].date, "06-September-2018");  //TODO 2018-09-06
    assert_eq!(rev[2].description, "Updated document version numbers in Tab. 1.1. Updated CC conformance to v3.1 rev5.");

    assert_eq!(rev[3].version, "2.1");
    assert_eq!(rev[3].date, "15-November-2018");  //TODO 2018-11-15
    assert_eq!(rev[3].description, "Updated SP 800-67 reference. Updated delivery information in section 1.4.1.1.");

    assert_eq!(rev[4].version, "2.2");
    assert_eq!(rev[4].date, "09-May-2019");  //TODO 2019-05-09
    assert_eq!(rev[4].description, "Removed single-DES and 2-key TDES references.");

    assert_eq!(rev[5].version, "2.3");
    assert_eq!(rev[5].date, "06-June-2019");  //TODO 2019-06-06
    assert_eq!(rev[5].description, "Updated Guidance and Operation Manual reference.");


    text = String::from(
        "NXP Semiconductors                         JCOP 4.x on P73.2C2/2C6
                                                                               Security Target Lite
                                                                                 Company Public



Rev       Date             Description

2.0       2019-07-24       Maintenance on JCOP 4.2 certified product
2.1       2019-08-07       Align with Security Target
2.2       2020-01-08       Add reference to P73.2C6 certification
2.3       2020-01-09       P73.2C2 and P73.2C6 are combined in P73.2C2/2C6 certificate
3.0       2020-08-26       Re-certification to add JCOP 4.8
3.1       2020-09-01       Update guidance reference
3.2       2020-09-04       Update guidance reference




Release                                                                           ©NXP B.V. 2020. All rights reserved.",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 7);
    assert_eq!(rev[0].version, "2.0");
    assert_eq!(rev[0].date, "2019-07-24");
    assert_eq!(rev[0].description, "Maintenance on JCOP 4.2 certified product");

    assert_eq!(rev[1].version, "2.1");
    assert_eq!(rev[1].date, "2019-08-07");
    assert_eq!(rev[1].description, "Align with Security Target");

    assert_eq!(rev[2].version, "2.2");
    assert_eq!(rev[2].date, "2020-01-08");
    assert_eq!(rev[2].description, "Add reference to P73.2C6 certification");

    assert_eq!(rev[3].version, "2.3");
    assert_eq!(rev[3].date, "2020-01-09");
    assert_eq!(rev[3].description, "P73.2C2 and P73.2C6 are combined in P73.2C2/2C6 certificate");

    assert_eq!(rev[4].version, "3.0");
    assert_eq!(rev[4].date, "2020-08-26");
    assert_eq!(rev[4].description, "Re-certification to add JCOP 4.8");

    assert_eq!(rev[5].version, "3.1");
    assert_eq!(rev[5].date, "2020-09-01");
    assert_eq!(rev[5].description, "Update guidance reference");

    assert_eq!(rev[6].version, "3.2");
    assert_eq!(rev[6].date, "2020-09-04");
    assert_eq!(rev[6].description, "Update guidance reference");


    text = String::from(
        "                                                                                      Security Target Lite
                                                                                                              PUBLIC




Rev     Date               Description
1.0     31-December-2018   Initial version of this Security Target Lite based on Security Target Revision 1.9




Final   ",
    );

    let mut rev = find_revision_history_version_date_info(&text).unwrap();
    assert_eq!(rev.len(), 1);
    assert_eq!(rev[0].version, "1.0");
    assert_eq!(rev[0].date, "31-December-2018"); //TODO 2018-12-31
    assert_eq!(rev[0].description, "Initial version of this Security Target Lite based on Security Target Revision 1.9");
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
    assert_eq!(rev[0].date, "");
    assert_eq!(rev[0].description, "Initial Release (09-July-2020)");

    assert_eq!(rev[1].version, "Rev B");
    assert_eq!(rev[1].date, "");
    assert_eq!(rev[1].description, "29-July-2020 Review for AGD reference version");

    assert_eq!(rev[2].version, "Rev C");
    assert_eq!(rev[2].date, "");
    assert_eq!(rev[2].description, "27-October-2020 TOE version update");

    text = String::from(
        "


14       Revision History
Version       Description of change
   0.1        Initial Version
   0.2        Inclusion of comments on GBIC requirements issued to vendors end of September 2016
   0.3        Update of asymmetric cryptographic library, update of ANSI X9.63 standard reference, removal of
              unused standards
   0.4        Update of the NRG version
   0.5        Update of the ACL user guidance document
   0.6        Assurance Continuity Maintenance to include second firmware identifier as alternative, correction in
              table 7.1.4.1
   0.7        Inclusion of further optional and alternative software packages HSL, SCL, ACL and CIPURSE™
              Cryptographic Library, update of standard reference [38], update of lower border RSA key length,
              update wording in chapter 10 (GBIC annex); update of user guidance reference (CCL), update of
              cryptographic table and declaration with footnotes in chapter 7.1.4.
   0.8        Inclusion of further library alternatives, additional CMAC functionality of the SCL, user guidance
              documents and standard reference update, removal of summarizing cryptographic table from
              chapter 7.1.4.1 as this is now part of the certification report by BSI, typo correction; update of ACL
              user guidance in chapter 9, literature and references
   0.9        Updated reference [6]
   1.0        Corrected FW ID, new HSL version
   1.1        Hash value correction, update of user guidance chapter 9, editorial changes, additional wording at
              FCS_CKM.1/RSA on BSI request, wording changed, update of HSL user guidance reference, added
              additional Flash Loader information at chapters 5.8, 7.2 and 7.4.1.3
   1.2        Inclusion of second production lines, corrections/updates on standard references in chapter 7.1.4
   1.3        On BSI request: Inclusion of many redundant text information in chapter 1 and 2 regarding TSF
              contribution of SW libraries.
   1.4        Following the business decision to remove the oldest NRG version from the certification process as
              it is no longer required, update of ACL user guidance and security guideline document, inclusion of
              fourth optional and alternative HSL, completion of standard references
   1.5        Inclusion of the new Hash Cryptographic Library HCL, considering ORs 2019-02-13, 2019-03-15,
              completion with SFR for HSL, editorial regarding one CCL, typo corrections
   1.6        Inclusion of additional FW Identifier and updated User Guidance, move of a table to confidential
              Security Target [8]
   1.7        NRG Terminology introduction. Removal of the optional HCL library. Addition of another hardware
              configuration option (Power Configuration 0 and Power Configuration 1). Actualization of the user
              guidance.
   1.8        Editorial clean-up


CC Document    ",
    );

    rev = find_revision_history_end(&text).unwrap();
    assert_eq!(rev.len(), 18);
    assert_eq!(rev[0].version, "0.1");
    assert_eq!(rev[0].date, "");
    assert_eq!(rev[0].description, "Initial Version");

    assert_eq!(rev[1].version, "0.2");
    assert_eq!(rev[1].date, "");
    assert_eq!(rev[1].description, "Inclusion of comments on GBIC requirements issued to vendors end of September 2016");

    assert_eq!(rev[2].version, "0.3");
    assert_eq!(rev[2].date, "");
    assert_eq!(rev[2].description, "Update of asymmetric cryptographic library, update of ANSI X9.63 standard reference, removal of unused standards");

    assert_eq!(rev[3].version, "0.4");
    assert_eq!(rev[3].date, "");
    assert_eq!(rev[3].description, "Update of the NRG version");

    assert_eq!(rev[4].version, "0.5");
    assert_eq!(rev[4].date, "");
    assert_eq!(rev[4].description, "Update of the ACL user guidance document");

    assert_eq!(rev[5].version, "0.6");
    assert_eq!(rev[5].date, "");
    assert_eq!(rev[5].description, "Assurance Continuity Maintenance to include second firmware identifier as alternative, correction in table 7.1.4.1");

    assert_eq!(rev[6].version, "0.7");
    assert_eq!(rev[6].date, "");
    assert_eq!(rev[6].description, "Inclusion of further optional and alternative software packages HSL, SCL, ACL and CIPURSE™ Cryptographic Library, update of standard reference [38], update of lower border RSA key length, update wording in chapter 10 (GBIC annex); update of user guidance reference (CCL), update of cryptographic table and declaration with footnotes in chapter 7.1.4.");

    assert_eq!(rev[7].version, "0.8");
    assert_eq!(rev[7].date, "");
    assert_eq!(rev[7].description, "Inclusion of further library alternatives, additional CMAC functionality of the SCL, user guidance documents and standard reference update, removal of summarizing cryptographic table from chapter 7.1.4.1 as this is now part of the certification report by BSI, typo correction; update of ACL user guidance in chapter 9, literature and references");

    assert_eq!(rev[8].version, "0.9");
    assert_eq!(rev[8].date, "");
    assert_eq!(rev[8].description, "Updated reference [6]");

    assert_eq!(rev[9].version, "1.0");
    assert_eq!(rev[9].date, "");
    assert_eq!(rev[9].description, "Corrected FW ID, new HSL version");

    assert_eq!(rev[10].version, "1.1");
    assert_eq!(rev[10].date, "");
    assert_eq!(rev[10].description, "Hash value correction, update of user guidance chapter 9, editorial changes, additional wording at FCS_CKM.1/RSA on BSI request, wording changed, update of HSL user guidance reference, added additional Flash Loader information at chapters 5.8, 7.2 and 7.4.1.3");

    assert_eq!(rev[11].version, "1.2");
    assert_eq!(rev[11].date, "");
    assert_eq!(rev[11].description, "Inclusion of second production lines, corrections/updates on standard references in chapter 7.1.4");

    assert_eq!(rev[12].version, "1.3");
    assert_eq!(rev[12].date, "");
    assert_eq!(rev[12].description, "On BSI request: Inclusion of many redundant text information in chapter 1 and 2 regarding TSF contribution of SW libraries.");

    assert_eq!(rev[13].version, "1.4");
    assert_eq!(rev[13].date, "");
    assert_eq!(rev[13].description, "Following the business decision to remove the oldest NRG version from the certification process as it is no longer required, update of ACL user guidance and security guideline document, inclusion of fourth optional and alternative HSL, completion of standard references");


    assert_eq!(rev[14].version, "1.5");
    assert_eq!(rev[14].date, "");
    assert_eq!(rev[14].description, "Inclusion of the new Hash Cryptographic Library HCL, considering ORs 2019-02-13, 2019-03-15, completion with SFR for HSL, editorial regarding one CCL, typo corrections");

    assert_eq!(rev[15].version, "1.6");
    assert_eq!(rev[15].date, "");
    assert_eq!(rev[15].description, "Inclusion of additional FW Identifier and updated User Guidance, move of a table to confidential Security Target [8]");

    assert_eq!(rev[16].version, "1.7");
    assert_eq!(rev[16].date, "");
    assert_eq!(rev[16].description, "NRG Terminology introduction. Removal of the optional HCL library. Addition of another hardware configuration option (Power Configuration 0 and Power Configuration 1). Actualization of the user guidance.");

    assert_eq!(rev[17].version, "1.8");
    assert_eq!(rev[17].date, "");
    assert_eq!(rev[17].description, "Editorial clean-up");

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