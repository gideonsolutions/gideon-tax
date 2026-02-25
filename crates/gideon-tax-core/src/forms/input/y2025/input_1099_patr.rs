use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form 1099-PATR (2025), Taxable Distributions Received From Cooperatives.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Input1099Patr {
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

    // =====================================================================
    // Boxes 1–7: Patronage distributions
    // =====================================================================
    /// Box 1: Patronage dividends
    pub patronage_dividends_amt: Usd,
    /// Box 2: Nonpatronage distributions
    pub nonpatronage_distributions_amt: Usd,
    /// Box 3: Per-unit retain allocations
    pub per_unit_retain_allocations_amt: Usd,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: Redeemed nonqualified notices
    pub redeemed_nonqualified_notices_amt: Usd,
    /// Box 6: Section 199A(g) deduction
    pub section_199ag_deduction_amt: Usd,
    /// Box 7: Investment credit
    pub investment_credit_amt: Usd,

    // =====================================================================
    // Boxes 8–12: Credits and deductions
    // =====================================================================
    /// Box 8: Work opportunity credit
    pub work_opportunity_credit_amt: Usd,
    /// Box 9: Form 8844 credit (Empowerment zone employment credit)
    pub form_8844_credit_amt: Usd,
    /// Box 10: Form 8896 credit (Low sulfur diesel fuel production credit)
    pub form_8896_credit_amt: Usd,
    /// EPA sulfur regulation deduction
    pub epa_sulfur_reg_deduction_amt: Usd,
    /// Form 8864 credit (Biodiesel and renewable diesel fuels credit)
    pub form_8864_credit_amt: Usd,
    /// Form 8932 credit (Differential wage payments credit)
    pub form_8932_credit_amt: Usd,
    /// Form 8941 credit (Small employer health insurance premiums credit)
    pub form_8941_credit_amt: Usd,
    /// CUSIP number or obligation description
    pub cusip_number_or_obligation_desc: String,

    // =====================================================================
    // Section 199A — Qualified business income
    // =====================================================================
    /// Specified cooperative indicator
    pub specified_cooperative_ind: bool,
    /// Section 199A(a) qualified items amount
    pub section_199aa_qualified_items_amt: Usd,
    /// Section 199A(a) SSTB items amount
    pub section_199aa_sstb_items_amt: Usd,
    /// Section 199A(b)(7) qualified payment amount
    pub section_199ab7_qualified_pymt_amt: Usd,
}

impl Form for Input1099Patr {
    fn name() -> &'static str {
        "Form 1099-PATR"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1099Patr {}
