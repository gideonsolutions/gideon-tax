use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form W-2 (2025), Wage and Tax Statement.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct InputW2 {
    /// Corrected W-2 indicator
    pub corrected_w2_ind: bool,
    /// Standard or non-standard code
    pub standard_or_non_standard_cd: String,

    // =====================================================================
    // Employer identification
    // =====================================================================
    /// Box b: Employer identification number (EIN)
    pub employer_ein: String,
    /// Employer's name — business name line 1
    pub employer_name_line_1_txt: String,
    /// Employer's name — business name line 2
    pub employer_name_line_2_txt: String,
    /// Employer name control text
    pub employer_name_control_txt: String,
    /// Employer's US address
    pub employer_us_address_line_1_txt: String,
    /// Employer's US address line 2
    pub employer_us_address_line_2_txt: String,
    /// Employer's foreign address
    pub employer_foreign_address: String,
    /// Agent for employer indicator
    pub agent_for_employer_ind: bool,

    // =====================================================================
    // Employee identification
    // =====================================================================
    /// Box a: Employee's social security number
    pub employee_ssn: String,
    /// Box d: Control number
    pub control_num: String,
    /// Box e/f: Employee's name
    pub employee_nm: String,
    /// Employee's US address line 1
    pub employee_us_address_line_1_txt: String,
    /// Employee's US address line 2
    pub employee_us_address_line_2_txt: String,
    /// Employee's foreign address
    pub employee_foreign_address: String,

    // =====================================================================
    // Boxes 1–13: Wages, taxes, and indicators
    // =====================================================================
    /// Box 1: Wages, tips, other compensation
    pub wages_amt: Usd,
    /// Box 2: Federal income tax withheld
    pub withholding_amt: Usd,
    /// Box 3: Social security wages
    pub social_security_wages_amt: Usd,
    /// Box 4: Social security tax withheld
    pub social_security_tax_amt: Usd,
    /// Box 5: Medicare wages and tips
    pub medicare_wages_and_tips_amt: Usd,
    /// Box 6: Medicare tax withheld
    pub medicare_tax_withheld_amt: Usd,
    /// Box 7: Social security tips
    pub social_security_tips_amt: Usd,
    /// Box 8: Allocated tips
    pub allocated_tips_amt: Usd,
    /// Box 10: Dependent care benefits
    pub dependent_care_benefits_amt: Usd,
    /// Box 11: Nonqualified plans
    pub nonqualified_plans_amt: Usd,
    /// Box 13: Statutory employee
    pub statutory_employee_ind: bool,
    /// Box 13: Retirement plan
    pub retirement_plan_ind: bool,
    /// Box 13: Third-party sick pay
    pub third_party_sick_pay_ind: bool,

    // =====================================================================
    // W-2 Security Information
    // =====================================================================
    /// W-2 download code
    pub w2_download_cd: String,
    /// W-2 download failed attempt count
    pub w2_download_failed_attempt_cnt: String,
    /// W-2 download result code
    pub w2_download_result_cd: String,

    // =====================================================================
    // Boxes 15–20: State and local tax information
    // =====================================================================
    /// Box 15: State abbreviation code
    pub state_abbreviation_cd: String,
    /// Box 15: Employer's state ID number
    pub employer_state_id_num: String,
    /// Box 16: State wages, tips, etc.
    pub state_wages_amt: Usd,
    /// Box 17: State income tax
    pub state_income_tax_amt: Usd,
    /// Box 18: Local wages, tips, etc.
    pub local_wages_amt: Usd,
    /// Box 19: Local income tax
    pub local_income_tax_amt: Usd,
    /// Box 20: Locality name
    pub locality_nm: String,
}

impl Form for InputW2 {
    fn name() -> &'static str {
        "Form W-2"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for InputW2 {}
