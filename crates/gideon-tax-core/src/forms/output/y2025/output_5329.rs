use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 5329 (2025) — Additional Taxes on
/// Qualified Plans (Including IRAs) and Other Tax-Favored Accounts.
///
/// Covers Parts I–IX: early distributions, education account distributions,
/// excess contributions to traditional IRAs / Roth IRAs / Coverdell ESAs /
/// Archer MSAs / HSAs / ABLE accounts, and excess accumulation in qualified
/// retirement plans.
#[derive(Debug, Clone)]
pub struct F5329Input {
    // ── Header ────────────────────────────────────────────────────────
    /// Name of individual subject to additional tax
    pub person_nm: String,
    /// Your social security number
    pub ssn: String,
    /// If this is an amended return, check here
    pub amended_return_ind: bool,

    // ── Part I — Additional Tax on Early Distributions ─────────────────
    /// Line 1: Early distributions includible in income
    pub early_distributions_amt: Usd,
    /// Line 2: Early distributions included on line 1 that are not subject
    /// to the additional tax
    pub early_distri_not_subject_to_tax_amt: Usd,
    /// Line 2: Exception reason code
    pub early_distri_exception_reason_cd: String,

    // ── Part II — Additional Tax on Certain Distributions From
    //    Education Accounts and ABLE Accounts ──────────────────────────
    /// Line 5: Distributions included in income from a Coverdell ESA,
    /// QTP, or ABLE account
    pub educ_acct_distribution_amt: Usd,
    /// Line 6: Distributions on line 5 not subject to the additional tax
    pub educ_acct_distri_not_subj_to_tax_amt: Usd,

    // ── Part III — Additional Tax on Excess Contributions to
    //    Traditional IRAs ──────────────────────────────────────────────
    /// Line 9: Excess contributions from line 16 of your 2024 Form 5329
    pub ira_excess_contri_prior_year_amt: Usd,
    /// Line 10: Difference if contributions < max allowable; else 0
    pub ira_excess_contri_current_year_amt: Usd,
    /// Line 11: 2025 traditional IRA distributions included in income
    pub ira_distri_included_in_income_amt: Usd,
    /// Line 12: 2025 distributions of prior year excess contributions
    pub ira_excess_contri_withdrawn_amt: Usd,
    /// Line 15: Excess contributions for 2025
    pub ira_excess_contri_credit_amt: Usd,
    /// Value of traditional IRAs on December 31, 2025 (for line 17 calculation)
    pub ira_year_end_value: Usd,

    // ── Part IV — Additional Tax on Excess Contributions to Roth IRAs ──
    /// Line 18: Excess contributions from line 24 of your 2024 Form 5329
    pub roth_ira_excess_contri_prior_yr_amt: Usd,
    /// Line 19: Difference if contributions < max allowable; else 0
    pub roth_ira_excess_contri_cy_amt: Usd,
    /// Line 20: 2025 distributions from Roth IRAs
    pub roth_ira_distri_included_in_cy_amt: Usd,
    /// Line 23: Excess contributions for 2025
    pub roth_ira_excess_contri_credit_amt: Usd,
    /// Value of Roth IRAs on December 31, 2025 (for line 25 calculation)
    pub roth_ira_year_end_value: Usd,

    // ── Part V — Additional Tax on Excess Contributions to Coverdell ESAs
    /// Line 26: Excess contributions from line 32 of your 2024 Form 5329
    pub esa_excess_contri_prior_year_amt: Usd,
    /// Line 27: Difference if contributions < max allowable; else 0
    pub esa_excess_contri_cy_amt: Usd,
    /// Line 28: 2025 distributions from Coverdell ESAs
    pub esa_distri_included_in_cy_amt: Usd,
    /// Line 31: Excess contributions for 2025
    pub esa_excess_contri_credit_amt: Usd,
    /// Value of Coverdell ESAs on December 31, 2025 (for line 33 calculation)
    pub esa_year_end_value: Usd,

    // ── Part VI — Additional Tax on Excess Contributions to Archer MSAs
    /// Line 34: Excess contributions from line 40 of your 2024 Form 5329
    pub archer_msa_excess_contri_pr_yr_amt: Usd,
    /// Line 35: Difference if contributions < max allowable; else 0
    pub archer_msa_excess_contri_cy_amt: Usd,
    /// Line 36: 2025 distributions from Archer MSAs (Form 8853, line 8)
    pub archer_msa_excess_contri_adj_amt: Usd,
    /// Line 39: Excess contributions for 2025
    pub taxable_archer_msa_distri_amt: Usd,
    /// Value of Archer MSAs on December 31, 2025 (for line 41 calculation)
    pub archer_msa_year_end_value: Usd,

    // ── Part VII — Additional Tax on Excess Contributions to HSAs ─────
    /// Line 42: Excess contributions from line 48 of your 2024 Form 5329
    pub hsa_excess_contri_prior_year_amt: Usd,
    /// Line 43: Difference if contributions < max allowable; else 0
    pub hsa_excess_contri_current_year_amt: Usd,
    /// Line 44: 2025 distributions from HSAs (Form 8889, line 16)
    pub hsa_excess_contri_py_adjusted_amt: Usd,
    /// Line 47: Excess contributions for 2025
    pub taxable_hsa_distribution_amt: Usd,
    /// Value of HSAs on December 31, 2025 (for line 49 calculation)
    pub hsa_year_end_value: Usd,

    // ── Part VIII — Additional Tax on Excess Contributions to ABLE Account
    /// Line 50: Excess contributions for 2025
    pub able_excess_contri_cy_amt: Usd,
    /// Value of ABLE account on December 31, 2025 (for line 51 calculation)
    pub able_year_end_value: Usd,

