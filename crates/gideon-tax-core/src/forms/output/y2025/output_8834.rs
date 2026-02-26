use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8834.
///
/// The foreign tax credit (line 3a) comes from Form 1116, and certain
/// credits (line 3b) and regular tax before credits (line 2) come from
/// Form 1040. The corresponding dependencies are declared in
/// [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct F8834Input {
    /// Line 1: Qualified electric vehicle passive activity credits
    /// allowed for your current tax year (from Form 8582-CR or
    /// prior-year Form 8834 carryforward)
    pub qlfy_elec_motor_veh_cr_amt: Usd,
    /// Line 2: Regular tax before credits (see instructions)
    pub qlfy_elec_veh_regular_tx_bfr_cr_amt: Usd,
    /// Line 3a: Foreign tax credit
    pub foreign_tax_credit_amt: Usd,
    /// Line 3b: Certain allowable credits (see instructions)
    pub certain_allowable_credits_amt: Usd,
    /// Line 5: Tentative minimum tax (see instructions)
    pub qlfy_elec_veh_tentative_min_tax_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8834 (2025) — Qualified Electric Vehicle Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8834 {
    /// Line 1: Qualified electric vehicle passive activity credits allowed for your current tax year
    pub qlfy_elec_motor_veh_cr_amt: Usd,
    /// Line 2: Regular tax before credits
    pub qlfy_elec_veh_regular_tx_bfr_cr_amt: Usd,
    /// Line 3a: Foreign tax credit
    pub foreign_tax_credit_amt: Usd,
    /// Line 3b: Certain allowable credits (see instructions)
    pub certain_allowable_credits_amt: Usd,
    /// Line 3c: Add lines 3a and 3b
    pub tot_tax_cr_bfr_qlfy_elec_veh_cr_amt: Usd,
    /// Line 4: Net regular tax. Subtract line 3c from line 2. If zero or less, enter -0-
    pub qlfy_elec_veh_net_regular_tax_amt: Usd,
    /// Line 5: Tentative minimum tax
    pub qlfy_elec_veh_tentative_min_tax_amt: Usd,
    /// Line 6: Subtract line 5 from line 4. If zero or less, enter -0-
    pub qlfy_elec_veh_adj_regular_tax_amt: Usd,
    /// Line 7: Qualified electric vehicle credit. Enter the smaller of line 1 or line 6
    pub qlfy_elec_veh_pssv_acty_cr_allw_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8834 {
    fn name() -> &'static str {
        "Form 8834"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8834 {
    type Input = F8834Input;

    fn must_file(input: &Self::Input) -> bool {
        input.qlfy_elec_motor_veh_cr_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Line 3c: Add lines 3a and 3b
        let line3c = input.foreign_tax_credit_amt + input.certain_allowable_credits_amt;

        // Line 4: Line 2 - Line 3c (min 0)
        let line4 =
            (input.qlfy_elec_veh_regular_tx_bfr_cr_amt - line3c).max(Usd::ZERO);

        // Line 6: Line 4 - Line 5 (min 0)
        let line6 =
            (line4 - input.qlfy_elec_veh_tentative_min_tax_amt).max(Usd::ZERO);

        // Line 7: min(Line 1, Line 6)
        let line7 = input.qlfy_elec_motor_veh_cr_amt.min(line6);

        Ok(Output8834 {
            qlfy_elec_motor_veh_cr_amt: input.qlfy_elec_motor_veh_cr_amt,
            qlfy_elec_veh_regular_tx_bfr_cr_amt: input.qlfy_elec_veh_regular_tx_bfr_cr_amt,
            foreign_tax_credit_amt: input.foreign_tax_credit_amt,
            certain_allowable_credits_amt: input.certain_allowable_credits_amt,
            tot_tax_cr_bfr_qlfy_elec_veh_cr_amt: line3c,
            qlfy_elec_veh_net_regular_tax_amt: line4,
            qlfy_elec_veh_tentative_min_tax_amt: input.qlfy_elec_veh_tentative_min_tax_amt,
            qlfy_elec_veh_adj_regular_tax_amt: line6,
            qlfy_elec_veh_pssv_acty_cr_allw_amt: line7,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040, DynForm::F1116]
    }

    fn is_valid(&self) -> bool {
        // Line 3c = Line 3a + Line 3b
        let line3c_ok = self.tot_tax_cr_bfr_qlfy_elec_veh_cr_amt
            == self.foreign_tax_credit_amt + self.certain_allowable_credits_amt;

        // Line 4 = max(Line 2 - Line 3c, 0)
        let line4_ok = self.qlfy_elec_veh_net_regular_tax_amt
            == (self.qlfy_elec_veh_regular_tx_bfr_cr_amt
                - self.tot_tax_cr_bfr_qlfy_elec_veh_cr_amt)
                .max(Usd::ZERO);

        // Line 6 = max(Line 4 - Line 5, 0)
        let line6_ok = self.qlfy_elec_veh_adj_regular_tax_amt
            == (self.qlfy_elec_veh_net_regular_tax_amt
                - self.qlfy_elec_veh_tentative_min_tax_amt)
                .max(Usd::ZERO);

        // Line 7 = min(Line 1, Line 6)
        let line7_ok = self.qlfy_elec_veh_pssv_acty_cr_allw_amt
            == self
                .qlfy_elec_motor_veh_cr_amt
                .min(self.qlfy_elec_veh_adj_regular_tax_amt);

        line3c_ok && line4_ok && line6_ok && line7_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F8834Input {
        F8834Input {
            qlfy_elec_motor_veh_cr_amt: Usd::from_dollars(7_500),
            qlfy_elec_veh_regular_tx_bfr_cr_amt: Usd::from_dollars(20_000),
            foreign_tax_credit_amt: Usd::from_dollars(1_000),
            certain_allowable_credits_amt: Usd::from_dollars(2_000),
            qlfy_elec_veh_tentative_min_tax_amt: Usd::from_dollars(5_000),
        }
    }

    #[test]
    fn must_file_with_credit() {
        let input = basic_input();
        assert!(Output8834::must_file(&input));
    }

    #[test]
    fn must_file_no_credit() {
        let mut input = basic_input();
        input.qlfy_elec_motor_veh_cr_amt = Usd::ZERO;
        assert!(!Output8834::must_file(&input));
    }

    #[test]
    fn basic_computation() {
        let form = Output8834::try_new(basic_input()).unwrap();
        // Line 3c: 1,000 + 2,000 = 3,000
        assert_eq!(
            form.tot_tax_cr_bfr_qlfy_elec_veh_cr_amt,
            Usd::from_dollars(3_000)
        );
        // Line 4: 20,000 - 3,000 = 17,000
        assert_eq!(
            form.qlfy_elec_veh_net_regular_tax_amt,
            Usd::from_dollars(17_000)
        );
        // Line 6: 17,000 - 5,000 = 12,000
        assert_eq!(
            form.qlfy_elec_veh_adj_regular_tax_amt,
            Usd::from_dollars(12_000)
        );
        // Line 7: min(7,500, 12,000) = 7,500
        assert_eq!(
            form.qlfy_elec_veh_pssv_acty_cr_allw_amt,
            Usd::from_dollars(7_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn credit_limited_by_line6() {
        let mut input = basic_input();
        // Increase tentative minimum tax so line 6 is smaller than line 1
        input.qlfy_elec_veh_tentative_min_tax_amt = Usd::from_dollars(14_000);
        let form = Output8834::try_new(input).unwrap();
        // Line 4: 20,000 - 3,000 = 17,000
        assert_eq!(
            form.qlfy_elec_veh_net_regular_tax_amt,
            Usd::from_dollars(17_000)
        );
        // Line 6: 17,000 - 14,000 = 3,000
        assert_eq!(
            form.qlfy_elec_veh_adj_regular_tax_amt,
            Usd::from_dollars(3_000)
        );
        // Line 7: min(7,500, 3,000) = 3,000
        assert_eq!(
            form.qlfy_elec_veh_pssv_acty_cr_allw_amt,
            Usd::from_dollars(3_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn line4_floors_at_zero() {
        let mut input = basic_input();
        // Credits exceed regular tax
        input.foreign_tax_credit_amt = Usd::from_dollars(15_000);
        input.certain_allowable_credits_amt = Usd::from_dollars(10_000);
        let form = Output8834::try_new(input).unwrap();
        // Line 3c: 15,000 + 10,000 = 25,000
        assert_eq!(
            form.tot_tax_cr_bfr_qlfy_elec_veh_cr_amt,
            Usd::from_dollars(25_000)
        );
        // Line 4: 20,000 - 25,000 → 0
        assert_eq!(form.qlfy_elec_veh_net_regular_tax_amt, Usd::ZERO);
        // Line 6: 0 - 5,000 → 0
        assert_eq!(form.qlfy_elec_veh_adj_regular_tax_amt, Usd::ZERO);
        // Line 7: min(7,500, 0) = 0
        assert_eq!(form.qlfy_elec_veh_pssv_acty_cr_allw_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn line6_floors_at_zero() {
        let mut input = basic_input();
        // Tentative minimum tax exceeds net regular tax
        input.qlfy_elec_veh_tentative_min_tax_amt = Usd::from_dollars(25_000);
        let form = Output8834::try_new(input).unwrap();
        // Line 4: 20,000 - 3,000 = 17,000
        assert_eq!(
            form.qlfy_elec_veh_net_regular_tax_amt,
            Usd::from_dollars(17_000)
        );
        // Line 6: 17,000 - 25,000 → 0
        assert_eq!(form.qlfy_elec_veh_adj_regular_tax_amt, Usd::ZERO);
        // Line 7: min(7,500, 0) = 0
        assert_eq!(form.qlfy_elec_veh_pssv_acty_cr_allw_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn zero_credit_zero_output() {
        let input = F8834Input {
            qlfy_elec_motor_veh_cr_amt: Usd::ZERO,
            qlfy_elec_veh_regular_tx_bfr_cr_amt: Usd::from_dollars(20_000),
            foreign_tax_credit_amt: Usd::from_dollars(1_000),
            certain_allowable_credits_amt: Usd::from_dollars(2_000),
            qlfy_elec_veh_tentative_min_tax_amt: Usd::from_dollars(5_000),
        };
        let form = Output8834::try_new(input).unwrap();
        // Line 7: min(0, 12,000) = 0
        assert_eq!(form.qlfy_elec_veh_pssv_acty_cr_allw_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn all_zero_inputs() {
        let input = F8834Input {
            qlfy_elec_motor_veh_cr_amt: Usd::ZERO,
            qlfy_elec_veh_regular_tx_bfr_cr_amt: Usd::ZERO,
            foreign_tax_credit_amt: Usd::ZERO,
            certain_allowable_credits_amt: Usd::ZERO,
            qlfy_elec_veh_tentative_min_tax_amt: Usd::ZERO,
        };
        let form = Output8834::try_new(input).unwrap();
        assert_eq!(form.tot_tax_cr_bfr_qlfy_elec_veh_cr_amt, Usd::ZERO);
        assert_eq!(form.qlfy_elec_veh_net_regular_tax_amt, Usd::ZERO);
        assert_eq!(form.qlfy_elec_veh_adj_regular_tax_amt, Usd::ZERO);
        assert_eq!(form.qlfy_elec_veh_pssv_acty_cr_allw_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn dependencies_include_f1040_and_f1116() {
        let deps = Output8834::dependencies();
        assert!(deps.contains(&DynForm::F1040));
        assert!(deps.contains(&DynForm::F1116));
        assert_eq!(deps.len(), 2);
    }
}
