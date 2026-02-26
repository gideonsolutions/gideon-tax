use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8880.
///
/// This form has two columns: (a) You and (b) Your spouse. Each per-person
/// field has a `primary_` and `spouse_` variant. The spouse fields should be
/// `Usd::ZERO` when filing single / head of household.
#[derive(Debug, Clone)]
pub struct F8880Input {
    /// Line 1(a): Traditional and Roth IRA contributions, and ABLE account
    /// contributions — You
    pub primary_roth_ira_for_current_yr_amt: Usd,
    /// Line 1(b): Traditional and Roth IRA contributions, and ABLE account
    /// contributions — Your spouse
    pub spouse_roth_ira_for_current_yr_amt: Usd,
    /// Line 2(a): Elective deferrals to a 401(k) or other qualified employer
    /// plan, voluntary employee contributions, and 501(c)(18)(D) plan
    /// contributions — You
    pub primary_contributions_amt: Usd,
    /// Line 2(b): Elective deferrals — Your spouse
    pub spouse_contributions_amt: Usd,
    /// Line 4(a): Certain distributions received after 2022 and before the
    /// due date (including extensions) of your 2025 return — You
    pub prim_taxable_distributions_amt: Usd,
    /// Line 4(b): Certain distributions — Your spouse
    pub sps_taxable_distributions_amt: Usd,
    /// Line 8: Adjusted gross income from Form 1040, 1040-SR, or 1040-NR,
    /// line 11a
    pub tax_return_agi_amt: Usd,
    /// Line 9: Applicable decimal amount from the table based on filing
    /// status and AGI. Stored as Usd where cents represent the rate × 10 000
    /// (e.g. 0.50 → `Usd::from_cents(5000)`, 0.20 → `Usd::from_cents(2000)`).
    pub qlfy_retirement_sav_decimal_amt: Usd,
    /// Line 11: Limitation based on tax liability from the Credit Limit
    /// Worksheet
    pub calculated_credits_from_tax_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8880 (2025) — Credit for Qualified Retirement Savings Contributions.
#[derive(Debug, Clone, Default)]
pub struct Output8880 {
    /// Line 1: Traditional and Roth IRA contributions, and ABLE account contributions (a) You
    pub primary_roth_ira_for_current_yr_amt: Usd,
    /// Line 1: Traditional and Roth IRA contributions, and ABLE account contributions (b) Your spouse
    pub spouse_roth_ira_for_current_yr_amt: Usd,
    /// Line 2: Elective deferrals to a 401(k) or other qualified employer plan (a) You
    pub primary_contributions_amt: Usd,
    /// Line 2: Elective deferrals to a 401(k) or other qualified employer plan (b) Your spouse
    pub spouse_contributions_amt: Usd,
    /// Line 3: Add lines 1 and 2 (a) You
    pub add_prim_roth_ira_to_cy_contri_amt: Usd,
    /// Line 3: Add lines 1 and 2 (b) Your spouse
    pub add_sp_roth_ira_to_cy_contri_amt: Usd,
    /// Line 4: Certain distributions received after 2022 and before the due date (a) You
    pub prim_taxable_distributions_amt: Usd,
    /// Line 4: Certain distributions received after 2022 and before the due date (b) Your spouse
    pub sps_taxable_distributions_amt: Usd,
    /// Line 5: Subtract line 4 from line 3. If zero or less, enter -0- (a) You
    pub calculate_prim_distrib_from_tot_amt: Usd,
    /// Line 5: Subtract line 4 from line 3. If zero or less, enter -0- (b) Your spouse
    pub calculate_sps_distrib_from_tot_amt: Usd,
    /// Line 6: In each column, enter the smaller of line 5 or $2,000 (a) You
    pub prim_smaller_of_calculation_amt: Usd,
    /// Line 6: In each column, enter the smaller of line 5 or $2,000 (b) Your spouse
    pub sps_smaller_of_calculation_amt: Usd,
    /// Line 7: Add the amounts on line 6
    pub total_calculated_amt: Usd,
    /// Line 8: Enter the amount from Form 1040, 1040-SR, or 1040-NR, line 11a
    pub tax_return_agi_amt: Usd,
    /// Line 9: Enter the applicable decimal amount from the table
    pub qlfy_retirement_sav_decimal_amt: Usd,
    /// Line 10: Multiply line 7 by line 9
    pub calculated_amt_by_decimal_amt: Usd,
    /// Line 11: Limitation based on tax liability from the Credit Limit Worksheet
    pub calculated_credits_from_tax_amt: Usd,
    /// Line 12: Credit for qualified retirement savings contributions. Enter the smaller of line 10 or line 11
    pub cr_qualified_retirement_sav_amt: Usd,
}

// =========================================================================
// Constants
// =========================================================================

/// $2,000 cap per person on line 6
const LINE_6_CAP: Usd = Usd::from_dollars(2_000);

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8880 {
    fn name() -> &'static str {
        "Form 8880"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8880 {
    type Input = F8880Input;

    fn must_file(input: &Self::Input) -> bool {
        // Line 3(a) + Line 3(b)
        let line3a = input.primary_roth_ira_for_current_yr_amt + input.primary_contributions_amt;
        let line3b = input.spouse_roth_ira_for_current_yr_amt + input.spouse_contributions_amt;

        // Line 5(a) and 5(b): subtract distributions, min 0
        let line5a = (line3a - input.prim_taxable_distributions_amt).max(Usd::ZERO);
        let line5b = (line3b - input.sps_taxable_distributions_amt).max(Usd::ZERO);

        // Line 6(a) and 6(b): cap at $2,000
        let line6a = line5a.min(LINE_6_CAP);
        let line6b = line5b.min(LINE_6_CAP);

        // Line 7: total
        let line7 = line6a + line6b;

        line7 > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Validate that line 9 decimal is in the allowed set {0, 1000, 2000, 5000}
        // representing {0.0, 0.10, 0.20, 0.50}
        let valid_decimals = [0, 1000, 2000, 5000];
        if !valid_decimals.contains(&input.qlfy_retirement_sav_decimal_amt.cents()) {
            return Err(GideonTaxError::OutOfBounds(format!(
                "Line 9 decimal must be one of 0.0, 0.10, 0.20, 0.50 (encoded as 0, 1000, 2000, 5000 cents), got {}",
                input.qlfy_retirement_sav_decimal_amt.cents()
            )));
        }

        // Lines 1(a), 1(b): pass through from input
        let line1a = input.primary_roth_ira_for_current_yr_amt;
        let line1b = input.spouse_roth_ira_for_current_yr_amt;

        // Lines 2(a), 2(b): pass through from input
        let line2a = input.primary_contributions_amt;
        let line2b = input.spouse_contributions_amt;

        // Lines 3(a), 3(b): Add lines 1 and 2
        let line3a = line1a + line2a;
        let line3b = line1b + line2b;

        // Lines 4(a), 4(b): pass through from input
        let line4a = input.prim_taxable_distributions_amt;
        let line4b = input.sps_taxable_distributions_amt;

        // Lines 5(a), 5(b): Line 3 - Line 4, min 0
        let line5a = (line3a - line4a).max(Usd::ZERO);
        let line5b = (line3b - line4b).max(Usd::ZERO);

        // Lines 6(a), 6(b): smaller of line 5 or $2,000
        let line6a = line5a.min(LINE_6_CAP);
        let line6b = line5b.min(LINE_6_CAP);

        // Line 7: sum of line 6 columns
        let line7 = line6a + line6b;

        // Line 8: AGI (pass through)
        let line8 = input.tax_return_agi_amt;

        // Line 9: applicable decimal (pass through)
        let line9 = input.qlfy_retirement_sav_decimal_amt;

        // Line 10: line 7 × line 9 (decimal encoded as cents / 10_000)
        let line10 = Usd::from_cents(line7.cents() * line9.cents() / 10_000);

        // Line 11: tax liability limit (pass through)
        let line11 = input.calculated_credits_from_tax_amt;

        // Line 12: credit = min(line 10, line 11)
        let line12 = line10.min(line11);

        Ok(Output8880 {
            primary_roth_ira_for_current_yr_amt: line1a,
            spouse_roth_ira_for_current_yr_amt: line1b,
            primary_contributions_amt: line2a,
            spouse_contributions_amt: line2b,
            add_prim_roth_ira_to_cy_contri_amt: line3a,
            add_sp_roth_ira_to_cy_contri_amt: line3b,
            prim_taxable_distributions_amt: line4a,
            sps_taxable_distributions_amt: line4b,
            calculate_prim_distrib_from_tot_amt: line5a,
            calculate_sps_distrib_from_tot_amt: line5b,
            prim_smaller_of_calculation_amt: line6a,
            sps_smaller_of_calculation_amt: line6b,
            total_calculated_amt: line7,
            tax_return_agi_amt: line8,
            qlfy_retirement_sav_decimal_amt: line9,
            calculated_amt_by_decimal_amt: line10,
            calculated_credits_from_tax_amt: line11,
            cr_qualified_retirement_sav_amt: line12,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040]
    }

    fn is_valid(&self) -> bool {
        // Line 3 = Line 1 + Line 2
        let line3a_ok = self.add_prim_roth_ira_to_cy_contri_amt
            == self.primary_roth_ira_for_current_yr_amt + self.primary_contributions_amt;
        let line3b_ok = self.add_sp_roth_ira_to_cy_contri_amt
            == self.spouse_roth_ira_for_current_yr_amt + self.spouse_contributions_amt;

        // Line 5 = max(Line 3 - Line 4, 0)
        let line5a_ok = self.calculate_prim_distrib_from_tot_amt
            == (self.add_prim_roth_ira_to_cy_contri_amt - self.prim_taxable_distributions_amt)
                .max(Usd::ZERO);
        let line5b_ok = self.calculate_sps_distrib_from_tot_amt
            == (self.add_sp_roth_ira_to_cy_contri_amt - self.sps_taxable_distributions_amt)
                .max(Usd::ZERO);

        // Line 6 = min(Line 5, $2,000)
        let line6a_ok = self.prim_smaller_of_calculation_amt
            == self.calculate_prim_distrib_from_tot_amt.min(LINE_6_CAP);
        let line6b_ok = self.sps_smaller_of_calculation_amt
            == self.calculate_sps_distrib_from_tot_amt.min(LINE_6_CAP);

        // Line 7 = Line 6(a) + Line 6(b)
        let line7_ok = self.total_calculated_amt
            == self.prim_smaller_of_calculation_amt + self.sps_smaller_of_calculation_amt;

        // Line 10 = Line 7 × Line 9 / 10_000
        let line10_ok = self.calculated_amt_by_decimal_amt
            == Usd::from_cents(
                self.total_calculated_amt.cents()
                    * self.qlfy_retirement_sav_decimal_amt.cents()
                    / 10_000,
            );

        // Line 12 = min(Line 10, Line 11)
        let line12_ok = self.cr_qualified_retirement_sav_amt
            == self
                .calculated_amt_by_decimal_amt
                .min(self.calculated_credits_from_tax_amt);

        line3a_ok
            && line3b_ok
            && line5a_ok
            && line5b_ok
            && line6a_ok
            && line6b_ok
            && line7_ok
            && line10_ok
            && line12_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: basic input with both primary and spouse contributions.
    fn both_persons_input() -> F8880Input {
        F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(1_000),
            spouse_roth_ira_for_current_yr_amt: Usd::from_dollars(500),
            primary_contributions_amt: Usd::from_dollars(800),
            spouse_contributions_amt: Usd::from_dollars(1_200),
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(30_000),
            // 0.50 rate
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(5000),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        }
    }

    /// Helper: single filer (no spouse columns).
    fn single_person_input() -> F8880Input {
        F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(1_500),
            spouse_roth_ira_for_current_yr_amt: Usd::ZERO,
            primary_contributions_amt: Usd::from_dollars(1_000),
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(20_000),
            // 0.50 rate
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(5000),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        }
    }

    #[test]
    fn must_file_both_persons() {
        let input = both_persons_input();
        assert!(Output8880::must_file(&input));
    }

    #[test]
    fn must_file_single_person() {
        let input = single_person_input();
        assert!(Output8880::must_file(&input));
    }

    #[test]
    fn must_file_zero_contributions() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::ZERO,
            spouse_roth_ira_for_current_yr_amt: Usd::ZERO,
            primary_contributions_amt: Usd::ZERO,
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(30_000),
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(5000),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        };
        assert!(!Output8880::must_file(&input));
    }

