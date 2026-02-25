use crate::Usd;

/// Output fields for IRS Schedule H (Form 1040) — Household Employment Taxes (2025).
///
/// Covers Social Security, Medicare, Withheld Income, and Federal Unemployment
/// (FUTA) Taxes.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleH {
    // -----------------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------------
    /// Name of employer
    pub household_employer_nm: String,
    /// Social security number
    pub ssn: String,
    /// Employer identification number (EIN)
    pub employer_ein: String,
    /// Employer name control
    pub employer_name_control_txt: String,
    /// Applied for EIN reason code
    pub applied_for_ein_reason_cd: String,
    /// Line A: Did you pay any one household employee cash wages of $2,800 or
    /// more in 2025?
    pub hshld_empl_pd_cash_wage_over_lmt_cy_ind: bool,
    /// Line B: Did you withhold federal income tax during 2025 for any
    /// household employee?
    pub hshld_empl_fed_incm_tax_withheld_ind: bool,
    /// Line C: Did you pay total cash wages of $1,000 or more in any calendar
    /// quarter of 2024 or 2025 to all household employees?
    pub hshld_empl_pd_tot_cash_wage_any_qtr_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Social Security, Medicare, and Federal Income Taxes
    // -----------------------------------------------------------------------
    /// Line 1: Total cash wages subject to social security tax
    pub social_security_tax_cash_wages_amt: Usd,
    /// Line 2: Social security tax. Multiply line 1 by 12.4% (0.124)
    pub social_security_tax_amt: Usd,
    /// Line 3: Total cash wages subject to Medicare tax
    pub medicare_tax_cash_wages_amt: Usd,
    /// Line 4: Medicare tax. Multiply line 3 by 2.9% (0.029)
    pub medicare_tax_withheld_amt: Usd,
    /// Line 5: Total cash wages subject to Additional Medicare Tax withholding
    pub tot_medcr_tax_cash_wages_addnl_wh_amt: Usd,
    /// Line 6: Additional Medicare Tax withholding. Multiply line 5 by 0.9%
    /// (0.009)
    pub addnl_medicare_tax_withholding_amt: Usd,
    /// Line 7: Federal income tax withheld, if any
    pub federal_income_tax_withheld_amt: Usd,
    /// Line 8: Total social security, Medicare, and federal income taxes. Add
    /// lines 2, 4, 6, and 7
    pub tot_soc_sec_medcr_and_fed_incm_tax_amt: Usd,
    /// Line 9: Did you pay total cash wages of $1,000 or more in any calendar
    /// quarter of 2024 or 2025 to all household employees?
    pub hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Federal Unemployment (FUTA) Tax
    // -----------------------------------------------------------------------
    /// Line 10: Did you pay unemployment contributions to only one state? If
    /// you paid contributions to a credit reduction state, see instructions and
    /// check "No"
    pub unempl_paid_only_one_state_ind: bool,
    /// Line 11: Did you pay all state unemployment contributions for 2025 by
    /// April 15, 2026? Fiscal year filers, see instructions
    pub pay_all_state_unempl_contri_ind: bool,
    /// Line 12: Were all wages that are taxable for FUTA tax also taxable for
    /// your state's unemployment tax?
    pub txbl_futa_wages_also_txbl_unempl_ind: bool,

    // ── Section A ───────────────────────────────────────────────────────────
    /// Line 13: Name of the state where you paid unemployment contributions
    pub single_state_cd: String,
    /// Line 14: Contributions paid to your state unemployment fund
    pub contri_paid_to_state_unempl_fund_amt: Usd,
    /// Line 15: Total cash wages subject to FUTA tax
    pub single_state_total_cash_wages_subj_futa_tax_amt: Usd,
    /// Line 16: FUTA tax. Multiply line 15 by 0.6% (0.006). Enter the result
    /// here, skip Section B, and go to line 25
    pub single_state_futa_tax_amt: Usd,
    /// Unemployment fund zero rate code
    pub unemployment_fund_zero_rate_cd: String,

    // ── Section B ───────────────────────────────────────────────────────────
    /// Line 18: Totals
    pub total_contri_state_unempl_fund_amt: Usd,
    /// Line 19: Add columns (g) and (h) of line 18
    pub total_unempl_additional_tax_cr_amt: Usd,
    /// Line 20: Total cash wages subject to FUTA tax (see the line 15
    /// instructions)
    pub multi_state_total_cash_wages_subj_futa_tax_amt: Usd,
    /// Line 21: Multiply line 20 by 6.0% (0.06)
    pub gross_futa_tax_credit_amt: Usd,
    /// Line 22: Multiply line 20 by 5.4% (0.054)
    pub futa_tax_credit_max_allowed_amt: Usd,
    /// Line 23: Enter the smaller of line 19 or line 22
    pub unempl_smaller_tax_adjustment_amt: Usd,
    /// Line 23 checkbox: If you paid state unemployment contributions late or
    /// you're in a credit reduction state, see instructions and check here
    pub credit_reduction_state_wrksht_ind: bool,
    /// Line 23: Tentative FUTA credit amount
    pub tentative_futa_credit_amt: Usd,
    /// Line 24: FUTA tax. Subtract line 23 from line 21. Enter the result here
    /// and go to line 25
    pub multi_state_futa_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Total Household Employment Taxes
    // -----------------------------------------------------------------------
    /// Line 25: Enter the amount from line 8. If you checked the "Yes" box on
    /// line C of page 1, enter -0-
    pub total_tax_household_empl_calc_amt: Usd,
    /// Line 26: Add line 16 (or line 24) and line 25
    pub combined_futa_tax_plus_net_taxes_amt: Usd,
    /// Line 27: Are you required to file Form 1040?
    pub required_to_file_form_1040_ind: bool,
    /// State disability payment code
    pub hshld_empl_state_disability_pymt_cd: String,
    /// State disability payment amount
    pub hshld_empl_state_disability_pymt_amt: Usd,
}
