use regex::Regex;
use regex::Match;

/*
notes:
- Security Target Lite `M7892 B11 Recertification` Common Criteria
- The product `Infineon Security Controller M7893 B11 with optional RSA2048/4096
v2.03.008, EC v2.03.008, SHA-2 v1.01, SCL v2.02.010 libraries and Toolbox v2.03.008
and with specific IC dedicated software (firmware)`
    = The Target of Evaluation (TOE) is an `Infineon Security Controller M7893 B11 with optional
Software Library RSA2048/4096 v2.03.008, EC v2.03.008, SHA-2 v1.01, SCL v2.02.010
libraries, and Toolbox v2.03.008, as well as with specific IC dedicated software (firmware)`
- Security Target Lite of the
                    `NXP Secure Smart Card Controller P60D024/016/012yVB(Y/Z/A)/yVF`
- Target of Evaluation (TOE) for this evaluation is the `NXP eDoc Suite v3.5 on JCOP4 P71 -
       cryptovision ePasslet Suite – Java Card applet configuration providing Machine Readable Travel
         Document with „ICAO Application”, Extended Access Control with PACE` from cv cryptovision GmbH
         located in Gelsenkirchen, Germany

 */

fn find(regex: regex::Regex, text: &str) -> Match {
    let results = regex.find(&text).unwrap();
    results
}

fn replace_whitespace_with_space(text: &str) -> String {
    let re = Regex::new(r"\s+").unwrap();
    let result = re.replace_all(text, " ");
    String::from(result)
}

pub fn find_title(text: &str) {
    let result = find(
        Regex::new(r"(^|\s)\w{3}-\w{3}-\w{2}-\d{4}-\w{2}-\d{4}\s*for\s*").unwrap(),
        &text,
    );
    //result
}