    #[test]
    fn must_file_distributions_wipe_contributions() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(1_000),
            spouse_roth_ira_for_current_yr_amt: Usd::ZERO,
            primary_contributions_amt: Usd::ZERO,
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::from_dollars(2_000),
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(30_000),
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(5000),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        };
        // Line 3a = 1000, Line 5a = max(1000 - 2000, 0) = 0 → Line 6a = 0
        // Line 7 = 0 → must_file = false
        assert!(!Output8880::must_file(&input));
    }

    #[test]
    fn both_persons_basic() {
        let form = Output8880::try_new(both_persons_input()).unwrap();
        // Line 3(a): 1000 + 800 = 1800
        assert_eq!(
            form.add_prim_roth_ira_to_cy_contri_amt,
            Usd::from_dollars(1_800)
        );
        // Line 3(b): 500 + 1200 = 1700
        assert_eq!(
            form.add_sp_roth_ira_to_cy_contri_amt,
            Usd::from_dollars(1_700)
        );
        // Line 5(a): 1800 - 0 = 1800
        assert_eq!(
            form.calculate_prim_distrib_from_tot_amt,
            Usd::from_dollars(1_800)
        );
        // Line 5(b): 1700 - 0 = 1700
        assert_eq!(
            form.calculate_sps_distrib_from_tot_amt,
            Usd::from_dollars(1_700)
        );
        // Line 6(a): min(1800, 2000) = 1800
        assert_eq!(
            form.prim_smaller_of_calculation_amt,
            Usd::from_dollars(1_800)
        );
        // Line 6(b): min(1700, 2000) = 1700
        assert_eq!(
            form.sps_smaller_of_calculation_amt,
            Usd::from_dollars(1_700)
        );
        // Line 7: 1800 + 1700 = 3500
        assert_eq!(form.total_calculated_amt, Usd::from_dollars(3_500));
        // Line 10: 3500 × 0.50 = 1750
        assert_eq!(
            form.calculated_amt_by_decimal_amt,
            Usd::from_dollars(1_750)
        );
        // Line 12: min(1750, 5000) = 1750
        assert_eq!(
            form.cr_qualified_retirement_sav_amt,
            Usd::from_dollars(1_750)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn single_person_basic() {
        let form = Output8880::try_new(single_person_input()).unwrap();
        // Line 3(a): 1500 + 1000 = 2500
        assert_eq!(
            form.add_prim_roth_ira_to_cy_contri_amt,
            Usd::from_dollars(2_500)
        );
        // Line 3(b): 0
        assert_eq!(form.add_sp_roth_ira_to_cy_contri_amt, Usd::ZERO);
        // Line 5(a): 2500
        assert_eq!(
            form.calculate_prim_distrib_from_tot_amt,
            Usd::from_dollars(2_500)
        );
        // Line 6(a): min(2500, 2000) = 2000 (capped!)
        assert_eq!(
            form.prim_smaller_of_calculation_amt,
            Usd::from_dollars(2_000)
        );
        // Line 6(b): 0
        assert_eq!(form.sps_smaller_of_calculation_amt, Usd::ZERO);
        // Line 7: 2000
        assert_eq!(form.total_calculated_amt, Usd::from_dollars(2_000));
        // Line 10: 2000 × 0.50 = 1000
        assert_eq!(
            form.calculated_amt_by_decimal_amt,
            Usd::from_dollars(1_000)
        );
        // Line 12: min(1000, 5000) = 1000
        assert_eq!(
            form.cr_qualified_retirement_sav_amt,
            Usd::from_dollars(1_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn cap_at_2000_per_person() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(3_000),
            spouse_roth_ira_for_current_yr_amt: Usd::from_dollars(4_000),
            primary_contributions_amt: Usd::from_dollars(2_000),
            spouse_contributions_amt: Usd::from_dollars(1_000),
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(30_000),
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(5000),
            calculated_credits_from_tax_amt: Usd::from_dollars(10_000),
        };
        let form = Output8880::try_new(input).unwrap();
        // Line 5(a): 5000 - 0 = 5000; Line 6(a): min(5000, 2000) = 2000
        assert_eq!(
            form.prim_smaller_of_calculation_amt,
            Usd::from_dollars(2_000)
        );
        // Line 5(b): 5000 - 0 = 5000; Line 6(b): min(5000, 2000) = 2000
        assert_eq!(
            form.sps_smaller_of_calculation_amt,
            Usd::from_dollars(2_000)
        );
        // Line 7: 2000 + 2000 = 4000
        assert_eq!(form.total_calculated_amt, Usd::from_dollars(4_000));
        // Line 10: 4000 × 0.50 = 2000
        assert_eq!(
            form.calculated_amt_by_decimal_amt,
            Usd::from_dollars(2_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn zero_contributions() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::ZERO,
            spouse_roth_ira_for_current_yr_amt: Usd::ZERO,
            primary_contributions_amt: Usd::ZERO,
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(30_000),
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(5000),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        };
        let form = Output8880::try_new(input).unwrap();
        assert_eq!(form.total_calculated_amt, Usd::ZERO);
        assert_eq!(form.calculated_amt_by_decimal_amt, Usd::ZERO);
        assert_eq!(form.cr_qualified_retirement_sav_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn distributions_reduce_contributions() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(1_000),
            spouse_roth_ira_for_current_yr_amt: Usd::from_dollars(2_000),
            primary_contributions_amt: Usd::from_dollars(500),
            spouse_contributions_amt: Usd::from_dollars(500),
            prim_taxable_distributions_amt: Usd::from_dollars(800),
            sps_taxable_distributions_amt: Usd::from_dollars(3_000),
            tax_return_agi_amt: Usd::from_dollars(25_000),
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(5000),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        };
        let form = Output8880::try_new(input).unwrap();
        // Line 3(a): 1000 + 500 = 1500
        // Line 5(a): max(1500 - 800, 0) = 700
        assert_eq!(
            form.calculate_prim_distrib_from_tot_amt,
            Usd::from_dollars(700)
        );
        // Line 3(b): 2000 + 500 = 2500
        // Line 5(b): max(2500 - 3000, 0) = 0
        assert_eq!(form.calculate_sps_distrib_from_tot_amt, Usd::ZERO);
        // Line 6(a): min(700, 2000) = 700
        assert_eq!(
            form.prim_smaller_of_calculation_amt,
            Usd::from_dollars(700)
        );
        // Line 6(b): 0
        assert_eq!(form.sps_smaller_of_calculation_amt, Usd::ZERO);
        // Line 7: 700
        assert_eq!(form.total_calculated_amt, Usd::from_dollars(700));
        // Line 10: 700 × 0.50 = 350
        assert_eq!(form.calculated_amt_by_decimal_amt, Usd::from_dollars(350));
        // Line 12: min(350, 5000) = 350
        assert_eq!(
            form.cr_qualified_retirement_sav_amt,
            Usd::from_dollars(350)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn tax_liability_limits_credit() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(2_000),
            spouse_roth_ira_for_current_yr_amt: Usd::from_dollars(2_000),
            primary_contributions_amt: Usd::ZERO,
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(30_000),
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(5000),
            // Tax liability limit is only $500
            calculated_credits_from_tax_amt: Usd::from_dollars(500),
        };
        let form = Output8880::try_new(input).unwrap();
        // Line 7: 2000 + 2000 = 4000
        // Line 10: 4000 × 0.50 = 2000
        assert_eq!(
            form.calculated_amt_by_decimal_amt,
            Usd::from_dollars(2_000)
        );
        // Line 12: min(2000, 500) = 500
        assert_eq!(
            form.cr_qualified_retirement_sav_amt,
            Usd::from_dollars(500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn decimal_rate_020() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(2_000),
            spouse_roth_ira_for_current_yr_amt: Usd::ZERO,
            primary_contributions_amt: Usd::ZERO,
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(35_000),
            // 0.20 rate
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(2000),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        };
        let form = Output8880::try_new(input).unwrap();
        // Line 7: 2000
        // Line 10: 2000 × 0.20 = 400
        assert_eq!(form.calculated_amt_by_decimal_amt, Usd::from_dollars(400));
        // Line 12: min(400, 5000) = 400
        assert_eq!(
            form.cr_qualified_retirement_sav_amt,
            Usd::from_dollars(400)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn decimal_rate_010() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(2_000),
            spouse_roth_ira_for_current_yr_amt: Usd::ZERO,
            primary_contributions_amt: Usd::ZERO,
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(40_000),
            // 0.10 rate
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(1000),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        };
        let form = Output8880::try_new(input).unwrap();
        // Line 7: 2000
        // Line 10: 2000 × 0.10 = 200
        assert_eq!(form.calculated_amt_by_decimal_amt, Usd::from_dollars(200));
        assert!(form.is_valid());
    }

    #[test]
    fn decimal_rate_zero_no_credit() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(2_000),
            spouse_roth_ira_for_current_yr_amt: Usd::ZERO,
            primary_contributions_amt: Usd::ZERO,
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(80_000),
            // 0.0 rate (AGI too high)
            qlfy_retirement_sav_decimal_amt: Usd::ZERO,
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        };
        let form = Output8880::try_new(input).unwrap();
        // Line 10: 2000 × 0.0 = 0
        assert_eq!(form.calculated_amt_by_decimal_amt, Usd::ZERO);
        // Line 12: min(0, 5000) = 0
        assert_eq!(form.cr_qualified_retirement_sav_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn invalid_decimal_rate_rejected() {
        let input = F8880Input {
            primary_roth_ira_for_current_yr_amt: Usd::from_dollars(2_000),
            spouse_roth_ira_for_current_yr_amt: Usd::ZERO,
            primary_contributions_amt: Usd::ZERO,
            spouse_contributions_amt: Usd::ZERO,
            prim_taxable_distributions_amt: Usd::ZERO,
            sps_taxable_distributions_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(30_000),
            // Invalid: 0.75 is not a valid rate
            qlfy_retirement_sav_decimal_amt: Usd::from_cents(7500),
            calculated_credits_from_tax_amt: Usd::from_dollars(5_000),
        };
        assert!(Output8880::try_new(input).is_err());
    }

    #[test]
    fn form_metadata() {
        assert_eq!(Output8880::name(), "Form 8880");
        let form = Output8880::try_new(single_person_input()).unwrap();
        assert_eq!(form.year(), TaxYear::Y2025);
        assert_eq!(<Output8880 as Form>::form_type(), FormType::Output);
    }

    #[test]
    fn dependencies_include_f1040() {
        assert_eq!(Output8880::dependencies(), &[DynForm::F1040]);
    }
}
