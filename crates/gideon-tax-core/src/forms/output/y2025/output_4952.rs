use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 4952.
///
/// Schedule B feeds into this form; the corresponding dependency is declared
/// in [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct F4952Input {
    /// Line 1: Investment interest expense paid or accrued in 2025
    pub investment_interest_expense_amt: Usd,
    /// Line 2: Disallowed investment interest expense from 2024 Form 4952, line 7
    pub prior_yr_disallow_invsmt_int_exp_amt: Usd,
    /// Line 4a: Gross income from property held for investment
    /// (excluding any net gain from the disposition of property held for investment)
    pub investment_prop_gross_income_amt: Usd,
    /// Line 4b: Qualified dividends included on line 4a
    pub investment_prop_qual_dividends_amt: Usd,
    /// Line 4d: Net gain from the disposition of property held for investment
    pub investment_prop_net_disp_gain_amt: Usd,
    /// Line 4e: Enter the smaller of line 4d or your net capital gain
    /// from the disposition of property held for investment
    pub investment_net_gain_less_small_amt: Usd,
    /// Line 4g: Enter the amount from lines 4b and 4e that you elect
    /// to include in investment income (see instructions)
    pub investment_prop_gain_elected_amt: Usd,
    /// Line 5: Investment expenses (see instructions)
    pub investment_expense_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 4952 (2025) — Investment Interest Expense Deduction.
