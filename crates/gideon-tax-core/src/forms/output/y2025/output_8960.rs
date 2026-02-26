use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8960.
///
/// Lines that are computed (4c, 5d, 8, 9d, 11, 12, 15–17, 18c, 19c, 20)
/// are **not** part of the input; they are derived in [`OutputForm::try_new`].
#[derive(Debug, Clone)]
pub struct F8960Input {
    // ── Checkboxes ──
    /// Section 6013(g) election indicator
    pub section6013g_ind: bool,
    /// Section 6013(h) election indicator
    pub section6013h_ind: bool,
    /// Regulations section 1.1411-10(g) election indicator
    pub reg_section1141110g_ind: bool,

    // ── Part I — Investment Income ──
    /// Line 1: Taxable interest
    pub taxable_interest_amt: Usd,
    /// Line 2: Ordinary dividends
    pub ordinary_dividends_amt: Usd,
    /// Line 3: Annuities from nonqualified plans
    pub annuities_from_non_qlf_plans_amt: Usd,
    /// Line 4a: Net rental income or loss
    pub net_rental_income_or_loss_amt: Usd,
    /// Line 4b: Adjustment for net income/loss from non-section 1411 trade or business
    pub adj_net_incm_or_loss_non_sect1411_amt: Usd,
    /// Line 5a: Net gain or loss from disposition of property
    pub property_dispos_gain_or_loss_amt: Usd,
    /// Line 5b: Net gain or loss from disposition of property not subject to NIIT
    pub non_niit_prop_dispos_gain_or_loss_amt: Usd,
    /// Line 5c: Adjustment from disposition of partnership interest or S corporation stock
    pub adj_from_dispos_of_stock_amt: Usd,
    /// Line 6: Adjustments to investment income for certain CFCs and PFICs
    pub cfc_and_pfic_invst_incm_changes_amt: Usd,
    /// Line 7: Other modifications to investment income
    pub other_investment_income_or_loss_amt: Usd,

    // ── Part II — Investment Expenses ──
    /// Line 9a: Investment interest expenses
    pub investment_interest_amt: Usd,
    /// Line 9b: State, local, and foreign income tax
    pub state_local_foreign_income_tax_amt: Usd,
    /// Line 9c: Miscellaneous investment expenses
    pub misc_investment_expense_amt: Usd,
    /// Line 10: Additional modifications
    pub additional_modification_amt: Usd,

    // ── Part III — Tax Computation — Individuals ──
    /// Line 13: Modified adjusted gross income
    pub modified_agi_amt: Usd,
    /// Line 14: Threshold based on filing status ($200k Single, $250k MFJ, $125k MFS)
    pub filing_threshold_amt: Usd,

