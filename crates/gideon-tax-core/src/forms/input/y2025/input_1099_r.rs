use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form 1099-R (2025), Distributions From Pensions, Annuities, Retirement or
/// Profit-Sharing Plans, IRAs, Insurance Contracts, etc.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Input1099R {
    /// Corrected indicator
    pub corrected_ind: bool,
    /// Standard or non-standard code
    pub standard_or_non_standard_cd: String,

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
    /// Payer's US address line 1
    pub payer_us_address_line_1_txt: String,
    /// Payer's US address line 2
    pub payer_us_address_line_2_txt: String,
    /// Payer's foreign address line 1
    pub payer_foreign_address_line_1_txt: String,
    /// Payer's foreign address line 2
    pub payer_foreign_address_line_2_txt: String,
    /// Payer's foreign city name
    pub payer_foreign_city_nm: String,
    /// Payer's foreign province or state name
    pub payer_foreign_province_or_state_nm: String,
    /// Payer's foreign country code
    pub payer_foreign_country_cd: String,
    /// Payer's foreign postal code
    pub payer_foreign_postal_cd: String,
    /// Payer's telephone number
    pub phone_num: String,
    /// Optional foreign telephone number
    pub optional_foreign_telephone_num: String,

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
    /// Payer's recipient account number
    pub payer_recipient_account_num: String,
    /// FATCA filing requirement indicator
    pub fatca_filing_requirement_ind: bool,

    // =====================================================================
    // Boxes 1–11: Distribution details
    // =====================================================================
    /// Box 1: Gross distribution
    pub gross_distribution_amt: Usd,
    /// Box 2a: Taxable amount
    pub taxable_amt: Usd,
    /// Box 2b: Taxable amount not determined indicator
    pub txbl_amount_not_determined_ind: bool,
    /// Box 2b: Total distribution indicator
    pub total_distribution_ind: bool,
    /// Box 3: Capital gain (included in Box 2a)
    pub capital_gain_amt: Usd,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: Employee contributions / Designated Roth contributions or insurance premiums
    pub employee_contributions_amt: Usd,
    /// Box 6: Net unrealized appreciation in employer's securities
    pub net_unrlzd_securities_apprcn_amt: Usd,
    /// Box 7: Distribution code(s)
    pub f1099r_distribution_cd: String,
    /// Box 7: IRA/SEP/SIMPLE indicator
    pub ira_sep_simple_ind: bool,
    /// Box 8: Other amount
    pub other_distribution_amt: Usd,
    /// Box 8: Recipient's total distribution percentage
    pub rcpnt_total_distribution_pct: String,
    /// Box 8: Recipient's other distribution percentage
    pub rcpnt_oth_distribution_pct: String,
    /// Box 9a: Total employee contributions
    pub total_employee_contributions_amt: Usd,
    /// Box 9b: IRR allocated amount
    pub irr_allocated_amt: Usd,
    /// Box 10: Amount allocable to IRR within 5 years
    pub payment_dt: String,
    /// Box 11: Designated Roth account first year
    pub designated_roth_acct_first_yr: String,
}

impl Form for Input1099R {
    fn name() -> &'static str {
        "Form 1099-R"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1099R {}