#[derive(Debug, Clone, Default)]
pub struct Output4952 {
    // -----------------------------------------------------------------------
    // Part I — Total Investment Interest Expense
    // -----------------------------------------------------------------------
    /// Line 1: Investment interest expense paid or accrued in 2025 (see instructions)
    pub investment_interest_expense_amt: Usd,
    /// Line 2: Disallowed investment interest expense from 2024 Form 4952, line 7
    pub prior_yr_disallow_invsmt_int_exp_amt: Usd,
    /// Line 3: Total investment interest expense. Add lines 1 and 2
    pub total_investment_interest_exp_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Net Investment Income
    // -----------------------------------------------------------------------
    /// Line 4a: Gross income from property held for investment (excluding any net gain from the disposition of property held for investment)
    pub investment_prop_gross_income_amt: Usd,
    /// Line 4b: Qualified dividends included on line 4a
    pub investment_prop_qual_dividends_amt: Usd,
    /// Line 4c: Subtract line 4b from line 4a
    pub investment_prop_net_gross_inc_amt: Usd,
    /// Line 4d: Net gain from the disposition of property held for investment
    pub investment_prop_net_disp_gain_amt: Usd,
    /// Line 4e: Enter the smaller of line 4d or your net capital gain from the disposition of property held for investment
    pub investment_net_gain_less_small_amt: Usd,
    /// Line 4f: Subtract line 4e from line 4d
    pub property_dspstn_cap_gain_inv_inc_amt: Usd,
    /// Line 4g: Enter the amount from lines 4b and 4e that you elect to include in investment income (see instructions)
    pub investment_prop_gain_elected_amt: Usd,
    /// Line 4g: Election code
    pub investment_prop_gain_elected_cd: String,
    /// Line 4h: Investment income. Add lines 4c, 4f, and 4g
    pub investment_income_amt: Usd,
    /// Line 5: Investment expenses (see instructions)
    pub investment_expense_amt: Usd,
    /// Line 6: Net investment income. Subtract line 5 from line 4h. If zero or less, enter -0-
    pub net_investment_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Investment Interest Expense Deduction
    // -----------------------------------------------------------------------
    /// Line 7: Disallowed investment interest expense to be carried forward to 2026. Subtract line 6 from line 3. If zero or less, enter -0-
    pub disallowed_carry_forward_exp_amt: Usd,
    /// Line 8: Investment interest expense deduction. Enter the smaller of line 3 or line 6
    pub investment_interest_exp_deduct_amt: Usd,
    /// Investment income election amount (amount elected to treat as investment income)
    pub investment_income_election_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output4952 {
    fn name() -> &'static str {
        "Form 4952"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output4952 {
    type Input = F4952Input;

    fn must_file(input: &Self::Input) -> bool {
        input.investment_interest_expense_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Validate: line 4b <= line 4a
        if input.investment_prop_qual_dividends_amt > input.investment_prop_gross_income_amt {
            return Err(GideonTaxError::OutOfBounds(format!(
                "investment_prop_qual_dividends_amt ({}) exceeds \
                 investment_prop_gross_income_amt ({})",
                input.investment_prop_qual_dividends_amt, input.investment_prop_gross_income_amt
            )));
        }

        // Validate: line 4e <= line 4d
        if input.investment_net_gain_less_small_amt > input.investment_prop_net_disp_gain_amt {
            return Err(GideonTaxError::OutOfBounds(format!(
                "investment_net_gain_less_small_amt ({}) exceeds \
                 investment_prop_net_disp_gain_amt ({})",
                input.investment_net_gain_less_small_amt, input.investment_prop_net_disp_gain_amt
            )));
        }

        // Validate: line 4g <= line 4b + line 4e
        let max_election =
            input.investment_prop_qual_dividends_amt + input.investment_net_gain_less_small_amt;
        if input.investment_prop_gain_elected_amt > max_election {
            return Err(GideonTaxError::OutOfBounds(format!(
                "investment_prop_gain_elected_amt ({}) exceeds \
                 sum of lines 4b and 4e ({})",
                input.investment_prop_gain_elected_amt, max_election
            )));
        }

        // Part I
        // Line 3: Line 1 + Line 2
        let line3 =
            input.investment_interest_expense_amt + input.prior_yr_disallow_invsmt_int_exp_amt;

        // Part II
        // Line 4c: Line 4a - Line 4b
        let line4c =
            input.investment_prop_gross_income_amt - input.investment_prop_qual_dividends_amt;

        // Line 4f: Line 4d - Line 4e
        let line4f =
            input.investment_prop_net_disp_gain_amt - input.investment_net_gain_less_small_amt;

        // Line 4g (passthrough)
        let line4g = input.investment_prop_gain_elected_amt;

        let election_cd = if line4g > Usd::ZERO {
            "SEC. 163(d)(4)(B)".to_string()
        } else {
            String::new()
        };

        // Line 4h: Line 4c + Line 4f + Line 4g
        let line4h = line4c + line4f + line4g;

        // Line 6: max(Line 4h - Line 5, 0)
        let line6 = (line4h - input.investment_expense_amt).max(Usd::ZERO);

        // Part III
        // Line 7: max(Line 3 - Line 6, 0)
        let line7 = (line3 - line6).max(Usd::ZERO);

        // Line 8: min(Line 3, Line 6)
        let line8 = line3.min(line6);

        Ok(Output4952 {
            investment_interest_expense_amt: input.investment_interest_expense_amt,
            prior_yr_disallow_invsmt_int_exp_amt: input.prior_yr_disallow_invsmt_int_exp_amt,
            total_investment_interest_exp_amt: line3,
            investment_prop_gross_income_amt: input.investment_prop_gross_income_amt,
            investment_prop_qual_dividends_amt: input.investment_prop_qual_dividends_amt,
            investment_prop_net_gross_inc_amt: line4c,
            investment_prop_net_disp_gain_amt: input.investment_prop_net_disp_gain_amt,
            investment_net_gain_less_small_amt: input.investment_net_gain_less_small_amt,
            property_dspstn_cap_gain_inv_inc_amt: line4f,
            investment_prop_gain_elected_amt: line4g,
            investment_prop_gain_elected_cd: election_cd,
            investment_income_amt: line4h,
            investment_expense_amt: input.investment_expense_amt,
            net_investment_income_amt: line6,
            disallowed_carry_forward_exp_amt: line7,
            investment_interest_exp_deduct_amt: line8,
            investment_income_election_amt: line4g,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::ScheduleB]
    }

    fn is_valid(&self) -> bool {
        // Line 3 = Line 1 + Line 2
        let line3_ok = self.total_investment_interest_exp_amt
            == self.investment_interest_expense_amt + self.prior_yr_disallow_invsmt_int_exp_amt;

        // Line 4c = Line 4a - Line 4b
        let line4c_ok = self.investment_prop_net_gross_inc_amt
            == self.investment_prop_gross_income_amt - self.investment_prop_qual_dividends_amt;

        // Line 4f = Line 4d - Line 4e
        let line4f_ok = self.property_dspstn_cap_gain_inv_inc_amt
            == self.investment_prop_net_disp_gain_amt - self.investment_net_gain_less_small_amt;

        // Line 4h = Line 4c + Line 4f + Line 4g
        let line4h_ok = self.investment_income_amt
            == self.investment_prop_net_gross_inc_amt
                + self.property_dspstn_cap_gain_inv_inc_amt
                + self.investment_prop_gain_elected_amt;

        // Line 6 = max(Line 4h - Line 5, 0)
        let line6_ok = self.net_investment_income_amt
            == (self.investment_income_amt - self.investment_expense_amt).max(Usd::ZERO);

        // Line 7 = max(Line 3 - Line 6, 0)
        let line7_ok = self.disallowed_carry_forward_exp_amt
            == (self.total_investment_interest_exp_amt - self.net_investment_income_amt)
                .max(Usd::ZERO);

        // Line 8 = min(Line 3, Line 6)
        let line8_ok = self.investment_interest_exp_deduct_amt
            == self
                .total_investment_interest_exp_amt
                .min(self.net_investment_income_amt);

        // investment_income_election_amt = line 4g
        let election_ok =
            self.investment_income_election_amt == self.investment_prop_gain_elected_amt;

        line3_ok
            && line4c_ok
            && line4f_ok
            && line4h_ok
            && line6_ok
            && line7_ok
            && line8_ok
            && election_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F4952Input {
        F4952Input {
            investment_interest_expense_amt: Usd::from_dollars(5_000),
            prior_yr_disallow_invsmt_int_exp_amt: Usd::from_dollars(1_000),
            investment_prop_gross_income_amt: Usd::from_dollars(10_000),
            investment_prop_qual_dividends_amt: Usd::from_dollars(2_000),
            investment_prop_net_disp_gain_amt: Usd::from_dollars(3_000),
            investment_net_gain_less_small_amt: Usd::from_dollars(1_000),
            investment_prop_gain_elected_amt: Usd::from_dollars(500),
            investment_expense_amt: Usd::from_dollars(1_500),
        }
    }

    #[test]
    fn must_file_with_interest_expense() {
        let input = basic_input();
        assert!(Output4952::must_file(&input));
    }

    #[test]
    fn must_file_no_interest_expense() {
        let mut input = basic_input();
        input.investment_interest_expense_amt = Usd::ZERO;
        assert!(!Output4952::must_file(&input));
    }

    #[test]
    fn basic_computation() {
        let form = Output4952::try_new(basic_input()).unwrap();

        // Line 3: 5,000 + 1,000 = 6,000
        assert_eq!(
            form.total_investment_interest_exp_amt,
            Usd::from_dollars(6_000)
        );
        // Line 4c: 10,000 - 2,000 = 8,000
        assert_eq!(
            form.investment_prop_net_gross_inc_amt,
            Usd::from_dollars(8_000)
        );
        // Line 4f: 3,000 - 1,000 = 2,000
        assert_eq!(
            form.property_dspstn_cap_gain_inv_inc_amt,
            Usd::from_dollars(2_000)
        );
        // Line 4g: 500
        assert_eq!(
            form.investment_prop_gain_elected_amt,
            Usd::from_dollars(500)
        );
        // Line 4h: 8,000 + 2,000 + 500 = 10,500
        assert_eq!(form.investment_income_amt, Usd::from_dollars(10_500));
        // Line 6: max(10,500 - 1,500, 0) = 9,000
        assert_eq!(
            form.net_investment_income_amt,
            Usd::from_dollars(9_000)
        );
        // Line 7: max(6,000 - 9,000, 0) = 0
        assert_eq!(form.disallowed_carry_forward_exp_amt, Usd::ZERO);
        // Line 8: min(6,000, 9,000) = 6,000
        assert_eq!(
            form.investment_interest_exp_deduct_amt,
            Usd::from_dollars(6_000)
        );
        // election_amt = line 4g = 500
        assert_eq!(
            form.investment_income_election_amt,
            Usd::from_dollars(500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn disallowed_carryforward() {
        // When total interest expense exceeds net investment income
        let input = F4952Input {
            investment_interest_expense_amt: Usd::from_dollars(10_000),
            prior_yr_disallow_invsmt_int_exp_amt: Usd::from_dollars(2_000),
            investment_prop_gross_income_amt: Usd::from_dollars(5_000),
            investment_prop_qual_dividends_amt: Usd::ZERO,
            investment_prop_net_disp_gain_amt: Usd::ZERO,
            investment_net_gain_less_small_amt: Usd::ZERO,
            investment_prop_gain_elected_amt: Usd::ZERO,
            investment_expense_amt: Usd::from_dollars(1_000),
        };
        let form = Output4952::try_new(input).unwrap();

        // Line 3: 10,000 + 2,000 = 12,000
        assert_eq!(
            form.total_investment_interest_exp_amt,
            Usd::from_dollars(12_000)
        );
        // Line 4h: 5,000 + 0 + 0 = 5,000
        assert_eq!(form.investment_income_amt, Usd::from_dollars(5_000));
        // Line 6: max(5,000 - 1,000, 0) = 4,000
        assert_eq!(
            form.net_investment_income_amt,
            Usd::from_dollars(4_000)
        );
        // Line 7: max(12,000 - 4,000, 0) = 8,000
        assert_eq!(
            form.disallowed_carry_forward_exp_amt,
            Usd::from_dollars(8_000)
        );
        // Line 8: min(12,000, 4,000) = 4,000
        assert_eq!(
            form.investment_interest_exp_deduct_amt,
            Usd::from_dollars(4_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn zero_investment_income() {
        let input = F4952Input {
            investment_interest_expense_amt: Usd::from_dollars(5_000),
            prior_yr_disallow_invsmt_int_exp_amt: Usd::ZERO,
            investment_prop_gross_income_amt: Usd::ZERO,
            investment_prop_qual_dividends_amt: Usd::ZERO,
            investment_prop_net_disp_gain_amt: Usd::ZERO,
            investment_net_gain_less_small_amt: Usd::ZERO,
            investment_prop_gain_elected_amt: Usd::ZERO,
            investment_expense_amt: Usd::ZERO,
        };
        let form = Output4952::try_new(input).unwrap();

        // Line 3: 5,000
        assert_eq!(
            form.total_investment_interest_exp_amt,
            Usd::from_dollars(5_000)
        );
        // Line 6: 0
        assert_eq!(form.net_investment_income_amt, Usd::ZERO);
        // Line 7: max(5,000 - 0, 0) = 5,000 (all carried forward)
        assert_eq!(
            form.disallowed_carry_forward_exp_amt,
            Usd::from_dollars(5_000)
        );
        // Line 8: min(5,000, 0) = 0 (no deduction)
        assert_eq!(form.investment_interest_exp_deduct_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn expenses_exceed_investment_income() {
        let input = F4952Input {
            investment_interest_expense_amt: Usd::from_dollars(3_000),
            prior_yr_disallow_invsmt_int_exp_amt: Usd::ZERO,
            investment_prop_gross_income_amt: Usd::from_dollars(2_000),
            investment_prop_qual_dividends_amt: Usd::ZERO,
            investment_prop_net_disp_gain_amt: Usd::ZERO,
            investment_net_gain_less_small_amt: Usd::ZERO,
            investment_prop_gain_elected_amt: Usd::ZERO,
            investment_expense_amt: Usd::from_dollars(5_000),
        };
        let form = Output4952::try_new(input).unwrap();

        // Line 4h: 2,000
        assert_eq!(form.investment_income_amt, Usd::from_dollars(2_000));
        // Line 6: max(2,000 - 5,000, 0) = 0
        assert_eq!(form.net_investment_income_amt, Usd::ZERO);
        // Line 7: max(3,000 - 0, 0) = 3,000
        assert_eq!(
            form.disallowed_carry_forward_exp_amt,
            Usd::from_dollars(3_000)
        );
        // Line 8: min(3,000, 0) = 0
        assert_eq!(form.investment_interest_exp_deduct_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn prior_year_disallowed_only() {
        let input = F4952Input {
            investment_interest_expense_amt: Usd::ZERO,
            prior_yr_disallow_invsmt_int_exp_amt: Usd::from_dollars(3_000),
            investment_prop_gross_income_amt: Usd::from_dollars(10_000),
            investment_prop_qual_dividends_amt: Usd::ZERO,
            investment_prop_net_disp_gain_amt: Usd::ZERO,
            investment_net_gain_less_small_amt: Usd::ZERO,
            investment_prop_gain_elected_amt: Usd::ZERO,
            investment_expense_amt: Usd::ZERO,
        };
        let form = Output4952::try_new(input).unwrap();

        // Line 3: 0 + 3,000 = 3,000
        assert_eq!(
            form.total_investment_interest_exp_amt,
            Usd::from_dollars(3_000)
        );
        // Line 6: 10,000
        assert_eq!(
            form.net_investment_income_amt,
            Usd::from_dollars(10_000)
        );
        // Line 7: max(3,000 - 10,000, 0) = 0
        assert_eq!(form.disallowed_carry_forward_exp_amt, Usd::ZERO);
        // Line 8: min(3,000, 10,000) = 3,000
        assert_eq!(
            form.investment_interest_exp_deduct_amt,
            Usd::from_dollars(3_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn election_code_set_when_electing() {
        let form = Output4952::try_new(basic_input()).unwrap();
        assert_eq!(form.investment_prop_gain_elected_cd, "SEC. 163(d)(4)(B)");
    }

    #[test]
    fn election_code_empty_when_no_election() {
        let mut input = basic_input();
        input.investment_prop_gain_elected_amt = Usd::ZERO;
        let form = Output4952::try_new(input).unwrap();
        assert_eq!(form.investment_prop_gain_elected_cd, "");
        assert!(form.is_valid());
    }

    #[test]
    fn qual_dividends_exceed_gross_income_returns_error() {
        let mut input = basic_input();
        input.investment_prop_qual_dividends_amt = Usd::from_dollars(15_000);
        let err = Output4952::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn line_4e_exceeds_line_4d_returns_error() {
        let mut input = basic_input();
        input.investment_net_gain_less_small_amt = Usd::from_dollars(5_000);
        let err = Output4952::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn elected_amt_exceeds_4b_plus_4e_returns_error() {
        let mut input = basic_input();
        // 4b = 2,000, 4e = 1,000, so max election = 3,000
        input.investment_prop_gain_elected_amt = Usd::from_dollars(4_000);
        let err = Output4952::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn all_zeros() {
        let input = F4952Input {
            investment_interest_expense_amt: Usd::ZERO,
            prior_yr_disallow_invsmt_int_exp_amt: Usd::ZERO,
            investment_prop_gross_income_amt: Usd::ZERO,
            investment_prop_qual_dividends_amt: Usd::ZERO,
            investment_prop_net_disp_gain_amt: Usd::ZERO,
            investment_net_gain_less_small_amt: Usd::ZERO,
            investment_prop_gain_elected_amt: Usd::ZERO,
            investment_expense_amt: Usd::ZERO,
        };
        let form = Output4952::try_new(input).unwrap();
        assert_eq!(form.total_investment_interest_exp_amt, Usd::ZERO);
        assert_eq!(form.investment_income_amt, Usd::ZERO);
        assert_eq!(form.net_investment_income_amt, Usd::ZERO);
        assert_eq!(form.disallowed_carry_forward_exp_amt, Usd::ZERO);
        assert_eq!(form.investment_interest_exp_deduct_amt, Usd::ZERO);
        assert_eq!(form.investment_income_election_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn cents_precision() {
        let input = F4952Input {
            investment_interest_expense_amt: Usd::from_cents(550_00),
            prior_yr_disallow_invsmt_int_exp_amt: Usd::from_cents(125_50),
            investment_prop_gross_income_amt: Usd::from_cents(1_000_00),
            investment_prop_qual_dividends_amt: Usd::from_cents(200_50),
            investment_prop_net_disp_gain_amt: Usd::from_cents(300_75),
            investment_net_gain_less_small_amt: Usd::from_cents(100_25),
            investment_prop_gain_elected_amt: Usd::from_cents(50_00),
            investment_expense_amt: Usd::from_cents(150_00),
        };
        let form = Output4952::try_new(input).unwrap();

        // Line 3: 5,500.00 + 1,255.00 = 6,755.00 (wait, 550_00 = $550.00)
        // Let me recalculate: 550.00 + 125.50 = 675.50
        assert_eq!(form.total_investment_interest_exp_amt, Usd::from_cents(675_50));
        // Line 4c: 1,000.00 - 200.50 = 799.50
        assert_eq!(form.investment_prop_net_gross_inc_amt, Usd::from_cents(799_50));
        // Line 4f: 300.75 - 100.25 = 200.50
        assert_eq!(form.property_dspstn_cap_gain_inv_inc_amt, Usd::from_cents(200_50));
        // Line 4h: 799.50 + 200.50 + 50.00 = 1,050.00
        assert_eq!(form.investment_income_amt, Usd::from_cents(1_050_00));
        // Line 6: max(1,050.00 - 150.00, 0) = 900.00
        assert_eq!(form.net_investment_income_amt, Usd::from_cents(900_00));
        // Line 7: max(675.50 - 900.00, 0) = 0
        assert_eq!(form.disallowed_carry_forward_exp_amt, Usd::ZERO);
        // Line 8: min(675.50, 900.00) = 675.50
        assert_eq!(form.investment_interest_exp_deduct_amt, Usd::from_cents(675_50));
        assert!(form.is_valid());
    }

    #[test]
    fn full_deduction_when_income_exceeds_expense() {
        let input = F4952Input {
            investment_interest_expense_amt: Usd::from_dollars(2_000),
            prior_yr_disallow_invsmt_int_exp_amt: Usd::ZERO,
            investment_prop_gross_income_amt: Usd::from_dollars(20_000),
            investment_prop_qual_dividends_amt: Usd::ZERO,
            investment_prop_net_disp_gain_amt: Usd::ZERO,
            investment_net_gain_less_small_amt: Usd::ZERO,
            investment_prop_gain_elected_amt: Usd::ZERO,
            investment_expense_amt: Usd::ZERO,
        };
        let form = Output4952::try_new(input).unwrap();

        // Full deduction: line 8 should equal line 3
        assert_eq!(
            form.investment_interest_exp_deduct_amt,
            Usd::from_dollars(2_000)
        );
        assert_eq!(form.disallowed_carry_forward_exp_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
