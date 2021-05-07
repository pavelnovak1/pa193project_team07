use regex::Regex;

pub fn replace_whitespace_with_space(text: &str) -> String {
    let mut re = Regex::new(r"[--]\s*\n\s*").unwrap();
    let result = String::from(re.replace_all(text, "-"));
    re = Regex::new(r"\s+").unwrap();
    let result2 = re.replace_all(&result, " ");
    re = Regex::new(r"\s+$").unwrap();
    let result3 = re.replace_all(&result2, "");
    String::from(result3)
}


pub(crate) fn find_and_get_string_after_match(text: &&str, regex_version_start: Regex) -> Option<String> {
    let version_start = regex_version_start.find(&text)?;
    let (_, version_start_text) = text.split_at(version_start.end());
    Some(version_start_text.to_string())
}


pub(crate) fn find_and_get_string_before_match(regex_version_end: &Regex, version_start_text: &str) -> Option<String> {
    let version_end = regex_version_end.find(version_start_text)?;
    let (version_to_parse, _) = version_start_text.split_at(version_end.start());
    Some(version_to_parse.to_string())
}


pub fn format_date(orig_date: &str) -> String {
    let re = Regex::new(r"^(?P<d>\d{1,2})[. -](?P<m>\w+)[. -](?P<y>\d{4})").unwrap();
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
    new_date.push('-');
    new_date.push_str(month);
    new_date.push('-');
    new_date.push_str(&cap["d"]);
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