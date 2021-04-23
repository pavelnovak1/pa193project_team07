use regex::Match;
use regex::Regex;

use crate::tools::*;

/*
test_dataset/0782V5b_pdf.txt  --  11  -- tohle ai nepujde vylepsit
test_dataset/1126b_pdf.txt  --  18  -- nejde, spatne expected ?
test_dataset/NSCIB-CC-0075446-CRv2.txt  --  18  --  nejde, spatne expected ?
test_dataset/NSCIB-CC_0075541-ST.txt
8
test_dataset/NSCIB-CC-0145426-ST_rev_C-final.txt  --  10  -- nejde, spatne expected ?
test_dataset/[ST-Lite-EAC]_(v1.1)_2018_2000036361_-_Security_Target_Lite_IDeal_Pass_v2.3-n_(SAC_EAC_Polymorphic).txt
9
test_dataset/[ST-Mercury]_Security_Target_Mercury_v3.5.txt
15
 */

fn find(regex: regex::Regex, text: &str) -> Match {
    let results = regex.find(&text).unwrap();
    results
}
/*
fn replace_whitespace_with_space(text: &str) -> String {
    let mut re = Regex::new(r"[--]\s*\n\s*").unwrap();
    let result = String::from(re.replace_all(text, "-"));
    re = Regex::new(r"\s+").unwrap();
    let result2 = re.replace_all(&result, " ");
    let result3 = String::from(result2);
    result3
}
*/
/*
fn find_and_get_string_after_match(text: &&str, regex_version_start: Regex) -> Option<String> {
    let version_start = regex_version_start.find(&text)?;
    let (_, version_start_text) = text.split_at(version_start.end());
    Some(version_start_text.to_string())
}


fn find_and_get_string_before_match(regex_version_end: &Regex, version_start_text: &str) -> Option<String> {
    let version_end = regex_version_end.find(version_start_text)?;
    let (version_to_parse, _) = version_start_text.split_at(version_end.start());
    Some(version_to_parse.to_string())
}
*/

fn find_title_certification_report(text: &str) -> Option<String> {
    let regex_cap_for_from = Regex::new(r"(?:^|\s)(?:\w|\s)+Certification Report\s+Version \d{4}-\d+\s+(?P<title>(.*|\s)*[^\s])\s+Sponsor(?: and developer)?:").unwrap();
    let caps = regex_cap_for_from.captures(text)?;
    let title = caps.name("title").unwrap().as_str();
    Some(replace_whitespace_with_space(title))
}


fn find_title_for_from(text: &str) -> Option<String> {
    let regex_cap_for_from =
        Regex::new(r"(?:^|\s)\n\s*for\s*\n\s*\n\s*(?P<title>(.*|\s)*.)\s*\n\s*\n\s*from\s*\n")
            .unwrap();
    let caps = regex_cap_for_from.captures(text)?;
    let title = caps.name("title").unwrap().as_str();
    Some(replace_whitespace_with_space(title))
}


fn find_title_security_target_lite_before(text: &str) -> Option<String> {
    let regex_sec_target = Regex::new(r"(?:^(:?\s*Public\s*)?(?:\s*\n\s*\n\s*\n)?)\s*(?:\s+Common Criteria.*)?(?:(?:Security Target(?: Lite)?)|(?:SECURITY TARGET (?:LITE)?))(?:\s+Common Criteria.*)?(?:\s*(?:EAL.*))?\s+").unwrap();
    let mut result = regex_sec_target.find(&text)?;
    let (_, title_start) = text.split_at(result.end());
    result = find(
        Regex::new(r"\s*(?:Common Criteria)|(?:\s*\n\s*Reference:)|(?:\s*\n\s*\n\s*)").unwrap(),
        &title_start,
    );
    let (title, _) = title_start.split_at(result.start());
    Some(replace_whitespace_with_space(title))
}


