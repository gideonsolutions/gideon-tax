use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 6781.
///
/// Section 1256 contract gains/losses, Form 1099-B adjustments, loss
/// carryback elections, and straddle gains/losses feed into Schedule D;
/// the corresponding dependency is declared in
/// [`OutputForm::dependencies`].
#[derive(Debug, Clone, Default)]
pub struct F6781Input {
    // -- Top-of-form checkboxes --
    /// Box A: Mixed straddle election
    pub mixed_straddle_election_ind: bool,
    /// Box B: Straddle-by-straddle identification election
    pub straddle_by_straddle_ind: bool,
    /// Box C: Mixed straddle account election
    pub mixed_straddle_account_ind: bool,
    /// Box D: Net section 1256 contracts loss election
    pub net_section1256_election_ind: bool,

    // -- Part I inputs --
    /// Line 1 column (c): Total gain from section 1256 contracts
    pub total_section1256_cntrcts_gain_amt: Usd,
    /// Line 1 column (b): Total loss from section 1256 contracts
    pub total_section1256_cntrcts_loss_amt: Usd,
    /// Line 3: Combined net gain or (loss) from section 1256 contracts
    pub net_gain_amt: Usd,
    /// Line 4: Form 1099-B adjustments
    pub form1099_b_adjustments_amt: Usd,
    /// Line 6: Loss carryback amount (entered as positive)
    pub section1256_carried_back_amt: Usd,

    // -- Part II inputs (pass-through) --
    /// Line 11a: Short-term portion of recognized losses from straddles
    pub short_term_portion_rcgnz_loss_amt: Usd,
    /// Line 11b: Long-term portion of recognized losses from straddles
    pub long_term_portion_rcgnz_loss_amt: Usd,
    /// Line 13a: Short-term portion of gains from straddles
    pub short_term_portion_of_gain_amt: Usd,
    /// Line 13b: Long-term portion of gains from straddles
    pub long_term_portion_of_gain_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 6781 (2025) — Gains and Losses From Section 1256 Contracts and Straddles.
#[derive(Debug, Clone, Default)]
pub struct Output6781 {
    // -----------------------------------------------------------------------
    // Top-of-form — Check all applicable boxes
    // -----------------------------------------------------------------------
    /// Box A: Mixed straddle election
    pub mixed_straddle_election_ind: bool,
    /// Box B: Straddle-by-straddle identification election
    pub straddle_by_straddle_ind: bool,
    /// Box C: Mixed straddle account election
    pub mixed_straddle_account_ind: bool,
    /// Box D: Net section 1256 contracts loss election
    pub net_section1256_election_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Section 1256 Contracts Marked to Market
    // -----------------------------------------------------------------------
    /// Line 1: Identification of account — gains and losses from section 1256 contracts
    /// (reported in columns (b) Loss and (c) Gain on the form)
    pub total_section1256_cntrcts_gain_amt: Usd,
    /// Line 1: Total section 1256 contracts loss amount
    pub total_section1256_cntrcts_loss_amt: Usd,
    /// Line 3: Net gain or (loss). Combine line 2, columns (b) and (c)
    pub net_gain_amt: Usd,
    /// Line 4: Form 1099-B adjustments. See instructions and attach statement
    pub form1099_b_adjustments_amt: Usd,
    /// Line 5: Combine lines 3 and 4
    pub net_gain_and1099_b_adjustments_amt: Usd,
    /// Line 6: If you have a net section 1256 contracts loss and checked box D above, enter the
    /// amount of loss to be carried back. Enter the loss as a positive number
    pub section1256_carried_back_amt: Usd,
    /// Line 7: Combine lines 5 and 6
    pub net_gain_and_adj_plus_carryback_amt: Usd,
    /// Line 8: Short-term capital gain or (loss). Multiply line 7 by 40% (0.40). Enter here and
    /// include on line 4 of Schedule D or on Form 8949. See instructions
    pub short_term_capital_gain_amt: Usd,
    /// Line 9: Long-term capital gain or (loss). Multiply line 7 by 60% (0.60). Enter here and
    /// include on line 11 of Schedule D or on Form 8949. See instructions
    pub long_term_capital_gain_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Gains and Losses From Straddles
    // -----------------------------------------------------------------------

