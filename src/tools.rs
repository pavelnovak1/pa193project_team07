use regex::{Captures, Match};
use regex::Regex;

pub fn replace_whitespace_with_space(text: &str) -> String {
    let mut re = Regex::new(r"[--]\s*\n\s*").unwrap();
    let result = String::from(re.replace_all(text, "-"));
    re = Regex::new(r"\s+").unwrap();
    let result2 = re.replace_all(&result, " ");
    re = Regex::new(r"\s+$").unwrap();
    let result3 = re.replace_all(&result2, "");
    let result4 = String::from(result3);
    result4
}

pub fn format_date(orig_date: &str) -> String {
    println!("orig == {}", orig_date);
    let mut re = Regex::new(r"^(?P<d>\d{1,2})[. -](?P<m>\w+)[. -](?P<y>\d{4})").unwrap();
    let cap = match re.captures(orig_date) {
        Some(inner) => inner,
        None => return orig_date.to_string()
    };
    let month = match &cap["m"] {
        "January" => "01",
        "February" => "02",
        "March" => "03",
        "April" => "04",
        "May" => "05",
        "June" => "06",
        "July" => "07",
        "August" => "08",
        "September" => "09",
        "October" => "10",
        "November" => "11",
        "December" => "12",
        _ => &cap["m"]
    };

    let mut new_date = cap["y"].to_string();
    new_date.push_str("-");
    new_date.push_str(month);
    new_date.push_str("-");
    new_date.push_str(&cap["d"]);
    println!("new == {}", new_date);
    new_date
}


#[test]
fn format_date_test() {
    let mut old_date = String::from("30 June 2020");
    let mut new_date = format_date(&old_date);
    assert_eq!(new_date, "2020-06-30");

    old_date = String::from("06.04.2018");
    new_date = format_date(&old_date);
    assert_eq!(new_date, "2018-04-06");

    old_date = String::from("23-November-2015");
    new_date = format_date(&old_date);
    assert_eq!(new_date, "2015-11-23");


    old_date = String::from("29-May-2018");
    new_date = format_date(&old_date);
    assert_eq!(new_date, "2018-05-29");


    old_date = String::from("21-June-2018");
    new_date = format_date(&old_date);
    assert_eq!(new_date, "2018-06-21");


    old_date = String::from("2019-07-24");
    new_date = format_date(&old_date);
    assert_eq!(new_date, "2019-07-24");
}