fn find_title_security_target_after(text: &str) -> Option<String> {
    let regex_title_before_security_target = Regex::new(r"\s*Security Target\s+")
        .unwrap();
    let regex_cap_for_from =
        Regex::new(r"(?:^|\s)\s*(?:STMicroelectronics)?(?:PUBLIC)?\s*(?P<title>(.|\s)+[^\s])")
            .unwrap();
    let string_with_title = find_and_get_string_before_match(&regex_title_before_security_target, &text)?;
    let caps = regex_cap_for_from.captures(&string_with_title)?;
    let title = caps.name("title").unwrap().as_str();
    Some(replace_whitespace_with_space(title))
}


fn find_title_first_lines(text: &str) -> Option<String> {
    let regex_cap_for_from =
        Regex::new(r"(?:^|\s)\s*(?P<title>[^\s](.|\s)+[^\s])\s*\n\s*\n\s*\n")
            .unwrap();
    let caps = regex_cap_for_from.captures(text)?;
    let title = caps.name("title").unwrap().as_str();
    Some(replace_whitespace_with_space(title))
}


pub fn find_title(text: &str) -> String {
    let mut result = find_title_certification_report(&text);
    if result.is_none() {
        result = find_title_for_from(&text);
    }
    if result.is_none() {
        result = find_title_security_target_lite_before(&text);
    }
    if result.is_none() {
        result = find_title_security_target_after(&text);
    }
    if result.is_none() {
        result = find_title_first_lines(&text);
    }
    result.unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_title_for_from_test() {
        let mut text = String::from(
            "           BSI-DSZ-CC-0945-V3-2018

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
",
        );
        assert_eq!(find_title_for_from(&text).unwrap(), String::from("Infineon smart card IC (Security Controller) IFX_CCI_000003h, 000005h, 000008h, 00000Ch, 000013h, 000014h, 000015h, 00001Ch, 00001Dh, 000021h, 00022Dh, design step H13 with optional libraries CIPURSE™ CL, RSA2048/4096, EC, Toolbox, HSL, MCS, SCL and with specific IC dedicated software"));
        assert_eq!(find_title(&text), String::from("Infineon smart card IC (Security Controller) IFX_CCI_000003h, 000005h, 000008h, 00000Ch, 000013h, 000014h, 000015h, 00001Ch, 00001Dh, 000021h, 00022Dh, design step H13 with optional libraries CIPURSE™ CL, RSA2048/4096, EC, Toolbox, HSL, MCS, SCL and with specific IC dedicated software"));

        text = String::from(
            "           BSI-DSZ-CC-0879-V4-2020

                       for

   Infineon Security Controller M7893 B11 with
 optional RSA2048/4096 v2.03.008, EC v2.03.008,
SHA-2 v1.01, SCL v2.02.010 libraries and Toolbox
v2.03.008 and with specific IC dedicated software
                    (firmware)

                      from

           Infineon Technologies AG",
        );

        assert_eq!(find_title_for_from(&text).unwrap(), String::from("Infineon Security Controller M7893 B11 with optional RSA2048/4096 v2.03.008, EC v2.03.008, SHA-2 v1.01, SCL v2.02.010 libraries and Toolbox v2.03.008 and with specific IC dedicated software (firmware)"));
        assert_eq!(find_title(&text), String::from("Infineon Security Controller M7893 B11 with optional RSA2048/4096 v2.03.008, EC v2.03.008, SHA-2 v1.01, SCL v2.02.010 libraries and Toolbox v2.03.008 and with specific IC dedicated software (firmware)"));


        text = String::from(
            "        BSI-DSZ-CC-0977-V2-2019

                   for

NXP Secure Smart Card Controller N7021 VA
     including IC Dedicated Software

                  from

   NXP Semiconductors Germany GmbH",
        );

        assert_eq!(
            find_title_for_from(&text).unwrap(),
            String::from(
                "NXP Secure Smart Card Controller N7021 VA including IC Dedicated Software"
            )
        );
        assert_eq!(
            find_title(&text),
            String::from(
                "NXP Secure Smart Card Controller N7021 VA including IC Dedicated Software"
            )
        );


        text = String::from(
            "          BSI-DSZ-CC-1072-2018

                    for

NXP Secure Smart Card Controller P6021y VB*
      including IC Dedicated Software

                   from

   NXP Semiconductors Germany GmbH",
        );

        assert_eq!(
            find_title_for_from(&text).unwrap(),
            String::from(
                "NXP Secure Smart Card Controller P6021y VB* including IC Dedicated Software"
            )
        );
        assert_eq!(
            find_title(&text),
            String::from(
                "NXP Secure Smart Card Controller P6021y VB* including IC Dedicated Software"
            )
        );


        text = String::from(
            "          BSI-DSZ-CC-1110-V2-2019

                      for

 Infineon Security Controller IFX_CCI_000003h,
000005h, 000008h, 00000Ch, 000013h, 000014h,
 000015h, 00001Ch, 00001Dh, 000021h, 000022h
  H13 including the products from the second
production line and optional software packages:
    Flash Loader, Asymmetric Crypto Library,
  Symmetric Cryptographic Library, Hardware
   Support Layer, Hash Crypto Library, Mifare
  Compatible Software, and CIPURSE™ Crypto
                    Library

                     from

          Infineon Technologies AG",
        );

        assert_eq!(find_title_for_from(&text).unwrap(), String::from("Infineon Security Controller IFX_CCI_000003h, 000005h, 000008h, 00000Ch, 000013h, 000014h, 000015h, 00001Ch, 00001Dh, 000021h, 000022h H13 including the products from the second production line and optional software packages: Flash Loader, Asymmetric Crypto Library, Symmetric Cryptographic Library, Hardware Support Layer, Hash Crypto Library, Mifare Compatible Software, and CIPURSE™ Crypto Library"));
        assert_eq!(find_title(&text), String::from("Infineon Security Controller IFX_CCI_000003h, 000005h, 000008h, 00000Ch, 000013h, 000014h, 000015h, 00001Ch, 00001Dh, 000021h, 000022h H13 including the products from the second production line and optional software packages: Flash Loader, Asymmetric Crypto Library, Symmetric Cryptographic Library, Hardware Support Layer, Hash Crypto Library, Mifare Compatible Software, and CIPURSE™ Crypto Library"));


        text = String::from(
            "             BSI-DSZ-CC-1102-2019

                       for

     Infineon Technologies Security Controller
 IFX_CCI_001Fh, IFX_CCI_002Fh, IFX_CCI_0030h,
 IFX_CCI_0033h, IFX_CCI_0035h, IFX_CCI_0036h,
   IFX_CCI_0038h design step S11 and M11 with
   optional HSL v2.62.7626, optional SCL version
v2.04.003, UMSLC lib v01.00.0234 with specific IC-
dedicated firmware identifier 80.304.01.0 and user
                     guidance

                      from

            Infineon Technologies AG",
        );

        assert_eq!(find_title_for_from(&text).unwrap(), String::from("Infineon Technologies Security Controller IFX_CCI_001Fh, IFX_CCI_002Fh, IFX_CCI_0030h, IFX_CCI_0033h, IFX_CCI_0035h, IFX_CCI_0036h, IFX_CCI_0038h design step S11 and M11 with optional HSL v2.62.7626, optional SCL version v2.04.003, UMSLC lib v01.00.0234 with specific IC-dedicated firmware identifier 80.304.01.0 and user guidance"));
        assert_eq!(find_title(&text), String::from("Infineon Technologies Security Controller IFX_CCI_001Fh, IFX_CCI_002Fh, IFX_CCI_0030h, IFX_CCI_0033h, IFX_CCI_0035h, IFX_CCI_0036h, IFX_CCI_0038h design step S11 and M11 with optional HSL v2.62.7626, optional SCL version v2.04.003, UMSLC lib v01.00.0234 with specific IC-dedicated firmware identifier 80.304.01.0 and user guidance"));
    }

    #[test]
    fn find_title_certification_report_test() {
        let mut text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2018-1




                                                                                                                                           Crypto Library V3.1.x on P6022y VB




                                                                                                                  Sponsor and developer:           NXP Semiconductors Germany GmbH
                                                                                                                                                   Business Unit Security & Connectivity
                                                                                                                                                   Troplowitzstrasse 20, 22529 Hamburg
                                                                                                                                                   Germany");
        assert_eq!(
            find_title_certification_report(&text).unwrap(),
            String::from("Crypto Library V3.1.x on P6022y VB")
        );
        assert_eq!(
            find_title(&text),
            String::from("Crypto Library V3.1.x on P6022y VB")
        );


        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-3




                                                                                                                    NXP eDoc Suite v3.5 on JCOP4 P71 - cryptovision ePasslet
                                                                                                                    Suite – Java Card applet configuration providing Machine
                                                                                                                      Readable Travel Document with „ICAO Application”,
                                                                                                                              Extended Access Control with PACE

                                                                                                                  Sponsor:                         NXP Semiconductors Germany GmbH");
        assert_eq!(find_title_certification_report(&text).unwrap(), String::from("NXP eDoc Suite v3.5 on JCOP4 P71 - cryptovision ePasslet Suite – Java Card applet configuration providing Machine Readable Travel Document with „ICAO Application”, Extended Access Control with PACE"));
        assert_eq!(find_title(&text), String::from("NXP eDoc Suite v3.5 on JCOP4 P71 - cryptovision ePasslet Suite – Java Card applet configuration providing Machine Readable Travel Document with „ICAO Application”, Extended Access Control with PACE"));


        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2019-4




                                                                                                                                 NXP JCOP 5.2 on SN100.C58 Secure Element




                                                                                                                  Sponsor and developer:           NXP Semiconductors GmbH");
        assert_eq!(
            find_title_certification_report(&text).unwrap(),
            String::from("NXP JCOP 5.2 on SN100.C58 Secure Element")
        );
        assert_eq!(
            find_title(&text),
            String::from("NXP JCOP 5.2 on SN100.C58 Secure Element")
        );


        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-3




                                                                                                                        NXP eDoc Suite v3.5 on JCOP4 71 - cryptovision ePasslet
                                                                                                                        Suite – Java Card applet configuration providing Secure
                                                                                                                           Signature Creation Device with key import (SSCD)


                                                                                                                  Sponsor:                         NXP Semiconductors Germany GmbH
                                                                                                                                                   Troplowitzstrasse 20");
        assert_eq!(find_title_certification_report(&text).unwrap(), String::from("NXP eDoc Suite v3.5 on JCOP4 71 - cryptovision ePasslet Suite – Java Card applet configuration providing Secure Signature Creation Device with key import (SSCD)"));
        assert_eq!(find_title(&text), String::from("NXP eDoc Suite v3.5 on JCOP4 71 - cryptovision ePasslet Suite – Java Card applet configuration providing Secure Signature Creation Device with key import (SSCD)"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-2




                                                                                                                                                              JCOP 4 SE050M




                                                                                                                  Sponsor and developer:           NXP Semiconductors Germany GmbH");
        assert_eq!(
            find_title_certification_report(&text).unwrap(),
            String::from("JCOP 4 SE050M")
        );
        assert_eq!(find_title(&text), String::from("JCOP 4 SE050M"));
    }

    #[test]
    fn find_title_security_target_lite_before_test() {
        let mut text = String::from(
            "   Security Target Lite

   M7892 B11

   Recertification

   Common Criteria CCv3.1 EAL6 augmented (EAL6+)

   Resistance to attackers with HIGH attack potential



",
        );
        assert_eq!(
            find_title_security_target_lite_before(&text).unwrap(),
            String::from("M7892 B11")
        ); //TODO should be "M7892 B11 Recertification"
        assert_eq!(find_title(&text), String::from("M7892 B11")); //TODO should be "M7892 B11 Recertification"

        text = String::from(
            "Security Target

OPTIGATM Trusted Platform Module
SLB9670_2.0 v7.85
Common Criteria CCv3.1 EAL4 augmented (EAL4+)
Resistance to attackers with MODERATE attack potential",
        );
        assert_eq!(
            find_title_security_target_lite_before(&text).unwrap(),
            String::from("OPTIGATM Trusted Platform Module SLB9670_2.0 v7.85")
        );
        assert_eq!(
            find_title(&text),
            String::from("OPTIGATM Trusted Platform Module SLB9670_2.0 v7.85")
        );

        text = String::from("       SECURITY TARGET LITE
 IDEAL PASS v2.3-n JC WITH PRIVACY
PROTECTION (SAC/EAC/POLYMORPHIC
      EMRTD CONFIGURATION)


         Reference: 2018_2000036361
                                          Security Target Lite
                                                                                                                                 Ref.:
                                   IDeal Pass v2.3-n JC with Privacy
                                                                                                                           2018_2000036361
                                   Protection (SAC/EAC/Polymorphic
");
        assert_eq!(find_title_security_target_lite_before(&text).unwrap(), String::from("IDEAL PASS v2.3-n JC WITH PRIVACY PROTECTION (SAC/EAC/POLYMORPHIC EMRTD CONFIGURATION)"));
        assert_eq!(find_title(&text), String::from("IDEAL PASS v2.3-n JC WITH PRIVACY PROTECTION (SAC/EAC/POLYMORPHIC EMRTD CONFIGURATION)"));

        text = String::from(
            "Security Target Lite
Common Criteria EAL6 augment ed / EAL6+




M7892 Design Step P11




Document version 2.2 as of 2020-04-23


Author: Infineon Technologies

",
        );
        assert_eq!(
            find_title_security_target_lite_before(&text).unwrap(),
            String::from("M7892 Design Step P11")
        );
        assert_eq!(find_title(&text), String::from("M7892 Design Step P11"));

        text = String::from(
            "Security Target Mercury ePassport v2.20




Revision: 3.5




CC Document  ",
        );
        assert_eq!(
            find_title_security_target_lite_before(&text).unwrap(),
            String::from("Mercury ePassport v2.20")
        );
        assert_eq!(find_title(&text), String::from("Mercury ePassport v2.20"));

        text = String::from(
            "Public




   Common Criteria Public Security Target
   EAL6 augmented / EAL6+
   IFX_CCI_000003h
   IFX_CCI_000005h
   IFX_CCI_000008h
   IFX_CCI_00000Ch
   IFX_CCI_000013h
   IFX_CCI_000014h
   IFX_CCI_000015h
   IFX_CCI_00001Ch
   IFX_CCI_00001Dh
   IFX_CCI_000021h
   IFX_CCI_000022h

   H13

   Resistance to attackers with HIGH attack potential
                                    Including optional Software Libraries
                            Flash Loader – 3x ACL – 4x HSL – 2x SCL – NRG – CCL




 Author: Infineon Technologies
 Revision: 1.8
",
        );
        assert_eq!(find_title_security_target_lite_before(&text).unwrap(), String::from("IFX_CCI_000003h IFX_CCI_000005h IFX_CCI_000008h IFX_CCI_00000Ch IFX_CCI_000013h IFX_CCI_000014h IFX_CCI_000015h IFX_CCI_00001Ch IFX_CCI_00001Dh IFX_CCI_000021h IFX_CCI_000022h"));
        assert_eq!(find_title(&text), String::from("IFX_CCI_000003h IFX_CCI_000005h IFX_CCI_000008h IFX_CCI_00000Ch IFX_CCI_000013h IFX_CCI_000014h IFX_CCI_000015h IFX_CCI_00001Ch IFX_CCI_00001Dh IFX_CCI_000021h IFX_CCI_000022h"));
    }

    #[test]
    fn find_title_security_target_after_test() {

        let mut text = String::from(
            "  NXP Secure Smart Card
  Controller P6022y VB
  Security Target Lite
  Rev. 2.1 — 6 April 2018                                        Evaluation document
  BSI-DSZ-CC-1059     ",
        );
        assert_eq!(
            find_title_security_target_after(&text).unwrap(),
            String::from("NXP Secure Smart Card Controller P6022y VB")
        );
        assert_eq!(
            find_title(&text),
            String::from("NXP Secure Smart Card Controller P6022y VB")
        );

        text = String::from(
            "   NXP Secure Smart Card Controller
   P60D024/016/012yVB(Y/Z/A)/yVF
   Security Target Lite
   Rev. 4.4 — 29 October 2018   ",
        );
        assert_eq!(
            find_title_security_target_after(&text).unwrap(),
            String::from("NXP Secure Smart Card Controller P60D024/016/012yVB(Y/Z/A)/yVF")
        );
        assert_eq!(
            find_title(&text),
            String::from("NXP Secure Smart Card Controller P60D024/016/012yVB(Y/Z/A)/yVF")
        );

        text = String::from(
            "   NXP Secure Smart Card Controller
   P60D024/016/012yVB(Y/Z/A)/yVF
   Security Target Lite
   Rev. 4.4 — 29 October 2018   ",
        );
        assert_eq!(
            find_title_security_target_after(&text).unwrap(),
            String::from("NXP Secure Smart Card Controller P60D024/016/012yVB(Y/Z/A)/yVF")
        );
        assert_eq!(
            find_title(&text),
            String::from("NXP Secure Smart Card Controller P60D024/016/012yVB(Y/Z/A)/yVF")
        );

        text = String::from(
            "  JCOP 4.7 SE051
  Security Target Lite
  Rev. 1.3 — 30 June 2020  ",
        );
        assert_eq!(
            find_title_security_target_after(&text).unwrap(),
            String::from("JCOP 4.7 SE051")
        );
        assert_eq!(find_title(&text), String::from("JCOP 4.7 SE051"));

        text = String::from(
            "       NXP JCOP4.x on P73N2M0B0.2C2/2C6
       Secure Element
       Security Target Lite
       Rev. 3.2 – 2020-09-04   ",
        );
        assert_eq!(
            find_title_security_target_after(&text).unwrap(),
            String::from("NXP JCOP4.x on P73N2M0B0.2C2/2C6 Secure Element")
        );
        assert_eq!(
            find_title(&text),
            String::from("NXP JCOP4.x on P73N2M0B0.2C2/2C6 Secure Element")
        );

        text = String::from(
            "  NXP eDoc Suite v3.5 on JCOP4 P71 –
     cryptovision ePasslet Suite ––
Java Card applet configuration providing
 Secure Signature Creation Device with
           key import (SSCD)

                Security Target Lite
                      NSCIB-CC-00229287

                Common Criteria / ISO 15408 / EAL 5+",
        );
        assert_eq!(find_title_security_target_after(&text).unwrap(), String::from("NXP eDoc Suite v3.5 on JCOP4 P71 – cryptovision ePasslet Suite –– Java Card applet configuration providing Secure Signature Creation Device with key import (SSCD)"));
        assert_eq!(find_title(&text), String::from("NXP eDoc Suite v3.5 on JCOP4 P71 – cryptovision ePasslet Suite –– Java Card applet configuration providing Secure Signature Creation Device with key import (SSCD)"));

        text = String::from(
            "       MF2DL(H)x0, MF2ID(H)10, NT4H2x21Gf
       and NT4H2x21Tf
       Security Target Lite
       Rev. 1.0 – 2018-12-31  ",
        );
        assert_eq!(
            find_title_security_target_after(&text).unwrap(),
            String::from("MF2DL(H)x0, MF2ID(H)10, NT4H2x21Gf and NT4H2x21Tf")
        );
        assert_eq!(
            find_title(&text),
            String::from("MF2DL(H)x0, MF2ID(H)10, NT4H2x21Gf and NT4H2x21Tf")
        );

        text = String::from(
            "IFX_CCI_00002Dh, IFX_CCI_000039h,
IFX_CCI_00003Ah, IFX_CCI_000044h,
IFX_CCI_000045h, IFX_CCI_000046h,
IFX_CCI_000047h, IFX_CCI_000048h,
IFX_CCI_000049h, IFX_CCI_00004Ah,
IFX_CCI_00004Bh, IFX_CCI_00004Ch,
IFX_CCI_00004Dh, IFX_CCI_00004Eh T11
Security Target Lite




Revision: v4.0",
        );
        assert_eq!(find_title_security_target_after(&text).unwrap(), String::from("IFX_CCI_00002Dh, IFX_CCI_000039h, IFX_CCI_00003Ah, IFX_CCI_000044h, IFX_CCI_000045h, IFX_CCI_000046h, IFX_CCI_000047h, IFX_CCI_000048h, IFX_CCI_000049h, IFX_CCI_00004Ah, IFX_CCI_00004Bh, IFX_CCI_00004Ch, IFX_CCI_00004Dh, IFX_CCI_00004Eh T11"));
        assert_eq!(find_title(&text), String::from("IFX_CCI_00002Dh, IFX_CCI_000039h, IFX_CCI_00003Ah, IFX_CCI_000044h, IFX_CCI_000045h, IFX_CCI_000046h, IFX_CCI_000047h, IFX_CCI_000048h, IFX_CCI_000049h, IFX_CCI_00004Ah, IFX_CCI_00004Bh, IFX_CCI_00004Ch, IFX_CCI_00004Dh, IFX_CCI_00004Eh T11"));

        text = String::from(
            "  NXP eDoc Suite v3.5 on JCOP4 P71 –
      cryptovision ePasslet Suite –
Java Card applet configuration providing
 Secure Signature Creation Device with
         Key generation (SSCD)

                Security Target Lite
                      NSCIB-CC-00229286",
        );

        assert_eq!(find_title_security_target_after(&text).unwrap(), String::from("NXP eDoc Suite v3.5 on JCOP4 P71 – cryptovision ePasslet Suite – Java Card applet configuration providing Secure Signature Creation Device with Key generation (SSCD)"));
        assert_eq!(find_title(&text), String::from("NXP eDoc Suite v3.5 on JCOP4 P71 – cryptovision ePasslet Suite – Java Card applet configuration providing Secure Signature Creation Device with Key generation (SSCD)"));


        text = String::from(
            "PUBLIC




    IFX_CCI_000Dh, IFX_CCI_0020h, IFX_CCI_0031h,
    IFX_CCI_0032h, IFX_CCI_0034h, IFX_CCI_0037h
    T31 and M31
    Security Target Lite

",
        );

        assert_eq!(find_title_security_target_after(&text).unwrap(), String::from("IFX_CCI_000Dh, IFX_CCI_0020h, IFX_CCI_0031h, IFX_CCI_0032h, IFX_CCI_0034h, IFX_CCI_0037h T31 and M31"));
        assert_eq!(find_title(&text), String::from("IFX_CCI_000Dh, IFX_CCI_0020h, IFX_CCI_0031h, IFX_CCI_0032h, IFX_CCI_0034h, IFX_CCI_0037h T31 and M31"));

        text = String::from(
            "STMicroelectronics



                   jePASS
        BAC Security Target
                       Lite
                     Common Criteria for IT security
                                         evaluation
                                                   Rev. C
                                          27-October-2020




                                             P a g e 1 | 80");

        assert_eq!(find_title_security_target_after(&text).unwrap(), String::from("jePASS BAC"));
        assert_eq!(find_title(&text), String::from("jePASS BAC"));

    }

    #[test]
    fn find_title_first_lines_test() {
        let text = String::from("ePassport configuration of SECORA™ ID S Infineon Applet Collection - eMRTD V1.0




CC Document    Please read the Important Notice and Warnings at the end of this document   1.2
www.infineon.com                                                                           2020-05-27
Table of Contents");


        assert_eq!(find_title_first_lines(&text).unwrap(), String::from("ePassport configuration of SECORA™ ID S Infineon Applet Collection - eMRTD V1.0"));
        assert_eq!(find_title(&text), String::from("ePassport configuration of SECORA™ ID S Infineon Applet Collection - eMRTD V1.0"));
    }

}
