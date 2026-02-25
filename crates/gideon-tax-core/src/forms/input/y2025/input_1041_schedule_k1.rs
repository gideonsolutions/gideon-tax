use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// Input fields for IRS Schedule K-1 (Form 1041) 2025 — Beneficiary's Share of Income, Deductions, Credits, etc.
#[derive(Debug, Clone, Default)]
pub struct Input1041ScheduleK1 {
    // -----------------------------------------------------------------------
    // Top-of-form — Final/Amended indicators
    // -----------------------------------------------------------------------
    /// Final K-1 checkbox
    pub final_k1_ind: bool,
    /// Amended K-1 checkbox
    pub amended_k1_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Information About the Estate or Trust
    // -----------------------------------------------------------------------
    /// Item A: Estate's or trust's employer identification number
    pub ein: String,
    /// Item A: Missing SSN/EIN reason code
    pub missing_ssnein_reason_cd: String,
    /// Item B: Estate's or trust's name — Name line 1
    pub business_name_line1_txt: String,
    /// Item B: Estate's or trust's name — Name line 2
    pub business_name_line2_txt: String,
    /// Item C: Fiduciary's name, address, city, state, and ZIP code — In care of name
    pub in_care_of_nm: String,
    /// Item C: Fiduciary's U.S. address
    pub us_address: String,
    /// Item C: Fiduciary's foreign address
    pub foreign_address: String,
    /// Item D: Check if Form 1041-T was filed and enter the date it was filed
    pub form1041_t_filed_ind: bool,
    /// Item D: Date Form 1041-T was filed
    pub form1041_t_filed_dt: String,
    /// Item E: Check if this is the final Form 1041 for the estate or trust
    pub future_filing_not_required_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Information About the Beneficiary
    // -----------------------------------------------------------------------
    /// Item F: Beneficiary's identifying number (SSN)
    pub ssn: String,
    /// Item G: Beneficiary's name, address, city, state, and ZIP code
    pub beneficiary_person_nm: String,
    /// Item H: Domestic beneficiary checkbox
    pub domestic_beneficiary_ind: bool,
    /// Item H: Foreign beneficiary checkbox
    pub foreign_beneficiary_ind: bool,
    /// Form 1042-S code (for foreign beneficiaries)
    pub form1042_s_cd: String,

    // -----------------------------------------------------------------------
    // Part III — Beneficiary's Share of Current Year Income, Deductions, Credits, and Other Items
    // -----------------------------------------------------------------------
    /// Line 1: Interest income
    pub interest_income_amt: Usd,
    /// Line 2a: Ordinary dividends
    pub ordinary_dividends_amt: Usd,
    /// Line 2b: Qualified dividends
    pub qualified_dividends_amt: Usd,
    /// Line 3: Net short-term capital gain
    pub net_st_capital_gain_amt: Usd,
    /// Line 4a: Net long-term capital gain
    pub net_lt_capital_gain_amt: Usd,
    /// Line 4b: 28% rate gain
    pub collectibles28_percent_gain_amt: Usd,
    /// Line 4c: Unrecaptured section 1250 gain
    pub unrecaptured_section1250_gain_amt: Usd,
    /// Line 5: Other portfolio and nonbusiness income
    pub other_portfolio_income_loss_amt: Usd,
    /// Line 6: Ordinary business income
    pub ordinary_business_income_amt: Usd,
    /// Line 7: Net rental real estate income
    pub net_rental_income_real_estate_amt: Usd,
    /// Line 8: Other rental income
    pub other_rental_income_amt: Usd,
    // Line 9: Directly apportioned deductions
    // (see attached statement for codes and amounts)
    /// Line 10: Estate tax deduction
    pub estate_tax_deduction_amt: Usd,
    // Line 11: Final year deductions
    // (see attached statement for codes and amounts)

    // Line 12: Alternative minimum tax adjustment
    // (see attached statement for codes and amounts)

    // Line 13: Credits and credit recapture
    // (see attached statement for codes and amounts)

    // Line 14: Other information
    // (see attached statement for codes and amounts)
}

impl Form for Input1041ScheduleK1 {
    fn name() -> &'static str {
        "Schedule K-1 (Form 1041)"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1041ScheduleK1 {}
