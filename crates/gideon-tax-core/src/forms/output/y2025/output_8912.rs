use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8912.
///
/// Bond credits from Form 1097-BTC, credits not reported on 1097-BTC,
/// prior-year carryforwards, and the various credits that reduce the
/// allowable amount all feed into the computation.
#[derive(Debug, Clone)]
pub struct F8912Input {
    /// Line 1: Bond credit(s) reported on Form(s) 1097-BTC
    pub total_all_form1097_btc_amt: Usd,
    /// Line 2: Bond credit(s) not reported on Form(s) 1097-BTC
    pub total_other_not_rpt_f1097_btc_amt: Usd,
    /// Line 3: Carryforward of credits from prior year
    pub carryforward_py_bond_credit_amt: Usd,
    /// Line 5: Amount allocated to beneficiaries of the estate or trust
    pub estate_or_trust_allocated_benef_amt: Usd,
    /// Line 7: Regular tax before credits
    pub regular_tax_before_credit_amt: Usd,
    /// Line 8: Alternative minimum tax
    pub alternative_minimum_tax_amt: Usd,
    /// Line 10a: Foreign tax credit
    pub foreign_tax_credit_amt: Usd,
    /// Line 10b: Certain allowable credits
    pub certain_allowable_credits_amt: Usd,
    /// Line 10c: General business credit
    pub general_business_credit_amt: Usd,
    /// Line 10d: Credit for prior year minimum tax (Form 8801 or Form 8827)
    pub credit_prior_year_minimum_tax_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8912 (2025) — Credit to Holders of Tax Credit Bonds.
#[derive(Debug, Clone, Default)]
pub struct Output8912 {
    // -----------------------------------------------------------------------
    // Part I — Current Year Credit
    // -----------------------------------------------------------------------
    /// Line 1: Bond credit(s) from Part III (amount from line 14)
    pub total_all_form1097_btc_amt: Usd,
    /// Line 2: Bond credit(s) from Part IV (amount from line 20)
    pub total_other_not_rpt_f1097_btc_amt: Usd,
    /// Line 3: Carryforward of credits for qualified tax credit bonds and build America bonds to 2021
    pub carryforward_py_bond_credit_amt: Usd,
    /// Line 4: Total credit (add lines 1 through 3)
    pub total_credit_amt: Usd,
    /// Line 5: Amount allocated to beneficiaries of the estate or trust
    pub estate_or_trust_allocated_benef_amt: Usd,
    /// Line 6: Estates and trusts (subtract line 5 from line 4)
    pub est_tr_cy_bond_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Allowable Credit
    // -----------------------------------------------------------------------
    /// Line 7: Regular tax before credits
    pub regular_tax_before_credit_amt: Usd,
    /// Line 8: Alternative minimum tax
    pub alternative_minimum_tax_amt: Usd,
    /// Line 9: Add line 7 and line 8
    pub sum_regular_tax_and_alt_min_tx_amt: Usd,
    /// Line 10a: Foreign tax credit
    pub foreign_tax_credit_amt: Usd,
    /// Line 10b: Certain allowable credits
    pub certain_allowable_credits_amt: Usd,
    /// Line 10c: General business credit
    pub general_business_credit_amt: Usd,
    /// Line 10d: Credit for prior year minimum tax (Form 8801 or Form 8827)
    pub credit_prior_year_minimum_tax_amt: Usd,
    /// Line 10e: Add lines 10a through 10d
    pub total_credits_amt: Usd,
    /// Line 11: Net income tax (subtract line 10e from line 9)
    pub net_income_tax_amt: Usd,
    /// Line 12: Credit to holders of tax credit bonds allowed for the current year
    pub current_year_allowable_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Bond Credit(s) Reported to You on Form(s) 1097-BTC
    // -----------------------------------------------------------------------
    /// Line 14: Total of amounts reported on Form(s) 1097-BTC (enter here and on line 1)
    pub new_clean_energy_bond_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8912 {
    fn name() -> &'static str {
        "Form 8912"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8912 {
    type Input = F8912Input;

    fn must_file(input: &Self::Input) -> bool {
        input.total_all_form1097_btc_amt > Usd::ZERO
            || input.total_other_not_rpt_f1097_btc_amt > Usd::ZERO
            || input.carryforward_py_bond_credit_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Part I — Current Year Credit
        // Line 1: passthrough from input (also line 14)
        let line1 = input.total_all_form1097_btc_amt;

        // Line 2: passthrough from input
        let line2 = input.total_other_not_rpt_f1097_btc_amt;

        // Line 3: passthrough from input
        let line3 = input.carryforward_py_bond_credit_amt;

        // Line 4: Line 1 + Line 2 + Line 3
        let line4 = line1 + line2 + line3;

        // Line 5: passthrough from input
        let line5 = input.estate_or_trust_allocated_benef_amt;

        // Line 6: Line 4 - Line 5
        let line6 = (line4 - line5).max(Usd::ZERO);

        // Part II — Allowable Credit
        // Line 7: passthrough from input
        let line7 = input.regular_tax_before_credit_amt;

        // Line 8: passthrough from input
        let line8 = input.alternative_minimum_tax_amt;

        // Line 9: Line 7 + Line 8
        let line9 = line7 + line8;

        // Lines 10a-10d: passthroughs from input
        let line10a = input.foreign_tax_credit_amt;
        let line10b = input.certain_allowable_credits_amt;
        let line10c = input.general_business_credit_amt;
        let line10d = input.credit_prior_year_minimum_tax_amt;

        // Line 10e: Sum of 10a through 10d
        let line10e = line10a + line10b + line10c + line10d;

        // Line 11: Line 9 - Line 10e (min 0)
        let line11 = (line9 - line10e).max(Usd::ZERO);

        // Line 12: min(Line 6, Line 11)
        let line12 = line6.min(line11);

        // Part III — Line 14: same as Line 1
        let line14 = line1;

        Ok(Output8912 {
            total_all_form1097_btc_amt: line1,
            total_other_not_rpt_f1097_btc_amt: line2,
            carryforward_py_bond_credit_amt: line3,
            total_credit_amt: line4,
            estate_or_trust_allocated_benef_amt: line5,
            est_tr_cy_bond_cr_amt: line6,
            regular_tax_before_credit_amt: line7,
            alternative_minimum_tax_amt: line8,
            sum_regular_tax_and_alt_min_tx_amt: line9,
            foreign_tax_credit_amt: line10a,
            certain_allowable_credits_amt: line10b,
            general_business_credit_amt: line10c,
            credit_prior_year_minimum_tax_amt: line10d,
            total_credits_amt: line10e,
            net_income_tax_amt: line11,
            current_year_allowable_credit_amt: line12,
            new_clean_energy_bond_amt: line14,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040, DynForm::F1116, DynForm::F3800]
    }

    fn is_valid(&self) -> bool {
        // Line 4 = Line 1 + Line 2 + Line 3
        let line4_ok = self.total_credit_amt
            == self.total_all_form1097_btc_amt
                + self.total_other_not_rpt_f1097_btc_amt
                + self.carryforward_py_bond_credit_amt;

        // Line 6 = max(Line 4 - Line 5, 0)
        let line6_ok = self.est_tr_cy_bond_cr_amt
            == (self.total_credit_amt - self.estate_or_trust_allocated_benef_amt).max(Usd::ZERO);

        // Line 9 = Line 7 + Line 8
        let line9_ok = self.sum_regular_tax_and_alt_min_tx_amt
            == self.regular_tax_before_credit_amt + self.alternative_minimum_tax_amt;

        // Line 10e = 10a + 10b + 10c + 10d
        let line10e_ok = self.total_credits_amt
            == self.foreign_tax_credit_amt
                + self.certain_allowable_credits_amt
                + self.general_business_credit_amt
                + self.credit_prior_year_minimum_tax_amt;

        // Line 11 = max(Line 9 - Line 10e, 0)
        let line11_ok = self.net_income_tax_amt
            == (self.sum_regular_tax_and_alt_min_tx_amt - self.total_credits_amt).max(Usd::ZERO);

        // Line 12 = min(Line 6, Line 11)
        let line12_ok = self.current_year_allowable_credit_amt
            == self.est_tr_cy_bond_cr_amt.min(self.net_income_tax_amt);

        // Line 14 = Line 1
        let line14_ok = self.new_clean_energy_bond_amt == self.total_all_form1097_btc_amt;

        line4_ok && line6_ok && line9_ok && line10e_ok && line11_ok && line12_ok && line14_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F8912Input {
        F8912Input {
            total_all_form1097_btc_amt: Usd::from_dollars(1_000),
            total_other_not_rpt_f1097_btc_amt: Usd::from_dollars(500),
            carryforward_py_bond_credit_amt: Usd::from_dollars(200),
            estate_or_trust_allocated_benef_amt: Usd::ZERO,
            regular_tax_before_credit_amt: Usd::from_dollars(50_000),
            alternative_minimum_tax_amt: Usd::ZERO,
            foreign_tax_credit_amt: Usd::from_dollars(2_000),
            certain_allowable_credits_amt: Usd::from_dollars(1_000),
            general_business_credit_amt: Usd::ZERO,
            credit_prior_year_minimum_tax_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_with_line1_credits() {
        let input = basic_input();
        assert!(Output8912::must_file(&input));
    }

    #[test]
    fn must_file_with_only_line2_credits() {
        let mut input = basic_input();
        input.total_all_form1097_btc_amt = Usd::ZERO;
        input.carryforward_py_bond_credit_amt = Usd::ZERO;
        assert!(Output8912::must_file(&input));
    }

    #[test]
    fn must_file_with_only_carryforward() {
        let mut input = basic_input();
        input.total_all_form1097_btc_amt = Usd::ZERO;
        input.total_other_not_rpt_f1097_btc_amt = Usd::ZERO;
        assert!(Output8912::must_file(&input));
    }

    #[test]
    fn must_file_no_credits() {
        let mut input = basic_input();
        input.total_all_form1097_btc_amt = Usd::ZERO;
        input.total_other_not_rpt_f1097_btc_amt = Usd::ZERO;
        input.carryforward_py_bond_credit_amt = Usd::ZERO;
        assert!(!Output8912::must_file(&input));
    }

    #[test]
    fn basic_computation() {
        let form = Output8912::try_new(basic_input()).unwrap();
        // Line 4: 1,000 + 500 + 200 = 1,700
        assert_eq!(form.total_credit_amt, Usd::from_dollars(1_700));
        // Line 6: 1,700 - 0 = 1,700
        assert_eq!(form.est_tr_cy_bond_cr_amt, Usd::from_dollars(1_700));
        // Line 9: 50,000 + 0 = 50,000
        assert_eq!(
            form.sum_regular_tax_and_alt_min_tx_amt,
            Usd::from_dollars(50_000)
        );
        // Line 10e: 2,000 + 1,000 + 0 + 0 = 3,000
        assert_eq!(form.total_credits_amt, Usd::from_dollars(3_000));
        // Line 11: 50,000 - 3,000 = 47,000
        assert_eq!(form.net_income_tax_amt, Usd::from_dollars(47_000));
        // Line 12: min(1,700, 47,000) = 1,700
        assert_eq!(
            form.current_year_allowable_credit_amt,
            Usd::from_dollars(1_700)
        );
        // Line 14 = Line 1
        assert_eq!(
            form.new_clean_energy_bond_amt,
            Usd::from_dollars(1_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn estate_trust_allocation_reduces_line6() {
        let mut input = basic_input();
        input.estate_or_trust_allocated_benef_amt = Usd::from_dollars(700);
        let form = Output8912::try_new(input).unwrap();
        // Line 4: 1,700
        assert_eq!(form.total_credit_amt, Usd::from_dollars(1_700));
        // Line 6: 1,700 - 700 = 1,000
        assert_eq!(form.est_tr_cy_bond_cr_amt, Usd::from_dollars(1_000));
        // Line 12: min(1,000, 47,000) = 1,000
        assert_eq!(
            form.current_year_allowable_credit_amt,
            Usd::from_dollars(1_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn credit_limited_by_net_income_tax() {
        let mut input = basic_input();
        // Make net income tax very small: regular tax = 4,000, credits = 3,000 => net = 1,000
        input.regular_tax_before_credit_amt = Usd::from_dollars(4_000);
        let form = Output8912::try_new(input).unwrap();
        // Line 9: 4,000
        assert_eq!(
            form.sum_regular_tax_and_alt_min_tx_amt,
            Usd::from_dollars(4_000)
        );
        // Line 10e: 3,000
        assert_eq!(form.total_credits_amt, Usd::from_dollars(3_000));
        // Line 11: 4,000 - 3,000 = 1,000
        assert_eq!(form.net_income_tax_amt, Usd::from_dollars(1_000));
        // Line 6: 1,700; Line 11: 1,000 => Line 12: min(1,700, 1,000) = 1,000
        assert_eq!(
            form.current_year_allowable_credit_amt,
            Usd::from_dollars(1_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn net_income_tax_floored_at_zero() {
        let mut input = basic_input();
        // Credits exceed regular tax + AMT
        input.regular_tax_before_credit_amt = Usd::from_dollars(1_000);
        input.foreign_tax_credit_amt = Usd::from_dollars(5_000);
        let form = Output8912::try_new(input).unwrap();
        // Line 9: 1,000
        // Line 10e: 5,000 + 1,000 + 0 + 0 = 6,000
        // Line 11: max(1,000 - 6,000, 0) = 0
        assert_eq!(form.net_income_tax_amt, Usd::ZERO);
        // Line 12: min(1,700, 0) = 0
        assert_eq!(form.current_year_allowable_credit_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn alternative_minimum_tax_included() {
        let mut input = basic_input();
        input.alternative_minimum_tax_amt = Usd::from_dollars(10_000);
        let form = Output8912::try_new(input).unwrap();
        // Line 9: 50,000 + 10,000 = 60,000
        assert_eq!(
            form.sum_regular_tax_and_alt_min_tx_amt,
            Usd::from_dollars(60_000)
        );
        // Line 11: 60,000 - 3,000 = 57,000
        assert_eq!(form.net_income_tax_amt, Usd::from_dollars(57_000));
        assert!(form.is_valid());
    }

    #[test]
    fn all_credit_types_sum_in_line10e() {
        let mut input = basic_input();
        input.foreign_tax_credit_amt = Usd::from_dollars(1_000);
        input.certain_allowable_credits_amt = Usd::from_dollars(2_000);
        input.general_business_credit_amt = Usd::from_dollars(3_000);
        input.credit_prior_year_minimum_tax_amt = Usd::from_dollars(4_000);
        let form = Output8912::try_new(input).unwrap();
        // Line 10e: 1,000 + 2,000 + 3,000 + 4,000 = 10,000
        assert_eq!(form.total_credits_amt, Usd::from_dollars(10_000));
        assert!(form.is_valid());
    }

    #[test]
    fn zero_credits_zero_output() {
        let input = F8912Input {
            total_all_form1097_btc_amt: Usd::ZERO,
            total_other_not_rpt_f1097_btc_amt: Usd::ZERO,
            carryforward_py_bond_credit_amt: Usd::ZERO,
            estate_or_trust_allocated_benef_amt: Usd::ZERO,
            regular_tax_before_credit_amt: Usd::from_dollars(50_000),
            alternative_minimum_tax_amt: Usd::ZERO,
            foreign_tax_credit_amt: Usd::ZERO,
            certain_allowable_credits_amt: Usd::ZERO,
            general_business_credit_amt: Usd::ZERO,
            credit_prior_year_minimum_tax_amt: Usd::ZERO,
        };
        let form = Output8912::try_new(input).unwrap();
        assert_eq!(form.total_credit_amt, Usd::ZERO);
        assert_eq!(form.est_tr_cy_bond_cr_amt, Usd::ZERO);
        assert_eq!(form.current_year_allowable_credit_amt, Usd::ZERO);
        assert_eq!(form.new_clean_energy_bond_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn line6_floored_at_zero() {
        let mut input = basic_input();
        // Allocation exceeds total credit
        input.estate_or_trust_allocated_benef_amt = Usd::from_dollars(5_000);
        let form = Output8912::try_new(input).unwrap();
        // Line 4: 1,700; Line 5: 5,000
        // Line 6: max(1,700 - 5,000, 0) = 0
        assert_eq!(form.est_tr_cy_bond_cr_amt, Usd::ZERO);
        // Line 12: min(0, 47,000) = 0
        assert_eq!(form.current_year_allowable_credit_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn dependencies_are_correct() {
        let deps = Output8912::dependencies();
        assert_eq!(deps.len(), 3);
        assert!(deps.contains(&DynForm::F1040));
        assert!(deps.contains(&DynForm::F1116));
        assert!(deps.contains(&DynForm::F3800));
    }
}
