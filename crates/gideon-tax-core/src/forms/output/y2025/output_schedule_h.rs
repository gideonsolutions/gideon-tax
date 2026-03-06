use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::rules::TaxYearRules;
use crate::rules::y2025::Rules2025;
use crate::{GideonTaxError, Usd};

// =========================================================================
// Supporting types
// =========================================================================

/// Input for one row of Section B, line 17 — state-by-state unemployment data.
#[derive(Debug, Clone)]
pub struct UnemploymentStateTaxInput {
    /// (a) Name of state
    pub state_cd: String,
    /// (b) Taxable wages (as defined in state act)
    pub taxable_wages_amt: Usd,
    /// (c) State experience rate period — From
    pub experience_rate_period_from: String,
    /// (c) State experience rate period — To
    pub experience_rate_period_to: String,
    /// (d) State experience rate in basis points (e.g. 320 = 3.20%)
    pub experience_rate_bps: u16,
    /// (h) Contributions paid to state unemployment fund
    pub contributions_paid_amt: Usd,
}

/// Computed output for one row of Section B, line 17.
#[derive(Debug, Clone, Default)]
pub struct UnemploymentStateTaxEntry {
    /// (a) Name of state
    pub state_cd: String,
    /// (b) Taxable wages (as defined in state act)
    pub taxable_wages_amt: Usd,
    /// (c) State experience rate period — From
    pub experience_rate_period_from: String,
    /// (c) State experience rate period — To
    pub experience_rate_period_to: String,
    /// (d) State experience rate in basis points
    pub experience_rate_bps: u16,
    /// (e) col (b) × 0.054
    pub base_credit_amt: Usd,
    /// (f) col (b) × col (d)
    pub state_credit_amt: Usd,
    /// (g) col (e) − col (f); if zero or less, enter -0-
    pub additional_credit_amt: Usd,
    /// (h) Contributions paid to state unemployment fund
    pub contributions_paid_amt: Usd,
}

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Schedule H (Form 1040).
///
/// Household employer name, SSN/EIN, screening question answers (Lines A–C),
/// cash wages subject to SS/Medicare/Additional Medicare Tax, federal income
/// tax withheld, and FUTA details (single-state or multi-state) feed into
/// the computation.
///
/// # Cash-wage exclusions
///
/// When figuring total cash wages paid in 2025 to each household employee,
/// do **not** include amounts paid to any of the following individuals:
///
/// - Your **spouse**.
/// - Your **child** who was under age 21.
/// - Your **parent** (see *Exception for parents* below).
/// - Your **employee who was under age 18** at any time during 2025.
///   If the employee wasn't a student, see *Exception for employees under
///   age 18* below.
///
/// ## Exception for parents
///
/// Include the cash wages you paid your parent for work in or around your
/// home if **both** of the following apply:
///
/// 1. Your child (including an adopted child or stepchild) who lived with
///    you was under age 18 or had a physical or mental condition that
///    required the personal care of an adult for at least 4 continuous
///    weeks during the calendar quarter in which services were performed.
///    A calendar quarter is January through March, April through June,
///    July through September, or October through December.
/// 2. You were divorced and not remarried, a widow or widower, or married
///    to and living with a person whose physical or mental condition
///    prevented your spouse from caring for the child for at least 4
///    continuous weeks during the calendar quarter in which services were
///    performed.
///
/// ## Exception for employees under age 18
///
/// Include the cash wages you paid to a person who was under age 18 and
/// not a student if providing household services was the employee's
/// principal occupation.
#[derive(Debug, Clone)]
pub struct ScheduleHInput {
    // ── Header ────────────────────────────────────────────────────────────
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

    // ── Screening questions ───────────────────────────────────────────────
    /// Line A: Did you pay any one household employee cash wages of $2,800
    /// or more in 2025?
    pub hshld_empl_pd_cash_wage_over_lmt_cy_ind: bool,
    /// Line B: Did you withhold federal income tax during 2025 for any
    /// household employee?
    pub hshld_empl_fed_incm_tax_withheld_ind: bool,
    /// Line C: Did you pay total cash wages of $1,000 or more in any
    /// calendar quarter of 2024 or 2025 to all household employees?
    pub hshld_empl_pd_tot_cash_wage_any_qtr_ind: bool,

    // ── Part I ────────────────────────────────────────────────────────────
    /// Line 1: Total cash wages subject to social security tax
    pub social_security_tax_cash_wages_amt: Usd,
    /// Line 3: Total cash wages subject to Medicare tax
    pub medicare_tax_cash_wages_amt: Usd,
    /// Line 5: Total cash wages subject to Additional Medicare Tax
    /// withholding
    pub tot_medcr_tax_cash_wages_addnl_wh_amt: Usd,
    /// Line 7: Federal income tax withheld, if any
    pub federal_income_tax_withheld_amt: Usd,
    /// Line 9: Did you pay total cash wages of $1,000 or more in any
    /// calendar quarter of 2024 or 2025?
    pub hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind: bool,

    // ── Part II ───────────────────────────────────────────────────────────
    /// Line 10: Did you pay unemployment contributions to only one state?
    pub unempl_paid_only_one_state_ind: bool,
    /// Line 11: Did you pay all state unemployment contributions for 2025
    /// by April 15, 2026?
    pub pay_all_state_unempl_contri_ind: bool,
    /// Line 12: Were all wages that are taxable for FUTA tax also taxable
    /// for your state's unemployment tax?
    pub txbl_futa_wages_also_txbl_unempl_ind: bool,

    // ── Section A (single state) ──────────────────────────────────────────
    /// Line 13: Name of state
    pub single_state_cd: String,
    /// Line 14: Contributions paid to your state unemployment fund
    pub contri_paid_to_state_unempl_fund_amt: Usd,
    /// Line 15: Total cash wages subject to FUTA tax.
    ///
    /// Do **not** include cash wages paid in 2025 to any of the following:
    /// - Your **spouse**.
    /// - Your **child** who was under age 21.
    /// - Your **parent**.
    ///
    /// If you paid any household employee more than $7,000 in 2025, include
    /// on line 15 only the first $7,000 of that employee's cash wages.
    pub single_state_total_cash_wages_subj_futa_tax_amt: Usd,
    /// Unemployment fund zero rate code
    pub unemployment_fund_zero_rate_cd: String,

