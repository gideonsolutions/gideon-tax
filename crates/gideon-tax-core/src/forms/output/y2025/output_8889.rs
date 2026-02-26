use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8889.
///
/// HSA contributions, distributions, and HDHP failure amounts feed into the
/// three parts of the form; the corresponding dependency is declared in
/// [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct F8889Input {
    /// Name of HSA beneficiary
    pub person_nm: String,
    /// Social security number of HSA beneficiary
    pub recipient_ssn: String,

    // -- Part I inputs --
    /// Line 1: Self-only HDHP coverage indicator
    pub hdhp_self_only_coverage_ind: bool,
    /// Line 1: Family HDHP coverage indicator
    pub hdhp_family_coverage_ind: bool,
    /// Line 2: HSA contributions you made for 2025 (not including employer
    /// contributions, rollovers, or qualified HSA funding distributions)
    pub hsa_contribution_amt: Usd,
    /// Line 3: HSA deduction limit based on coverage type and age
    pub hsa_limited_annual_deductible_amt: Usd,
    /// Line 4: Employer contributions to your Archer MSAs for 2025
    pub total_archer_msa_contribution_amt: Usd,
    /// Line 6: HSA deduction amount for separate HSAs (see instructions)
    pub hsa_limited_contribution_amt: Usd,
    /// Line 7: Additional contribution amount if age 55 or older
    pub hsa_addnl_contribution_amt: Usd,
    /// Line 9: Employer contributions made to your HSAs for 2025
    pub hsa_employer_contribution_amt: Usd,
    /// Line 10: Qualified HSA funding distributions
    pub hsa_qualified_funding_distri_amt: Usd,

    // -- Part II inputs --
    /// Line 14a: Total distributions you received in 2025 from all HSAs
    pub total_hsa_distribution_amt: Usd,
    /// Line 14b: Rollover and excess contribution distributions included on
    /// line 14a
    pub hsa_distribution_rollover_amt: Usd,
    /// Line 15: Qualified medical expenses paid using HSA distributions
    pub unreimb_qual_med_and_dental_exp_amt: Usd,
    /// Line 17a: Exception to the Additional 20% Tax indicator
    pub hsa_distri_addnl_percent_tax_exc_ind: bool,

    // -- Part III inputs --
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
        // Must file if there are HSA contributions, distributions, or HDHP
        // failure amounts.
        input.hsa_contribution_amt > Usd::ZERO
            || input.hsa_employer_contribution_amt > Usd::ZERO
            || input.hsa_qualified_funding_distri_amt > Usd::ZERO
            || input.total_hsa_distribution_amt > Usd::ZERO
            || input.hdhp_coverage_fail_partial_yr_amt > Usd::ZERO
            || input.hdhp_coverage_fail_fund_distri_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // -- Part I --

        // Line 5: max(Line 3 - Line 4, 0)
        let line5 =
            (input.hsa_limited_annual_deductible_amt - input.total_archer_msa_contribution_amt)
                .max(Usd::ZERO);

        // Line 8: Line 6 + Line 7
        let line8 = input.hsa_limited_contribution_amt + input.hsa_addnl_contribution_amt;

        // Line 11: Line 9 + Line 10
        let line11 =
            input.hsa_employer_contribution_amt + input.hsa_qualified_funding_distri_amt;

        // Line 12: max(Line 8 - Line 11, 0)
        let line12 = (line8 - line11).max(Usd::ZERO);

        // Line 13: min(Line 2, Line 12)
        let line13 = input.hsa_contribution_amt.min(line12);

        // -- Part II --

        // Line 14c: Line 14a - Line 14b
        let line14c = input.total_hsa_distribution_amt - input.hsa_distribution_rollover_amt;

        // Line 16: max(Line 14c - Line 15, 0)
        let line16 = (line14c - input.unreimb_qual_med_and_dental_exp_amt).max(Usd::ZERO);

        // Line 17b: if no exception, Line 16 × 20%; if exception, 0
        let line17b = if input.hsa_distri_addnl_percent_tax_exc_ind {
            Usd::ZERO
        } else {
            Usd::from_cents(line16.cents() * 20 / 100)
        };

        // -- Part III --

        // Line 20: Line 18 + Line 19
        let line20 =
            input.hdhp_coverage_fail_partial_yr_amt + input.hdhp_coverage_fail_fund_distri_amt;

        // Line 21: Line 20 × 10%
        let line21 = Usd::from_cents(line20.cents() * 10 / 100);

        Ok(Output8889 {
            // Top-of-form
            person_nm: input.person_nm,
            recipient_ssn: input.recipient_ssn,

            // Part I
            hdhp_self_only_coverage_ind: input.hdhp_self_only_coverage_ind,
            hdhp_family_coverage_ind: input.hdhp_family_coverage_ind,
            hsa_contribution_amt: input.hsa_contribution_amt,
            hsa_limited_annual_deductible_amt: input.hsa_limited_annual_deductible_amt,
            total_archer_msa_contribution_amt: input.total_archer_msa_contribution_amt,
            hsa_limited_deductible_allwd_amt: line5,
            hsa_limited_contribution_amt: input.hsa_limited_contribution_amt,
            hsa_addnl_contribution_amt: input.hsa_addnl_contribution_amt,
            hsa_limited_gross_contribution_amt: line8,
            hsa_employer_contribution_amt: input.hsa_employer_contribution_amt,
            hsa_qualified_funding_distri_amt: input.hsa_qualified_funding_distri_amt,
            total_hsa_contribution_amt: line11,
            hsa_family_deductible_amt: line12,
            total_hsa_deduction_amt: line13,

            // Part II
            total_hsa_distribution_amt: input.total_hsa_distribution_amt,
            hsa_distribution_rollover_amt: input.hsa_distribution_rollover_amt,
            hsa_net_distribution_amt: line14c,
            unreimb_qual_med_and_dental_exp_amt: input.unreimb_qual_med_and_dental_exp_amt,
            taxable_hsa_distribution_amt: line16,
            hsa_distri_addnl_percent_tax_exc_ind: input.hsa_distri_addnl_percent_tax_exc_ind,
            hsa_distri_addnl_percent_tax_amt: line17b,

            // Part III
            hdhp_coverage_fail_partial_yr_amt: input.hdhp_coverage_fail_partial_yr_amt,
            hdhp_coverage_fail_fund_distri_amt: input.hdhp_coverage_fail_fund_distri_amt,
            hdhp_coverage_income_amt: line20,
            hdhp_coverage_addnl_tax_amt: line21,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040]
    }

    fn is_valid(&self) -> bool {
        // Part I checks
        let line5_ok = self.hsa_limited_deductible_allwd_amt
            == (self.hsa_limited_annual_deductible_amt - self.total_archer_msa_contribution_amt)
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

        // Part II checks
        let line14c_ok = self.hsa_net_distribution_amt
            == self.total_hsa_distribution_amt - self.hsa_distribution_rollover_amt;

        let line16_ok = self.taxable_hsa_distribution_amt
            == (self.hsa_net_distribution_amt - self.unreimb_qual_med_and_dental_exp_amt)
                .max(Usd::ZERO);

        let line17b_ok = self.hsa_distri_addnl_percent_tax_amt
            == if self.hsa_distri_addnl_percent_tax_exc_ind {
                Usd::ZERO
            } else {
                Usd::from_cents(self.taxable_hsa_distribution_amt.cents() * 20 / 100)
            };

        // Part III checks
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

    fn basic_input() -> F8889Input {
        F8889Input {
            person_nm: "Jane Doe".to_string(),
            recipient_ssn: "123-45-6789".to_string(),
            hdhp_self_only_coverage_ind: true,
            hdhp_family_coverage_ind: false,
            hsa_contribution_amt: Usd::from_dollars(3_000),
            hsa_limited_annual_deductible_amt: Usd::from_dollars(4_300),
            total_archer_msa_contribution_amt: Usd::ZERO,
            hsa_limited_contribution_amt: Usd::from_dollars(4_300),
            hsa_addnl_contribution_amt: Usd::ZERO,
            hsa_employer_contribution_amt: Usd::from_dollars(1_000),
            hsa_qualified_funding_distri_amt: Usd::ZERO,
            total_hsa_distribution_amt: Usd::ZERO,
            hsa_distribution_rollover_amt: Usd::ZERO,
            unreimb_qual_med_and_dental_exp_amt: Usd::ZERO,
            hsa_distri_addnl_percent_tax_exc_ind: false,
            hdhp_coverage_fail_partial_yr_amt: Usd::ZERO,
            hdhp_coverage_fail_fund_distri_amt: Usd::ZERO,
        }
    }

    // -- must_file tests --

    #[test]
    fn must_file_with_contributions() {
        let input = basic_input();
        assert!(Output8889::must_file(&input));
    }

    #[test]
    fn must_file_no_activity() {
        let mut input = basic_input();
        input.hsa_contribution_amt = Usd::ZERO;
        input.hsa_employer_contribution_amt = Usd::ZERO;
        input.hsa_qualified_funding_distri_amt = Usd::ZERO;
        input.total_hsa_distribution_amt = Usd::ZERO;
        input.hdhp_coverage_fail_partial_yr_amt = Usd::ZERO;
        input.hdhp_coverage_fail_fund_distri_amt = Usd::ZERO;
        assert!(!Output8889::must_file(&input));
    }

    #[test]
    fn must_file_distributions_only() {
        let mut input = basic_input();
        input.hsa_contribution_amt = Usd::ZERO;
        input.hsa_employer_contribution_amt = Usd::ZERO;
        input.total_hsa_distribution_amt = Usd::from_dollars(500);
        assert!(Output8889::must_file(&input));
    }

    #[test]
    fn must_file_hdhp_failure_only() {
        let mut input = basic_input();
        input.hsa_contribution_amt = Usd::ZERO;
        input.hsa_employer_contribution_amt = Usd::ZERO;
        input.hdhp_coverage_fail_partial_yr_amt = Usd::from_dollars(200);
        assert!(Output8889::must_file(&input));
    }

    // -- Part I tests --

    #[test]
    fn basic_self_only_contribution() {
        let form = Output8889::try_new(basic_input()).unwrap();
        // Line 5: max(4,300 - 0, 0) = 4,300
        assert_eq!(
            form.hsa_limited_deductible_allwd_amt,
            Usd::from_dollars(4_300)
        );
        // Line 8: 4,300 + 0 = 4,300
        assert_eq!(
            form.hsa_limited_gross_contribution_amt,
            Usd::from_dollars(4_300)
        );
        // Line 11: 1,000 + 0 = 1,000
        assert_eq!(
            form.total_hsa_contribution_amt,
            Usd::from_dollars(1_000)
        );
        // Line 12: max(4,300 - 1,000, 0) = 3,300
        assert_eq!(
            form.hsa_family_deductible_amt,
            Usd::from_dollars(3_300)
        );
        // Line 13: min(3,000, 3,300) = 3,000
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(3_000));
        assert!(form.is_valid());
    }

    #[test]
    fn archer_msa_reduces_line5() {
        let mut input = basic_input();
        input.total_archer_msa_contribution_amt = Usd::from_dollars(2_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 5: max(4,300 - 2,000, 0) = 2,300
        assert_eq!(
            form.hsa_limited_deductible_allwd_amt,
            Usd::from_dollars(2_300)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn archer_msa_exceeds_limit_line5_zero() {
        let mut input = basic_input();
        input.total_archer_msa_contribution_amt = Usd::from_dollars(5_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 5: max(4,300 - 5,000, 0) = 0
        assert_eq!(form.hsa_limited_deductible_allwd_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn additional_contribution_55_plus() {
        let mut input = basic_input();
        input.hsa_addnl_contribution_amt = Usd::from_dollars(1_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 8: 4,300 + 1,000 = 5,300
        assert_eq!(
            form.hsa_limited_gross_contribution_amt,
            Usd::from_dollars(5_300)
        );
        // Line 12: max(5,300 - 1,000, 0) = 4,300
        assert_eq!(
            form.hsa_family_deductible_amt,
            Usd::from_dollars(4_300)
        );
        // Line 13: min(3,000, 4,300) = 3,000
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(3_000));
        assert!(form.is_valid());
    }

    #[test]
    fn employer_exceeds_limit_line12_zero() {
        let mut input = basic_input();
        input.hsa_employer_contribution_amt = Usd::from_dollars(5_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 11: 5,000 + 0 = 5,000
        // Line 12: max(4,300 - 5,000, 0) = 0
        assert_eq!(form.hsa_family_deductible_amt, Usd::ZERO);
        // Line 13: min(3,000, 0) = 0
        assert_eq!(form.total_hsa_deduction_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn qualified_funding_distribution_in_line11() {
        let mut input = basic_input();
        input.hsa_qualified_funding_distri_amt = Usd::from_dollars(500);
        let form = Output8889::try_new(input).unwrap();
        // Line 11: 1,000 + 500 = 1,500
        assert_eq!(
            form.total_hsa_contribution_amt,
            Usd::from_dollars(1_500)
        );
        // Line 12: max(4,300 - 1,500, 0) = 2,800
        assert_eq!(
            form.hsa_family_deductible_amt,
            Usd::from_dollars(2_800)
        );
        // Line 13: min(3,000, 2,800) = 2,800
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(2_800));
        assert!(form.is_valid());
    }

    #[test]
    fn line13_capped_by_contribution() {
        let mut input = basic_input();
        input.hsa_contribution_amt = Usd::from_dollars(500);
        let form = Output8889::try_new(input).unwrap();
        // Line 12: max(4,300 - 1,000, 0) = 3,300
        // Line 13: min(500, 3,300) = 500
        assert_eq!(form.total_hsa_deduction_amt, Usd::from_dollars(500));
        assert!(form.is_valid());
    }

    // -- Part II tests --

    #[test]
    fn distributions_fully_qualified() {
        let mut input = basic_input();
        input.total_hsa_distribution_amt = Usd::from_dollars(2_000);
        input.unreimb_qual_med_and_dental_exp_amt = Usd::from_dollars(2_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 14c: 2,000 - 0 = 2,000
        assert_eq!(form.hsa_net_distribution_amt, Usd::from_dollars(2_000));
        // Line 16: max(2,000 - 2,000, 0) = 0
        assert_eq!(form.taxable_hsa_distribution_amt, Usd::ZERO);
        // Line 17b: 0 × 20% = 0
        assert_eq!(form.hsa_distri_addnl_percent_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn taxable_distribution_with_penalty() {
        let mut input = basic_input();
        input.total_hsa_distribution_amt = Usd::from_dollars(5_000);
        input.hsa_distribution_rollover_amt = Usd::from_dollars(1_000);
        input.unreimb_qual_med_and_dental_exp_amt = Usd::from_dollars(2_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 14c: 5,000 - 1,000 = 4,000
        assert_eq!(form.hsa_net_distribution_amt, Usd::from_dollars(4_000));
        // Line 16: max(4,000 - 2,000, 0) = 2,000
        assert_eq!(
            form.taxable_hsa_distribution_amt,
            Usd::from_dollars(2_000)
        );
        // Line 17b: 2,000 × 20% = 400
        assert_eq!(
            form.hsa_distri_addnl_percent_tax_amt,
            Usd::from_dollars(400)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn taxable_distribution_with_exception() {
        let mut input = basic_input();
        input.total_hsa_distribution_amt = Usd::from_dollars(5_000);
        input.hsa_distribution_rollover_amt = Usd::from_dollars(1_000);
        input.unreimb_qual_med_and_dental_exp_amt = Usd::from_dollars(2_000);
        input.hsa_distri_addnl_percent_tax_exc_ind = true;
        let form = Output8889::try_new(input).unwrap();
        // Line 16: max(4,000 - 2,000, 0) = 2,000
        assert_eq!(
            form.taxable_hsa_distribution_amt,
            Usd::from_dollars(2_000)
        );
        // Line 17b: exception, so 0
        assert_eq!(form.hsa_distri_addnl_percent_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn medical_expenses_exceed_distributions() {
        let mut input = basic_input();
        input.total_hsa_distribution_amt = Usd::from_dollars(1_000);
        input.unreimb_qual_med_and_dental_exp_amt = Usd::from_dollars(3_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 14c: 1,000 - 0 = 1,000
        // Line 16: max(1,000 - 3,000, 0) = 0
        assert_eq!(form.taxable_hsa_distribution_amt, Usd::ZERO);
        assert_eq!(form.hsa_distri_addnl_percent_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn penalty_rounding() {
        let mut input = basic_input();
        input.total_hsa_distribution_amt = Usd::from_cents(1_001); // $10.01
        input.unreimb_qual_med_and_dental_exp_amt = Usd::ZERO;
        let form = Output8889::try_new(input).unwrap();
        // Line 16: 1001 cents
        // Line 17b: 1001 × 20 / 100 = 200 cents = $2.00 (truncated)
        assert_eq!(
            form.hsa_distri_addnl_percent_tax_amt,
            Usd::from_cents(200)
        );
        assert!(form.is_valid());
    }

    // -- Part III tests --

    #[test]
    fn hdhp_failure_income_and_tax() {
        let mut input = basic_input();
        input.hdhp_coverage_fail_partial_yr_amt = Usd::from_dollars(2_000);
        input.hdhp_coverage_fail_fund_distri_amt = Usd::from_dollars(1_000);
        let form = Output8889::try_new(input).unwrap();
        // Line 20: 2,000 + 1,000 = 3,000
        assert_eq!(form.hdhp_coverage_income_amt, Usd::from_dollars(3_000));
        // Line 21: 3,000 × 10% = 300
        assert_eq!(form.hdhp_coverage_addnl_tax_amt, Usd::from_dollars(300));
        assert!(form.is_valid());
    }

    #[test]
    fn hdhp_failure_zero() {
        let form = Output8889::try_new(basic_input()).unwrap();
        assert_eq!(form.hdhp_coverage_income_amt, Usd::ZERO);
        assert_eq!(form.hdhp_coverage_addnl_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn hdhp_failure_tax_rounding() {
        let mut input = basic_input();
        input.hdhp_coverage_fail_partial_yr_amt = Usd::from_cents(1_001); // $10.01
        let form = Output8889::try_new(input).unwrap();
        // Line 20: 1001 cents
        // Line 21: 1001 × 10 / 100 = 100 cents = $1.00 (truncated)
        assert_eq!(form.hdhp_coverage_addnl_tax_amt, Usd::from_cents(100));
        assert!(form.is_valid());
    }

    // -- Passthrough tests --

    #[test]
    fn passthrough_fields() {
        let form = Output8889::try_new(basic_input()).unwrap();
        assert_eq!(form.person_nm, "Jane Doe");
        assert_eq!(form.recipient_ssn, "123-45-6789");
        assert!(form.hdhp_self_only_coverage_ind);
        assert!(!form.hdhp_family_coverage_ind);
    }

    // -- Dependencies and metadata tests --

    #[test]
    fn dependencies_includes_f1040() {
        assert_eq!(Output8889::dependencies(), &[DynForm::F1040]);
    }

    #[test]
    fn form_name() {
        assert_eq!(Output8889::name(), "Form 8889");
    }

    // -- Zero activity test --

    #[test]
    fn zero_activity() {
        let mut input = basic_input();
        input.hsa_contribution_amt = Usd::ZERO;
        input.hsa_employer_contribution_amt = Usd::ZERO;
        input.hsa_limited_contribution_amt = Usd::ZERO;
        input.hsa_limited_annual_deductible_amt = Usd::ZERO;
        let form = Output8889::try_new(input).unwrap();
        assert_eq!(form.total_hsa_deduction_amt, Usd::ZERO);
        assert_eq!(form.taxable_hsa_distribution_amt, Usd::ZERO);
        assert_eq!(form.hsa_distri_addnl_percent_tax_amt, Usd::ZERO);
        assert_eq!(form.hdhp_coverage_income_amt, Usd::ZERO);
        assert_eq!(form.hdhp_coverage_addnl_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
