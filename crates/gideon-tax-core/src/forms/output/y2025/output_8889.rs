use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8889 (2025) — Health Savings Accounts.
///
/// HSA contributions, employer contributions, distributions, qualified medical
/// expenses, and HDHP coverage failure data feed into the computation.
/// Dependence on Form 8853 (Archer MSA contributions for line 4) is declared
/// in [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct F8889Input {
    /// Name of HSA beneficiary
    pub person_nm: String,
    /// Social security number of HSA beneficiary
    pub recipient_ssn: String,

    // ── Part I ────────────────────────────────────────────────────────
    /// Line 1: Self-only HDHP coverage indicator
    pub hdhp_self_only_coverage_ind: bool,
    /// Line 1: Family HDHP coverage indicator
    pub hdhp_family_coverage_ind: bool,
    /// Line 2: HSA contributions you made for 2025 (not employer, not cafeteria plan)
    pub hsa_contribution_amt: Usd,
    /// Line 3: HSA deduction limit (pre-computed from coverage type/months)
    pub hsa_limited_annual_deductible_amt: Usd,
    /// Line 4: Employer contributions to Archer MSAs (from Form 8853, lines 1+2)
    pub total_archer_msa_contribution_amt: Usd,
    /// Line 6: Amount from line 5 (or special rule for separate HSAs)
    pub hsa_limited_contribution_amt: Usd,
    /// Line 7: Additional contribution if age 55+ ($1,000)
    pub hsa_addnl_contribution_amt: Usd,
    /// Line 9: Employer contributions to your HSAs for 2025
    pub hsa_employer_contribution_amt: Usd,
    /// Line 10: Qualified HSA funding distributions
    pub hsa_qualified_funding_distri_amt: Usd,

    // ── Part II ───────────────────────────────────────────────────────
    /// Line 14a: Total distributions from all HSAs
    pub total_hsa_distribution_amt: Usd,
    /// Line 14b: Rollovers and excess contribution withdrawals
    pub hsa_distribution_rollover_amt: Usd,
    /// Line 15: Qualified medical expenses paid using HSA distributions
    pub unreimb_qual_med_and_dental_exp_amt: Usd,
    /// Line 17a: Exception to the Additional 20% Tax indicator
    pub hsa_distri_addnl_percent_tax_exc_ind: bool,

    // ── Part III ──────────────────────────────────────────────────────
    /// Line 18: Last-month rule income amount
    pub hdhp_coverage_fail_partial_yr_amt: Usd,
    /// Line 19: Qualified HSA funding distribution income
    pub hdhp_coverage_fail_fund_distri_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8889 (2025) — Health Savings Accounts.
