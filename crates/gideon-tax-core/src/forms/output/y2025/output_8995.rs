use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8995 — Qualified Business Income
/// Deduction Simplified Computation (2025).
///
/// This form is used by taxpayers with taxable income at or below the
/// threshold amount to compute the QBI deduction.
#[derive(Debug, Clone)]
pub struct F8995Input {
    /// Line 1 (column c total): Qualified business income or (loss)
    /// across all trades/businesses
    pub tot_qlfy_business_income_or_loss_amt: Usd,
    /// Line 2: Total qualified business income or (loss). Same as
    /// line 1 column (c) total.
    pub tot_qualified_business_income_amt: Usd,
    /// Line 3: Qualified business net (loss) carryforward from the
    /// prior year (negative or zero)
    pub py_qlfy_business_net_loss_cfwd_amt: Usd,
    /// Line 6: Qualified REIT dividends and publicly traded
    /// partnership (PTP) income or (loss)
    pub qlfy_reit_div_ptp_income_loss_amt: Usd,
    /// Line 7: Qualified REIT dividends and qualified PTP (loss)
    /// carryforward from the prior year (negative or zero)
    pub py_qlfy_reit_div_ptp_loss_cfwd_amt: Usd,
    /// Line 11: Taxable income before qualified business income
    /// deduction
    pub taxable_income_before_qbi_ded_amt: Usd,
    /// Line 12: Net capital gain (including qualified dividends)
    pub net_capital_gain_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8995 (2025) — Qualified Business Income Deduction Simplified Computation.
#[derive(Debug, Clone, Default)]
pub struct Output8995 {
    // -----------------------------------------------------------------------
    // Lines 1-5 — Qualified Business Income
    // -----------------------------------------------------------------------
    /// Line 1 (column c): Qualified business income or (loss) per trade/business (lines i-v)
    pub tot_qlfy_business_income_or_loss_amt: Usd,
    /// Line 2: Total qualified business income or (loss). Combine lines 1i through 1v, column (c)
    pub tot_qualified_business_income_amt: Usd,
    /// Line 3: Qualified business net (loss) carryforward from the prior year
    pub py_qlfy_business_net_loss_cfwd_amt: Usd,
    /// Line 4: Total qualified business income. Combine lines 2 and 3. If zero or less, enter -0-
    pub qbi_component_amt: Usd,
    /// Line 5: Qualified business income component. Multiply line 4 by 20% (0.20)
    pub qbi_ded_bfr_income_limitation_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 6-9 — REIT Dividends and PTP Income
    // -----------------------------------------------------------------------
    /// Line 6: Qualified REIT dividends and publicly traded partnership (PTP) income or (loss)
    pub qlfy_reit_div_ptp_income_loss_amt: Usd,
    /// Line 7: Qualified REIT dividends and qualified PTP (loss) carryforward from the prior year
    pub py_qlfy_reit_div_ptp_loss_cfwd_amt: Usd,
    /// Line 8: Total qualified REIT dividends and PTP income. Combine lines 6 and 7. If zero or less, enter -0-
    pub tot_qlfy_reit_div_ptp_income_amt: Usd,
    /// Line 9: REIT and PTP component. Multiply line 8 by 20% (0.20)
    pub reitptp_component_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 10-15 — Income Limitation and QBI Deduction
    // -----------------------------------------------------------------------
    /// Line 10: Qualified business income deduction before the income limitation. Add lines 5 and 9
    pub qualified_business_income_ded_amt: Usd,
    /// Line 11: Taxable income before qualified business income deduction (see instructions)
    pub taxable_income_before_qbi_ded_amt: Usd,
    /// Line 12: Enter your net capital gain, if any, increased by any qualified dividends (see instructions)
    pub net_capital_gain_amt: Usd,
    /// Line 13: Subtract line 12 from line 11. If zero or less, enter -0-
    pub adjusted_taxable_income_amt: Usd,
    /// Line 14: Income limitation. Multiply line 13 by 20% (0.20)
    pub income_limitation_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 16-17 — Loss Carryforwards
    // -----------------------------------------------------------------------
    /// Line 16: Total qualified business (loss) carryforward. Combine lines 2 and 3. If greater than zero, enter -0-
    pub tot_qlfy_bus_loss_carryforward_amt: Usd,
    /// Line 17: Total qualified REIT dividends and PTP (loss) carryforward. Combine lines 6 and 7. If greater than zero, enter -0-
    pub tot_qlfy_reit_div_ptp_loss_cfwd_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8995 {
    fn name() -> &'static str {
        "Form 8995"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8995 {
    type Input = F8995Input;

    fn must_file(input: &Self::Input) -> bool {
        input.tot_qlfy_business_income_or_loss_amt != Usd::ZERO
            || input.qlfy_reit_div_ptp_income_loss_amt != Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Validate: line 3 must be negative or zero
        if input.py_qlfy_business_net_loss_cfwd_amt > Usd::ZERO {
            return Err(GideonTaxError::OutOfBounds(
                "py_qlfy_business_net_loss_cfwd_amt must be negative or zero".to_string(),
            ));
        }

        // Validate: line 7 must be negative or zero
        if input.py_qlfy_reit_div_ptp_loss_cfwd_amt > Usd::ZERO {
            return Err(GideonTaxError::OutOfBounds(
                "py_qlfy_reit_div_ptp_loss_cfwd_amt must be negative or zero".to_string(),
            ));
        }

        // Line 1c (pass-through)
        let line1c = input.tot_qlfy_business_income_or_loss_amt;

        // Line 2 (pass-through)
        let line2 = input.tot_qualified_business_income_amt;

        // Line 3 (pass-through)
        let line3 = input.py_qlfy_business_net_loss_cfwd_amt;

        // Line 4: max(line2 + line3, 0)
        let line4 = (line2 + line3).max(Usd::ZERO);

        // Line 5: line4 * 20%
        let line5 = Usd::from_cents(line4.cents() * 20 / 100);

        // Line 6 (pass-through)
        let line6 = input.qlfy_reit_div_ptp_income_loss_amt;

        // Line 7 (pass-through)
        let line7 = input.py_qlfy_reit_div_ptp_loss_cfwd_amt;

        // Line 8: max(line6 + line7, 0)
        let line8 = (line6 + line7).max(Usd::ZERO);

        // Line 9: line8 * 20%
        let line9 = Usd::from_cents(line8.cents() * 20 / 100);

        // Line 10: line5 + line9
        let line10 = line5 + line9;

        // Line 11 (pass-through)
        let line11 = input.taxable_income_before_qbi_ded_amt;

        // Line 12 (pass-through)
        let line12 = input.net_capital_gain_amt;

        // Line 13: max(line11 - line12, 0)
        let line13 = (line11 - line12).max(Usd::ZERO);

        // Line 14: line13 * 20%
        let line14 = Usd::from_cents(line13.cents() * 20 / 100);

        // Line 15 (the actual deduction): min(line10, line14)
        // Not stored as a separate field — the deduction is implied
        // by taking min(line10, line14).

        // Line 16: min(line2 + line3, 0) — negative loss carryforward, or 0
        let line16 = (line2 + line3).min(Usd::ZERO);

        // Line 17: min(line6 + line7, 0)
        let line17 = (line6 + line7).min(Usd::ZERO);

        Ok(Output8995 {
            tot_qlfy_business_income_or_loss_amt: line1c,
            tot_qualified_business_income_amt: line2,
            py_qlfy_business_net_loss_cfwd_amt: line3,
            qbi_component_amt: line4,
            qbi_ded_bfr_income_limitation_amt: line5,
            qlfy_reit_div_ptp_income_loss_amt: line6,
            py_qlfy_reit_div_ptp_loss_cfwd_amt: line7,
            tot_qlfy_reit_div_ptp_income_amt: line8,
            reitptp_component_amt: line9,
            qualified_business_income_ded_amt: line10,
            taxable_income_before_qbi_ded_amt: line11,
            net_capital_gain_amt: line12,
            adjusted_taxable_income_amt: line13,
            income_limitation_amt: line14,
            tot_qlfy_bus_loss_carryforward_amt: line16,
            tot_qlfy_reit_div_ptp_loss_cfwd_amt: line17,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040]
    }

    fn is_valid(&self) -> bool {
        // Line 4 = max(Line 2 + Line 3, 0)
        let line4_ok = self.qbi_component_amt
            == (self.tot_qualified_business_income_amt + self.py_qlfy_business_net_loss_cfwd_amt)
                .max(Usd::ZERO);

        // Line 5 = Line 4 * 20%
        let line5_ok = self.qbi_ded_bfr_income_limitation_amt
            == Usd::from_cents(self.qbi_component_amt.cents() * 20 / 100);

        // Line 8 = max(Line 6 + Line 7, 0)
        let line8_ok = self.tot_qlfy_reit_div_ptp_income_amt
            == (self.qlfy_reit_div_ptp_income_loss_amt + self.py_qlfy_reit_div_ptp_loss_cfwd_amt)
                .max(Usd::ZERO);

        // Line 9 = Line 8 * 20%
        let line9_ok = self.reitptp_component_amt
            == Usd::from_cents(self.tot_qlfy_reit_div_ptp_income_amt.cents() * 20 / 100);

        // Line 10 = Line 5 + Line 9
        let line10_ok = self.qualified_business_income_ded_amt
            == self.qbi_ded_bfr_income_limitation_amt + self.reitptp_component_amt;

        // Line 13 = max(Line 11 - Line 12, 0)
        let line13_ok = self.adjusted_taxable_income_amt
            == (self.taxable_income_before_qbi_ded_amt - self.net_capital_gain_amt)
                .max(Usd::ZERO);

        // Line 14 = Line 13 * 20%
        let line14_ok = self.income_limitation_amt
            == Usd::from_cents(self.adjusted_taxable_income_amt.cents() * 20 / 100);

        // Line 16 = min(Line 2 + Line 3, 0)
        let line16_ok = self.tot_qlfy_bus_loss_carryforward_amt
            == (self.tot_qualified_business_income_amt + self.py_qlfy_business_net_loss_cfwd_amt)
                .min(Usd::ZERO);

        // Line 17 = min(Line 6 + Line 7, 0)
        let line17_ok = self.tot_qlfy_reit_div_ptp_loss_cfwd_amt
            == (self.qlfy_reit_div_ptp_income_loss_amt + self.py_qlfy_reit_div_ptp_loss_cfwd_amt)
                .min(Usd::ZERO);

        line4_ok
            && line5_ok
            && line8_ok
            && line9_ok
            && line10_ok
            && line13_ok
            && line14_ok
            && line16_ok
            && line17_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F8995Input {
        F8995Input {
            tot_qlfy_business_income_or_loss_amt: Usd::from_dollars(50_000),
            tot_qualified_business_income_amt: Usd::from_dollars(50_000),
            py_qlfy_business_net_loss_cfwd_amt: Usd::ZERO,
            qlfy_reit_div_ptp_income_loss_amt: Usd::ZERO,
            py_qlfy_reit_div_ptp_loss_cfwd_amt: Usd::ZERO,
            taxable_income_before_qbi_ded_amt: Usd::from_dollars(100_000),
            net_capital_gain_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_with_qbi() {
        let input = basic_input();
        assert!(Output8995::must_file(&input));
    }

    #[test]
    fn must_file_with_reit_only() {
        let mut input = basic_input();
        input.tot_qlfy_business_income_or_loss_amt = Usd::ZERO;
        input.qlfy_reit_div_ptp_income_loss_amt = Usd::from_dollars(10_000);
        assert!(Output8995::must_file(&input));
    }

    #[test]
    fn must_file_neither() {
        let mut input = basic_input();
        input.tot_qlfy_business_income_or_loss_amt = Usd::ZERO;
        input.tot_qualified_business_income_amt = Usd::ZERO;
        input.qlfy_reit_div_ptp_income_loss_amt = Usd::ZERO;
        assert!(!Output8995::must_file(&input));
    }

    #[test]
    fn basic_qbi_deduction() {
        let form = Output8995::try_new(basic_input()).unwrap();
        // Line 4: max(50,000 + 0, 0) = 50,000
        assert_eq!(form.qbi_component_amt, Usd::from_dollars(50_000));
        // Line 5: 50,000 * 20% = 10,000
        assert_eq!(
            form.qbi_ded_bfr_income_limitation_amt,
            Usd::from_dollars(10_000)
        );
        // Line 8: max(0 + 0, 0) = 0
        assert_eq!(form.tot_qlfy_reit_div_ptp_income_amt, Usd::ZERO);
        // Line 9: 0 * 20% = 0
        assert_eq!(form.reitptp_component_amt, Usd::ZERO);
        // Line 10: 10,000 + 0 = 10,000
        assert_eq!(
            form.qualified_business_income_ded_amt,
            Usd::from_dollars(10_000)
        );
        // Line 13: max(100,000 - 0, 0) = 100,000
        assert_eq!(
            form.adjusted_taxable_income_amt,
            Usd::from_dollars(100_000)
        );
        // Line 14: 100,000 * 20% = 20,000
        assert_eq!(form.income_limitation_amt, Usd::from_dollars(20_000));
        // Line 15 (implied): min(10,000, 20,000) = 10,000
        // No loss carryforwards
        assert_eq!(form.tot_qlfy_bus_loss_carryforward_amt, Usd::ZERO);
        assert_eq!(form.tot_qlfy_reit_div_ptp_loss_cfwd_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn income_limitation_caps_deduction() {
        let mut input = basic_input();
        // QBI is 50,000 → line 5 = 10,000
        // But taxable income is only 30,000 with 0 cap gain
        // Line 13 = 30,000, Line 14 = 6,000
        // Line 15 = min(10,000, 6,000) = 6,000
        input.taxable_income_before_qbi_ded_amt = Usd::from_dollars(30_000);
        let form = Output8995::try_new(input).unwrap();
        assert_eq!(
            form.qualified_business_income_ded_amt,
            Usd::from_dollars(10_000)
        );
        assert_eq!(form.income_limitation_amt, Usd::from_dollars(6_000));
        assert!(form.is_valid());
    }

    #[test]
    fn net_capital_gain_reduces_income_limitation() {
        let mut input = basic_input();
        input.net_capital_gain_amt = Usd::from_dollars(60_000);
        let form = Output8995::try_new(input).unwrap();
        // Line 13: max(100,000 - 60,000, 0) = 40,000
        assert_eq!(
            form.adjusted_taxable_income_amt,
            Usd::from_dollars(40_000)
        );
        // Line 14: 40,000 * 20% = 8,000
        assert_eq!(form.income_limitation_amt, Usd::from_dollars(8_000));
        assert!(form.is_valid());
    }

    #[test]
    fn net_capital_gain_exceeds_taxable_income() {
        let mut input = basic_input();
        input.net_capital_gain_amt = Usd::from_dollars(150_000);
        let form = Output8995::try_new(input).unwrap();
        // Line 13: max(100,000 - 150,000, 0) = 0
        assert_eq!(form.adjusted_taxable_income_amt, Usd::ZERO);
        // Line 14: 0 * 20% = 0
        assert_eq!(form.income_limitation_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn qbi_loss_carryforward() {
        let mut input = basic_input();
        input.tot_qlfy_business_income_or_loss_amt = Usd::from_dollars(10_000);
        input.tot_qualified_business_income_amt = Usd::from_dollars(10_000);
        input.py_qlfy_business_net_loss_cfwd_amt = Usd::from_dollars(-30_000);
        let form = Output8995::try_new(input).unwrap();
        // Line 4: max(10,000 + (-30,000), 0) = max(-20,000, 0) = 0
        assert_eq!(form.qbi_component_amt, Usd::ZERO);
        // Line 5: 0 * 20% = 0
        assert_eq!(form.qbi_ded_bfr_income_limitation_amt, Usd::ZERO);
        // Line 16: min(10,000 + (-30,000), 0) = min(-20,000, 0) = -20,000
        assert_eq!(
            form.tot_qlfy_bus_loss_carryforward_amt,
            Usd::from_dollars(-20_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn reit_ptp_income_and_loss() {
        let mut input = basic_input();
        input.qlfy_reit_div_ptp_income_loss_amt = Usd::from_dollars(20_000);
        input.py_qlfy_reit_div_ptp_loss_cfwd_amt = Usd::from_dollars(-5_000);
        let form = Output8995::try_new(input).unwrap();
        // Line 8: max(20,000 + (-5,000), 0) = 15,000
        assert_eq!(
            form.tot_qlfy_reit_div_ptp_income_amt,
            Usd::from_dollars(15_000)
        );
        // Line 9: 15,000 * 20% = 3,000
        assert_eq!(form.reitptp_component_amt, Usd::from_dollars(3_000));
        // Line 10: 10,000 + 3,000 = 13,000
        assert_eq!(
            form.qualified_business_income_ded_amt,
            Usd::from_dollars(13_000)
        );
        // Line 17: min(20,000 + (-5,000), 0) = min(15,000, 0) = 0
        assert_eq!(form.tot_qlfy_reit_div_ptp_loss_cfwd_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn reit_ptp_loss_carryforward() {
        let mut input = basic_input();
        input.qlfy_reit_div_ptp_income_loss_amt = Usd::from_dollars(5_000);
        input.py_qlfy_reit_div_ptp_loss_cfwd_amt = Usd::from_dollars(-15_000);
        let form = Output8995::try_new(input).unwrap();
        // Line 8: max(5,000 + (-15,000), 0) = max(-10,000, 0) = 0
        assert_eq!(form.tot_qlfy_reit_div_ptp_income_amt, Usd::ZERO);
        // Line 9: 0 * 20% = 0
        assert_eq!(form.reitptp_component_amt, Usd::ZERO);
        // Line 17: min(5,000 + (-15,000), 0) = min(-10,000, 0) = -10,000
        assert_eq!(
            form.tot_qlfy_reit_div_ptp_loss_cfwd_amt,
            Usd::from_dollars(-10_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn all_zeros() {
        let mut input = basic_input();
        input.tot_qlfy_business_income_or_loss_amt = Usd::ZERO;
        input.tot_qualified_business_income_amt = Usd::ZERO;
        input.taxable_income_before_qbi_ded_amt = Usd::ZERO;
        let form = Output8995::try_new(input).unwrap();
        assert_eq!(form.qbi_component_amt, Usd::ZERO);
        assert_eq!(form.qbi_ded_bfr_income_limitation_amt, Usd::ZERO);
        assert_eq!(form.tot_qlfy_reit_div_ptp_income_amt, Usd::ZERO);
        assert_eq!(form.reitptp_component_amt, Usd::ZERO);
        assert_eq!(form.qualified_business_income_ded_amt, Usd::ZERO);
        assert_eq!(form.adjusted_taxable_income_amt, Usd::ZERO);
        assert_eq!(form.income_limitation_amt, Usd::ZERO);
        assert_eq!(form.tot_qlfy_bus_loss_carryforward_amt, Usd::ZERO);
        assert_eq!(form.tot_qlfy_reit_div_ptp_loss_cfwd_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn line3_positive_returns_error() {
        let mut input = basic_input();
        input.py_qlfy_business_net_loss_cfwd_amt = Usd::from_dollars(1_000);
        let err = Output8995::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn line7_positive_returns_error() {
        let mut input = basic_input();
        input.py_qlfy_reit_div_ptp_loss_cfwd_amt = Usd::from_dollars(1_000);
        let err = Output8995::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn combined_qbi_and_reit_deduction() {
        let mut input = basic_input();
        input.tot_qlfy_business_income_or_loss_amt = Usd::from_dollars(80_000);
        input.tot_qualified_business_income_amt = Usd::from_dollars(80_000);
        input.qlfy_reit_div_ptp_income_loss_amt = Usd::from_dollars(20_000);
        input.taxable_income_before_qbi_ded_amt = Usd::from_dollars(200_000);
        let form = Output8995::try_new(input).unwrap();
        // Line 4: 80,000
        assert_eq!(form.qbi_component_amt, Usd::from_dollars(80_000));
        // Line 5: 80,000 * 20% = 16,000
        assert_eq!(
            form.qbi_ded_bfr_income_limitation_amt,
            Usd::from_dollars(16_000)
        );
        // Line 8: 20,000
        assert_eq!(
            form.tot_qlfy_reit_div_ptp_income_amt,
            Usd::from_dollars(20_000)
        );
        // Line 9: 20,000 * 20% = 4,000
        assert_eq!(form.reitptp_component_amt, Usd::from_dollars(4_000));
        // Line 10: 16,000 + 4,000 = 20,000
        assert_eq!(
            form.qualified_business_income_ded_amt,
            Usd::from_dollars(20_000)
        );
        // Line 14: 200,000 * 20% = 40,000
        assert_eq!(form.income_limitation_amt, Usd::from_dollars(40_000));
        // Line 15 (implied): min(20,000, 40,000) = 20,000
        assert!(form.is_valid());
    }

    #[test]
    fn fractional_cents_20_percent() {
        let mut input = basic_input();
        // 33,333 dollars → line 5 = 33,333 * 20% = 6,666.60
        input.tot_qlfy_business_income_or_loss_amt = Usd::from_dollars(33_333);
        input.tot_qualified_business_income_amt = Usd::from_dollars(33_333);
        input.taxable_income_before_qbi_ded_amt = Usd::from_dollars(200_000);
        let form = Output8995::try_new(input).unwrap();
        // 33,333.00 in cents = 3_333_300, * 20 / 100 = 666_660 cents = $6,666.60
        assert_eq!(
            form.qbi_ded_bfr_income_limitation_amt,
            Usd::from_cents(666_660)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn must_file_with_negative_qbi() {
        let mut input = basic_input();
        input.tot_qlfy_business_income_or_loss_amt = Usd::from_dollars(-5_000);
        input.tot_qualified_business_income_amt = Usd::from_dollars(-5_000);
        assert!(Output8995::must_file(&input));
    }
}
