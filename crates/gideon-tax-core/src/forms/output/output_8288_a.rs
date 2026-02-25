use crate::Usd;

/// Output fields for IRS Form 8288-A (2025) — Statement of Withholding on Dispositions by Foreign Persons of U.S. Real Property Interests.
#[derive(Debug, Clone, Default)]
pub struct Output8288A {
    // -----------------------------------------------------------------------
    // Withholding Agent Information
    // -----------------------------------------------------------------------
    /// Withholding agent: Address line 1
    pub address_line1_txt: String,
    /// Withholding agent: Address line 2
    pub address_line2_txt: String,
    /// Withholding agent: Phone number
    pub phone_num: String,
    /// Withholding agent: U.S. TIN (EIN)
    pub transferee_ein: String,
    /// Withholding agent: Person name
    pub transferee_person_nm: String,
    /// Withholding agent: SSN
    pub transferee_ssn: String,
    /// Withholding agent: U.S. address
    pub transferee_us_address: String,
    /// Withholding agent: Foreign address
    pub transferee_foreign_address: String,

    // -----------------------------------------------------------------------
    // Foreign Person Subject to Withholding — Identification
    // -----------------------------------------------------------------------
    /// Foreign person: First name
    pub person_first_nm: String,
    /// Foreign person: Last name
    pub person_last_nm: String,
    /// Foreign person: Business name line 1
    pub business_name_line1_txt: String,
    /// Foreign person: Business name line 2
    pub business_name_line2_txt: String,
    /// Foreign person: In care of name
    pub in_care_of_nm: String,
    /// Foreign person: SSN (U.S. TIN)
    pub ssn: String,
    /// Foreign person: EIN (U.S. TIN)
    pub ein: String,
    /// Foreign person: Foreign address
    pub foreign_address: String,
    /// Foreign person: U.S. address (mailing address if different)
    pub us_address: String,
    /// Foreign person: Foreign phone number
    pub foreign_phone_num: String,

    // -----------------------------------------------------------------------
    // Boxes 1–8
    // -----------------------------------------------------------------------
    /// Box 1: Date of transfer (mm/dd/yyyy)
    pub transfer_dt: String,
    /// Box 2: Gain recognized by foreign corporation
    pub gain_recognized_on_transfer_amt: Usd,
    /// Box 3: Amount realized
    pub realized_amt: Usd,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5a: Withholding under section 1445
    pub section1445_withholding_ind: bool,
    /// Box 5b: Withholding under section 1446(f)(1)
    pub section1446f_withholding_ind: bool,
    /// Box 6a: Foreign person subject to withholding — Individual
    pub individual_ind: bool,
    /// Box 6b: Foreign person subject to withholding — Corporation
    pub corporation_ind: bool,
    /// Box 6c: Foreign person subject to withholding — Partnership
    pub partnership_ind: bool,
    /// Box 6d: Foreign person subject to withholding — Other (specify)
    pub other_withholding_type_cd: String,
    /// Box 7: Country code of foreign person subject to withholding
    pub country_cd: String,
    /// Box 8: Description of property transferred
    pub property_desc: String,
}