    // ── Section B (multi state) ───────────────────────────────────────────
    /// Line 17: Multi-state table entries
    pub unemployment_state_tax_entries: Vec<UnemploymentStateTaxInput>,
    /// Line 20: Total cash wages subject to FUTA tax
    pub multi_state_total_cash_wages_subj_futa_tax_amt: Usd,
    /// Line 23 checkbox: Credit reduction state worksheet indicator
    pub credit_reduction_state_wrksht_ind: bool,
    /// Worksheet 1, line 4: Total contributions paid to the state(s) after
    /// the Form 1040 or 1040-SR due date. Only applicable when line 11 is
    /// "No" (state unemployment contributions were not all paid on time).
    pub late_contributions_paid_amt: Usd,

    // ── Part III ──────────────────────────────────────────────────────────
    /// Line 27: Are you required to file Form 1040?
    pub required_to_file_form_1040_ind: bool,
    /// State disability payment code
    pub hshld_empl_state_disability_pymt_cd: String,
    /// State disability payment amount
    pub hshld_empl_state_disability_pymt_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

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
    /// Line 15: Total cash wages subject to FUTA tax.
    ///
    /// Do **not** include cash wages paid in 2025 to any of the following:
    /// - Your **spouse**.
    /// - Your **child** who was under age 21.
    /// - Your **parent**.
    ///
    /// If you paid any household employee more than $7,000 in 2025, include
    /// on line 15 only the first $7,000 of that employee's cash wages.
    pub single_state_total_cash_wages_subj_futa_tax_amt: Usd,
    /// Line 16: FUTA tax. Multiply line 15 by 0.6% (0.006). Enter the result
    /// here, skip Section B, and go to line 25
    pub single_state_futa_tax_amt: Usd,
    /// Unemployment fund zero rate code
    pub unemployment_fund_zero_rate_cd: String,

