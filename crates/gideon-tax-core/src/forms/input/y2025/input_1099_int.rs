use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form 1099-INT (2025), Interest Income.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Input1099Int {
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
    /// Payer's routing transit number
    pub payer_routing_transit_num: String,
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
    // Boxes 1–13: Interest income detail
    // =====================================================================
    /// Box 1: Interest income
    pub interest_income_amt: Usd,
    /// Box 2: Early withdrawal penalty
    pub early_withdrawal_penalty_amt: Usd,
    /// Box 3: Interest on U.S. Savings Bonds and Treasury obligations
    pub us_savings_bonds_treas_oblig_int_amt: Usd,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: Investment expenses
    pub investment_expense_amt: Usd,
    /// Box 6: Foreign tax paid
    pub foreign_taxes_paid_amt: Usd,
    /// Box 7: Foreign country or U.S. possession
    pub foreign_country_or_us_possession_cd: String,
    /// Box 8: Tax-exempt interest
    pub tax_exempt_interest_amt: Usd,
    /// Box 9: Specified private activity bond interest
    pub spcfd_prvt_acty_bond_interest_amt: Usd,
    /// Box 10: Market discount
    pub market_discount_amt: Usd,
    /// Box 11: Bond premium
    pub bond_premium_amt: Usd,
    /// Box 12: Bond premium on Treasury obligations
    pub treasury_oblig_bond_premium_amt: Usd,
    /// Box 13: Bond premium on tax-exempt bond
    pub tax_exempt_bond_premium_amt: Usd,
    /// Tax-exempt tax credit bond CUSIP number
    pub tax_exempt_tax_credit_bond_cusip_num: String,

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

impl Form for Input1099Int {
    fn name() -> &'static str {
        "Form 1099-INT"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1099Int {}