pub fn find_title_for_from(text: &String) -> String {
    let mut result = find(
        Regex::new(r"(^|\s)\w{3}-\w{3}-\w{2}-\d{4}(-\w{2})?-\d{4}\s*for\s*").unwrap(),
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
    //result
    replace_whitespace_with_space(title)
}


pub fn find_title_security_target_lite_before(text: &String) -> String {
    let mut result = find(
        Regex::new(r"(^|\s)Security Target( Lite)?\s+").unwrap(),
        &text,
    );
    let (_, title_start) = text.split_at(result.end());
    result = find(
        Regex::new(r"\s+Common Criteria").unwrap(),
        &title_start,
    );
    let (title, _) = title_start.split_at(result.start());

    //result
    replace_whitespace_with_space(title)
}

pub fn find_title_security_target_after(text: &String) -> String {
    let mut result = find(
        Regex::new(r"(^|\s)(\w|\s)+Security Target\s+").unwrap(),
        &text,
    );
    let (title_start_space, _) = text.split_at(result.end());

    result = find(
        Regex::new(r"(^|\s)\s*(PUBLIC)?\s*").unwrap(),
        &text,
    );
    let (_, title_start) = title_start_space.split_at(result.end());

    result = find(
        Regex::new(r"\s+Security Target").unwrap(),
        &title_start,
    );
    let (title, _) = title_start.split_at(result.start());

    //result
    replace_whitespace_with_space(title)
}


pub fn find_title_certification_report(text: &String) -> String {
    let mut result = find(
        Regex::new(r"(^|\s)(\w|\s)+Certification Report\s+Version \d{4}-\d+\s+").unwrap(),
        &text,
    );
    let (_, title_from) = text.split_at(result.end());
    println!("{}", title_from);

    result = find(
        Regex::new(r"\s+Sponsor( and developer)?:").unwrap(),
        &title_from,
    );
    let (title, _) = title_from.split_at(result.start());

    //result
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


        text = String::from("             BSI-DSZ-CC-1022-2018

                       for

NXP eDoc Suite v3.0 - cryptovision ePasslet Suite
   - Java Card applet configuration providing
 Machine Readable Travel Document with \"ICAO
        Application\", Extended Access Control with PACE


                      from

      NXP Semiconductors Germany GmbH");

        assert_eq!(find_title_for_from(&text),String::from("NXP eDoc Suite v3.0 - cryptovision ePasslet Suite - Java Card applet configuration providing Machine Readable Travel Document with \"ICAO Application\", Extended Access Control with PACE"));



        text = String::from("          BSI-DSZ-CC-1025-V3-2020

                      for

IFX_CCI_000011h, 00001Bh, 00001Eh, 000025h,
design step G12 with optional libraries and with
        specific IC dedicated software

                     from

           Infineon Technologies AG
                     BSI - Bundes");

        assert_eq!(find_title_for_from(&text),String::from("IFX_CCI_000011h, 00001Bh, 00001Eh, 000025h, design step G12 with optional libraries and with specific IC dedicated software"));

        text = String::from("            BSI-DSZ-CC-1051-2019

                     for

NXP Smart Card Controller P61N1M3VD/VD-1/PVE-1
           with IC Dedicated Software

                     from

      NXP Semiconductors Germany GmbH
                     BSI - Bundesamt für Sicherheit in der Informationstechnik, Postfach 20 03 63");

        assert_eq!(find_title_for_from(&text),String::from("NXP Smart Card Controller P61N1M3VD/VD-1/PVE-1 with IC Dedicated Software"));


        text = String::from("          BSI-DSZ-CC-1072-2018

                    for

NXP Secure Smart Card Controller P6021y VB*
      including IC Dedicated Software

                   from

   NXP Semiconductors Germany GmbH");

        assert_eq!(find_title_for_from(&text),String::from("NXP Secure Smart Card Controller P6021y VB* including IC Dedicated Software"));

        text = String::from("       BSI-DSZ-CC-1098-2020

                for

IDEMIA_HC_Germany_NEO_G2.1_COS, V1

               from

       IDEMIA Germany GmbH");

        assert_eq!(find_title_for_from(&text),String::from("IDEMIA_HC_Germany_NEO_G2.1_COS, V1"));

        text = String::from("             BSI-DSZ-CC-1107-2020

                       for

      IFX_CCI_00002Dh, IFX_CCI_000039h,
      IFX_CCI_00003Ah, IFX_CCI_000044h,
       IFX_CCI_000045h, IFX_CCI_000046h,
       IFX_CCI_000047h, IFX_CCI_000048h,
      IFX_CCI_000049h, IFX_CCI_00004Ah,
      IFX_CCI_00004Bh, IFX_CCI_00004Ch,
IFX_CCI_00004Dh, IFX_CCI_00004Eh design step
T11 with firmware 80.306.16.0, optional NRG™ SW
 05.03.4097, optional HSL v3.52.9708, UMSLC lib
v01.30.0564, optional SCL v2.11.003, optional ACL
            v3.02.000 and user guidance

                      from

           Infineon Technologies AG");

        assert_eq!(find_title_for_from(&text),String::from("IFX_CCI_00002Dh, IFX_CCI_000039h, IFX_CCI_00003Ah, IFX_CCI_000044h, IFX_CCI_000045h, IFX_CCI_000046h, IFX_CCI_000047h, IFX_CCI_000048h, IFX_CCI_000049h, IFX_CCI_00004Ah, IFX_CCI_00004Bh, IFX_CCI_00004Ch, IFX_CCI_00004Dh, IFX_CCI_00004Eh design step T11 with firmware 80.306.16.0, optional NRG™ SW 05.03.4097, optional HSL v3.52.9708, UMSLC lib v01.30.0564, optional SCL v2.11.003, optional ACL v3.02.000 and user guidance"));

        text = String::from("          BSI-DSZ-CC-1110-V2-2019

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

          Infineon Technologies AG");

        assert_eq!(find_title_for_from(&text),String::from("Infineon Security Controller IFX_CCI_000003h, 000005h, 000008h, 00000Ch, 000013h, 000014h, 000015h, 00001Ch, 00001Dh, 000021h, 000022h H13 including the products from the second production line and optional software packages: Flash Loader, Asymmetric Crypto Library, Symmetric Cryptographic Library, Hardware Support Layer, Hash Crypto Library, Mifare Compatible Software, and CIPURSE™ Crypto Library"));



        text = String::from("             BSI-DSZ-CC-1126-2019

                       for

IFX_CCI_000Dh, IFX_CCI_0020h, IFX_CCI_0031h,
 IFX_CCI_0032h, IFX_CCI_0034h, IFX_CCI_0037h
   design step T31 and M31 with optional HSL
   v2.62.7626, optional SCL version v2.04.003,
UMSLC lib v01.00.0234 with specific IC-dedicated
firmware identifier 80.301.05.1 and user guidance

                      from

           Infineon Technologies AG");

        assert_eq!(find_title_for_from(&text),String::from("IFX_CCI_000Dh, IFX_CCI_0020h, IFX_CCI_0031h, IFX_CCI_0032h, IFX_CCI_0034h, IFX_CCI_0037h design step T31 and M31 with optional HSL v2.62.7626, optional SCL version v2.04.003, UMSLC lib v01.00.0234 with specific IC-dedicated firmware identifier 80.301.05.1 and user guidance"));

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
        assert_eq!(find_title_certification_report(&text),String::from("Crypto Library V3.1.x on P6022y VB"));


        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2019-1




                                                                                                                    CombICAO Applet in EAC configuration on ID-ONE Cosmo
                                                                                                                                        V9 Essential



                                                                                                                  Sponsor and developer:           IDEMIA");
        assert_eq!(find_title_certification_report(&text),String::from("CombICAO Applet in EAC configuration on ID-ONE Cosmo V9 Essential"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-2




                                                                                                                                                                  JCOP 4 P71




                                                                                                                  Sponsor and developer:           NXP Semiconductors Germany GmbH");
        assert_eq!(find_title_certification_report(&text),String::from("JCOP 4 P71"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2019-1




                                                                                                                    CombICAO Applet in BAC and CA configuration on ID-ONE
                                                                                                                                     Cosmo V9 Essential



                                                                                                                  Sponsor and developer:           IDEMIA
                                                       ");
        assert_eq!(find_title_certification_report(&text),String::from("CombICAO Applet in BAC and CA configuration on ID-ONE Cosmo V9 Essential"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-3




                                                                                                                    NXP eDoc Suite v3.5 on JCOP4 P71 - cryptovision ePasslet
                                                                                                                    Suite – Java Card applet configuration providing Machine
                                                                                                                      Readable Travel Document with „ICAO Application”,
                                                                                                                              Extended Access Control with PACE

                                                                                                                  Sponsor:                         NXP Semiconductors Germany GmbH");
        assert_eq!(find_title_certification_report(&text),String::from("NXP eDoc Suite v3.5 on JCOP4 P71 - cryptovision ePasslet Suite – Java Card applet configuration providing Machine Readable Travel Document with „ICAO Application”, Extended Access Control with PACE"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-2




                                                                                                                                                                   MF3D(H)x3




                                                                                                                  Sponsor and developer:           NXP Semiconductors Germany GmbH");
        assert_eq!(find_title_certification_report(&text),String::from("MF3D(H)x3"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2019-4




                                                                                                                                 NXP JCOP 5.2 on SN100.C58 Secure Element




                                                                                                                  Sponsor and developer:           NXP Semiconductors GmbH");
        assert_eq!(find_title_certification_report(&text),String::from("NXP JCOP 5.2 on SN100.C58 Secure Element"));

        text = String::from(" Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-2




                                                                                                                                                              JCOP 4.7 SE051




                                                                                                                  Sponsor and developer:           NXP Semiconductors Germany GmbH");
        assert_eq!(find_title_certification_report(&text),String::from("JCOP 4.7 SE051"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-2




                                                                                                                                                       Voyager ePassport v2.20




                                                                                                                  Sponsor and developer:           Infineon Technologies AG");
        assert_eq!(find_title_certification_report(&text),String::from("Voyager ePassport v2.20"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-2




                                                                                                                                                          jePASS EAC V.1.1.4




                                                                                                                  Sponsor and developer:           ST Microelectronics S.r.l");
        assert_eq!(find_title_certification_report(&text),String::from("jePASS EAC V.1.1.4"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-2




                                                                                                                            JCOP 4.x on P73N2M0B0.2C2/2C6 Secure Element




                                                                                                                  Sponsor and developer:           NXP Semiconductors GmbH");
        assert_eq!(find_title_certification_report(&text),String::from("JCOP 4.x on P73N2M0B0.2C2/2C6 Secure Element"));
        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-3




                                                                                                                            SN200 Series - Secure Element with Crypto Library
                                                                                                                                           SN200_SE B1.1 C04



                                                                                                                  Sponsor and developer:           NXP Semiconductors Germany GmbH");
        assert_eq!(find_title_certification_report(&text),String::from("SN200 Series - Secure Element with Crypto Library SN200_SE B1.1 C04"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-3




                                                                                                                        NXP eDoc Suite v3.5 on JCOP4 71 - cryptovision ePasslet
                                                                                                                        Suite – Java Card applet configuration providing Secure
                                                                                                                           Signature Creation Device with key import (SSCD)


                                                                                                                  Sponsor:                         NXP Semiconductors Germany GmbH
                                                                                                                                                   Troplowitzstrasse 20");
        assert_eq!(find_title_certification_report(&text),String::from("NXP eDoc Suite v3.5 on JCOP4 71 - cryptovision ePasslet Suite – Java Card applet configuration providing Secure Signature Creation Device with key import (SSCD)"));

        text = String::from("                                                                                                 TÜV Rheinland Nederland B.V.




                                                                                                                                                          Certification Report
Version 2020-3




                                                                                                                        NXP eDoc Suite v3.5 on JCOP4 71 - cryptovision ePasslet
                                                                                                                        Suite – Java Card applet configuration providing Secure
                                                                                                                           Signature Creation Device with key import (SSCD)


                                                                                                                  Sponsor:                         NXP Semiconductors Germany GmbH
                                                                                                                                                   Troplowitzstrasse 20");
        assert_eq!(find_title_certification_report(&text),String::from("NXP eDoc S"));

    }





    #[test]
    fn find_title_security_target_lite_before_test() {
        let mut text = String::from("   Security Target Lite

   M7892 B11

   Recertification

   Common Criteria CCv3.1 EAL6 augmented (EAL6+)

   Resistance to attackers with HIGH attack potential



");
        assert_eq!(find_title_security_target_lite_before(&text),String::from("M7892 B11 Recertification"));

        text = String::from("Security Target

OPTIGATM Trusted Platform Module
SLB9670_2.0 v7.85
Common Criteria CCv3.1 EAL4 augmented (EAL4+)
Resistance to attackers with MODERATE attack potential");
        assert_eq!(find_title_security_target_lite_before(&text),String::from("OPTIGATM Trusted Platform Module SLB9670_2.0 v7.85"));


    }


    #[test]
    fn find_title_security_target_after_test() {
        let mut text = String::from("   ChipDoc v3 on JCOP 4 P71 in
   ICAO EAC with PACE
   configuration
   Security Target Lite
   Rev. 1.0 — 12 February 2020                             Evaluation documentation
   Final ");
        assert_eq!(find_title_security_target_after(&text),String::from("ChipDoc v3 on JCOP 4 P71 in ICAO EAC with PACE configuration"));

        text = String::from("  NXP Secure Smart Card
  Controller P6022y VB
  Security Target Lite
  Rev. 2.1 — 6 April 2018                                        Evaluation document
  BSI-DSZ-CC-1059     ");
        assert_eq!(find_title_security_target_after(&text),String::from("NXP Secure Smart Card Controller P6022y VB"));

    text = String::from("  NXP Secure Smart Card
  Controller N7121 with IC
  Dedicated Software and Crypto
  Library
  Security Target Lite
  Rev. 1.1 — 31 May 2019  ");
    assert_eq!(find_title_security_target_after(&text),String::from("NXP Secure Smart Card Controller N7121 with IC Dedicated Software and Crypto Library"));

        text = String::from("  Crypto Library Cobalt on N7021
  VA
  Security Target Lite
  Rev. 2.3 — 5 June 2019                               Product evaluation document
  BSI-DSZ-CC-1019-V2  ");
        assert_eq!(find_title_security_target_after(&text),String::from("Crypto Library Cobalt on N7021 VA"));

        text = String::from("       NXP Secure Smart Card Controller
       N7021 VA
       Security Target Lite
       Rev. 2.3 – 2019-06-04 ");
        assert_eq!(find_title_security_target_after(&text),String::from("NXP Secure Smart Card Controller N7021 VA"));

        text = String::from("   NXP Secure Smart Card Controller
   P60D024/016/012yVB(Y/Z/A)/yVF
   Security Target Lite
   Rev. 4.4 — 29 October 2018   ");
        assert_eq!(find_title_security_target_after(&text),String::from("NXP Secure Smart Card Controller P60D024/016/012yVB(Y/Z/A)/yVF"));

        text = String::from("   NXP Secure Smart Card Controller
   P60D024/016/012yVB(Y/Z/A)/yVF
   Security Target Lite
   Rev. 4.4 — 29 October 2018   ");
        assert_eq!(find_title_security_target_after(&text),String::from("NXP Secure Smart Card Controller P60D024/016/012yVB(Y/Z/A)/yVF"));

        text = String::from("  JCOP 4.7 SE051
  Security Target Lite
  Rev. 1.3 — 30 June 2020  ");
        assert_eq!(find_title_security_target_after(&text),String::from("JCOP 4.7 SE051"));

        text = String::from("       NXP JCOP4.x on P73N2M0B0.2C2/2C6
       Secure Element
       Security Target Lite
       Rev. 3.2 – 2020-09-04   ");
        assert_eq!(find_title_security_target_after(&text),String::from("NXP JCOP4.x on P73N2M0B0.2C2/2C6 Secure Element"));

        text = String::from("  NXP eDoc Suite v3.5 on JCOP4 P71 –
     cryptovision ePasslet Suite ––
Java Card applet configuration providing
 Secure Signature Creation Device with
           key import (SSCD)

                Security Target Lite
                      NSCIB-CC-00229287

                Common Criteria / ISO 15408 / EAL 5+");
        assert_eq!(find_title_security_target_after(&text),String::from("NXP eDoc Suite v3.5 on JCOP4 P71 – cryptovision ePasslet Suite –– Java Card applet configuration providing Secure Signature Creation Device with key import (SSCD)"));

        text = String::from("       MF2DL(H)x0, MF2ID(H)10, NT4H2x21Gf
       and NT4H2x21Tf
       Security Target Lite
       Rev. 1.0 – 2018-12-31  ");
        assert_eq!(find_title_security_target_after(&text),String::from("MF2DL(H)x0, MF2ID(H)10, NT4H2x21Gf and NT4H2x21Tf"));

        text = String::from("IFX_CCI_00002Dh, IFX_CCI_000039h,
IFX_CCI_00003Ah, IFX_CCI_000044h,
IFX_CCI_000045h, IFX_CCI_000046h,
IFX_CCI_000047h, IFX_CCI_000048h,
IFX_CCI_000049h, IFX_CCI_00004Ah,
IFX_CCI_00004Bh, IFX_CCI_00004Ch,
IFX_CCI_00004Dh, IFX_CCI_00004Eh T11
Security Target Lite




Revision: v4.0");
        assert_eq!(find_title_security_target_after(&text),String::from("IFX_CCI_00002Dh, IFX_CCI_000039h, IFX_CCI_00003Ah, IFX_CCI_000044h, IFX_CCI_000045h, IFX_CCI_000046h, IFX_CCI_000047h, IFX_CCI_000048h, IFX_CCI_000049h, IFX_CCI_00004Ah, IFX_CCI_00004Bh, IFX_CCI_00004Ch, IFX_CCI_00004Dh, IFX_CCI_00004Eh T11"));


        text = String::from("  NXP eDoc Suite v3.5 on JCOP4 P71 –
      cryptovision ePasslet Suite –
Java Card applet configuration providing
 Secure Signature Creation Device with
         Key generation (SSCD)

                Security Target Lite
                      NSCIB-CC-00229286");

        assert_eq!(find_title_security_target_after(&text),String::from("NXP eDoc Suite v3.5 on JCOP4 P71 – cryptovision ePasslet Suite – Java Card applet configuration providing Secure Signature Creation Device with Key generation (SSCD)"));



    }
}
