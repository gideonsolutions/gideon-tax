use us_tax_brackets::TaxYear;

use crate::forms::election::ElectionForm;
use crate::forms::{Form, FormType};

/// Category of ministerial service for Form 4361, line 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MinisterialCategory {
    /// Christian Science practitioner
    ChristianSciencePractitioner,
    /// Ordained minister, priest, rabbi
    OrdainedMinister,
    /// Member of religious order not under a vow of poverty
    ReligiousOrderMember,
    /// Commissioned or licensed minister (see line 6)
    CommissionedOrLicensedMinister,
}

/// IRS Form 4361 (Rev. January 2011) — Application for Exemption From
/// Self-Employment Tax for Use by Ministers, Members of Religious Orders
/// and Christian Science Practitioners.
///
/// Filed once to apply for exemption from self-employment tax on
/// ministerial earnings under IRC § 1402(e). The exemption is granted
/// only if the IRS returns a copy marked "Approved." Once approved,
/// it cannot be revoked.
///
/// **Caution:** Form 4361 is *not* proof of exemption from federal income
/// tax withholding, social security tax, parsonage allowance exclusion
/// (§ 107), assignment by religious superiors, or the tax-exempt status
/// of the ordaining body.
#[derive(Debug, Clone)]
pub struct Election4361 {
    // -----------------------------------------------------------------
    // Line 1 — Applicant identification
    // -----------------------------------------------------------------
    /// Line 1: Name of taxpayer applying for exemption (as shown on Form 1040)
    pub taxpayer_nm: String,
    /// Line 1 (continued): Social security number
    pub ssn: String,
    /// Line 1 (continued): Number and street (including apt. no.)
    pub address_txt: String,
    /// Line 1 (continued): Telephone number (optional)
    pub phone_num: Option<String>,
    /// Line 1 (continued): City or town, state, and ZIP code
    pub city_state_zip_txt: String,

    // -----------------------------------------------------------------
    // Line 2 — Category
    // -----------------------------------------------------------------
    /// Line 2: Check one box indicating ministerial category
    pub ministerial_category: MinisterialCategory,

    // -----------------------------------------------------------------
    // Lines 3–4 — Ordaining body
    // -----------------------------------------------------------------
    /// Line 3: Date ordained, licensed, etc.
    pub ordination_dt: String,
    /// Line 4: Legal name of ordaining, licensing, or commissioning body
    /// or religious order
    pub ordaining_body_nm: String,
    /// Line 4 (continued): Number, street, and room or suite no.
    pub ordaining_body_address_txt: String,
    /// Line 4 (continued): Employer identification number
    pub ordaining_body_ein: String,
    /// Line 4 (continued): City or town, state, and ZIP code
    pub ordaining_body_city_state_zip_txt: String,

    // -----------------------------------------------------------------
    // Line 5 — First 2 qualifying years
    // -----------------------------------------------------------------
    /// Line 5: First 2 years after the date on line 3 that the applicant
    /// had net self-employment earnings of $400 or more, any of which
    /// came from ministerial services. Stored as a pair of tax years.
    pub first_qualifying_years: (String, String),

    // -----------------------------------------------------------------
    // Line 6 — Licensed/commissioned minister explanation
    // -----------------------------------------------------------------
    /// Line 6: If applying as a licensed or commissioned minister whose
    /// denomination also ordains ministers, how ecclesiastical powers
    /// differ from those of an ordained minister. `None` if not
    /// applicable.
    pub ecclesiastical_powers_explanation: Option<String>,

    // -----------------------------------------------------------------
    // Line 7 — Certification (implicit in signature)
    // -----------------------------------------------------------------
    /// Applicant signature date
    pub applicant_signature_dt: String,
}

impl Form for Election4361 {
    fn name() -> &'static str {
        "Form 4361"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Election
    }
}

impl ElectionForm for Election4361 {}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_ordained() -> Election4361 {
        Election4361 {
            taxpayer_nm: "Rev. John Smith".to_string(),
            ssn: "987-65-4321".to_string(),
            address_txt: "789 Parsonage Way, Apt 2".to_string(),
            phone_num: Some("555-0123".to_string()),
            city_state_zip_txt: "Springfield, IL 62701".to_string(),
            ministerial_category: MinisterialCategory::OrdainedMinister,
            ordination_dt: "06/15/2015".to_string(),
            ordaining_body_nm: "First Baptist Convention".to_string(),
            ordaining_body_address_txt: "100 Convention Blvd, Suite 200".to_string(),
            ordaining_body_ein: "12-3456789".to_string(),
            ordaining_body_city_state_zip_txt: "Nashville, TN 37201".to_string(),
            first_qualifying_years: ("2016".to_string(), "2017".to_string()),
            ecclesiastical_powers_explanation: None,
            applicant_signature_dt: "03/15/2025".to_string(),
        }
    }

    fn sample_licensed() -> Election4361 {
        Election4361 {
            taxpayer_nm: "Pastor Jane Doe".to_string(),
            ssn: "111-22-3333".to_string(),
            address_txt: "321 Ministry Dr".to_string(),
            phone_num: None,
            city_state_zip_txt: "Dallas, TX 75201".to_string(),
            ministerial_category: MinisterialCategory::CommissionedOrLicensedMinister,
            ordination_dt: "09/01/2018".to_string(),
            ordaining_body_nm: "Community Church Fellowship".to_string(),
            ordaining_body_address_txt: "500 Fellowship Rd".to_string(),
            ordaining_body_ein: "98-7654321".to_string(),
            ordaining_body_city_state_zip_txt: "Dallas, TX 75202".to_string(),
            first_qualifying_years: ("2019".to_string(), "2020".to_string()),
            ecclesiastical_powers_explanation: Some(
                "Licensed ministers may perform all functions of ordained ministers \
                 except ordaining other ministers."
                    .to_string(),
            ),
            applicant_signature_dt: "04/15/2025".to_string(),
        }
    }

    #[test]
    fn form_metadata() {
        assert_eq!(Election4361::name(), "Form 4361");
        assert_eq!(Election4361::form_type(), FormType::Election);
        let form = sample_ordained();
        assert_eq!(form.year(), TaxYear::Y2025);
    }

    #[test]
    fn ordained_minister_fields() {
        let form = sample_ordained();
        assert_eq!(
            form.ministerial_category,
            MinisterialCategory::OrdainedMinister
        );
        assert_eq!(form.taxpayer_nm, "Rev. John Smith");
        assert_eq!(form.ordaining_body_nm, "First Baptist Convention");
        assert_eq!(
            form.first_qualifying_years,
            ("2016".to_string(), "2017".to_string())
        );
        assert!(form.ecclesiastical_powers_explanation.is_none());
        assert!(form.phone_num.is_some());
    }

    #[test]
    fn licensed_minister_has_explanation() {
        let form = sample_licensed();
        assert_eq!(
            form.ministerial_category,
            MinisterialCategory::CommissionedOrLicensedMinister
        );
        assert!(form.ecclesiastical_powers_explanation.is_some());
        assert!(form.phone_num.is_none());
    }

    #[test]
    fn all_categories() {
        // Ensure all enum variants are constructible
        let categories = [
            MinisterialCategory::ChristianSciencePractitioner,
            MinisterialCategory::OrdainedMinister,
            MinisterialCategory::ReligiousOrderMember,
            MinisterialCategory::CommissionedOrLicensedMinister,
        ];
        assert_eq!(categories.len(), 4);
    }
}
