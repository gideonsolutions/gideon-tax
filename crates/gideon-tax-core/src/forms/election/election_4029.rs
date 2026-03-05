use us_tax_brackets::TaxYear;

use crate::forms::election::ElectionForm;
use crate::forms::{Form, FormType};

/// IRS Form 4029 (Rev. November 2018) — Application for Exemption From
/// Social Security and Medicare Taxes and Waiver of Benefits.
///
/// Filed once by members of recognized religious groups to apply for
/// exemption from social security and Medicare taxes under IRC §§ 1402(g)(1)
/// and 3127. The exemption is granted only if the IRS returns a copy
/// marked "Approved."
///
/// **Caution:** Approval exempts the applicant from social security and
/// Medicare taxes only — not from federal income tax.
#[derive(Debug, Clone)]
pub struct Election4029 {
    // -----------------------------------------------------------------
    // Part I — To Be Completed by Applicant
    // -----------------------------------------------------------------
    /// Line 1: Name of taxpayer
    pub taxpayer_nm: String,
    /// Line 1 (continued): Address (number, street, or P.O. box)
    pub address_txt: String,
    /// Line 1 (continued): City or town, state, and ZIP code
    pub city_state_zip_txt: String,
    /// Line 2: Social security number
    pub ssn: String,
    /// Line 3: Date of birth
    pub birth_dt: String,
    /// Line 4: Contact phone number (optional)
    pub phone_num: Option<String>,
    /// Line 5: Do not send me my Social Security Statement (checkbox)
    pub opt_out_ss_statement_ind: bool,
    /// Certification: Name of religious group
    pub religious_group_nm: String,
    /// Certification: Religious district or congregation, and county
    /// and/or city, state, and ZIP code
    pub religious_district_txt: String,
    /// Certification: Membership since — month
    pub membership_since_month: String,
    /// Certification: Membership since — day
    pub membership_since_day: String,
    /// Certification: Membership since — year
    pub membership_since_year: String,
    /// Applicant signature date
    pub applicant_signature_dt: String,

    // -----------------------------------------------------------------
    // Part II — Authorized Representative of Religious Group
    // -----------------------------------------------------------------
    /// Name of taxpayer (as certified by representative)
    pub certified_taxpayer_nm: String,
    /// Name of religious group/district/congregation
    pub certified_religious_group_nm: String,
    /// Name of authorized representative
    pub authorized_rep_nm: String,
    /// Address of authorized representative
    pub authorized_rep_address_txt: String,
    /// Title of authorized representative
    pub authorized_rep_title_txt: String,
    /// Authorized representative signature date
    pub authorized_rep_signature_dt: String,
}

impl Form for Election4029 {
    fn name() -> &'static str {
        "Form 4029"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Election
    }
}

impl ElectionForm for Election4029 {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_4029() -> Election4029 {
        Election4029 {
            taxpayer_nm: "Jacob Yoder".to_string(),
            address_txt: "123 Farm Rd".to_string(),
            city_state_zip_txt: "Lancaster, PA 17601".to_string(),
            ssn: "123-45-6789".to_string(),
            birth_dt: "01/15/1980".to_string(),
            phone_num: None,
            opt_out_ss_statement_ind: true,
            religious_group_nm: "Old Order Amish".to_string(),
            religious_district_txt: "Conewango Valley North District, Cattaraugus, NY 14719"
                .to_string(),
            membership_since_month: "03".to_string(),
            membership_since_day: "15".to_string(),
            membership_since_year: "1998".to_string(),
            applicant_signature_dt: "01/10/2025".to_string(),
            certified_taxpayer_nm: "Jacob Yoder".to_string(),
            certified_religious_group_nm: "Old Order Amish / Conewango Valley North District"
                .to_string(),
            authorized_rep_nm: "Bishop Eli Miller".to_string(),
            authorized_rep_address_txt: "456 Church Ln, Lancaster, PA 17601".to_string(),
            authorized_rep_title_txt: "Bishop".to_string(),
            authorized_rep_signature_dt: "01/12/2025".to_string(),
        }
    }

    #[test]
    fn form_metadata() {
        assert_eq!(Election4029::name(), "Form 4029");
        assert_eq!(Election4029::form_type(), FormType::Election);
        let form = sample_4029();
        assert_eq!(form.year(), TaxYear::Y2025);
    }

    #[test]
    fn fields_preserved() {
        let form = sample_4029();
        assert_eq!(form.taxpayer_nm, "Jacob Yoder");
        assert_eq!(form.ssn, "123-45-6789");
        assert_eq!(form.religious_group_nm, "Old Order Amish");
        assert!(form.opt_out_ss_statement_ind);
        assert!(form.phone_num.is_none());
    }
}
