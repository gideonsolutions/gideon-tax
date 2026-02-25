use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form 1099-B (2025), Proceeds From Broker and Barter Exchange Transactions.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Input1099B {
    /// Corrected indicator
    pub corrected_ind: bool,

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
    // Boxes 1–16: Transaction details
    // =====================================================================
    /// Box 1a: Description of property
    pub property_desc: String,
    /// Box 1b: Date acquired
    pub acquired_dt: String,
    /// Box 1c: Date sold or disposed
    pub sold_or_disposed_dt: String,
    /// Box 1d: Proceeds
    pub proceeds_amt: Usd,
    /// Box 1e: Cost or other basis
    pub cost_or_other_basis_amt: Usd,
    /// Box 2: Short-term gain or loss indicator
    pub short_term_gain_loss_ind: bool,
    /// Box 2: Long-term gain or loss indicator
    pub long_term_gain_loss_ind: bool,
    /// Box 2: Ordinary indicator
    pub ordinary_ind: bool,
    /// Box 3: Form 8949 applicable checkbox code
    pub form_8949_applicable_checkbox_cd: String,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: Noncovered security indicator
    pub noncovered_security_ind: bool,
    /// Box 6: Basis reported indicator
    pub basis_reported_ind: bool,
    /// Box 7: Loss not allowed indicator
    pub loss_not_allowed_ind: bool,
    /// Gross proceeds indicator
    pub gross_proceeds_ind: bool,
    /// Net proceeds indicator
    pub net_proceeds_ind: bool,
    /// Box 1f: Accrued market discount
    pub accrued_market_discount_amt: Usd,
    /// Box 1g: Wash sale loss disallowed
    pub nondeductible_wash_sale_loss_amt: Usd,
    /// CUSIP number
    pub cusip_num: String,
    /// Collectibles (28%) indicator
    pub collectibles_ind: bool,
    /// Qualified Opportunity Fund (QOF) indicator
    pub qof_ind: bool,
    /// Barter exchange amount
    pub barter_amt: Usd,

    // =====================================================================
    // Section 1256 contracts (Boxes 8–11)
    // =====================================================================
    /// Box 8: Profit or (loss) realized in current year on closed contracts
    pub ty_closed_contract_profit_loss_amt: Usd,
    /// Box 9: Unrealized profit or (loss) on open contracts — current year
    pub cy_open_cntrct_profit_loss_amt: Usd,
    /// Box 10: Unrealized profit or (loss) on open contracts — prior year
    pub prior_yr_open_cntrct_profit_loss_amt: Usd,
    /// Box 11: Aggregate profit or (loss) on contracts
    pub cntrct_aggregate_profit_loss_amt: Usd,

    // =====================================================================
    // State/local (Boxes 14–16)
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

impl Form for Input1099B {
    fn name() -> &'static str {
        "Form 1099-B"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1099B {}