    // ── Part IX — Additional Tax on Excess Accumulation in
    //    Qualified Retirement Plans ────────────────────────────────────
    /// Line 52a: Minimum required distribution for 2025 (correction window plans)
    pub qlfy_retire_plan_min_rqr_distri_amt: Usd,
    /// Line 52b: Minimum required distribution for 2025 (all other plans)
    pub all_oth_qlfy_plan_min_rqr_distri_amt: Usd,
    /// Line 53a: Amount actually distributed (correction window plans)
    pub qlfy_retire_plan_actual_distri_amt: Usd,
    /// Line 53b: Amount actually distributed (all other plans)
    pub all_oth_qlfy_plan_actual_distri_amt: Usd,
    /// Line 54b: Waiver of tax statement code
    pub waive_tax_on_ex_accum_qrp_stmt_cd: String,
    /// Line 54b: Waiver of tax statement amount
    pub waive_tax_on_ex_accum_qrp_stmt_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 5329 (2025) — Additional Taxes on Qualified Plans and Other Tax-Favored Accounts.
#[derive(Debug, Clone, Default)]
pub struct Output5329 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of individual subject to additional tax
    pub person_nm: String,
    /// Your social security number
    pub ssn: String,
    /// If this is an amended return, check here
    pub amended_return_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Additional Tax on Early Distributions
    // -----------------------------------------------------------------------
    /// Line 1: Early distributions includible in income
    pub early_distributions_amt: Usd,
    /// Line 2: Early distributions included on line 1 that are not subject to the additional tax.
    /// Enter the appropriate exception number from the instructions
    pub early_distri_not_subject_to_tax_amt: Usd,
    /// Line 2: Exception reason code
    pub early_distri_exception_reason_cd: String,
    /// Line 3: Amount subject to additional tax. Subtract line 2 from line 1
    pub early_distri_subject_to_tax_amt: Usd,
    /// Line 4: Additional tax. Enter 10% (0.10) of line 3
    pub ira_early_distributions_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Additional Tax on Certain Distributions From Education Accounts and ABLE Accounts
    // -----------------------------------------------------------------------
    /// Line 5: Distributions included in income from a Coverdell ESA, a QTP, or an ABLE account
    pub educ_acct_distribution_amt: Usd,
    /// Line 6: Distributions included on line 5 that are not subject to the additional tax
    pub educ_acct_distri_not_subj_to_tax_amt: Usd,
    /// Line 7: Amount subject to additional tax. Subtract line 6 from line 5
    pub educ_acct_distri_subject_to_tax_amt: Usd,
    /// Line 8: Additional tax. Enter 10% (0.10) of line 7
    pub educ_ira_distributions_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Additional Tax on Excess Contributions to Traditional IRAs
    // -----------------------------------------------------------------------
    /// Line 9: Enter your excess contributions from line 16 of your 2024 Form 5329. If zero, go to line 15
    pub ira_excess_contri_prior_year_amt: Usd,
    /// Line 10: If your traditional IRA contributions for 2025 are less than your maximum allowable
    /// contribution, enter the difference. Otherwise, enter -0-
    pub ira_excess_contri_current_year_amt: Usd,
    /// Line 11: 2025 traditional IRA distributions included in income
    pub ira_distri_included_in_income_amt: Usd,
    /// Line 12: 2025 distributions of prior year excess contributions to traditional IRAs
    pub ira_excess_contri_withdrawn_amt: Usd,
    /// Line 13: Add lines 10, 11, and 12
    pub ira_excess_contri_adjustment_amt: Usd,
    /// Line 14: Prior year excess contributions. Subtract line 13 from line 9. If zero or less, enter -0-
    pub ira_excess_contri_pr_yr_adjust_amt: Usd,
    /// Line 15: Excess contributions for 2025
    pub ira_excess_contri_credit_amt: Usd,
    /// Line 16: Total excess contributions. Add lines 14 and 15
    pub ira_excess_contri_total_amt: Usd,
    /// Line 17: Additional tax. Enter 6% (0.06) of the smaller of line 16 or the value of your
    /// traditional IRAs on December 31, 2025
    pub ira_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Additional Tax on Excess Contributions to Roth IRAs
    // -----------------------------------------------------------------------
    /// Line 18: Enter your excess contributions from line 24 of your 2024 Form 5329. If zero, go to line 23
    pub roth_ira_excess_contri_prior_yr_amt: Usd,
    /// Line 19: If your Roth IRA contributions for 2025 are less than your maximum allowable
    /// contribution, enter the difference. Otherwise, enter -0-
    pub roth_ira_excess_contri_cy_amt: Usd,
    /// Line 20: 2025 distributions from your Roth IRAs
    pub roth_ira_distri_included_in_cy_amt: Usd,
    /// Line 21: Add lines 19 and 20
    pub roth_ira_excess_contri_adjust_amt: Usd,
    /// Line 22: Prior year excess contributions. Subtract line 21 from line 18. If zero or less, enter -0-
    pub roth_ira_excess_contri_py_wthdrw_amt: Usd,
    /// Line 23: Excess contributions for 2025
    pub roth_ira_excess_contri_credit_amt: Usd,
    /// Line 24: Total excess contributions. Add lines 22 and 23
    pub roth_ira_excess_contri_total_amt: Usd,
    /// Line 25: Additional tax. Enter 6% (0.06) of the smaller of line 24 or the value of your
    /// Roth IRAs on December 31, 2025
    pub roth_ira_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Additional Tax on Excess Contributions to Coverdell ESAs
    // -----------------------------------------------------------------------
    /// Line 26: Enter the excess contributions from line 32 of your 2024 Form 5329. If zero, go to line 31
    pub esa_excess_contri_prior_year_amt: Usd,
    /// Line 27: If the contributions to your Coverdell ESAs for 2025 were less than the maximum
    /// allowable contribution, enter the difference. Otherwise, enter -0-
    pub esa_excess_contri_cy_amt: Usd,
    /// Line 28: 2025 distributions from your Coverdell ESAs
    pub esa_distri_included_in_cy_amt: Usd,
    /// Line 29: Add lines 27 and 28
    pub esa_excess_contri_adjustment_amt: Usd,
    /// Line 30: Prior year excess contributions. Subtract line 29 from line 26. If zero or less, enter -0-
    pub esa_excess_contri_py_wthdrw_amt: Usd,
    /// Line 31: Excess contributions for 2025
    pub esa_excess_contri_credit_amt: Usd,
    /// Line 32: Total excess contributions. Add lines 30 and 31
    pub esa_excess_contri_total_amt: Usd,
    /// Line 33: Additional tax. Enter 6% (0.06) of the smaller of line 32 or the value of your
    /// Coverdell ESAs on December 31, 2025
    pub educ_ira_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VI — Additional Tax on Excess Contributions to Archer MSAs
    // -----------------------------------------------------------------------
    /// Line 34: Enter the excess contributions from line 40 of your 2024 Form 5329. If zero, go to line 39
    pub archer_msa_excess_contri_pr_yr_amt: Usd,
    /// Line 35: If the contributions to your Archer MSAs for 2025 are less than the maximum
    /// allowable contribution, enter the difference. Otherwise, enter -0-
    pub archer_msa_excess_contri_cy_amt: Usd,
    /// Line 36: 2025 distributions from your Archer MSAs from Form 8853, line 8
    pub archer_msa_excess_contri_adj_amt: Usd,
    /// Line 37: Add lines 35 and 36
    pub archer_msa_excess_contri_credit_amt: Usd,
    /// Line 38: Prior year excess contributions. Subtract line 37 from line 34. If zero or less, enter -0-
    pub archer_msa_ex_contri_py_wthdrw_amt: Usd,
    /// Line 39: Excess contributions for 2025
    pub taxable_archer_msa_distri_amt: Usd,
    /// Line 40: Total excess contributions. Add lines 38 and 39
    pub archer_msa_excess_contri_total_amt: Usd,
    /// Line 41: Additional tax. Enter 6% (0.06) of the smaller of line 40 or the value of your
    /// Archer MSAs on December 31, 2025
    pub msa_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VII — Additional Tax on Excess Contributions to Health Savings Accounts (HSAs)
    // -----------------------------------------------------------------------
    /// Line 42: Enter the excess contributions from line 48 of your 2024 Form 5329. If zero, go to line 47
    pub hsa_excess_contri_prior_year_amt: Usd,
    /// Line 43: If the contributions to your HSAs for 2025 are less than the maximum allowable
    /// contribution, enter the difference. Otherwise, enter -0-
    pub hsa_excess_contri_current_year_amt: Usd,
    /// Line 44: 2025 distributions from your HSAs from Form 8889, line 16
    pub hsa_excess_contri_py_adjusted_amt: Usd,
    /// Line 45: Add lines 43 and 44
    pub hsa_excess_contri_adjustment_amt: Usd,
    /// Line 46: Prior year excess contributions. Subtract line 45 from line 42. If zero or less, enter -0-
    pub hsa_excess_contri_credit_amt: Usd,
    /// Line 47: Excess contributions for 2025
    pub taxable_hsa_distribution_amt: Usd,
    /// Line 48: Total excess contributions. Add lines 46 and 47
    pub hsa_excess_contri_total_amt: Usd,
    /// Line 49: Additional tax. Enter 6% (0.06) of the smaller of line 48 or the value of your
    /// HSAs on December 31, 2025
    pub hsa_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VIII — Additional Tax on Excess Contributions to an ABLE Account
    // -----------------------------------------------------------------------
    /// Line 50: Excess contributions for 2025
    pub able_excess_contri_cy_amt: Usd,
    /// Line 51: Additional tax. Enter 6% (0.06) of the smaller of line 50 or the value of your
    /// ABLE account on December 31, 2025
    pub able_excess_contrib_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IX — Additional Tax on Excess Accumulation in Qualified Retirement Plans (Including IRAs)
    // -----------------------------------------------------------------------
    /// Line 52a: Minimum required distribution for 2025 from all qualified plans for which you
    /// received a distribution of the full amount of the excess accumulation during the correction window
    pub qlfy_retire_plan_min_rqr_distri_amt: Usd,
    /// Line 52b: Minimum required distribution for 2025 from all other plans
    pub all_oth_qlfy_plan_min_rqr_distri_amt: Usd,
    /// Line 53a: Amount distributed to you during 2025 from all qualified plans for which you
    /// received a distribution of the full amount of the excess accumulation during the correction window
    pub qlfy_retire_plan_actual_distri_amt: Usd,
    /// Line 53b: Amount distributed to you during 2025 from all other plans
    pub all_oth_qlfy_plan_actual_distri_amt: Usd,
    /// Line 54a: Subtract line 53a from line 52a and multiply the result by 10% (0.10). If zero or less, enter -0-
    pub qlfy_retire_plan_excess_accum_amt: Usd,
    /// Line 54b: Subtract line 53b from line 52b and multiply the result by 25% (0.25). If zero or less, enter -0-
    pub all_oth_qlfy_plan_excess_accum_amt: Usd,
    /// Line 54b: Waiver of tax on excess accumulation statement code
    pub waive_tax_on_ex_accum_qrp_stmt_cd: String,
    /// Line 54b: Waiver of tax on excess accumulation statement amount
    pub waive_tax_on_ex_accum_qrp_stmt_amt: Usd,
    /// Line 55: Add lines 54a and 54b. Include the total on Schedule 2 (Form 1040), line 8, or
    /// Form 1041, Schedule G, line 8
    pub rtmnt_annty_excess_contrib_tax_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output5329 {
    fn name() -> &'static str {
        "Form 5329"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

/// Computes 6% tax on the smaller of `total_excess` or `account_value`.
fn six_pct_tax(total_excess: Usd, account_value: Usd) -> Usd {
    Usd::from_cents(total_excess.min(account_value).cents() * 6 / 100)
}

impl OutputForm for Output5329 {
    type Input = F5329Input;

    fn must_file(input: &Self::Input) -> bool {
        input.early_distributions_amt > Usd::ZERO
            || input.educ_acct_distribution_amt > Usd::ZERO
            || input.ira_excess_contri_prior_year_amt > Usd::ZERO
            || input.ira_excess_contri_credit_amt > Usd::ZERO
            || input.roth_ira_excess_contri_prior_yr_amt > Usd::ZERO
            || input.roth_ira_excess_contri_credit_amt > Usd::ZERO
            || input.esa_excess_contri_prior_year_amt > Usd::ZERO
            || input.esa_excess_contri_credit_amt > Usd::ZERO
            || input.archer_msa_excess_contri_pr_yr_amt > Usd::ZERO
            || input.taxable_archer_msa_distri_amt > Usd::ZERO
            || input.hsa_excess_contri_prior_year_amt > Usd::ZERO
            || input.taxable_hsa_distribution_amt > Usd::ZERO
            || input.able_excess_contri_cy_amt > Usd::ZERO
            || input.qlfy_retire_plan_min_rqr_distri_amt > Usd::ZERO
            || input.all_oth_qlfy_plan_min_rqr_distri_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // ── Part I — Early Distributions ─────────────────────────────
        let line1 = input.early_distributions_amt;
        let line2 = input.early_distri_not_subject_to_tax_amt;
        let line3 = (line1 - line2).max(Usd::ZERO);
        let line4 = Usd::from_cents(line3.cents() * 10 / 100);

        // ── Part II — Education Account Distributions ────────────────
        let line5 = input.educ_acct_distribution_amt;
        let line6 = input.educ_acct_distri_not_subj_to_tax_amt;
        let line7 = (line5 - line6).max(Usd::ZERO);
        let line8 = Usd::from_cents(line7.cents() * 10 / 100);

        // ── Part III — Excess Contributions to Traditional IRAs ──────
        let line9 = input.ira_excess_contri_prior_year_amt;
        let line10 = input.ira_excess_contri_current_year_amt;
        let line11 = input.ira_distri_included_in_income_amt;
        let line12 = input.ira_excess_contri_withdrawn_amt;
        let line13 = line10 + line11 + line12;
        let line14 = (line9 - line13).max(Usd::ZERO);
        let line15 = input.ira_excess_contri_credit_amt;
        let line16 = line14 + line15;
        let line17 = six_pct_tax(line16, input.ira_year_end_value);

        // ── Part IV — Excess Contributions to Roth IRAs ──────────────
        let line18 = input.roth_ira_excess_contri_prior_yr_amt;
        let line19 = input.roth_ira_excess_contri_cy_amt;
        let line20 = input.roth_ira_distri_included_in_cy_amt;
        let line21 = line19 + line20;
        let line22 = (line18 - line21).max(Usd::ZERO);
        let line23 = input.roth_ira_excess_contri_credit_amt;
        let line24 = line22 + line23;
        let line25 = six_pct_tax(line24, input.roth_ira_year_end_value);

        // ── Part V — Excess Contributions to Coverdell ESAs ──────────
        let line26 = input.esa_excess_contri_prior_year_amt;
        let line27 = input.esa_excess_contri_cy_amt;
        let line28 = input.esa_distri_included_in_cy_amt;
        let line29 = line27 + line28;
        let line30 = (line26 - line29).max(Usd::ZERO);
        let line31 = input.esa_excess_contri_credit_amt;
        let line32 = line30 + line31;
        let line33 = six_pct_tax(line32, input.esa_year_end_value);

        // ── Part VI — Excess Contributions to Archer MSAs ────────────
        let line34 = input.archer_msa_excess_contri_pr_yr_amt;
        let line35 = input.archer_msa_excess_contri_cy_amt;
        let line36 = input.archer_msa_excess_contri_adj_amt;
        let line37 = line35 + line36;
        let line38 = (line34 - line37).max(Usd::ZERO);
        let line39 = input.taxable_archer_msa_distri_amt;
        let line40 = line38 + line39;
        let line41 = six_pct_tax(line40, input.archer_msa_year_end_value);

        // ── Part VII — Excess Contributions to HSAs ──────────────────
        let line42 = input.hsa_excess_contri_prior_year_amt;
        let line43 = input.hsa_excess_contri_current_year_amt;
        let line44 = input.hsa_excess_contri_py_adjusted_amt;
        let line45 = line43 + line44;
        let line46 = (line42 - line45).max(Usd::ZERO);
        let line47 = input.taxable_hsa_distribution_amt;
        let line48 = line46 + line47;
        let line49 = six_pct_tax(line48, input.hsa_year_end_value);

        // ── Part VIII — Excess Contributions to ABLE Account ─────────
        let line50 = input.able_excess_contri_cy_amt;
        let line51 = six_pct_tax(line50, input.able_year_end_value);

        // ── Part IX — Excess Accumulation ────────────────────────────
        let line52a = input.qlfy_retire_plan_min_rqr_distri_amt;
        let line52b = input.all_oth_qlfy_plan_min_rqr_distri_amt;
        let line53a = input.qlfy_retire_plan_actual_distri_amt;
        let line53b = input.all_oth_qlfy_plan_actual_distri_amt;
        let line54a = Usd::from_cents((line52a - line53a).max(Usd::ZERO).cents() * 10 / 100);
        let line54b = Usd::from_cents((line52b - line53b).max(Usd::ZERO).cents() * 25 / 100);
        let line55 = line54a + line54b;

        Ok(Output5329 {
            // Header
            person_nm: input.person_nm,
            ssn: input.ssn,
            amended_return_ind: input.amended_return_ind,
            // Part I
            early_distributions_amt: line1,
            early_distri_not_subject_to_tax_amt: line2,
            early_distri_exception_reason_cd: input.early_distri_exception_reason_cd,
            early_distri_subject_to_tax_amt: line3,
            ira_early_distributions_tax_amt: line4,
            // Part II
            educ_acct_distribution_amt: line5,
            educ_acct_distri_not_subj_to_tax_amt: line6,
            educ_acct_distri_subject_to_tax_amt: line7,
            educ_ira_distributions_tax_amt: line8,
            // Part III
            ira_excess_contri_prior_year_amt: line9,
            ira_excess_contri_current_year_amt: line10,
            ira_distri_included_in_income_amt: line11,
            ira_excess_contri_withdrawn_amt: line12,
            ira_excess_contri_adjustment_amt: line13,
            ira_excess_contri_pr_yr_adjust_amt: line14,
            ira_excess_contri_credit_amt: line15,
            ira_excess_contri_total_amt: line16,
            ira_excess_contrib_tax_amt: line17,
            // Part IV
            roth_ira_excess_contri_prior_yr_amt: line18,
            roth_ira_excess_contri_cy_amt: line19,
            roth_ira_distri_included_in_cy_amt: line20,
            roth_ira_excess_contri_adjust_amt: line21,
            roth_ira_excess_contri_py_wthdrw_amt: line22,
            roth_ira_excess_contri_credit_amt: line23,
            roth_ira_excess_contri_total_amt: line24,
            roth_ira_excess_contrib_tax_amt: line25,
            // Part V
            esa_excess_contri_prior_year_amt: line26,
            esa_excess_contri_cy_amt: line27,
            esa_distri_included_in_cy_amt: line28,
            esa_excess_contri_adjustment_amt: line29,
            esa_excess_contri_py_wthdrw_amt: line30,
            esa_excess_contri_credit_amt: line31,
            esa_excess_contri_total_amt: line32,
            educ_ira_excess_contrib_tax_amt: line33,
            // Part VI
            archer_msa_excess_contri_pr_yr_amt: line34,
            archer_msa_excess_contri_cy_amt: line35,
            archer_msa_excess_contri_adj_amt: line36,
            archer_msa_excess_contri_credit_amt: line37,
            archer_msa_ex_contri_py_wthdrw_amt: line38,
            taxable_archer_msa_distri_amt: line39,
            archer_msa_excess_contri_total_amt: line40,
            msa_excess_contrib_tax_amt: line41,
            // Part VII
            hsa_excess_contri_prior_year_amt: line42,
            hsa_excess_contri_current_year_amt: line43,
            hsa_excess_contri_py_adjusted_amt: line44,
            hsa_excess_contri_adjustment_amt: line45,
            hsa_excess_contri_credit_amt: line46,
            taxable_hsa_distribution_amt: line47,
            hsa_excess_contri_total_amt: line48,
            hsa_excess_contrib_tax_amt: line49,
            // Part VIII
            able_excess_contri_cy_amt: line50,
            able_excess_contrib_tax_amt: line51,
            // Part IX
            qlfy_retire_plan_min_rqr_distri_amt: line52a,
            all_oth_qlfy_plan_min_rqr_distri_amt: line52b,
            qlfy_retire_plan_actual_distri_amt: line53a,
            all_oth_qlfy_plan_actual_distri_amt: line53b,
            qlfy_retire_plan_excess_accum_amt: line54a,
            all_oth_qlfy_plan_excess_accum_amt: line54b,
            waive_tax_on_ex_accum_qrp_stmt_cd: input.waive_tax_on_ex_accum_qrp_stmt_cd,
            waive_tax_on_ex_accum_qrp_stmt_amt: input.waive_tax_on_ex_accum_qrp_stmt_amt,
            rtmnt_annty_excess_contrib_tax_amt: line55,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F8853, DynForm::F8889]
    }

    fn is_valid(&self) -> bool {
        // Part I
        let line3_ok = self.early_distri_subject_to_tax_amt
            == (self.early_distributions_amt - self.early_distri_not_subject_to_tax_amt)
                .max(Usd::ZERO);
        let line4_ok = self.ira_early_distributions_tax_amt
            == Usd::from_cents(self.early_distri_subject_to_tax_amt.cents() * 10 / 100);

        // Part II
        let line7_ok = self.educ_acct_distri_subject_to_tax_amt
            == (self.educ_acct_distribution_amt - self.educ_acct_distri_not_subj_to_tax_amt)
                .max(Usd::ZERO);
        let line8_ok = self.educ_ira_distributions_tax_amt
            == Usd::from_cents(self.educ_acct_distri_subject_to_tax_amt.cents() * 10 / 100);

        // Part III
        let line13_ok = self.ira_excess_contri_adjustment_amt
            == self.ira_excess_contri_current_year_amt
                + self.ira_distri_included_in_income_amt
                + self.ira_excess_contri_withdrawn_amt;
        let line14_ok = self.ira_excess_contri_pr_yr_adjust_amt
            == (self.ira_excess_contri_prior_year_amt - self.ira_excess_contri_adjustment_amt)
                .max(Usd::ZERO);
        let line16_ok = self.ira_excess_contri_total_amt
            == self.ira_excess_contri_pr_yr_adjust_amt + self.ira_excess_contri_credit_amt;

        // Part IV
        let line21_ok = self.roth_ira_excess_contri_adjust_amt
            == self.roth_ira_excess_contri_cy_amt + self.roth_ira_distri_included_in_cy_amt;
        let line22_ok = self.roth_ira_excess_contri_py_wthdrw_amt
            == (self.roth_ira_excess_contri_prior_yr_amt
                - self.roth_ira_excess_contri_adjust_amt)
                .max(Usd::ZERO);
        let line24_ok = self.roth_ira_excess_contri_total_amt
            == self.roth_ira_excess_contri_py_wthdrw_amt + self.roth_ira_excess_contri_credit_amt;

        // Part V
        let line29_ok = self.esa_excess_contri_adjustment_amt
            == self.esa_excess_contri_cy_amt + self.esa_distri_included_in_cy_amt;
        let line30_ok = self.esa_excess_contri_py_wthdrw_amt
            == (self.esa_excess_contri_prior_year_amt - self.esa_excess_contri_adjustment_amt)
                .max(Usd::ZERO);
        let line32_ok = self.esa_excess_contri_total_amt
            == self.esa_excess_contri_py_wthdrw_amt + self.esa_excess_contri_credit_amt;

        // Part VI
        let line37_ok = self.archer_msa_excess_contri_credit_amt
            == self.archer_msa_excess_contri_cy_amt + self.archer_msa_excess_contri_adj_amt;
        let line38_ok = self.archer_msa_ex_contri_py_wthdrw_amt
            == (self.archer_msa_excess_contri_pr_yr_amt
                - self.archer_msa_excess_contri_credit_amt)
                .max(Usd::ZERO);
        let line40_ok = self.archer_msa_excess_contri_total_amt
            == self.archer_msa_ex_contri_py_wthdrw_amt + self.taxable_archer_msa_distri_amt;

        // Part VII
        let line45_ok = self.hsa_excess_contri_adjustment_amt
            == self.hsa_excess_contri_current_year_amt + self.hsa_excess_contri_py_adjusted_amt;
        let line46_ok = self.hsa_excess_contri_credit_amt
            == (self.hsa_excess_contri_prior_year_amt - self.hsa_excess_contri_adjustment_amt)
                .max(Usd::ZERO);
        let line48_ok = self.hsa_excess_contri_total_amt
            == self.hsa_excess_contri_credit_amt + self.taxable_hsa_distribution_amt;

        // Part IX
        let line54a_ok = self.qlfy_retire_plan_excess_accum_amt
            == Usd::from_cents(
                (self.qlfy_retire_plan_min_rqr_distri_amt
                    - self.qlfy_retire_plan_actual_distri_amt)
                    .max(Usd::ZERO)
                    .cents()
                    * 10
                    / 100,
            );
        let line54b_ok = self.all_oth_qlfy_plan_excess_accum_amt
            == Usd::from_cents(
                (self.all_oth_qlfy_plan_min_rqr_distri_amt
                    - self.all_oth_qlfy_plan_actual_distri_amt)
                    .max(Usd::ZERO)
                    .cents()
                    * 25
                    / 100,
            );
        let line55_ok = self.rtmnt_annty_excess_contrib_tax_amt
            == self.qlfy_retire_plan_excess_accum_amt + self.all_oth_qlfy_plan_excess_accum_amt;

        line3_ok
            && line4_ok
            && line7_ok
            && line8_ok
            && line13_ok
            && line14_ok
            && line16_ok
            && line21_ok
            && line22_ok
            && line24_ok
            && line29_ok
            && line30_ok
            && line32_ok
            && line37_ok
            && line38_ok
            && line40_ok
            && line45_ok
            && line46_ok
            && line48_ok
            && line54a_ok
            && line54b_ok
            && line55_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_input() -> F5329Input {
        F5329Input {
            person_nm: String::new(),
            ssn: String::new(),
            amended_return_ind: false,
            early_distributions_amt: Usd::ZERO,
            early_distri_not_subject_to_tax_amt: Usd::ZERO,
            early_distri_exception_reason_cd: String::new(),
            educ_acct_distribution_amt: Usd::ZERO,
            educ_acct_distri_not_subj_to_tax_amt: Usd::ZERO,
            ira_excess_contri_prior_year_amt: Usd::ZERO,
            ira_excess_contri_current_year_amt: Usd::ZERO,
            ira_distri_included_in_income_amt: Usd::ZERO,
            ira_excess_contri_withdrawn_amt: Usd::ZERO,
            ira_excess_contri_credit_amt: Usd::ZERO,
            ira_year_end_value: Usd::ZERO,
            roth_ira_excess_contri_prior_yr_amt: Usd::ZERO,
            roth_ira_excess_contri_cy_amt: Usd::ZERO,
            roth_ira_distri_included_in_cy_amt: Usd::ZERO,
            roth_ira_excess_contri_credit_amt: Usd::ZERO,
            roth_ira_year_end_value: Usd::ZERO,
            esa_excess_contri_prior_year_amt: Usd::ZERO,
            esa_excess_contri_cy_amt: Usd::ZERO,
            esa_distri_included_in_cy_amt: Usd::ZERO,
            esa_excess_contri_credit_amt: Usd::ZERO,
            esa_year_end_value: Usd::ZERO,
            archer_msa_excess_contri_pr_yr_amt: Usd::ZERO,
            archer_msa_excess_contri_cy_amt: Usd::ZERO,
            archer_msa_excess_contri_adj_amt: Usd::ZERO,
            taxable_archer_msa_distri_amt: Usd::ZERO,
            archer_msa_year_end_value: Usd::ZERO,
            hsa_excess_contri_prior_year_amt: Usd::ZERO,
            hsa_excess_contri_current_year_amt: Usd::ZERO,
            hsa_excess_contri_py_adjusted_amt: Usd::ZERO,
            taxable_hsa_distribution_amt: Usd::ZERO,
            hsa_year_end_value: Usd::ZERO,
            able_excess_contri_cy_amt: Usd::ZERO,
            able_year_end_value: Usd::ZERO,
            qlfy_retire_plan_min_rqr_distri_amt: Usd::ZERO,
            all_oth_qlfy_plan_min_rqr_distri_amt: Usd::ZERO,
            qlfy_retire_plan_actual_distri_amt: Usd::ZERO,
            all_oth_qlfy_plan_actual_distri_amt: Usd::ZERO,
            waive_tax_on_ex_accum_qrp_stmt_cd: String::new(),
            waive_tax_on_ex_accum_qrp_stmt_amt: Usd::ZERO,
        }
    }

    // ── must_file ────────────────────────────────────────────────────

    #[test]
    fn must_file_false_no_activity() {
        assert!(!Output5329::must_file(&default_input()));
    }

    #[test]
    fn must_file_early_distributions() {
        let mut input = default_input();
        input.early_distributions_amt = Usd::from_dollars(10_000);
        assert!(Output5329::must_file(&input));
    }

    #[test]
    fn must_file_excess_ira_contributions() {
        let mut input = default_input();
        input.ira_excess_contri_credit_amt = Usd::from_dollars(500);
        assert!(Output5329::must_file(&input));
    }

    // ── Part I — Early Distributions ─────────────────────────────────

    #[test]
    fn part_i_early_distribution_10_percent() {
        let mut input = default_input();
        input.early_distributions_amt = Usd::from_dollars(20_000);
        input.early_distri_not_subject_to_tax_amt = Usd::from_dollars(5_000);
        let form = Output5329::try_new(input).unwrap();
        // line 3: 20,000 - 5,000 = 15,000
        assert_eq!(
            form.early_distri_subject_to_tax_amt,
            Usd::from_dollars(15_000)
        );
        // line 4: 10% of 15,000 = 1,500
        assert_eq!(
            form.ira_early_distributions_tax_amt,
            Usd::from_dollars(1_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_exception_exceeds_distribution() {
        let mut input = default_input();
        input.early_distributions_amt = Usd::from_dollars(5_000);
        input.early_distri_not_subject_to_tax_amt = Usd::from_dollars(8_000);
        let form = Output5329::try_new(input).unwrap();
        assert_eq!(form.early_distri_subject_to_tax_amt, Usd::ZERO);
        assert_eq!(form.ira_early_distributions_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    // ── Part II — Education Account Distributions ────────────────────

    #[test]
    fn part_ii_education_distribution_10_percent() {
        let mut input = default_input();
        input.educ_acct_distribution_amt = Usd::from_dollars(10_000);
        input.educ_acct_distri_not_subj_to_tax_amt = Usd::from_dollars(3_000);
        let form = Output5329::try_new(input).unwrap();
        // line 7: 10,000 - 3,000 = 7,000
        assert_eq!(
            form.educ_acct_distri_subject_to_tax_amt,
            Usd::from_dollars(7_000)
        );
        // line 8: 10% of 7,000 = 700
        assert_eq!(
            form.educ_ira_distributions_tax_amt,
            Usd::from_dollars(700)
        );
        assert!(form.is_valid());
    }

    // ── Part III — Traditional IRA Excess Contributions ──────────────

    #[test]
    fn part_iii_ira_excess_prior_year_carried() {
        let mut input = default_input();
        input.ira_excess_contri_prior_year_amt = Usd::from_dollars(3_000);
        input.ira_excess_contri_current_year_amt = Usd::from_dollars(1_000);
        input.ira_distri_included_in_income_amt = Usd::from_dollars(500);
        input.ira_excess_contri_withdrawn_amt = Usd::ZERO;
        input.ira_excess_contri_credit_amt = Usd::from_dollars(2_000);
        input.ira_year_end_value = Usd::from_dollars(50_000);
        let form = Output5329::try_new(input).unwrap();
        // line 13: 1,000 + 500 + 0 = 1,500
        assert_eq!(
            form.ira_excess_contri_adjustment_amt,
            Usd::from_dollars(1_500)
        );
        // line 14: max(3,000 - 1,500, 0) = 1,500
        assert_eq!(
            form.ira_excess_contri_pr_yr_adjust_amt,
            Usd::from_dollars(1_500)
        );
        // line 16: 1,500 + 2,000 = 3,500
        assert_eq!(
            form.ira_excess_contri_total_amt,
            Usd::from_dollars(3_500)
        );
        // line 17: 6% of min(3,500, 50,000) = 6% of 3,500 = 210
        assert_eq!(form.ira_excess_contrib_tax_amt, Usd::from_dollars(210));
        assert!(form.is_valid());
    }

    #[test]
    fn part_iii_ira_account_value_caps_tax() {
        let mut input = default_input();
        input.ira_excess_contri_credit_amt = Usd::from_dollars(10_000);
        input.ira_year_end_value = Usd::from_dollars(5_000);
        let form = Output5329::try_new(input).unwrap();
        // line 16: 0 + 10,000 = 10,000
        // line 17: 6% of min(10,000, 5,000) = 6% of 5,000 = 300
        assert_eq!(form.ira_excess_contrib_tax_amt, Usd::from_dollars(300));
        assert!(form.is_valid());
    }

    #[test]
    fn part_iii_ira_adjustments_exceed_prior_year() {
        let mut input = default_input();
        input.ira_excess_contri_prior_year_amt = Usd::from_dollars(2_000);
        input.ira_excess_contri_current_year_amt = Usd::from_dollars(1_000);
        input.ira_distri_included_in_income_amt = Usd::from_dollars(1_500);
        input.ira_year_end_value = Usd::from_dollars(50_000);
        let form = Output5329::try_new(input).unwrap();
        // line 13: 1,000 + 1,500 + 0 = 2,500
        // line 14: max(2,000 - 2,500, 0) = 0
        assert_eq!(form.ira_excess_contri_pr_yr_adjust_amt, Usd::ZERO);
        assert_eq!(form.ira_excess_contri_total_amt, Usd::ZERO);
        assert_eq!(form.ira_excess_contrib_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    // ── Part IV — Roth IRA Excess Contributions ──────────────────────

    #[test]
    fn part_iv_roth_ira_excess() {
        let mut input = default_input();
        input.roth_ira_excess_contri_prior_yr_amt = Usd::from_dollars(5_000);
        input.roth_ira_excess_contri_cy_amt = Usd::from_dollars(2_000);
        input.roth_ira_distri_included_in_cy_amt = Usd::from_dollars(1_000);
        input.roth_ira_excess_contri_credit_amt = Usd::from_dollars(500);
        input.roth_ira_year_end_value = Usd::from_dollars(100_000);
        let form = Output5329::try_new(input).unwrap();
        // line 21: 2,000 + 1,000 = 3,000
        // line 22: max(5,000 - 3,000, 0) = 2,000
        // line 24: 2,000 + 500 = 2,500
        // line 25: 6% of min(2,500, 100,000) = 150
        assert_eq!(
            form.roth_ira_excess_contri_total_amt,
            Usd::from_dollars(2_500)
        );
        assert_eq!(
            form.roth_ira_excess_contrib_tax_amt,
            Usd::from_dollars(150)
        );
        assert!(form.is_valid());
    }

    // ── Part VII — HSA Excess Contributions ──────────────────────────

    #[test]
    fn part_vii_hsa_excess() {
        let mut input = default_input();
        input.hsa_excess_contri_prior_year_amt = Usd::from_dollars(1_000);
        input.hsa_excess_contri_current_year_amt = Usd::from_dollars(300);
        input.hsa_excess_contri_py_adjusted_amt = Usd::from_dollars(200);
        input.taxable_hsa_distribution_amt = Usd::from_dollars(400);
        input.hsa_year_end_value = Usd::from_dollars(10_000);
        let form = Output5329::try_new(input).unwrap();
        // line 45: 300 + 200 = 500
        // line 46: max(1,000 - 500, 0) = 500
        // line 48: 500 + 400 = 900
        // line 49: 6% of min(900, 10,000) = 54
        assert_eq!(form.hsa_excess_contri_total_amt, Usd::from_dollars(900));
        assert_eq!(form.hsa_excess_contrib_tax_amt, Usd::from_dollars(54));
        assert!(form.is_valid());
    }

    // ── Part VIII — ABLE Account Excess Contributions ────────────────

    #[test]
    fn part_viii_able_excess() {
        let mut input = default_input();
        input.able_excess_contri_cy_amt = Usd::from_dollars(2_000);
        input.able_year_end_value = Usd::from_dollars(50_000);
        let form = Output5329::try_new(input).unwrap();
        // 6% of min(2,000, 50,000) = 120
        assert_eq!(form.able_excess_contrib_tax_amt, Usd::from_dollars(120));
        assert!(form.is_valid());
    }

    // ── Part IX — Excess Accumulation ────────────────────────────────

    #[test]
    fn part_ix_correction_window_10_percent() {
        let mut input = default_input();
        input.qlfy_retire_plan_min_rqr_distri_amt = Usd::from_dollars(10_000);
        input.qlfy_retire_plan_actual_distri_amt = Usd::from_dollars(6_000);
        let form = Output5329::try_new(input).unwrap();
        // line 54a: (10,000 - 6,000) * 10% = 400
        assert_eq!(
            form.qlfy_retire_plan_excess_accum_amt,
            Usd::from_dollars(400)
        );
        assert_eq!(
            form.rtmnt_annty_excess_contrib_tax_amt,
            Usd::from_dollars(400)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part_ix_other_plans_25_percent() {
        let mut input = default_input();
        input.all_oth_qlfy_plan_min_rqr_distri_amt = Usd::from_dollars(20_000);
        input.all_oth_qlfy_plan_actual_distri_amt = Usd::from_dollars(12_000);
        let form = Output5329::try_new(input).unwrap();
        // line 54b: (20,000 - 12,000) * 25% = 2,000
        assert_eq!(
            form.all_oth_qlfy_plan_excess_accum_amt,
            Usd::from_dollars(2_000)
        );
        assert_eq!(
            form.rtmnt_annty_excess_contrib_tax_amt,
            Usd::from_dollars(2_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part_ix_both_plans_combined() {
        let mut input = default_input();
        input.qlfy_retire_plan_min_rqr_distri_amt = Usd::from_dollars(10_000);
        input.qlfy_retire_plan_actual_distri_amt = Usd::from_dollars(6_000);
        input.all_oth_qlfy_plan_min_rqr_distri_amt = Usd::from_dollars(20_000);
        input.all_oth_qlfy_plan_actual_distri_amt = Usd::from_dollars(12_000);
        let form = Output5329::try_new(input).unwrap();
        // line 54a: 400, line 54b: 2,000, line 55: 2,400
        assert_eq!(
            form.rtmnt_annty_excess_contrib_tax_amt,
            Usd::from_dollars(2_400)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part_ix_no_shortfall() {
        let mut input = default_input();
        input.qlfy_retire_plan_min_rqr_distri_amt = Usd::from_dollars(10_000);
        input.qlfy_retire_plan_actual_distri_amt = Usd::from_dollars(15_000);
        let form = Output5329::try_new(input).unwrap();
        assert_eq!(form.qlfy_retire_plan_excess_accum_amt, Usd::ZERO);
        assert_eq!(form.rtmnt_annty_excess_contrib_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    // ── Zero everything ──────────────────────────────────────────────

    #[test]
    fn zero_everything() {
        let form = Output5329::try_new(default_input()).unwrap();
        assert_eq!(form.ira_early_distributions_tax_amt, Usd::ZERO);
        assert_eq!(form.educ_ira_distributions_tax_amt, Usd::ZERO);
        assert_eq!(form.ira_excess_contrib_tax_amt, Usd::ZERO);
        assert_eq!(form.roth_ira_excess_contrib_tax_amt, Usd::ZERO);
        assert_eq!(form.educ_ira_excess_contrib_tax_amt, Usd::ZERO);
        assert_eq!(form.msa_excess_contrib_tax_amt, Usd::ZERO);
        assert_eq!(form.hsa_excess_contrib_tax_amt, Usd::ZERO);
        assert_eq!(form.able_excess_contrib_tax_amt, Usd::ZERO);
        assert_eq!(form.rtmnt_annty_excess_contrib_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
