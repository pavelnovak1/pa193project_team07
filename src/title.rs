use regex::Regex;
use regex::Match;

fn find(regex: regex::Regex, text: &str) -> Match {
    let results = regex.find(&text).unwrap();
    results
}

fn replace_whitespace_with_space(text: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    println!("=========={}============", text);
    println!("{}",re.find(text).unwrap().start());
    let result = re.replace_all(text, " ");
    println!("=========={}============", result);
    String::from(result)
}

pub fn find_title_for_from(text: &String) -> String {
    let mut result = find(
        Regex::new(r"(^|\s)\w{3}-\w{3}-\w{2}-\d{4}-\w{2}-\d{4}\s*for\s*").unwrap(),
        &text,
    );
    let (_, title_part_from) = text.split_at(result.end());
    result = find(
        Regex::new(r"\s*(\n|.)*\n\s*from\s*\n").unwrap(),
        &title_part_from,
    );
    let (title_with_from, _) = title_part_from.split_at(result.end());
    result = find(
        Regex::new(r"\s*\n\s*from\s*\n").unwrap(),
        &title_with_from,
    );
    let (title, _) = title_with_from.split_at(result.start());
    replace_whitespace_with_space(title)
}


pub fn find_title_security_target_lite(text: &String) -> String {
    let mut result = find(
        Regex::new(r"(^|\s)Security Target Lite\s+").unwrap(),
        &text,
    );
    let (_, title_start) = text.split_at(result.end());
    result = find(
        Regex::new(r"\s+Common Criteria").unwrap(),
        &title_start,
    );
    let (title, _) = title_start.split_at(result.start());
    replace_whitespace_with_space(title)
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_title_for_from_test() {
        let mut text = String::from("           BSI-DSZ-CC-0945-V3-2018

                      for

    Infineon smart card IC (Security Controller)
 IFX_CCI_000003h, 000005h, 000008h, 00000Ch,
 000013h, 000014h, 000015h, 00001Ch, 00001Dh,
000021h, 00022Dh, design step H13 with optional
   libraries CIPURSE™ CL, RSA2048/4096, EC,
  Toolbox, HSL, MCS, SCL and with specific IC
                dedicated software

                     from

           Infineon Technologies AG
");
        assert_eq!(find_title_for_from(&text),String::from("Infineon smart card IC (Security Controller) IFX_CCI_000003h, 000005h, 000008h, 00000Ch, 000013h, 000014h, 000015h, 00001Ch, 00001Dh, 000021h, 00022Dh, design step H13 with optional libraries CIPURSE™ CL, RSA2048/4096, EC, Toolbox, HSL, MCS, SCL and with specific IC dedicated software"));

        text = String::from("           BSI-DSZ-CC-0879-V4-2020

                       for

   Infineon Security Controller M7893 B11 with
 optional RSA2048/4096 v2.03.008, EC v2.03.008,
SHA-2 v1.01, SCL v2.02.010 libraries and Toolbox
v2.03.008 and with specific IC dedicated software
                    (firmware)

                      from

           Infineon Technologies AG");

        assert_eq!(find_title_for_from(&text),String::from("Infineon Security Controller M7893 B11 with optional RSA2048/4096 v2.03.008, EC v2.03.008, SHA-2 v1.01, SCL v2.02.010 libraries and Toolbox v2.03.008 and with specific IC dedicated software (firmware)"));


        text = String::from("           BSI-DSZ-CC-0961-V3-2018

                      for

    Infineon smart card IC (Security Controller)
        IFX_CCI_000007h, IFX_CCI_000009h,
       IFX_CCI_00000Ah, IFX_CCI_00000Bh,
        IFX_CCI_000016h, IFX_CCI_000017h,
        IFX_CCI_000018h, IFX_CCI_000023h,
 IFX_CCI_000024h design step G13 with optional
   libraries CIPURSE™ CL, RSA2048/4096, EC,
Toolbox, HSL, SCL and with specific IC dedicated
                     software

                     from

           Infineon Technologies AG
");

        assert_eq!(find_title_for_from(&text),String::from("Infineon smart card IC (Security Controller) IFX_CCI_000007h, IFX_CCI_000009h, IFX_CCI_00000Ah, IFX_CCI_00000Bh, IFX_CCI_000016h, IFX_CCI_000017h, IFX_CCI_000018h, IFX_CCI_000023h, IFX_CCI_000024h design step G13 with optional libraries CIPURSE™ CL, RSA2048/4096, EC, Toolbox, HSL, SCL and with specific IC dedicated software"));

        text = String::from("        BSI-DSZ-CC-0977-V2-2019

                   for

NXP Secure Smart Card Controller N7021 VA
     including IC Dedicated Software

                  from

   NXP Semiconductors Germany GmbH");

        assert_eq!(find_title_for_from(&text),String::from("NXP Secure Smart Card Controller N7021 VA including IC Dedicated Software"));



    }


    #[test]
    fn find_title_security_target_lite_test() {
        let text = String::from("   Security Target Lite

   M7892 B11

   Recertification

   Common Criteria CCv3.1 EAL6 augmented (EAL6+)

   Resistance to attackers with HIGH attack potential



");
        assert_eq!(find_title_security_target_lite(&text),String::from("M7892 B11 Recertification"));

    }

}