#[derive(Debug, Clone, Default)]
pub struct Output8889 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of HSA beneficiary
    pub person_nm: String,
    /// Social security number of HSA beneficiary
    pub recipient_ssn: String,

    // -----------------------------------------------------------------------
    // Part I — HSA Contributions and Deduction
    // -----------------------------------------------------------------------
    /// Line 1: Check the box — Self-only coverage indicator
    pub hdhp_self_only_coverage_ind: bool,
    /// Line 1: Check the box — Family coverage indicator
    pub hdhp_family_coverage_ind: bool,
    /// Line 2: HSA contributions you made for 2025 (not including employer contributions)
    pub hsa_contribution_amt: Usd,
    /// Line 3: HSA deduction limit based on coverage type and age
    pub hsa_limited_annual_deductible_amt: Usd,
    /// Line 4: Employer contributions to your Archer MSAs for 2025
    pub total_archer_msa_contribution_amt: Usd,
    /// Line 5: Subtract line 4 from line 3 (if zero or less, enter -0-)
    pub hsa_limited_deductible_allwd_amt: Usd,
    /// Line 6: HSA deduction amount for separate HSAs (see instructions)
    pub hsa_limited_contribution_amt: Usd,
    /// Line 7: Additional contribution amount if age 55 or older
    pub hsa_addnl_contribution_amt: Usd,
    /// Line 8: Add lines 6 and 7
    pub hsa_limited_gross_contribution_amt: Usd,
    /// Line 9: Employer contributions made to your HSAs for 2025
    pub hsa_employer_contribution_amt: Usd,
    /// Line 10: Qualified HSA funding distributions
    pub hsa_qualified_funding_distri_amt: Usd,
    /// Line 11: Add lines 9 and 10
    pub total_hsa_contribution_amt: Usd,
    /// Line 12: Subtract line 11 from line 8 (if zero or less, enter -0-)
    pub hsa_family_deductible_amt: Usd,
    /// Line 13: HSA deduction — smaller of line 2 or line 12
    pub total_hsa_deduction_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — HSA Distributions
    // -----------------------------------------------------------------------
    /// Line 14a: Total distributions you received in 2025 from all HSAs
    pub total_hsa_distribution_amt: Usd,
    /// Line 14b: Rollover and excess contribution distributions included on line 14a
    pub hsa_distribution_rollover_amt: Usd,
    /// Line 14c: Subtract line 14b from line 14a
    pub hsa_net_distribution_amt: Usd,
    /// Line 15: Qualified medical expenses paid using HSA distributions
    pub unreimb_qual_med_and_dental_exp_amt: Usd,
    /// Line 16: Taxable HSA distributions (subtract line 15 from line 14c; if zero or less, enter -0-)
    pub taxable_hsa_distribution_amt: Usd,
    /// Line 17a: Exception to the Additional 20% Tax indicator
    pub hsa_distri_addnl_percent_tax_exc_ind: bool,
    /// Line 17b: Additional 20% tax on taxable distributions
    pub hsa_distri_addnl_percent_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Income and Additional Tax for Failure To Maintain HDHP Coverage
    // -----------------------------------------------------------------------
    /// Line 18: Last-month rule income amount
    pub hdhp_coverage_fail_partial_yr_amt: Usd,
    /// Line 19: Qualified HSA funding distribution income
    pub hdhp_coverage_fail_fund_distri_amt: Usd,
    /// Line 20: Total income (add lines 18 and 19)
    pub hdhp_coverage_income_amt: Usd,
    /// Line 21: Additional tax (multiply line 20 by 10%)
    pub hdhp_coverage_addnl_tax_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8889 {
    fn name() -> &'static str {
        "Form 8889"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8889 {
    type Input = F8889Input;

    fn must_file(input: &Self::Input) -> bool {
        input.hsa_contribution_amt > Usd::ZERO
            || input.hsa_employer_contribution_amt > Usd::ZERO
            || input.total_hsa_distribution_amt > Usd::ZERO
            || input.hdhp_coverage_fail_partial_yr_amt > Usd::ZERO
            || input.hdhp_coverage_fail_fund_distri_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // ── Part I ──────────────────────────────────────────────────
        let line2 = input.hsa_contribution_amt;
        let line3 = input.hsa_limited_annual_deductible_amt;
        let line4 = input.total_archer_msa_contribution_amt;
        let line5 = (line3 - line4).max(Usd::ZERO);
        let line6 = input.hsa_limited_contribution_amt;
        let line7 = input.hsa_addnl_contribution_amt;
        let line8 = line6 + line7;
        let line9 = input.hsa_employer_contribution_amt;
        let line10 = input.hsa_qualified_funding_distri_amt;
        let line11 = line9 + line10;
        let line12 = (line8 - line11).max(Usd::ZERO);
        let line13 = line2.min(line12);

        // ── Part II ─────────────────────────────────────────────────
        let line14a = input.total_hsa_distribution_amt;
        let line14b = input.hsa_distribution_rollover_amt;
        let line14c = line14a - line14b;
        let line15 = input.unreimb_qual_med_and_dental_exp_amt;
        let line16 = (line14c - line15).max(Usd::ZERO);
        let line17a = input.hsa_distri_addnl_percent_tax_exc_ind;
        // Line 17b: 20% of taxable distributions subject to additional tax.
        // If the exception applies to ALL distributions, the amount is 0.
        let line17b = if line17a {
            Usd::ZERO
        } else {
            Usd::from_cents(line16.cents() * 20 / 100)
        };

        // ── Part III ────────────────────────────────────────────────
        let line18 = input.hdhp_coverage_fail_partial_yr_amt;
        let line19 = input.hdhp_coverage_fail_fund_distri_amt;
        let line20 = line18 + line19;
        let line21 = Usd::from_cents(line20.cents() * 10 / 100);

        Ok(Output8889 {
            person_nm: input.person_nm,
            recipient_ssn: input.recipient_ssn,
            // Part I
            hdhp_self_only_coverage_ind: input.hdhp_self_only_coverage_ind,
            hdhp_family_coverage_ind: input.hdhp_family_coverage_ind,
            hsa_contribution_amt: line2,
            hsa_limited_annual_deductible_amt: line3,
            total_archer_msa_contribution_amt: line4,
            hsa_limited_deductible_allwd_amt: line5,
            hsa_limited_contribution_amt: line6,
            hsa_addnl_contribution_amt: line7,
            hsa_limited_gross_contribution_amt: line8,
            hsa_employer_contribution_amt: line9,
            hsa_qualified_funding_distri_amt: line10,
            total_hsa_contribution_amt: line11,
            hsa_family_deductible_amt: line12,
            total_hsa_deduction_amt: line13,
            // Part II
            total_hsa_distribution_amt: line14a,
            hsa_distribution_rollover_amt: line14b,
            hsa_net_distribution_amt: line14c,
            unreimb_qual_med_and_dental_exp_amt: line15,
            taxable_hsa_distribution_amt: line16,
            hsa_distri_addnl_percent_tax_exc_ind: line17a,
            hsa_distri_addnl_percent_tax_amt: line17b,
            // Part III
            hdhp_coverage_fail_partial_yr_amt: line18,
            hdhp_coverage_fail_fund_distri_amt: line19,
            hdhp_coverage_income_amt: line20,
            hdhp_coverage_addnl_tax_amt: line21,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F8853]
    }

    fn is_valid(&self) -> bool {
        // Part I
        let line5_ok =
            self.hsa_limited_deductible_allwd_amt
                == (self.hsa_limited_annual_deductible_amt
                    - self.total_archer_msa_contribution_amt)
                    .max(Usd::ZERO);
        let line8_ok = self.hsa_limited_gross_contribution_amt
            == self.hsa_limited_contribution_amt + self.hsa_addnl_contribution_amt;
        let line11_ok = self.total_hsa_contribution_amt
            == self.hsa_employer_contribution_amt + self.hsa_qualified_funding_distri_amt;
        let line12_ok = self.hsa_family_deductible_amt
            == (self.hsa_limited_gross_contribution_amt - self.total_hsa_contribution_amt)
                .max(Usd::ZERO);
        let line13_ok = self.total_hsa_deduction_amt
            == self
                .hsa_contribution_amt
                .min(self.hsa_family_deductible_amt);

        // Part II
        let line14c_ok = self.hsa_net_distribution_amt
            == self.total_hsa_distribution_amt - self.hsa_distribution_rollover_amt;
        let line16_ok = self.taxable_hsa_distribution_amt
            == (self.hsa_net_distribution_amt - self.unreimb_qual_med_and_dental_exp_amt)
                .max(Usd::ZERO);
        let line17b_ok = if self.hsa_distri_addnl_percent_tax_exc_ind {
            self.hsa_distri_addnl_percent_tax_amt == Usd::ZERO
        } else {
            self.hsa_distri_addnl_percent_tax_amt
                == Usd::from_cents(self.taxable_hsa_distribution_amt.cents() * 20 / 100)
        };

        // Part III
        let line20_ok = self.hdhp_coverage_income_amt
            == self.hdhp_coverage_fail_partial_yr_amt + self.hdhp_coverage_fail_fund_distri_amt;
        let line21_ok = self.hdhp_coverage_addnl_tax_amt
            == Usd::from_cents(self.hdhp_coverage_income_amt.cents() * 10 / 100);

        line5_ok
            && line8_ok
            && line11_ok
            && line12_ok
            && line13_ok
            && line14c_ok
            && line16_ok
            && line17b_ok
            && line20_ok
            && line21_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_input() -> F8889Input {
        F8889Input {
            person_nm: String::new(),
            recipient_ssn: String::new(),
            hdhp_self_only_coverage_ind: true,
            hdhp_family_coverage_ind: false,
            hsa_contribution_amt: Usd::from_dollars(3_000),
            hsa_limited_annual_deductible_amt: Usd::from_dollars(4_300),
            total_archer_msa_contribution_amt: Usd::ZERO,
            hsa_limited_contribution_amt: Usd::from_dollars(4_300),
            hsa_addnl_contribution_amt: Usd::ZERO,
            hsa_employer_contribution_amt: Usd::ZERO,
            hsa_qualified_funding_distri_amt: Usd::ZERO,
            total_hsa_distribution_amt: Usd::ZERO,
            hsa_distribution_rollover_amt: Usd::ZERO,
            unreimb_qual_med_and_dental_exp_amt: Usd::ZERO,
            hsa_distri_addnl_percent_tax_exc_ind: false,
            hdhp_coverage_fail_partial_yr_amt: Usd::ZERO,
            hdhp_coverage_fail_fund_distri_amt: Usd::ZERO,
        }
    }

    // ── must_file ─────────────────────────────────────────────────────

    #[test]
    fn must_file_with_contributions() {
        assert!(Output8889::must_file(&default_input()));
    }

    #[test]
    fn must_file_with_distributions_only() {
        let mut input = default_input();
        input.hsa_contribution_amt = Usd::ZERO;
        input.total_hsa_distribution_amt = Usd::from_dollars(500);
        assert!(Output8889::must_file(&input));
    }

    #[test]
    fn must_file_false_no_activity() {
        let mut input = default_input();
        input.hsa_contribution_amt = Usd::ZERO;
        assert!(!Output8889::must_file(&input));
    }

    // ── Part I ────────────────────────────────────────────────────────

    #[test]
    fn part_i_self_only_basic_deduction() {
        let form = Output8889::try_new(default_input()).unwrap();
        // Line 5: 4,300 - 0 = 4,300
        assert_eq!(
            form.hsa_limited_deductible_allwd_amt,
            Usd::from_dollars(4_300)
        );
        // Line 8: 4,300 + 0 = 4,300
        assert_eq!(
            form.hsa_limited_gross_contribution_amt,
            Usd::from_dollars(4_300)
        );
        // Line 12: 4,300 - 0 = 4,300
        assert_eq!(form.hsa_family_deductible_amt, Usd::from_dollars(4_300));
        // Line 13: min(3,000, 4,300) = 3,000
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(3_000));
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_family_coverage() {
        let mut input = default_input();
        input.hdhp_self_only_coverage_ind = false;
        input.hdhp_family_coverage_ind = true;
        input.hsa_contribution_amt = Usd::from_dollars(7_000);
        input.hsa_limited_annual_deductible_amt = Usd::from_dollars(8_550);
        input.hsa_limited_contribution_amt = Usd::from_dollars(8_550);
        let form = Output8889::try_new(input).unwrap();
        // Line 13: min(7,000, 8,550) = 7,000
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(7_000));
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_employer_contributions_reduce_deduction() {
        let mut input = default_input();
        input.hsa_contribution_amt = Usd::from_dollars(3_000);
        input.hsa_employer_contribution_amt = Usd::from_dollars(2_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 11: 2,000 + 0 = 2,000
        // Line 12: max(4,300 - 2,000, 0) = 2,300
        assert_eq!(form.hsa_family_deductible_amt, Usd::from_dollars(2_300));
        // Line 13: min(3,000, 2,300) = 2,300
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(2_300));
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_contributions_exceed_limit() {
        let mut input = default_input();
        input.hsa_contribution_amt = Usd::from_dollars(5_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 13: min(5,000, 4,300) = 4,300
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(4_300));
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_catch_up_contribution() {
        let mut input = default_input();
        input.hsa_contribution_amt = Usd::from_dollars(5_000);
        input.hsa_addnl_contribution_amt = Usd::from_dollars(1_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 8: 4,300 + 1,000 = 5,300
        assert_eq!(
            form.hsa_limited_gross_contribution_amt,
            Usd::from_dollars(5_300)
        );
        // Line 13: min(5,000, 5,300) = 5,000
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(5_000));
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_archer_msa_reduces_limit() {
        let mut input = default_input();
        input.total_archer_msa_contribution_amt = Usd::from_dollars(1_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 5: max(4,300 - 1,000, 0) = 3,300
        assert_eq!(
            form.hsa_limited_deductible_allwd_amt,
            Usd::from_dollars(3_300)
        );
        assert!(form.is_valid());
    }

    // ── Part II ───────────────────────────────────────────────────────

    #[test]
    fn part_ii_no_taxable_distribution() {
        let mut input = default_input();
        input.total_hsa_distribution_amt = Usd::from_dollars(2_000);
        input.unreimb_qual_med_and_dental_exp_amt = Usd::from_dollars(2_500);
        let form = Output8889::try_new(input).unwrap();
        assert_eq!(form.taxable_hsa_distribution_amt, Usd::ZERO);
        assert_eq!(form.hsa_distri_addnl_percent_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn part_ii_taxable_distribution_with_20_percent_tax() {
        let mut input = default_input();
        input.total_hsa_distribution_amt = Usd::from_dollars(5_000);
        input.unreimb_qual_med_and_dental_exp_amt = Usd::from_dollars(2_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 14c: 5,000 - 0 = 5,000
        // Line 16: max(5,000 - 2,000, 0) = 3,000
        assert_eq!(
            form.taxable_hsa_distribution_amt,
            Usd::from_dollars(3_000)
        );
        // Line 17b: 3,000 * 20% = 600
        assert_eq!(
            form.hsa_distri_addnl_percent_tax_amt,
            Usd::from_dollars(600)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part_ii_exception_to_additional_tax() {
        let mut input = default_input();
        input.total_hsa_distribution_amt = Usd::from_dollars(5_000);
        input.unreimb_qual_med_and_dental_exp_amt = Usd::from_dollars(2_000);
        input.hsa_distri_addnl_percent_tax_exc_ind = true;
        let form = Output8889::try_new(input).unwrap();
        assert_eq!(
            form.taxable_hsa_distribution_amt,
            Usd::from_dollars(3_000)
        );
        // Exception → no additional tax
        assert_eq!(form.hsa_distri_addnl_percent_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn part_ii_rollover_reduces_net_distribution() {
        let mut input = default_input();
        input.total_hsa_distribution_amt = Usd::from_dollars(5_000);
        input.hsa_distribution_rollover_amt = Usd::from_dollars(2_000);
        input.unreimb_qual_med_and_dental_exp_amt = Usd::from_dollars(1_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 14c: 5,000 - 2,000 = 3,000
        assert_eq!(form.hsa_net_distribution_amt, Usd::from_dollars(3_000));
        // Line 16: max(3,000 - 1,000, 0) = 2,000
        assert_eq!(
            form.taxable_hsa_distribution_amt,
            Usd::from_dollars(2_000)
        );
        assert!(form.is_valid());
    }

    // ── Part III ──────────────────────────────────────────────────────

    #[test]
    fn part_iii_failure_to_maintain_hdhp() {
        let mut input = default_input();
        input.hdhp_coverage_fail_partial_yr_amt = Usd::from_dollars(3_000);
        input.hdhp_coverage_fail_fund_distri_amt = Usd::from_dollars(1_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 20: 3,000 + 1,000 = 4,000
        assert_eq!(form.hdhp_coverage_income_amt, Usd::from_dollars(4_000));
        // Line 21: 4,000 * 10% = 400
        assert_eq!(form.hdhp_coverage_addnl_tax_amt, Usd::from_dollars(400));
        assert!(form.is_valid());
    }

    // ── Edge cases ────────────────────────────────────────────────────

    #[test]
    fn zero_everything() {
        let input = F8889Input {
            person_nm: String::new(),
            recipient_ssn: String::new(),
            hdhp_self_only_coverage_ind: false,
            hdhp_family_coverage_ind: false,
            hsa_contribution_amt: Usd::ZERO,
            hsa_limited_annual_deductible_amt: Usd::ZERO,
            total_archer_msa_contribution_amt: Usd::ZERO,
            hsa_limited_contribution_amt: Usd::ZERO,
            hsa_addnl_contribution_amt: Usd::ZERO,
            hsa_employer_contribution_amt: Usd::ZERO,
            hsa_qualified_funding_distri_amt: Usd::ZERO,
            total_hsa_distribution_amt: Usd::ZERO,
            hsa_distribution_rollover_amt: Usd::ZERO,
            unreimb_qual_med_and_dental_exp_amt: Usd::ZERO,
            hsa_distri_addnl_percent_tax_exc_ind: false,
            hdhp_coverage_fail_partial_yr_amt: Usd::ZERO,
            hdhp_coverage_fail_fund_distri_amt: Usd::ZERO,
        };
        let form = Output8889::try_new(input).unwrap();
        assert_eq!(form.total_hsa_deduction_amt, Usd::ZERO);
        assert_eq!(form.taxable_hsa_distribution_amt, Usd::ZERO);
        assert_eq!(form.hsa_distri_addnl_percent_tax_amt, Usd::ZERO);
        assert_eq!(form.hdhp_coverage_addnl_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn employer_exceeds_limit_zero_deduction() {
        let mut input = default_input();
        input.hsa_employer_contribution_amt = Usd::from_dollars(4_500);
        let form = Output8889::try_new(input).unwrap();
        // Line 12: max(4,300 - 4,500, 0) = 0
        assert_eq!(form.hsa_family_deductible_amt, Usd::ZERO);
        // Line 13: min(3,000, 0) = 0
        assert_eq!(form.total_hsa_deduction_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