    // ── Section B ───────────────────────────────────────────────────────────
    /// Line 17: Multi-state unemployment table entries
    pub unemployment_state_tax_entries: Vec<UnemploymentStateTaxEntry>,
    /// Line 18 col (h) total: Total contributions paid to state unemployment
    /// funds
    pub total_contri_state_unempl_fund_amt: Usd,
    /// Line 18 col (g) total: Total additional tax credits
    pub total_unempl_additional_tax_cr_amt: Usd,
    /// Line 19: Add columns (g) and (h) of line 18
    pub tentative_futa_credit_amt: Usd,
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
    /// Worksheet 1, line 4: Total contributions paid to the state(s) after
    /// the Form 1040 or 1040-SR due date. Only applicable when line 11 is
    /// "No" (state unemployment contributions were not all paid on time).
    pub late_contributions_paid_amt: Usd,
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

// =========================================================================
// Trait impls
// =========================================================================

impl Form for OutputScheduleH {
    fn name() -> &'static str {
        "Schedule H"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for OutputScheduleH {
    type Input = ScheduleHInput;

    fn must_file(input: &Self::Input) -> bool {
        input.hshld_empl_pd_cash_wage_over_lmt_cy_ind
            || input.hshld_empl_fed_incm_tax_withheld_ind
            || input.hshld_empl_pd_tot_cash_wage_any_qtr_ind
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        let ss_bps = Rules2025::SOCIAL_SECURITY_RATE_BPS as i64 * 2; // 12.4%
        let med_bps = Rules2025::MEDICARE_RATE_BPS as i64 * 2; // 2.9%
        let addl_med_bps = Rules2025::ADDITIONAL_MEDICARE_RATE_BPS as i64; // 0.9%
        let futa_bps = Rules2025::FUTA_RATE_BPS as i64; // 6.0%
        let futa_credit_bps = Rules2025::FUTA_CREDIT_RATE_BPS as i64; // 5.4%
        let futa_net_bps = futa_bps - futa_credit_bps; // 0.6%

        // Whether Part I was completed (A=Yes or B=Yes)
        let did_part_i = input.hshld_empl_pd_cash_wage_over_lmt_cy_ind
            || input.hshld_empl_fed_incm_tax_withheld_ind;

        // ── Part I ────────────────────────────────────────────────────
        let line1 = input.social_security_tax_cash_wages_amt;
        let line2 = Usd::from_cents(line1.cents() * ss_bps / 10_000);
        let line3 = input.medicare_tax_cash_wages_amt;
        let line4 = Usd::from_cents(line3.cents() * med_bps / 10_000);
        let line5 = input.tot_medcr_tax_cash_wages_addnl_wh_amt;
        let line6 = Usd::from_cents(line5.cents() * addl_med_bps / 10_000);
        let line7 = input.federal_income_tax_withheld_amt;
        let line8 = line2 + line4 + line6 + line7;

        // ── Part II — FUTA ────────────────────────────────────────────
        let use_section_a = input.unempl_paid_only_one_state_ind
            && input.pay_all_state_unempl_contri_ind
            && input.txbl_futa_wages_also_txbl_unempl_ind;

        // Section A
        let line15 = input.single_state_total_cash_wages_subj_futa_tax_amt;
        let line16 = if use_section_a {
            Usd::from_cents(line15.cents() * futa_net_bps / 10_000)
        } else {
            Usd::ZERO
        };

        // Section B
        let entries: Vec<UnemploymentStateTaxEntry> = if !use_section_a {
            input
                .unemployment_state_tax_entries
                .iter()
                .map(|e| {
                    let col_e =
                        Usd::from_cents(e.taxable_wages_amt.cents() * futa_credit_bps / 10_000);
                    let col_f = Usd::from_cents(
                        e.taxable_wages_amt.cents() * e.experience_rate_bps as i64 / 10_000,
                    );
                    let col_g = (col_e - col_f).max(Usd::ZERO);
                    UnemploymentStateTaxEntry {
                        state_cd: e.state_cd.clone(),
                        taxable_wages_amt: e.taxable_wages_amt,
                        experience_rate_period_from: e.experience_rate_period_from.clone(),
                        experience_rate_period_to: e.experience_rate_period_to.clone(),
                        experience_rate_bps: e.experience_rate_bps,
                        base_credit_amt: col_e,
                        state_credit_amt: col_f,
                        additional_credit_amt: col_g,
                        contributions_paid_amt: e.contributions_paid_amt,
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        // Line 18 totals
        let total_col_g = entries
            .iter()
            .fold(Usd::ZERO, |acc, e| acc + e.additional_credit_amt);
        let total_col_h = entries
            .iter()
            .fold(Usd::ZERO, |acc, e| acc + e.contributions_paid_amt);

        // Line 19
        let line19 = total_col_g + total_col_h;

        // Line 20
        let line20 = input.multi_state_total_cash_wages_subj_futa_tax_amt;

        // Line 21: line 20 × 6.0%
        let line21 = Usd::from_cents(line20.cents() * futa_bps / 10_000);

        // Line 22: line 20 × 5.4%
        let line22 = Usd::from_cents(line20.cents() * futa_credit_bps / 10_000);

        // Worksheet 1 — late state unemployment contributions.
        // Applies when line 11 = No (contributions not all paid by due date).
        //   ws1_1 = line 22
        //   ws1_2 = line 19
        //   ws1_3 = max(ws1_1 - ws1_2, 0)
        //   ws1_4 = late_contributions_paid_amt (input)
        //   ws1_5 = min(ws1_3, ws1_4)
        //   ws1_6 = ws1_5 × 90%
        //   ws1_7 = ws1_2 + ws1_6
        //   ws1_8 = min(ws1_1, ws1_7)
        let has_late_contributions = !use_section_a && !input.pay_all_state_unempl_contri_ind;
        let tentative_credit = if has_late_contributions {
            let ws1_3 = (line22 - line19).max(Usd::ZERO);
            let ws1_5 = ws1_3.min(input.late_contributions_paid_amt);
            let ws1_6 = Usd::from_cents(ws1_5.cents() * 9_000 / 10_000);
            let ws1_7 = line19 + ws1_6;
            line22.min(ws1_7)
        } else if !use_section_a {
            line19.min(line22)
        } else {
            Usd::ZERO
        };

        // Worksheet 2 — credit reduction for states with outstanding
        // federal unemployment loans (e.g. CA at 1.2%, VI at 4.5% for 2025).
        // For each Section B entry in a credit reduction state, multiply
        // FUTA taxable wages by the reduction rate and sum.
        let total_credit_reduction = if !use_section_a && input.credit_reduction_state_wrksht_ind {
            entries.iter().fold(Usd::ZERO, |acc, e| {
                let reduction_bps = Rules2025::FUTA_CREDIT_REDUCTION_STATES
                    .iter()
                    .find(|cr| cr.state_cd == e.state_cd)
                    .map_or(0, |cr| cr.reduction_rate_bps as i64);
                acc + Usd::from_cents(e.taxable_wages_amt.cents() * reduction_bps / 10_000)
            })
        } else {
            Usd::ZERO
        };

        // Line 23: tentative credit (from Worksheet 1 or min of 19/22),
        // minus credit reduction (Worksheet 2). If zero or less, enter -0-.
        let line23 = (tentative_credit - total_credit_reduction).max(Usd::ZERO);

        // Line 24: line 21 - line 23
        let line24 = if !use_section_a {
            line21 - line23
        } else {
            Usd::ZERO
        };

        // ── Part III ──────────────────────────────────────────────────
        // Line 25: line 8 if Part I was completed, else -0-
        let line25 = if did_part_i { line8 } else { Usd::ZERO };

        // Line 26: FUTA tax + line 25
        let futa_tax = if use_section_a { line16 } else { line24 };
        let line26 = futa_tax + line25;

        Ok(OutputScheduleH {
            // Header
            household_employer_nm: input.household_employer_nm,
            ssn: input.ssn,
            employer_ein: input.employer_ein,
            employer_name_control_txt: input.employer_name_control_txt,
            applied_for_ein_reason_cd: input.applied_for_ein_reason_cd,
            hshld_empl_pd_cash_wage_over_lmt_cy_ind: input.hshld_empl_pd_cash_wage_over_lmt_cy_ind,
            hshld_empl_fed_incm_tax_withheld_ind: input.hshld_empl_fed_incm_tax_withheld_ind,
            hshld_empl_pd_tot_cash_wage_any_qtr_ind: input.hshld_empl_pd_tot_cash_wage_any_qtr_ind,

            // Part I
            social_security_tax_cash_wages_amt: line1,
            social_security_tax_amt: line2,
            medicare_tax_cash_wages_amt: line3,
            medicare_tax_withheld_amt: line4,
            tot_medcr_tax_cash_wages_addnl_wh_amt: line5,
            addnl_medicare_tax_withholding_amt: line6,
            federal_income_tax_withheld_amt: line7,
            tot_soc_sec_medcr_and_fed_incm_tax_amt: line8,
            hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind: input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind,

            // Part II
            unempl_paid_only_one_state_ind: input.unempl_paid_only_one_state_ind,
            pay_all_state_unempl_contri_ind: input.pay_all_state_unempl_contri_ind,
            txbl_futa_wages_also_txbl_unempl_ind: input.txbl_futa_wages_also_txbl_unempl_ind,

            // Section A
            single_state_cd: input.single_state_cd,
            contri_paid_to_state_unempl_fund_amt: input.contri_paid_to_state_unempl_fund_amt,
            single_state_total_cash_wages_subj_futa_tax_amt: line15,
            single_state_futa_tax_amt: line16,
            unemployment_fund_zero_rate_cd: input.unemployment_fund_zero_rate_cd,

            // Section B
            unemployment_state_tax_entries: entries,
            total_contri_state_unempl_fund_amt: total_col_h,
            total_unempl_additional_tax_cr_amt: total_col_g,
            tentative_futa_credit_amt: line19,
            multi_state_total_cash_wages_subj_futa_tax_amt: line20,
            gross_futa_tax_credit_amt: line21,
            futa_tax_credit_max_allowed_amt: line22,
            unempl_smaller_tax_adjustment_amt: line23,
            credit_reduction_state_wrksht_ind: input.credit_reduction_state_wrksht_ind,
            late_contributions_paid_amt: input.late_contributions_paid_amt,
            multi_state_futa_tax_amt: line24,

            // Part III
            total_tax_household_empl_calc_amt: line25,
            combined_futa_tax_plus_net_taxes_amt: line26,
            required_to_file_form_1040_ind: input.required_to_file_form_1040_ind,
            hshld_empl_state_disability_pymt_cd: input.hshld_empl_state_disability_pymt_cd,
            hshld_empl_state_disability_pymt_amt: input.hshld_empl_state_disability_pymt_amt,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::W2]
    }

    fn is_valid(&self) -> bool {
        let ss_bps = Rules2025::SOCIAL_SECURITY_RATE_BPS as i64 * 2;
        let med_bps = Rules2025::MEDICARE_RATE_BPS as i64 * 2;
        let addl_med_bps = Rules2025::ADDITIONAL_MEDICARE_RATE_BPS as i64;
        let futa_bps = Rules2025::FUTA_RATE_BPS as i64;
        let futa_credit_bps = Rules2025::FUTA_CREDIT_RATE_BPS as i64;
        let futa_net_bps = futa_bps - futa_credit_bps;

        let did_part_i = self.hshld_empl_pd_cash_wage_over_lmt_cy_ind
            || self.hshld_empl_fed_incm_tax_withheld_ind;

        let use_section_a = self.unempl_paid_only_one_state_ind
            && self.pay_all_state_unempl_contri_ind
            && self.txbl_futa_wages_also_txbl_unempl_ind;

        // Part I
        let line2_ok = self.social_security_tax_amt
            == Usd::from_cents(self.social_security_tax_cash_wages_amt.cents() * ss_bps / 10_000);
        let line4_ok = self.medicare_tax_withheld_amt
            == Usd::from_cents(self.medicare_tax_cash_wages_amt.cents() * med_bps / 10_000);
        let line6_ok = self.addnl_medicare_tax_withholding_amt
            == Usd::from_cents(
                self.tot_medcr_tax_cash_wages_addnl_wh_amt.cents() * addl_med_bps / 10_000,
            );
        let line8_ok = self.tot_soc_sec_medcr_and_fed_incm_tax_amt
            == self.social_security_tax_amt
                + self.medicare_tax_withheld_amt
                + self.addnl_medicare_tax_withholding_amt
                + self.federal_income_tax_withheld_amt;

        // Section A
        let line16_ok = if use_section_a {
            self.single_state_futa_tax_amt
                == Usd::from_cents(
                    self.single_state_total_cash_wages_subj_futa_tax_amt.cents() * futa_net_bps
                        / 10_000,
                )
        } else {
            true
        };

        // Section B
        let section_b_ok = if !use_section_a {
            let total_g = self
                .unemployment_state_tax_entries
                .iter()
                .fold(Usd::ZERO, |acc, e| acc + e.additional_credit_amt);
            let total_h = self
                .unemployment_state_tax_entries
                .iter()
                .fold(Usd::ZERO, |acc, e| acc + e.contributions_paid_amt);
            let line19 = total_g + total_h;
            let line21 = Usd::from_cents(
                self.multi_state_total_cash_wages_subj_futa_tax_amt.cents() * futa_bps / 10_000,
            );
            let line22 = Usd::from_cents(
                self.multi_state_total_cash_wages_subj_futa_tax_amt.cents() * futa_credit_bps
                    / 10_000,
            );
            let total_credit_reduction = if self.credit_reduction_state_wrksht_ind {
                self.unemployment_state_tax_entries
                    .iter()
                    .fold(Usd::ZERO, |acc, e| {
                        let reduction_bps = Rules2025::FUTA_CREDIT_REDUCTION_STATES
                            .iter()
                            .find(|cr| cr.state_cd == e.state_cd)
                            .map_or(0, |cr| cr.reduction_rate_bps as i64);
                        acc + Usd::from_cents(e.taxable_wages_amt.cents() * reduction_bps / 10_000)
                    })
            } else {
                Usd::ZERO
            };
            // Worksheet 1 — late contributions
            let has_late = !self.pay_all_state_unempl_contri_ind;
            let tentative_credit = if has_late {
                let ws1_3 = (line22 - line19).max(Usd::ZERO);
                let ws1_5 = ws1_3.min(self.late_contributions_paid_amt);
                let ws1_6 = Usd::from_cents(ws1_5.cents() * 9_000 / 10_000);
                let ws1_7 = line19 + ws1_6;
                line22.min(ws1_7)
            } else {
                line19.min(line22)
            };
            let line23 = (tentative_credit - total_credit_reduction).max(Usd::ZERO);

            self.total_contri_state_unempl_fund_amt == total_h
                && self.total_unempl_additional_tax_cr_amt == total_g
                && self.tentative_futa_credit_amt == line19
                && self.gross_futa_tax_credit_amt == line21
                && self.futa_tax_credit_max_allowed_amt == line22
                && self.unempl_smaller_tax_adjustment_amt == line23
                && self.multi_state_futa_tax_amt == line21 - line23
        } else {
            true
        };

        // Part III
        let line25_ok = self.total_tax_household_empl_calc_amt
            == if did_part_i {
                self.tot_soc_sec_medcr_and_fed_incm_tax_amt
            } else {
                Usd::ZERO
            };

        let futa_tax = if use_section_a {
            self.single_state_futa_tax_amt
        } else {
            self.multi_state_futa_tax_amt
        };
        let line26_ok = self.combined_futa_tax_plus_net_taxes_amt
            == futa_tax + self.total_tax_household_empl_calc_amt;

        line2_ok
            && line4_ok
            && line6_ok
            && line8_ok
            && line16_ok
            && section_b_ok
            && line25_ok
            && line26_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_input() -> ScheduleHInput {
        ScheduleHInput {
            household_employer_nm: "Jane Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            employer_ein: "12-3456789".to_string(),
            employer_name_control_txt: "DOE".to_string(),
            applied_for_ein_reason_cd: String::new(),
            hshld_empl_pd_cash_wage_over_lmt_cy_ind: true,
            hshld_empl_fed_incm_tax_withheld_ind: false,
            hshld_empl_pd_tot_cash_wage_any_qtr_ind: false,
            social_security_tax_cash_wages_amt: Usd::ZERO,
            medicare_tax_cash_wages_amt: Usd::ZERO,
            tot_medcr_tax_cash_wages_addnl_wh_amt: Usd::ZERO,
            federal_income_tax_withheld_amt: Usd::ZERO,
            hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind: false,
            unempl_paid_only_one_state_ind: true,
            pay_all_state_unempl_contri_ind: true,
            txbl_futa_wages_also_txbl_unempl_ind: true,
            single_state_cd: String::new(),
            contri_paid_to_state_unempl_fund_amt: Usd::ZERO,
            single_state_total_cash_wages_subj_futa_tax_amt: Usd::ZERO,
            unemployment_fund_zero_rate_cd: String::new(),
            unemployment_state_tax_entries: Vec::new(),
            multi_state_total_cash_wages_subj_futa_tax_amt: Usd::ZERO,
            credit_reduction_state_wrksht_ind: false,
            late_contributions_paid_amt: Usd::ZERO,
            required_to_file_form_1040_ind: true,
            hshld_empl_state_disability_pymt_cd: String::new(),
            hshld_empl_state_disability_pymt_amt: Usd::ZERO,
        }
    }

    // ── must_file ─────────────────────────────────────────────────────

    #[test]
    fn must_file_line_a_yes() {
        let mut input = default_input();
        input.hshld_empl_pd_cash_wage_over_lmt_cy_ind = true;
        input.hshld_empl_fed_incm_tax_withheld_ind = false;
        input.hshld_empl_pd_tot_cash_wage_any_qtr_ind = false;
        assert!(OutputScheduleH::must_file(&input));
    }

    #[test]
    fn must_file_line_b_yes() {
        let mut input = default_input();
        input.hshld_empl_pd_cash_wage_over_lmt_cy_ind = false;
        input.hshld_empl_fed_incm_tax_withheld_ind = true;
        input.hshld_empl_pd_tot_cash_wage_any_qtr_ind = false;
        assert!(OutputScheduleH::must_file(&input));
    }

    #[test]
    fn must_file_line_c_yes() {
        let mut input = default_input();
        input.hshld_empl_pd_cash_wage_over_lmt_cy_ind = false;
        input.hshld_empl_fed_incm_tax_withheld_ind = false;
        input.hshld_empl_pd_tot_cash_wage_any_qtr_ind = true;
        assert!(OutputScheduleH::must_file(&input));
    }

    #[test]
    fn must_file_all_no() {
        let mut input = default_input();
        input.hshld_empl_pd_cash_wage_over_lmt_cy_ind = false;
        input.hshld_empl_fed_incm_tax_withheld_ind = false;
        input.hshld_empl_pd_tot_cash_wage_any_qtr_ind = false;
        assert!(!OutputScheduleH::must_file(&input));
    }

    // ── Basic Part I — SS + Medicare ──────────────────────────────────

    #[test]
    fn basic_part_i_wages() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // Line 2: 10,000 × 12.4% = 1,240
        assert_eq!(form.social_security_tax_amt, Usd::from_dollars(1_240));
        // Line 4: 10,000 × 2.9% = 290
        assert_eq!(form.medicare_tax_withheld_amt, Usd::from_dollars(290));
        // Line 6: 0 (no Additional Medicare)
        assert_eq!(form.addnl_medicare_tax_withholding_amt, Usd::ZERO);
        // Line 8: 1,240 + 290 + 0 + 0 = 1,530
        assert_eq!(
            form.tot_soc_sec_medcr_and_fed_incm_tax_amt,
            Usd::from_dollars(1_530)
        );
        assert!(form.is_valid());
    }

    // ── Part I with Additional Medicare Tax ───────────────────────────

    #[test]
    fn part_i_with_additional_medicare() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(50_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(250_000);
        input.tot_medcr_tax_cash_wages_addnl_wh_amt = Usd::from_dollars(50_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // Line 2: 50,000 × 12.4% = 6,200
        assert_eq!(form.social_security_tax_amt, Usd::from_dollars(6_200));
        // Line 4: 250,000 × 2.9% = 7,250
        assert_eq!(form.medicare_tax_withheld_amt, Usd::from_dollars(7_250));
        // Line 6: 50,000 × 0.9% = 450
        assert_eq!(
            form.addnl_medicare_tax_withholding_amt,
            Usd::from_dollars(450)
        );
        // Line 8: 6,200 + 7,250 + 450 + 0 = 13,900
        assert_eq!(
            form.tot_soc_sec_medcr_and_fed_incm_tax_amt,
            Usd::from_dollars(13_900)
        );
        assert!(form.is_valid());
    }

    // ── Part I with federal income tax withheld ───────────────────────

    #[test]
    fn part_i_with_fit_withheld() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.federal_income_tax_withheld_amt = Usd::from_dollars(2_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // Line 8: 1,240 + 290 + 0 + 2,000 = 3,530
        assert_eq!(
            form.tot_soc_sec_medcr_and_fed_incm_tax_amt,
            Usd::from_dollars(3_530)
        );
        assert!(form.is_valid());
    }

    // ── Section A — single state FUTA ─────────────────────────────────

    #[test]
    fn section_a_futa() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind = true;
        input.single_state_cd = "CA".to_string();
        input.contri_paid_to_state_unempl_fund_amt = Usd::from_dollars(200);
        input.single_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(7_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // Line 16: 7,000 × 0.6% = 42
        assert_eq!(form.single_state_futa_tax_amt, Usd::from_dollars(42));
        // Line 25 = line 8 = 1,530
        assert_eq!(
            form.total_tax_household_empl_calc_amt,
            Usd::from_dollars(1_530)
        );
        // Line 26 = 42 + 1,530 = 1,572
        assert_eq!(
            form.combined_futa_tax_plus_net_taxes_amt,
            Usd::from_dollars(1_572)
        );
        assert!(form.is_valid());
    }

    // ── Section B — multi-state FUTA ──────────────────────────────────

    #[test]
    fn section_b_futa_two_states() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(20_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(20_000);
        input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind = true;
        // Multi-state: set one of 10/11/12 to No
        input.unempl_paid_only_one_state_ind = false;
        input.unemployment_state_tax_entries = vec![
            UnemploymentStateTaxInput {
                state_cd: "CA".to_string(),
                taxable_wages_amt: Usd::from_dollars(7_000),
                experience_rate_period_from: "01/01/2025".to_string(),
                experience_rate_period_to: "12/31/2025".to_string(),
                experience_rate_bps: 340, // 3.4%
                contributions_paid_amt: Usd::from_dollars(238),
            },
            UnemploymentStateTaxInput {
                state_cd: "NY".to_string(),
                taxable_wages_amt: Usd::from_dollars(7_000),
                experience_rate_period_from: "01/01/2025".to_string(),
                experience_rate_period_to: "12/31/2025".to_string(),
                experience_rate_bps: 420, // 4.2%
                contributions_paid_amt: Usd::from_dollars(294),
            },
        ];
        input.multi_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(14_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // CA: (e) 7,000 × 5.4% = 378; (f) 7,000 × 3.4% = 238; (g) 378-238 = 140
        assert_eq!(
            form.unemployment_state_tax_entries[0].base_credit_amt,
            Usd::from_dollars(378)
        );
        assert_eq!(
            form.unemployment_state_tax_entries[0].state_credit_amt,
            Usd::from_dollars(238)
        );
        assert_eq!(
            form.unemployment_state_tax_entries[0].additional_credit_amt,
            Usd::from_dollars(140)
        );

        // NY: (e) 7,000 × 5.4% = 378; (f) 7,000 × 4.2% = 294; (g) 378-294 = 84
        assert_eq!(
            form.unemployment_state_tax_entries[1].base_credit_amt,
            Usd::from_dollars(378)
        );
        assert_eq!(
            form.unemployment_state_tax_entries[1].state_credit_amt,
            Usd::from_dollars(294)
        );
        assert_eq!(
            form.unemployment_state_tax_entries[1].additional_credit_amt,
            Usd::from_dollars(84)
        );

        // Line 18 totals: col (g) = 140 + 84 = 224; col (h) = 238 + 294 = 532
        assert_eq!(
            form.total_unempl_additional_tax_cr_amt,
            Usd::from_dollars(224)
        );
        assert_eq!(
            form.total_contri_state_unempl_fund_amt,
            Usd::from_dollars(532)
        );

        // Line 19 = 224 + 532 = 756
        assert_eq!(form.tentative_futa_credit_amt, Usd::from_dollars(756));

        // Line 21 = 14,000 × 6.0% = 840
        assert_eq!(form.gross_futa_tax_credit_amt, Usd::from_dollars(840));

        // Line 22 = 14,000 × 5.4% = 756
        assert_eq!(form.futa_tax_credit_max_allowed_amt, Usd::from_dollars(756));

        // Line 23 = min(756, 756) = 756
        assert_eq!(
            form.unempl_smaller_tax_adjustment_amt,
            Usd::from_dollars(756)
        );

        // Line 24 = 840 - 756 = 84
        assert_eq!(form.multi_state_futa_tax_amt, Usd::from_dollars(84));

        // Line 25 = line 8 = 20,000 × 12.4% + 20,000 × 2.9% = 2,480 + 580 = 3,060
        assert_eq!(
            form.total_tax_household_empl_calc_amt,
            Usd::from_dollars(3_060)
        );

        // Line 26 = 84 + 3,060 = 3,144
        assert_eq!(
            form.combined_futa_tax_plus_net_taxes_amt,
            Usd::from_dollars(3_144)
        );

        assert!(form.is_valid());
    }

    // ── Section B — state rate exceeds 5.4% → col (g) = 0 ────────────

    #[test]
    fn section_b_high_state_rate() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind = true;
        input.unempl_paid_only_one_state_ind = false;
        input.unemployment_state_tax_entries = vec![UnemploymentStateTaxInput {
            state_cd: "TX".to_string(),
            taxable_wages_amt: Usd::from_dollars(7_000),
            experience_rate_period_from: "01/01/2025".to_string(),
            experience_rate_period_to: "12/31/2025".to_string(),
            experience_rate_bps: 600, // 6.0% — exceeds 5.4%
            contributions_paid_amt: Usd::from_dollars(420),
        }];
        input.multi_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(7_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // (e) 7,000 × 5.4% = 378; (f) 7,000 × 6.0% = 420; (g) max(378-420, 0) = 0
        assert_eq!(
            form.unemployment_state_tax_entries[0].additional_credit_amt,
            Usd::ZERO
        );

        // Line 19 = 0 + 420 = 420
        assert_eq!(form.tentative_futa_credit_amt, Usd::from_dollars(420));

        // Line 21 = 7,000 × 6.0% = 420
        assert_eq!(form.gross_futa_tax_credit_amt, Usd::from_dollars(420));

        // Line 22 = 7,000 × 5.4% = 378
        assert_eq!(form.futa_tax_credit_max_allowed_amt, Usd::from_dollars(378));

        // Line 23 = min(420, 378) = 378
        assert_eq!(
            form.unempl_smaller_tax_adjustment_amt,
            Usd::from_dollars(378)
        );

        // Line 24 = 420 - 378 = 42
        assert_eq!(form.multi_state_futa_tax_amt, Usd::from_dollars(42));

        assert!(form.is_valid());
    }

    // ── FUTA only (Line C=Yes, A=No, B=No) ───────────────────────────

    #[test]
    fn futa_only_line_c_yes() {
        let mut input = default_input();
        input.hshld_empl_pd_cash_wage_over_lmt_cy_ind = false;
        input.hshld_empl_fed_incm_tax_withheld_ind = false;
        input.hshld_empl_pd_tot_cash_wage_any_qtr_ind = true;
        // Part I amounts should be zero (skipped)
        input.single_state_cd = "CA".to_string();
        input.single_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(7_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // Line 8 = 0 (no Part I)
        assert_eq!(form.tot_soc_sec_medcr_and_fed_incm_tax_amt, Usd::ZERO);
        // Line 16 = 7,000 × 0.6% = 42
        assert_eq!(form.single_state_futa_tax_amt, Usd::from_dollars(42));
        // Line 25 = -0- (C was Yes, A and B were No)
        assert_eq!(form.total_tax_household_empl_calc_amt, Usd::ZERO);
        // Line 26 = 42 + 0 = 42
        assert_eq!(
            form.combined_futa_tax_plus_net_taxes_amt,
            Usd::from_dollars(42)
        );

        assert!(form.is_valid());
    }

    // ── FIT withholding only (B=Yes, A=No) ────────────────────────────

    #[test]
    fn fit_withholding_only() {
        let mut input = default_input();
        input.hshld_empl_pd_cash_wage_over_lmt_cy_ind = false;
        input.hshld_empl_fed_incm_tax_withheld_ind = true;
        input.hshld_empl_pd_tot_cash_wage_any_qtr_ind = false;
        input.federal_income_tax_withheld_amt = Usd::from_dollars(500);
        let form = OutputScheduleH::try_new(input).unwrap();

        // Line 8 = 0 + 0 + 0 + 500 = 500
        assert_eq!(
            form.tot_soc_sec_medcr_and_fed_incm_tax_amt,
            Usd::from_dollars(500)
        );
        // Line 25 = 500 (Part I was done because B=Yes)
        assert_eq!(
            form.total_tax_household_empl_calc_amt,
            Usd::from_dollars(500)
        );
        assert!(form.is_valid());
    }

    // ── Zero everything ───────────────────────────────────────────────

    #[test]
    fn zero_everything() {
        let form = OutputScheduleH::try_new(default_input()).unwrap();
        assert_eq!(form.social_security_tax_amt, Usd::ZERO);
        assert_eq!(form.medicare_tax_withheld_amt, Usd::ZERO);
        assert_eq!(form.tot_soc_sec_medcr_and_fed_incm_tax_amt, Usd::ZERO);
        assert_eq!(form.combined_futa_tax_plus_net_taxes_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    // ── Credit reduction state (CA at 1.2%) ─────────────────────────

    #[test]
    fn section_b_credit_reduction_ca() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind = true;
        input.unempl_paid_only_one_state_ind = false;
        input.credit_reduction_state_wrksht_ind = true;
        input.unemployment_state_tax_entries = vec![UnemploymentStateTaxInput {
            state_cd: "CA".to_string(),
            taxable_wages_amt: Usd::from_dollars(7_000),
            experience_rate_period_from: "01/01/2025".to_string(),
            experience_rate_period_to: "12/31/2025".to_string(),
            experience_rate_bps: 340, // 3.4%
            contributions_paid_amt: Usd::from_dollars(238),
        }];
        input.multi_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(7_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // col (e) = 7,000 × 5.4% = 378
        // col (f) = 7,000 × 3.4% = 238
        // col (g) = 378 - 238 = 140
        // Line 18: col (g) total = 140, col (h) total = 238
        // Line 19 = 140 + 238 = 378
        assert_eq!(form.tentative_futa_credit_amt, Usd::from_dollars(378));

        // Line 21 = 7,000 × 6.0% = 420
        assert_eq!(form.gross_futa_tax_credit_amt, Usd::from_dollars(420));

        // Line 22 = 7,000 × 5.4% = 378
        assert_eq!(form.futa_tax_credit_max_allowed_amt, Usd::from_dollars(378));

        // Worksheet 2: credit reduction = 7,000 × 1.2% = 84
        // Line 23 = min(378, 378) - 84 = 294
        assert_eq!(
            form.unempl_smaller_tax_adjustment_amt,
            Usd::from_dollars(294)
        );

        // Line 24 = 420 - 294 = 126
        assert_eq!(form.multi_state_futa_tax_amt, Usd::from_dollars(126));

        assert!(form.is_valid());
    }

    // ── Credit reduction state (VI at 4.5%) ───────────────────────────

    #[test]
    fn section_b_credit_reduction_vi() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind = true;
        input.unempl_paid_only_one_state_ind = false;
        input.credit_reduction_state_wrksht_ind = true;
        input.unemployment_state_tax_entries = vec![UnemploymentStateTaxInput {
            state_cd: "VI".to_string(),
            taxable_wages_amt: Usd::from_dollars(7_000),
            experience_rate_period_from: "01/01/2025".to_string(),
            experience_rate_period_to: "12/31/2025".to_string(),
            experience_rate_bps: 270, // 2.7%
            contributions_paid_amt: Usd::from_dollars(189),
        }];
        input.multi_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(7_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // col (e) = 7,000 × 5.4% = 378
        // col (f) = 7,000 × 2.7% = 189
        // col (g) = 378 - 189 = 189
        // Line 19 = 189 + 189 = 378
        // Line 22 = 7,000 × 5.4% = 378
        // Worksheet 2: credit reduction = 7,000 × 4.5% = 315
        // Line 23 = min(378, 378) - 315 = 63
        assert_eq!(
            form.unempl_smaller_tax_adjustment_amt,
            Usd::from_dollars(63)
        );

        // Line 21 = 7,000 × 6.0% = 420
        // Line 24 = 420 - 63 = 357
        assert_eq!(form.multi_state_futa_tax_amt, Usd::from_dollars(357));

        assert!(form.is_valid());
    }

    // ── Credit reduction does not apply when checkbox is false ────────

    #[test]
    fn section_b_no_credit_reduction_without_checkbox() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind = true;
        input.unempl_paid_only_one_state_ind = false;
        input.credit_reduction_state_wrksht_ind = false; // not checked
        input.unemployment_state_tax_entries = vec![UnemploymentStateTaxInput {
            state_cd: "CA".to_string(),
            taxable_wages_amt: Usd::from_dollars(7_000),
            experience_rate_period_from: "01/01/2025".to_string(),
            experience_rate_period_to: "12/31/2025".to_string(),
            experience_rate_bps: 340,
            contributions_paid_amt: Usd::from_dollars(238),
        }];
        input.multi_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(7_000);
        let form = OutputScheduleH::try_new(input).unwrap();

        // No credit reduction applied — line 23 = min(378, 378) = 378
        assert_eq!(
            form.unempl_smaller_tax_adjustment_amt,
            Usd::from_dollars(378)
        );

        // Line 24 = 420 - 378 = 42
        assert_eq!(form.multi_state_futa_tax_amt, Usd::from_dollars(42));

        assert!(form.is_valid());
    }

    // ── Worksheet 1 — late contributions ─────────────────────────────

    #[test]
    fn section_b_late_contributions() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind = true;
        input.unempl_paid_only_one_state_ind = false;
        input.pay_all_state_unempl_contri_ind = false; // late!
        input.unemployment_state_tax_entries = vec![UnemploymentStateTaxInput {
            state_cd: "NY".to_string(),
            taxable_wages_amt: Usd::from_dollars(7_000),
            experience_rate_period_from: "01/01/2025".to_string(),
            experience_rate_period_to: "12/31/2025".to_string(),
            experience_rate_bps: 340, // 3.4%
            contributions_paid_amt: Usd::from_dollars(238),
        }];
        input.multi_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(7_000);
        input.late_contributions_paid_amt = Usd::from_dollars(100);
        let form = OutputScheduleH::try_new(input).unwrap();

        // col (g) = 7,000 × 5.4% - 7,000 × 3.4% = 378 - 238 = 140
        // Line 19 = 140 + 238 = 378
        // Line 22 = 7,000 × 5.4% = 378
        //
        // Worksheet 1:
        //   ws1_1 = 378 (line 22)
        //   ws1_2 = 378 (line 19)
        //   ws1_3 = max(378 - 378, 0) = 0
        //   ws1_5 = min(0, 100) = 0
        //   ws1_6 = 0 × 90% = 0
        //   ws1_7 = 378 + 0 = 378
        //   ws1_8 = min(378, 378) = 378
        // Line 23 = 378
        assert_eq!(
            form.unempl_smaller_tax_adjustment_amt,
            Usd::from_dollars(378)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn section_b_late_contributions_reduces_credit() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.hshld_empl_pd_cash_wage_ovr_lmt_qtr_ind = true;
        input.unempl_paid_only_one_state_ind = false;
        input.pay_all_state_unempl_contri_ind = false; // late!
        // State with low experience rate → line 19 < line 22
        input.unemployment_state_tax_entries = vec![UnemploymentStateTaxInput {
            state_cd: "NY".to_string(),
            taxable_wages_amt: Usd::from_dollars(7_000),
            experience_rate_period_from: "01/01/2025".to_string(),
            experience_rate_period_to: "12/31/2025".to_string(),
            experience_rate_bps: 200, // 2.0%
            contributions_paid_amt: Usd::from_dollars(100),
        }];
        input.multi_state_total_cash_wages_subj_futa_tax_amt = Usd::from_dollars(7_000);
        input.late_contributions_paid_amt = Usd::from_dollars(50);
        let form = OutputScheduleH::try_new(input).unwrap();

        // col (e) = 7,000 × 5.4% = 378
        // col (f) = 7,000 × 2.0% = 140
        // col (g) = 378 - 140 = 238
        // Line 19 = 238 + 100 = 338
        // Line 22 = 378
        //
        // Worksheet 1:
        //   ws1_3 = max(378 - 338, 0) = 40
        //   ws1_5 = min(40, 50) = 40
        //   ws1_6 = 40 × 90% = 36
        //   ws1_7 = 338 + 36 = 374
        //   ws1_8 = min(378, 374) = 374
        // Line 23 = 374
        assert_eq!(
            form.unempl_smaller_tax_adjustment_amt,
            Usd::from_dollars(374)
        );

        // Line 21 = 7,000 × 6.0% = 420
        // Line 24 = 420 - 374 = 46
        assert_eq!(form.multi_state_futa_tax_amt, Usd::from_dollars(46));
        assert!(form.is_valid());
    }

    // ── Validation catches wrong line 2 ───────────────────────────────

    #[test]
    fn validation_detects_bad_ss_tax() {
        let mut input = default_input();
        input.social_security_tax_cash_wages_amt = Usd::from_dollars(10_000);
        input.medicare_tax_cash_wages_amt = Usd::from_dollars(10_000);
        let mut form = OutputScheduleH::try_new(input).unwrap();
        form.social_security_tax_amt = Usd::from_dollars(999);
        assert!(!form.is_valid());
    }
}