    // -- Section A — Losses From Straddles --
    /// Line 11a: Enter the short-term portion of losses from line 10, column (h), here and
    /// include on line 4 of Schedule D or on Form 8949. See instructions
    pub short_term_portion_rcgnz_loss_amt: Usd,
    /// Line 11b: Enter the long-term portion of losses from line 10, column (h), here and include
    /// on line 11 of Schedule D or on Form 8949. See instructions
    pub long_term_portion_rcgnz_loss_amt: Usd,

    // -- Section B — Gains From Straddles --
    /// Line 13a: Enter the short-term portion of gains from line 12, column (f), here and include
    /// on line 4 of Schedule D or on Form 8949. See instructions
    pub short_term_portion_of_gain_amt: Usd,
    /// Line 13b: Enter the long-term portion of gains from line 12, column (f), here and include
    /// on line 11 of Schedule D or on Form 8949. See instructions
    pub long_term_portion_of_gain_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output6781 {
    fn name() -> &'static str {
        "Form 6781"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output6781 {
    type Input = F6781Input;

    fn must_file(input: &Self::Input) -> bool {
        input.total_section1256_cntrcts_gain_amt != Usd::ZERO
            || input.total_section1256_cntrcts_loss_amt != Usd::ZERO
            || input.short_term_portion_rcgnz_loss_amt != Usd::ZERO
            || input.long_term_portion_rcgnz_loss_amt != Usd::ZERO
            || input.short_term_portion_of_gain_amt != Usd::ZERO
            || input.long_term_portion_of_gain_amt != Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Line 5: Line 3 + Line 4
        let line5 = input.net_gain_amt + input.form1099_b_adjustments_amt;

        // Line 7: Line 5 + Line 6
        // (Line 6 is a loss carryback entered as a positive number;
        // adding it reduces the magnitude of a net loss.)
        let line7 = line5 + input.section1256_carried_back_amt;

        // Line 8: Short-term = line 7 × 40%
        let line8 = Usd::from_cents(line7.cents() * 40 / 100);

        // Line 9: Long-term = line 7 × 60%
        let line9 = Usd::from_cents(line7.cents() * 60 / 100);

        Ok(Output6781 {
            // Checkboxes
            mixed_straddle_election_ind: input.mixed_straddle_election_ind,
            straddle_by_straddle_ind: input.straddle_by_straddle_ind,
            mixed_straddle_account_ind: input.mixed_straddle_account_ind,
            net_section1256_election_ind: input.net_section1256_election_ind,

            // Part I
            total_section1256_cntrcts_gain_amt: input.total_section1256_cntrcts_gain_amt,
            total_section1256_cntrcts_loss_amt: input.total_section1256_cntrcts_loss_amt,
            net_gain_amt: input.net_gain_amt,
            form1099_b_adjustments_amt: input.form1099_b_adjustments_amt,
            net_gain_and1099_b_adjustments_amt: line5,
            section1256_carried_back_amt: input.section1256_carried_back_amt,
            net_gain_and_adj_plus_carryback_amt: line7,
            short_term_capital_gain_amt: line8,
            long_term_capital_gain_amt: line9,

            // Part II (pass-through)
            short_term_portion_rcgnz_loss_amt: input.short_term_portion_rcgnz_loss_amt,
            long_term_portion_rcgnz_loss_amt: input.long_term_portion_rcgnz_loss_amt,
            short_term_portion_of_gain_amt: input.short_term_portion_of_gain_amt,
            long_term_portion_of_gain_amt: input.long_term_portion_of_gain_amt,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::ScheduleD]
    }

    fn is_valid(&self) -> bool {
        // Line 5 = Line 3 + Line 4
        let line5_ok =
            self.net_gain_and1099_b_adjustments_amt == self.net_gain_amt + self.form1099_b_adjustments_amt;

        // Line 7 = Line 5 + Line 6
        let line7_ok = self.net_gain_and_adj_plus_carryback_amt
            == self.net_gain_and1099_b_adjustments_amt + self.section1256_carried_back_amt;

        // Line 8 = Line 7 × 40%
        let line8_ok = self.short_term_capital_gain_amt
            == Usd::from_cents(self.net_gain_and_adj_plus_carryback_amt.cents() * 40 / 100);

        // Line 9 = Line 7 × 60%
        let line9_ok = self.long_term_capital_gain_amt
            == Usd::from_cents(self.net_gain_and_adj_plus_carryback_amt.cents() * 60 / 100);

        line5_ok && line7_ok && line8_ok && line9_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F6781Input {
        F6781Input {
            mixed_straddle_election_ind: false,
            straddle_by_straddle_ind: false,
            mixed_straddle_account_ind: false,
            net_section1256_election_ind: false,
            total_section1256_cntrcts_gain_amt: Usd::from_dollars(10_000),
            total_section1256_cntrcts_loss_amt: Usd::from_dollars(3_000),
            net_gain_amt: Usd::from_dollars(7_000),
            form1099_b_adjustments_amt: Usd::ZERO,
            section1256_carried_back_amt: Usd::ZERO,
            short_term_portion_rcgnz_loss_amt: Usd::ZERO,
            long_term_portion_rcgnz_loss_amt: Usd::ZERO,
            short_term_portion_of_gain_amt: Usd::ZERO,
            long_term_portion_of_gain_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_with_section1256_gain() {
        let input = basic_input();
        assert!(Output6781::must_file(&input));
    }

    #[test]
    fn must_file_with_section1256_loss_only() {
        let mut input = F6781Input::default();
        input.total_section1256_cntrcts_loss_amt = Usd::from_dollars(500);
        assert!(Output6781::must_file(&input));
    }

    #[test]
    fn must_file_with_straddle_loss() {
        let mut input = F6781Input::default();
        input.short_term_portion_rcgnz_loss_amt = Usd::from_dollars(1_000);
        assert!(Output6781::must_file(&input));
    }

    #[test]
    fn must_file_with_straddle_gain() {
        let mut input = F6781Input::default();
        input.long_term_portion_of_gain_amt = Usd::from_dollars(2_000);
        assert!(Output6781::must_file(&input));
    }

    #[test]
    fn must_file_no_activity() {
        let input = F6781Input::default();
        assert!(!Output6781::must_file(&input));
    }

    #[test]
    fn basic_net_gain() {
        let form = Output6781::try_new(basic_input()).unwrap();
        // Line 5: 7,000 + 0 = 7,000
        assert_eq!(
            form.net_gain_and1099_b_adjustments_amt,
            Usd::from_dollars(7_000)
        );
        // Line 7: 7,000 + 0 = 7,000
        assert_eq!(
            form.net_gain_and_adj_plus_carryback_amt,
            Usd::from_dollars(7_000)
        );
        // Line 8: 7,000 × 40% = 2,800
        assert_eq!(
            form.short_term_capital_gain_amt,
            Usd::from_dollars(2_800)
        );
        // Line 9: 7,000 × 60% = 4,200
        assert_eq!(
            form.long_term_capital_gain_amt,
            Usd::from_dollars(4_200)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn with_1099b_adjustment() {
        let mut input = basic_input();
        input.form1099_b_adjustments_amt = Usd::from_dollars(500);
        let form = Output6781::try_new(input).unwrap();
        // Line 5: 7,000 + 500 = 7,500
        assert_eq!(
            form.net_gain_and1099_b_adjustments_amt,
            Usd::from_dollars(7_500)
        );
        // Line 7: 7,500 + 0 = 7,500
        assert_eq!(
            form.net_gain_and_adj_plus_carryback_amt,
            Usd::from_dollars(7_500)
        );
        // Line 8: 7,500 × 40% = 3,000
        assert_eq!(
            form.short_term_capital_gain_amt,
            Usd::from_dollars(3_000)
        );
        // Line 9: 7,500 × 60% = 4,500
        assert_eq!(
            form.long_term_capital_gain_amt,
            Usd::from_dollars(4_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn net_loss_with_carryback() {
        let mut input = F6781Input::default();
        input.total_section1256_cntrcts_loss_amt = Usd::from_dollars(8_000);
        input.net_gain_amt = Usd::from_dollars(-8_000);
        input.net_section1256_election_ind = true;
        input.section1256_carried_back_amt = Usd::from_dollars(3_000);
        let form = Output6781::try_new(input).unwrap();
        // Line 5: -8,000 + 0 = -8,000
        assert_eq!(
            form.net_gain_and1099_b_adjustments_amt,
            Usd::from_dollars(-8_000)
        );
        // Line 7: -8,000 + 3,000 = -5,000
        assert_eq!(
            form.net_gain_and_adj_plus_carryback_amt,
            Usd::from_dollars(-5_000)
        );
        // Line 8: -5,000 × 40% = -2,000
        assert_eq!(
            form.short_term_capital_gain_amt,
            Usd::from_dollars(-2_000)
        );
        // Line 9: -5,000 × 60% = -3,000
        assert_eq!(
            form.long_term_capital_gain_amt,
            Usd::from_dollars(-3_000)
        );
        assert!(form.net_section1256_election_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn straddle_passthrough() {
        let mut input = F6781Input::default();
        input.short_term_portion_rcgnz_loss_amt = Usd::from_dollars(1_000);
        input.long_term_portion_rcgnz_loss_amt = Usd::from_dollars(2_000);
        input.short_term_portion_of_gain_amt = Usd::from_dollars(500);
        input.long_term_portion_of_gain_amt = Usd::from_dollars(1_500);
        let form = Output6781::try_new(input).unwrap();
        assert_eq!(
            form.short_term_portion_rcgnz_loss_amt,
            Usd::from_dollars(1_000)
        );
        assert_eq!(
            form.long_term_portion_rcgnz_loss_amt,
            Usd::from_dollars(2_000)
        );
        assert_eq!(
            form.short_term_portion_of_gain_amt,
            Usd::from_dollars(500)
        );
        assert_eq!(
            form.long_term_portion_of_gain_amt,
            Usd::from_dollars(1_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn checkboxes_passthrough() {
        let mut input = basic_input();
        input.mixed_straddle_election_ind = true;
        input.straddle_by_straddle_ind = true;
        input.mixed_straddle_account_ind = true;
        input.net_section1256_election_ind = true;
        let form = Output6781::try_new(input).unwrap();
        assert!(form.mixed_straddle_election_ind);
        assert!(form.straddle_by_straddle_ind);
        assert!(form.mixed_straddle_account_ind);
        assert!(form.net_section1256_election_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn zero_activity() {
        let input = F6781Input::default();
        let form = Output6781::try_new(input).unwrap();
        assert_eq!(form.net_gain_and1099_b_adjustments_amt, Usd::ZERO);
        assert_eq!(form.net_gain_and_adj_plus_carryback_amt, Usd::ZERO);
        assert_eq!(form.short_term_capital_gain_amt, Usd::ZERO);
        assert_eq!(form.long_term_capital_gain_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn rounding_40_60_split() {
        // Use an amount that doesn't divide evenly by 100
        // $333 = 33300 cents. 40% = 13320 cents ($133.20), 60% = 19980 cents ($199.80)
        let mut input = F6781Input::default();
        input.total_section1256_cntrcts_gain_amt = Usd::from_dollars(333);
        input.net_gain_amt = Usd::from_dollars(333);
        let form = Output6781::try_new(input).unwrap();
        assert_eq!(form.short_term_capital_gain_amt, Usd::from_cents(13_320));
        assert_eq!(form.long_term_capital_gain_amt, Usd::from_cents(19_980));
        assert!(form.is_valid());
    }

    #[test]
    fn negative_1099b_adjustment() {
        let mut input = basic_input();
        input.form1099_b_adjustments_amt = Usd::from_dollars(-2_000);
        let form = Output6781::try_new(input).unwrap();
        // Line 5: 7,000 + (-2,000) = 5,000
        assert_eq!(
            form.net_gain_and1099_b_adjustments_amt,
            Usd::from_dollars(5_000)
        );
        // Line 8: 5,000 × 40% = 2,000
        assert_eq!(
            form.short_term_capital_gain_amt,
            Usd::from_dollars(2_000)
        );
        // Line 9: 5,000 × 60% = 3,000
        assert_eq!(
            form.long_term_capital_gain_amt,
            Usd::from_dollars(3_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn dependencies_includes_schedule_d() {
        assert_eq!(Output6781::dependencies(), &[DynForm::ScheduleD]);
    }
}
