use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form 1099-OID (2025), Original Issue Discount.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Input1099Oid {
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
    // Boxes 1–11: OID details
    // =====================================================================
    /// CUSIP number or obligation description
    pub cusip_number_or_obligation_desc: String,
    /// Box 1: Original issue discount for the year
    pub original_issue_discount_amt: Usd,
    /// Box 2: Other periodic interest
    pub other_periodic_interest_amt: Usd,
    /// Box 3: Early withdrawal penalty
    pub early_withdrawal_penalty_amt: Usd,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: Market discount
    pub market_discount_amt: Usd,
    /// Box 6: Acquisition premium
    pub acquisition_premium_amt: Usd,
    /// Box 8: OID on U.S. Treasury obligations
    pub treasury_obligation_oid_amt: Usd,
    /// Box 9: Investment expenses
    pub investment_expense_amt: Usd,
    /// Box 10: Bond premium
    pub bond_premium_amt: Usd,
    /// Box 11: Tax-exempt OID
    pub tax_exempt_oid_amt: Usd,

    // =====================================================================
    // State/local (Boxes 13–15)
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

impl Form for Input1099Oid {
    fn name() -> &'static str {
        "Form 1099-OID"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1099Oid {}
