use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form 1099-DIV (2025), Dividends and Distributions.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Input1099Div {
    /// Corrected indicator
    pub corrected_ind: bool,
    /// Calendar year
    pub calendar_yr: String,

    // =====================================================================
    // Payer identification
    // =====================================================================
    /// Payer's name — business name line 1
    pub payer_name_line_1_txt: String,
    /// Payer's name — business name line 2
    pub payer_name_line_2_txt: String,
    /// Payer name control text
    pub payer_name_control_txt: String,
    /// Payer's EIN
    pub payer_ein: String,
    /// Payer's US address
    pub payer_us_address: String,
    /// Payer's foreign address
    pub payer_foreign_address: String,
    /// Payer's telephone number
    pub phone_num: String,
    /// Foreign phone number
    pub foreign_phone_num: String,

    // =====================================================================
    // Recipient identification
    // =====================================================================
    /// Recipient's SSN
    pub recipient_ssn: String,
    /// Recipient's EIN
    pub recipient_ein: String,
    /// Recipient's name
    pub recipient_nm: String,
    /// Recipient's US address line 1
    pub recipient_us_address_line_1_txt: String,
    /// Recipient's US address line 2
    pub recipient_us_address_line_2_txt: String,
    /// Recipient's foreign address line 1
    pub recipient_foreign_address_line_1_txt: String,
    /// Recipient's foreign address line 2
    pub recipient_foreign_address_line_2_txt: String,
    /// Recipient's foreign country code
    pub recipient_foreign_country_cd: String,
    /// Recipient's account number
    pub recipient_account_num: String,
    /// FATCA filing requirement indicator
    pub fatca_filing_requirement_ind: bool,

    // =====================================================================
    // Boxes 1–13: Dividends and distributions
    // =====================================================================
    /// Box 1a: Total ordinary dividends
    pub total_ordinary_dividends_amt: Usd,
    /// Box 1b: Qualified dividends
    pub qualified_dividends_amt: Usd,
    /// Box 2a: Total capital gain distributions
    pub total_capital_distributions_amt: Usd,
    /// Box 2b: Unrecaptured section 1250 gain
    pub unrecaptured_section_1250_gain_amt: Usd,
    /// Box 2c: Section 1202 gain
    pub capital_gain_sect_1202_amt: Usd,
    /// Box 2d: Collectibles (28%) gain
    pub collectibles_28_percent_gain_amt: Usd,
    /// Box 2e: Section 897 ordinary dividends
    pub section_897_ordinary_dividends_amt: Usd,
    /// Box 2f: Section 897 capital gain
    pub section_897_capital_gain_amt: Usd,
    /// Box 3: Nondividend distributions
    pub nondividend_distributions_amt: Usd,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: Section 199A dividends
    pub section_199a_dividends_amt: Usd,
    /// Box 6: Investment expenses
    pub investment_expense_amt: Usd,
    /// Box 7: Foreign tax paid
    pub foreign_taxes_paid_amt: Usd,
    /// Box 8: Foreign country or U.S. possession
    pub foreign_country_or_us_possession_cd: String,
    /// Box 9: Cash liquidation distributions
    pub cash_liquidation_distri_amt: Usd,
    /// Box 10: Noncash liquidation distributions
    pub noncash_liquidation_distri_amt: Usd,
    /// Box 12: Exempt-interest dividends
    pub exempt_interest_dividends_amt: Usd,
    /// Box 13: Specified private activity bond interest dividends
    pub private_activity_bond_int_div_amt: Usd,

    // =====================================================================
    // State/local (Boxes 15–17)
    // =====================================================================
    /// State abbreviation code
    pub state_abbreviation_cd: String,
    /// State identification number
    pub state_id_num: String,
    /// State tax withheld
    pub state_tax_withheld_amt: Usd,
    /// Local abbreviation code text
    pub local_abbreviation_cd_txt: String,
    /// Local tax withheld
    pub local_tax_withheld_amt: Usd,
    /// Locality name
    pub locality_nm: String,
}

impl Form for Input1099Div {
    fn name() -> &'static str {
        "Form 1099-DIV"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1099Div {}