    // ── Part III — Tax Computation — Estates and Trusts ──
    /// Line 18b: Deductions for distributions of net investment income and charitable deductions
    pub income_distribution_deduction_amt: Usd,
    /// Line 19a: Adjusted gross income (estates/trusts)
    pub agi_less_trust_est_highest_tax_amt: Usd,
    /// Line 19b: Highest tax bracket for estates and trusts for the year
    pub trust_est_highest_tax_bracket_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8960 (2025) — Net Investment Income Tax — Individuals, Estates, and Trusts.
#[derive(Debug, Clone, Default)]
pub struct Output8960 {
    // -----------------------------------------------------------------------
    // Part I — Investment Income (checkboxes)
    // -----------------------------------------------------------------------
    /// Section 6013(g) election indicator
    pub section6013g_ind: bool,
    /// Section 6013(h) election indicator
    pub section6013h_ind: bool,
    /// Regulations section 1.1411-10(g) election indicator
    pub reg_section1141110g_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Investment Income (lines 1-8)
    // -----------------------------------------------------------------------
    /// Line 1: Taxable interest
    pub taxable_interest_amt: Usd,
    /// Line 2: Ordinary dividends
    pub ordinary_dividends_amt: Usd,
    /// Line 3: Annuities from nonqualified plans
    pub annuites_from_non_qlf_plans_amt: Usd,
    /// Line 4a: Rental real estate, royalties, partnerships, S corporations, trusts, trades or businesses
    pub net_rental_income_or_loss_amt: Usd,
    /// Line 4b: Adjustment for net income or loss derived in the ordinary course of a non-section 1411 trade or business
    pub adj_net_incm_or_loss_non_sect1411_amt: Usd,
    /// Line 4c: Combine lines 4a and 4b
    pub rental_re_and_adj_net_incm_or_loss_amt: Usd,
    /// Line 5a: Net gain or loss from disposition of property
    pub property_dispos_gain_or_loss_amt: Usd,
    /// Line 5b: Net gain or loss from disposition of property not subject to net investment income tax
    pub non_niit_prop_dispos_gain_or_loss_amt: Usd,
    /// Line 5c: Adjustment from disposition of partnership interest or S corporation stock
    pub adj_from_dispos_of_stock_amt: Usd,
    /// Line 5d: Combine lines 5a through 5c
    pub gain_or_loss_from_dispos_amt: Usd,
    /// Line 6: Adjustments to investment income for certain CFCs and PFICs
    pub cfc_and_pfic_invst_incm_changes_amt: Usd,
    /// Line 7: Other modifications to investment income
    pub other_investment_income_or_loss_amt: Usd,
    /// Line 8: Total investment income (combine lines 1, 2, 3, 4c, 5d, 6, and 7)
    pub total_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Investment Expenses Allocable to Investment Income and Modifications
    // -----------------------------------------------------------------------
    /// Line 9a: Investment interest expenses
    pub investment_interest_amt: Usd,
    /// Line 9b: State, local, and foreign income tax
    pub state_local_foreign_income_tax_amt: Usd,
    /// Line 9c: Miscellaneous investment expenses
    pub misc_investment_expense_amt: Usd,
    /// Line 9d: Add lines 9a, 9b, and 9c
    pub investment_expense_amt: Usd,
    /// Line 10: Additional modifications
    pub additional_modification_amt: Usd,
    /// Line 11: Total deductions and modifications (add lines 9d and 10)
    pub total_deduction_modification_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Tax Computation
    // -----------------------------------------------------------------------
    /// Line 12: Net investment income (subtract Part II, line 11 from Part I, line 8)
    pub net_investment_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Tax Computation — Individuals (lines 13-17)
    // -----------------------------------------------------------------------
    /// Line 13: Modified adjusted gross income
    pub modified_agi_amt: Usd,
    /// Line 14: Threshold based on filing status
    pub filing_threshold_amt: Usd,
    /// Line 15: Subtract line 14 from line 13 (if zero or less, enter -0-)
    pub magi_less_threshold_amt: Usd,
    /// Line 16: Smaller of line 12 or line 15
    pub smllr_incm_or_magi_less_thrshld_amt: Usd,
    /// Line 17: Net investment income tax for individuals (multiply line 16 by 3.8%)
    pub indiv_net_invst_income_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Tax Computation — Estates and Trusts (lines 18a-21)
    // -----------------------------------------------------------------------
    /// Line 18a: Net investment income (line 12 above)
    pub adjusted_gross_income_amt: Usd,
    /// Line 18b: Deductions for distributions of net investment income and charitable deductions
    pub income_distribution_deduction_amt: Usd,
    /// Line 18c: Undistributed net investment income (subtract line 18b from line 18a)
    pub undistributed_net_income_amt: Usd,
    /// Line 19a: Adjusted gross income
    pub agi_less_trust_est_highest_tax_amt: Usd,
    /// Line 19b: Highest tax bracket for estates and trusts for the year
    pub trust_est_highest_tax_bracket_amt: Usd,
    /// Line 19c: Subtract line 19b from line 19a (if zero or less, enter -0-)
    /// (AGI less highest tax bracket threshold)
    pub smllr_undistr_incm_agi_less_tax_amt: Usd,
    /// Line 20: Smaller of line 18c or line 19c
    pub est_trust_net_invst_income_tax_amt: Usd,
    // Line 21: Net investment income tax for estates and trusts (multiply line 20 by 3.8%)
    // (not stored separately; use est_trust_net_invst_income_tax_amt for the base)
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8960 {
    fn name() -> &'static str {
        "Form 8960"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8960 {
    type Input = F8960Input;

    fn must_file(input: &Self::Input) -> bool {
        // Must file if net investment income > 0 or MAGI exceeds threshold
        let line4c = input.net_rental_income_or_loss_amt
            + input.adj_net_incm_or_loss_non_sect1411_amt;
        let line5d = input.property_dispos_gain_or_loss_amt
            + input.non_niit_prop_dispos_gain_or_loss_amt
            + input.adj_from_dispos_of_stock_amt;
        let line8 = input.taxable_interest_amt
            + input.ordinary_dividends_amt
            + input.annuities_from_non_qlf_plans_amt
            + line4c
            + line5d
            + input.cfc_and_pfic_invst_incm_changes_amt
            + input.other_investment_income_or_loss_amt;
        let line9d = input.investment_interest_amt
            + input.state_local_foreign_income_tax_amt
            + input.misc_investment_expense_amt;
        let line11 = line9d + input.additional_modification_amt;
        let line12 = line8 - line11;

        line12 > Usd::ZERO || input.modified_agi_amt > input.filing_threshold_amt
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // ── Part I — Investment Income ──

        // Line 4c: Line 4a + Line 4b
        let line4c = input.net_rental_income_or_loss_amt
            + input.adj_net_incm_or_loss_non_sect1411_amt;

        // Line 5d: Line 5a + Line 5b + Line 5c
        let line5d = input.property_dispos_gain_or_loss_amt
            + input.non_niit_prop_dispos_gain_or_loss_amt
            + input.adj_from_dispos_of_stock_amt;

        // Line 8: Line 1 + Line 2 + Line 3 + Line 4c + Line 5d + Line 6 + Line 7
        let line8 = input.taxable_interest_amt
            + input.ordinary_dividends_amt
            + input.annuities_from_non_qlf_plans_amt
            + line4c
            + line5d
            + input.cfc_and_pfic_invst_incm_changes_amt
            + input.other_investment_income_or_loss_amt;

        // ── Part II — Investment Expenses ──

        // Line 9d: Line 9a + 9b + 9c
        let line9d = input.investment_interest_amt
            + input.state_local_foreign_income_tax_amt
            + input.misc_investment_expense_amt;

        // Line 11: Line 9d + Line 10
        let line11 = line9d + input.additional_modification_amt;

        // ── Part III — Tax Computation ──

        // Line 12: Line 8 - Line 11 (can be negative; net investment income)
        let line12 = line8 - line11;

        // ── Part III — Individuals (lines 13-17) ──

        // Line 15: max(Line 13 - Line 14, 0)
        let line15 = (input.modified_agi_amt - input.filing_threshold_amt).max(Usd::ZERO);

        // Line 16: min(max(Line 12, 0), Line 15) — only positive NII is taxed
        let line16 = line12.max(Usd::ZERO).min(line15);

        // Line 17: Line 16 x 3.8%
        let line17 = Usd::from_cents(line16.cents() * 38 / 1000);

        // ── Part III — Estates and Trusts (lines 18-20) ──

        // Line 18a: same as Line 12
        let line18a = line12;

        // Line 18c: max(Line 18a - Line 18b, 0)
        let line18c = (line18a - input.income_distribution_deduction_amt).max(Usd::ZERO);

        // Line 19c: max(Line 19a - Line 19b, 0)
        let line19c = (input.agi_less_trust_est_highest_tax_amt
            - input.trust_est_highest_tax_bracket_amt)
            .max(Usd::ZERO);

        // Line 20: min(Line 18c, Line 19c)
        let line20 = line18c.min(line19c);

        Ok(Output8960 {
            // Checkboxes
            section6013g_ind: input.section6013g_ind,
            section6013h_ind: input.section6013h_ind,
            reg_section1141110g_ind: input.reg_section1141110g_ind,

            // Part I
            taxable_interest_amt: input.taxable_interest_amt,
            ordinary_dividends_amt: input.ordinary_dividends_amt,
            annuites_from_non_qlf_plans_amt: input.annuities_from_non_qlf_plans_amt,
            net_rental_income_or_loss_amt: input.net_rental_income_or_loss_amt,
            adj_net_incm_or_loss_non_sect1411_amt: input.adj_net_incm_or_loss_non_sect1411_amt,
            rental_re_and_adj_net_incm_or_loss_amt: line4c,
            property_dispos_gain_or_loss_amt: input.property_dispos_gain_or_loss_amt,
            non_niit_prop_dispos_gain_or_loss_amt: input.non_niit_prop_dispos_gain_or_loss_amt,
            adj_from_dispos_of_stock_amt: input.adj_from_dispos_of_stock_amt,
            gain_or_loss_from_dispos_amt: line5d,
            cfc_and_pfic_invst_incm_changes_amt: input.cfc_and_pfic_invst_incm_changes_amt,
            other_investment_income_or_loss_amt: input.other_investment_income_or_loss_amt,
            total_income_amt: line8,

            // Part II
            investment_interest_amt: input.investment_interest_amt,
            state_local_foreign_income_tax_amt: input.state_local_foreign_income_tax_amt,
            misc_investment_expense_amt: input.misc_investment_expense_amt,
            investment_expense_amt: line9d,
            additional_modification_amt: input.additional_modification_amt,
            total_deduction_modification_amt: line11,

            // Part III — common
            net_investment_income_amt: line12,

            // Part III — Individuals
            modified_agi_amt: input.modified_agi_amt,
            filing_threshold_amt: input.filing_threshold_amt,
            magi_less_threshold_amt: line15,
            smllr_incm_or_magi_less_thrshld_amt: line16,
            indiv_net_invst_income_tax_amt: line17,

            // Part III — Estates and Trusts
            adjusted_gross_income_amt: line18a,
            income_distribution_deduction_amt: input.income_distribution_deduction_amt,
            undistributed_net_income_amt: line18c,
            agi_less_trust_est_highest_tax_amt: input.agi_less_trust_est_highest_tax_amt,
            trust_est_highest_tax_bracket_amt: input.trust_est_highest_tax_bracket_amt,
            smllr_undistr_incm_agi_less_tax_amt: line19c,
            est_trust_net_invst_income_tax_amt: line20,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040]
    }

    fn is_valid(&self) -> bool {
        // Line 4c = Line 4a + Line 4b
        let line4c_ok = self.rental_re_and_adj_net_incm_or_loss_amt
            == self.net_rental_income_or_loss_amt + self.adj_net_incm_or_loss_non_sect1411_amt;

        // Line 5d = Line 5a + Line 5b + Line 5c
        let line5d_ok = self.gain_or_loss_from_dispos_amt
            == self.property_dispos_gain_or_loss_amt
                + self.non_niit_prop_dispos_gain_or_loss_amt
                + self.adj_from_dispos_of_stock_amt;

        // Line 8 = Line 1 + Line 2 + Line 3 + Line 4c + Line 5d + Line 6 + Line 7
        let line8_ok = self.total_income_amt
            == self.taxable_interest_amt
                + self.ordinary_dividends_amt
                + self.annuites_from_non_qlf_plans_amt
                + self.rental_re_and_adj_net_incm_or_loss_amt
                + self.gain_or_loss_from_dispos_amt
                + self.cfc_and_pfic_invst_incm_changes_amt
                + self.other_investment_income_or_loss_amt;

        // Line 9d = Line 9a + 9b + 9c
        let line9d_ok = self.investment_expense_amt
            == self.investment_interest_amt
                + self.state_local_foreign_income_tax_amt
                + self.misc_investment_expense_amt;

        // Line 11 = Line 9d + Line 10
        let line11_ok =
            self.total_deduction_modification_amt == self.investment_expense_amt + self.additional_modification_amt;

        // Line 12 = Line 8 - Line 11
        let line12_ok =
            self.net_investment_income_amt == self.total_income_amt - self.total_deduction_modification_amt;

        // Line 15 = max(Line 13 - Line 14, 0)
        let line15_ok = self.magi_less_threshold_amt
            == (self.modified_agi_amt - self.filing_threshold_amt).max(Usd::ZERO);

        // Line 16 = min(max(Line 12, 0), Line 15)
        let line16_ok = self.smllr_incm_or_magi_less_thrshld_amt
            == self.net_investment_income_amt.max(Usd::ZERO).min(self.magi_less_threshold_amt);

        // Line 17 = Line 16 x 3.8%
        let line17_ok = self.indiv_net_invst_income_tax_amt
            == Usd::from_cents(self.smllr_incm_or_magi_less_thrshld_amt.cents() * 38 / 1000);

        // Line 18a = Line 12
        let line18a_ok = self.adjusted_gross_income_amt == self.net_investment_income_amt;

        // Line 18c = max(Line 18a - Line 18b, 0)
        let line18c_ok = self.undistributed_net_income_amt
            == (self.adjusted_gross_income_amt - self.income_distribution_deduction_amt)
                .max(Usd::ZERO);

        // Line 19c = max(Line 19a - Line 19b, 0)
        let line19c_ok = self.smllr_undistr_incm_agi_less_tax_amt
            == (self.agi_less_trust_est_highest_tax_amt - self.trust_est_highest_tax_bracket_amt)
                .max(Usd::ZERO);

        // Line 20 = min(Line 18c, Line 19c)
        let line20_ok = self.est_trust_net_invst_income_tax_amt
            == self
                .undistributed_net_income_amt
                .min(self.smllr_undistr_incm_agi_less_tax_amt);

        line4c_ok
            && line5d_ok
            && line8_ok
            && line9d_ok
            && line11_ok
            && line12_ok
            && line15_ok
            && line16_ok
            && line17_ok
            && line18a_ok
            && line18c_ok
            && line19c_ok
            && line20_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Builds a basic individual filer input: $300k MAGI, $250k threshold (MFJ),
    /// some investment income.
    fn basic_individual_input() -> F8960Input {
        F8960Input {
            section6013g_ind: false,
            section6013h_ind: false,
            reg_section1141110g_ind: false,
            // Part I
            taxable_interest_amt: Usd::from_dollars(10_000),
            ordinary_dividends_amt: Usd::from_dollars(5_000),
            annuities_from_non_qlf_plans_amt: Usd::ZERO,
            net_rental_income_or_loss_amt: Usd::from_dollars(15_000),
            adj_net_incm_or_loss_non_sect1411_amt: Usd::ZERO,
            property_dispos_gain_or_loss_amt: Usd::from_dollars(20_000),
            non_niit_prop_dispos_gain_or_loss_amt: Usd::ZERO,
            adj_from_dispos_of_stock_amt: Usd::ZERO,
            cfc_and_pfic_invst_incm_changes_amt: Usd::ZERO,
            other_investment_income_or_loss_amt: Usd::ZERO,
            // Part II
            investment_interest_amt: Usd::ZERO,
            state_local_foreign_income_tax_amt: Usd::ZERO,
            misc_investment_expense_amt: Usd::ZERO,
            additional_modification_amt: Usd::ZERO,
            // Part III — Individuals
            modified_agi_amt: Usd::from_dollars(300_000),
            filing_threshold_amt: Usd::from_dollars(250_000),
            // Part III — Estates/Trusts (zeroed for individual filer)
            income_distribution_deduction_amt: Usd::ZERO,
            agi_less_trust_est_highest_tax_amt: Usd::ZERO,
            trust_est_highest_tax_bracket_amt: Usd::ZERO,
        }
    }

    #[test]
    fn basic_individual_computation() {
        let form = Output8960::try_new(basic_individual_input()).unwrap();
        // Line 4c: 15,000 + 0 = 15,000
        assert_eq!(
            form.rental_re_and_adj_net_incm_or_loss_amt,
            Usd::from_dollars(15_000)
        );
        // Line 5d: 20,000 + 0 + 0 = 20,000
        assert_eq!(
            form.gain_or_loss_from_dispos_amt,
            Usd::from_dollars(20_000)
        );
        // Line 8: 10,000 + 5,000 + 0 + 15,000 + 20,000 + 0 + 0 = 50,000
        assert_eq!(form.total_income_amt, Usd::from_dollars(50_000));
        // Line 9d: 0
        assert_eq!(form.investment_expense_amt, Usd::ZERO);
        // Line 11: 0
        assert_eq!(form.total_deduction_modification_amt, Usd::ZERO);
        // Line 12: 50,000 - 0 = 50,000
        assert_eq!(
            form.net_investment_income_amt,
            Usd::from_dollars(50_000)
        );
        // Line 15: max(300,000 - 250,000, 0) = 50,000
        assert_eq!(
            form.magi_less_threshold_amt,
            Usd::from_dollars(50_000)
        );
        // Line 16: min(max(50,000, 0), 50,000) = 50,000
        assert_eq!(
            form.smllr_incm_or_magi_less_thrshld_amt,
            Usd::from_dollars(50_000)
        );
        // Line 17: 50,000 * 3.8% = 1,900
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(1_900)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn must_file_positive_nii() {
        let input = basic_individual_input();
        assert!(Output8960::must_file(&input));
    }

    #[test]
    fn must_file_magi_exceeds_threshold_but_no_nii() {
        let mut input = basic_individual_input();
        // Zero out all investment income
        input.taxable_interest_amt = Usd::ZERO;
        input.ordinary_dividends_amt = Usd::ZERO;
        input.net_rental_income_or_loss_amt = Usd::ZERO;
        input.property_dispos_gain_or_loss_amt = Usd::ZERO;
        // MAGI still exceeds threshold
        assert!(Output8960::must_file(&input));
    }

    #[test]
    fn must_file_false_when_no_nii_and_below_threshold() {
        let mut input = basic_individual_input();
        input.taxable_interest_amt = Usd::ZERO;
        input.ordinary_dividends_amt = Usd::ZERO;
        input.net_rental_income_or_loss_amt = Usd::ZERO;
        input.property_dispos_gain_or_loss_amt = Usd::ZERO;
        input.modified_agi_amt = Usd::from_dollars(200_000);
        input.filing_threshold_amt = Usd::from_dollars(250_000);
        assert!(!Output8960::must_file(&input));
    }

    #[test]
    fn magi_below_threshold_zero_tax() {
        let mut input = basic_individual_input();
        input.modified_agi_amt = Usd::from_dollars(200_000);
        input.filing_threshold_amt = Usd::from_dollars(250_000);
        let form = Output8960::try_new(input).unwrap();
        // Line 15: max(200,000 - 250,000, 0) = 0
        assert_eq!(form.magi_less_threshold_amt, Usd::ZERO);
        // Line 16: min(max(50,000, 0), 0) = 0
        assert_eq!(form.smllr_incm_or_magi_less_thrshld_amt, Usd::ZERO);
        // Line 17: 0
        assert_eq!(form.indiv_net_invst_income_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn nii_limited_by_magi_excess() {
        let mut input = basic_individual_input();
        // MAGI only $10k over threshold, but NII is $50k
        input.modified_agi_amt = Usd::from_dollars(260_000);
        input.filing_threshold_amt = Usd::from_dollars(250_000);
        let form = Output8960::try_new(input).unwrap();
        // Line 12: 50,000 (NII)
        // Line 15: max(260,000 - 250,000, 0) = 10,000
        assert_eq!(
            form.magi_less_threshold_amt,
            Usd::from_dollars(10_000)
        );
        // Line 16: min(50,000, 10,000) = 10,000
        assert_eq!(
            form.smllr_incm_or_magi_less_thrshld_amt,
            Usd::from_dollars(10_000)
        );
        // Line 17: 10,000 * 3.8% = 380
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(380)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn negative_nii_zero_tax() {
        let mut input = basic_individual_input();
        // Net rental loss of $100,000 and no other investment income
        input.taxable_interest_amt = Usd::ZERO;
        input.ordinary_dividends_amt = Usd::ZERO;
        input.net_rental_income_or_loss_amt = Usd::from_dollars(-100_000);
        input.property_dispos_gain_or_loss_amt = Usd::ZERO;
        let form = Output8960::try_new(input).unwrap();
        // Line 12: -100,000 (negative NII)
        assert_eq!(
            form.net_investment_income_amt,
            Usd::from_dollars(-100_000)
        );
        // Line 16: min(max(-100,000, 0), 50,000) = min(0, 50,000) = 0
        assert_eq!(form.smllr_incm_or_magi_less_thrshld_amt, Usd::ZERO);
        // Line 17: 0
        assert_eq!(form.indiv_net_invst_income_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn investment_expenses_reduce_nii() {
        let mut input = basic_individual_input();
        input.investment_interest_amt = Usd::from_dollars(3_000);
        input.state_local_foreign_income_tax_amt = Usd::from_dollars(2_000);
        input.misc_investment_expense_amt = Usd::from_dollars(1_000);
        input.additional_modification_amt = Usd::from_dollars(4_000);
        let form = Output8960::try_new(input).unwrap();
        // Line 9d: 3,000 + 2,000 + 1,000 = 6,000
        assert_eq!(
            form.investment_expense_amt,
            Usd::from_dollars(6_000)
        );
        // Line 11: 6,000 + 4,000 = 10,000
        assert_eq!(
            form.total_deduction_modification_amt,
            Usd::from_dollars(10_000)
        );
        // Line 12: 50,000 - 10,000 = 40,000
        assert_eq!(
            form.net_investment_income_amt,
            Usd::from_dollars(40_000)
        );
        // Line 16: min(max(40,000, 0), 50,000) = 40,000
        assert_eq!(
            form.smllr_incm_or_magi_less_thrshld_amt,
            Usd::from_dollars(40_000)
        );
        // Line 17: 40,000 * 3.8% = 1,520
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(1_520)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn property_disposition_lines() {
        let mut input = basic_individual_input();
        input.property_dispos_gain_or_loss_amt = Usd::from_dollars(30_000);
        input.non_niit_prop_dispos_gain_or_loss_amt = Usd::from_dollars(-10_000);
        input.adj_from_dispos_of_stock_amt = Usd::from_dollars(5_000);
        let form = Output8960::try_new(input).unwrap();
        // Line 5d: 30,000 + (-10,000) + 5,000 = 25,000
        assert_eq!(
            form.gain_or_loss_from_dispos_amt,
            Usd::from_dollars(25_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn rental_adjustment_lines() {
        let mut input = basic_individual_input();
        input.net_rental_income_or_loss_amt = Usd::from_dollars(20_000);
        input.adj_net_incm_or_loss_non_sect1411_amt = Usd::from_dollars(-8_000);
        let form = Output8960::try_new(input).unwrap();
        // Line 4c: 20,000 + (-8,000) = 12,000
        assert_eq!(
            form.rental_re_and_adj_net_incm_or_loss_amt,
            Usd::from_dollars(12_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn estates_trusts_computation() {
        let mut input = basic_individual_input();
        // Set up estate/trust scenario
        input.income_distribution_deduction_amt = Usd::from_dollars(10_000);
        input.agi_less_trust_est_highest_tax_amt = Usd::from_dollars(100_000);
        input.trust_est_highest_tax_bracket_amt = Usd::from_dollars(15_200);
        let form = Output8960::try_new(input).unwrap();
        // Line 18a = Line 12 = 50,000
        assert_eq!(
            form.adjusted_gross_income_amt,
            Usd::from_dollars(50_000)
        );
        // Line 18c: max(50,000 - 10,000, 0) = 40,000
        assert_eq!(
            form.undistributed_net_income_amt,
            Usd::from_dollars(40_000)
        );
        // Line 19c: max(100,000 - 15,200, 0) = 84,800
        assert_eq!(
            form.smllr_undistr_incm_agi_less_tax_amt,
            Usd::from_dollars(84_800)
        );
        // Line 20: min(40,000, 84,800) = 40,000
        assert_eq!(
            form.est_trust_net_invst_income_tax_amt,
            Usd::from_dollars(40_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn estates_trusts_limited_by_agi_excess() {
        let mut input = basic_individual_input();
        input.income_distribution_deduction_amt = Usd::ZERO;
        input.agi_less_trust_est_highest_tax_amt = Usd::from_dollars(20_000);
        input.trust_est_highest_tax_bracket_amt = Usd::from_dollars(15_200);
        let form = Output8960::try_new(input).unwrap();
        // Line 18c: max(50,000 - 0, 0) = 50,000
        assert_eq!(
            form.undistributed_net_income_amt,
            Usd::from_dollars(50_000)
        );
        // Line 19c: max(20,000 - 15,200, 0) = 4,800
        assert_eq!(
            form.smllr_undistr_incm_agi_less_tax_amt,
            Usd::from_dollars(4_800)
        );
        // Line 20: min(50,000, 4,800) = 4,800
        assert_eq!(
            form.est_trust_net_invst_income_tax_amt,
            Usd::from_dollars(4_800)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn estates_trusts_zero_when_agi_below_bracket() {
        let mut input = basic_individual_input();
        input.income_distribution_deduction_amt = Usd::ZERO;
        input.agi_less_trust_est_highest_tax_amt = Usd::from_dollars(10_000);
        input.trust_est_highest_tax_bracket_amt = Usd::from_dollars(15_200);
        let form = Output8960::try_new(input).unwrap();
        // Line 19c: max(10,000 - 15,200, 0) = 0
        assert_eq!(form.smllr_undistr_incm_agi_less_tax_amt, Usd::ZERO);
        // Line 20: min(50,000, 0) = 0
        assert_eq!(form.est_trust_net_invst_income_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn checkboxes_pass_through() {
        let mut input = basic_individual_input();
        input.section6013g_ind = true;
        input.section6013h_ind = true;
        input.reg_section1141110g_ind = true;
        let form = Output8960::try_new(input).unwrap();
        assert!(form.section6013g_ind);
        assert!(form.section6013h_ind);
        assert!(form.reg_section1141110g_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn single_filer_200k_threshold() {
        let mut input = basic_individual_input();
        input.modified_agi_amt = Usd::from_dollars(250_000);
        input.filing_threshold_amt = Usd::from_dollars(200_000);
        let form = Output8960::try_new(input).unwrap();
        // Line 15: max(250,000 - 200,000, 0) = 50,000
        assert_eq!(
            form.magi_less_threshold_amt,
            Usd::from_dollars(50_000)
        );
        // Line 16: min(50,000, 50,000) = 50,000
        assert_eq!(
            form.smllr_incm_or_magi_less_thrshld_amt,
            Usd::from_dollars(50_000)
        );
        // Line 17: 50,000 * 3.8% = 1,900
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(1_900)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn mfs_125k_threshold() {
        let mut input = basic_individual_input();
        input.modified_agi_amt = Usd::from_dollars(175_000);
        input.filing_threshold_amt = Usd::from_dollars(125_000);
        let form = Output8960::try_new(input).unwrap();
        // Line 15: max(175,000 - 125,000, 0) = 50,000
        assert_eq!(
            form.magi_less_threshold_amt,
            Usd::from_dollars(50_000)
        );
        // Line 16: min(50,000, 50,000) = 50,000
        assert_eq!(
            form.smllr_incm_or_magi_less_thrshld_amt,
            Usd::from_dollars(50_000)
        );
        // Line 17: 50,000 * 3.8% = 1,900
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(1_900)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn fractional_tax_computation() {
        let mut input = basic_individual_input();
        // Set up so line 16 is an amount that produces non-even cents
        input.taxable_interest_amt = Usd::from_dollars(1_000);
        input.ordinary_dividends_amt = Usd::ZERO;
        input.net_rental_income_or_loss_amt = Usd::ZERO;
        input.property_dispos_gain_or_loss_amt = Usd::ZERO;
        input.modified_agi_amt = Usd::from_dollars(251_000);
        input.filing_threshold_amt = Usd::from_dollars(250_000);
        let form = Output8960::try_new(input).unwrap();
        // Line 12: 1,000 (NII)
        // Line 15: max(251,000 - 250,000, 0) = 1,000
        // Line 16: min(1,000, 1,000) = 1,000
        // Line 17: 1,000 * 3.8% = 38 dollars
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(38)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn all_zero_input() {
        let input = F8960Input {
            section6013g_ind: false,
            section6013h_ind: false,
            reg_section1141110g_ind: false,
            taxable_interest_amt: Usd::ZERO,
            ordinary_dividends_amt: Usd::ZERO,
            annuities_from_non_qlf_plans_amt: Usd::ZERO,
            net_rental_income_or_loss_amt: Usd::ZERO,
            adj_net_incm_or_loss_non_sect1411_amt: Usd::ZERO,
            property_dispos_gain_or_loss_amt: Usd::ZERO,
            non_niit_prop_dispos_gain_or_loss_amt: Usd::ZERO,
            adj_from_dispos_of_stock_amt: Usd::ZERO,
            cfc_and_pfic_invst_incm_changes_amt: Usd::ZERO,
            other_investment_income_or_loss_amt: Usd::ZERO,
            investment_interest_amt: Usd::ZERO,
            state_local_foreign_income_tax_amt: Usd::ZERO,
            misc_investment_expense_amt: Usd::ZERO,
            additional_modification_amt: Usd::ZERO,
            modified_agi_amt: Usd::ZERO,
            filing_threshold_amt: Usd::from_dollars(200_000),
            income_distribution_deduction_amt: Usd::ZERO,
            agi_less_trust_est_highest_tax_amt: Usd::ZERO,
            trust_est_highest_tax_bracket_amt: Usd::ZERO,
        };
        let form = Output8960::try_new(input).unwrap();
        assert_eq!(form.total_income_amt, Usd::ZERO);
        assert_eq!(form.net_investment_income_amt, Usd::ZERO);
        assert_eq!(form.indiv_net_invst_income_tax_amt, Usd::ZERO);
        assert_eq!(form.est_trust_net_invst_income_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn form_metadata() {
        assert_eq!(Output8960::name(), "Form 8960");
        assert_eq!(<Output8960 as Form>::form_type(), FormType::Output);
        assert_eq!(Output8960::dependencies(), &[DynForm::F1040]);
    }
}
