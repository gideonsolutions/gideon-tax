use us_tax_brackets::{FilingStatus, TaxYear};

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::rules::TaxYearRules;
use crate::rules::y2025::Rules2025;
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8960 (2025) — Net Investment Income
/// Tax — Individuals, Estates, and Trusts.
///
/// The form computes a 3.8% tax on net investment income for individuals whose
/// MAGI exceeds the filing-status threshold, and for estates/trusts whose AGI
/// exceeds the highest bracket threshold.
#[derive(Debug, Clone)]
pub struct F8960Input {
    /// Filing status (determines the NIIT threshold for individuals)
    pub filing_status: FilingStatus,

    // ── Checkboxes ────────────────────────────────────────────────────
    /// Section 6013(g) election indicator
    pub section6013g_ind: bool,
    /// Section 6013(h) election indicator
    pub section6013h_ind: bool,
    /// Regulations section 1.1411-10(g) election indicator
    pub reg_section1141110g_ind: bool,

    // ── Part I — Investment Income ────────────────────────────────────
    /// Line 1: Taxable interest
    pub taxable_interest_amt: Usd,
    /// Line 2: Ordinary dividends
    pub ordinary_dividends_amt: Usd,
    /// Line 3: Annuities from nonqualified plans
    pub annuites_from_non_qlf_plans_amt: Usd,
    /// Line 4a: Rental real estate, royalties, partnerships, S corps, trusts
    pub net_rental_income_or_loss_amt: Usd,
    /// Line 4b: Adjustment for net income/loss from non-section 1411 trade/business
    pub adj_net_incm_or_loss_non_sect1411_amt: Usd,
    /// Line 5a: Net gain or loss from disposition of property
    pub property_dispos_gain_or_loss_amt: Usd,
    /// Line 5b: Net gain/loss from property not subject to NIIT
    pub non_niit_prop_dispos_gain_or_loss_amt: Usd,
    /// Line 5c: Adjustment from disposition of partnership interest or S corp stock
    pub adj_from_dispos_of_stock_amt: Usd,
    /// Line 6: Adjustments for certain CFCs and PFICs
    pub cfc_and_pfic_invst_incm_changes_amt: Usd,
    /// Line 7: Other modifications to investment income
    pub other_investment_income_or_loss_amt: Usd,

    // ── Part II — Investment Expenses ─────────────────────────────────
    /// Line 9a: Investment interest expenses
    pub investment_interest_amt: Usd,
    /// Line 9b: State, local, and foreign income tax
    pub state_local_foreign_income_tax_amt: Usd,
    /// Line 9c: Miscellaneous investment expenses
    pub misc_investment_expense_amt: Usd,
    /// Line 10: Additional modifications
    pub additional_modification_amt: Usd,

    // ── Part III — Tax Computation (Individuals) ──────────────────────
    /// Line 13: Modified adjusted gross income
    pub modified_agi_amt: Usd,

    // ── Part III — Tax Computation (Estates and Trusts) ───────────────
    /// Line 18b: Deductions for distributions of net investment income
    /// and charitable deductions
    pub income_distribution_deduction_amt: Usd,
    /// Line 19a: Adjusted gross income (for estates/trusts)
    pub adjusted_gross_income_amt: Usd,
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
    pub smllr_undistr_incm_agi_less_tax_amt: Usd,
    /// Line 20: Smaller of line 18c or line 19c
    pub est_trust_net_invst_income_tax_amt: Usd,
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
        let threshold = Rules2025::niit_threshold(input.filing_status);
        input.modified_agi_amt > threshold
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        let niit_bps = Rules2025::NIIT_RATE_BPS as i64;

        // ── Part I — Investment Income ───────────────────────────────
        let line1 = input.taxable_interest_amt;
        let line2 = input.ordinary_dividends_amt;
        let line3 = input.annuites_from_non_qlf_plans_amt;
        let line4a = input.net_rental_income_or_loss_amt;
        let line4b = input.adj_net_incm_or_loss_non_sect1411_amt;
        let line4c = line4a + line4b;
        let line5a = input.property_dispos_gain_or_loss_amt;
        let line5b = input.non_niit_prop_dispos_gain_or_loss_amt;
        let line5c = input.adj_from_dispos_of_stock_amt;
        let line5d = line5a + line5b + line5c;
        let line6 = input.cfc_and_pfic_invst_incm_changes_amt;
        let line7 = input.other_investment_income_or_loss_amt;
        let line8 = line1 + line2 + line3 + line4c + line5d + line6 + line7;

        // ── Part II — Investment Expenses ────────────────────────────
        let line9a = input.investment_interest_amt;
        let line9b = input.state_local_foreign_income_tax_amt;
        let line9c = input.misc_investment_expense_amt;
        let line9d = line9a + line9b + line9c;
        let line10 = input.additional_modification_amt;
        let line11 = line9d + line10;

        // ── Part III — Tax Computation ───────────────────────────────
        let line12 = (line8 - line11).max(Usd::ZERO);

        // ── Individuals (lines 13-17) ────────────────────────────────
        let line13 = input.modified_agi_amt;
        let line14 = Rules2025::niit_threshold(input.filing_status);
        let line15 = (line13 - line14).max(Usd::ZERO);
        let line16 = line12.min(line15);
        let line17 = Usd::from_cents(line16.cents() * niit_bps / 10_000);

        // ── Estates and Trusts (lines 18a-21) ────────────────────────
        let line18a = line12;
        let line18b = input.income_distribution_deduction_amt;
        let line18c = (line18a - line18b).max(Usd::ZERO);
        let line19a = input.adjusted_gross_income_amt;
        let line19b = Rules2025::ESTATE_TRUST_HIGHEST_BRACKET;
        let line19c = (line19a - line19b).max(Usd::ZERO);
        let line20 = line18c.min(line19c);

        Ok(Output8960 {
            // Checkboxes
            section6013g_ind: input.section6013g_ind,
            section6013h_ind: input.section6013h_ind,
            reg_section1141110g_ind: input.reg_section1141110g_ind,
            // Part I
            taxable_interest_amt: line1,
            ordinary_dividends_amt: line2,
            annuites_from_non_qlf_plans_amt: line3,
            net_rental_income_or_loss_amt: line4a,
            adj_net_incm_or_loss_non_sect1411_amt: line4b,
            rental_re_and_adj_net_incm_or_loss_amt: line4c,
            property_dispos_gain_or_loss_amt: line5a,
            non_niit_prop_dispos_gain_or_loss_amt: line5b,
            adj_from_dispos_of_stock_amt: line5c,
            gain_or_loss_from_dispos_amt: line5d,
            cfc_and_pfic_invst_incm_changes_amt: line6,
            other_investment_income_or_loss_amt: line7,
            total_income_amt: line8,
            // Part II
            investment_interest_amt: line9a,
            state_local_foreign_income_tax_amt: line9b,
            misc_investment_expense_amt: line9c,
            investment_expense_amt: line9d,
            additional_modification_amt: line10,
            total_deduction_modification_amt: line11,
            // Part III
            net_investment_income_amt: line12,
            // Individuals
            modified_agi_amt: line13,
            filing_threshold_amt: line14,
            magi_less_threshold_amt: line15,
            smllr_incm_or_magi_less_thrshld_amt: line16,
            indiv_net_invst_income_tax_amt: line17,
            // Estates and Trusts
            adjusted_gross_income_amt: line18a,
            income_distribution_deduction_amt: line18b,
            undistributed_net_income_amt: line18c,
            agi_less_trust_est_highest_tax_amt: line19a,
            trust_est_highest_tax_bracket_amt: line19b,
            smllr_undistr_incm_agi_less_tax_amt: line19c,
            est_trust_net_invst_income_tax_amt: line20,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[]
    }

    fn is_valid(&self) -> bool {
        let niit_bps = Rules2025::NIIT_RATE_BPS as i64;

        // Part I
        let line4c_ok = self.rental_re_and_adj_net_incm_or_loss_amt
            == self.net_rental_income_or_loss_amt + self.adj_net_incm_or_loss_non_sect1411_amt;
        let line5d_ok = self.gain_or_loss_from_dispos_amt
            == self.property_dispos_gain_or_loss_amt
                + self.non_niit_prop_dispos_gain_or_loss_amt
                + self.adj_from_dispos_of_stock_amt;
        let line8_ok = self.total_income_amt
            == self.taxable_interest_amt
                + self.ordinary_dividends_amt
                + self.annuites_from_non_qlf_plans_amt
                + self.rental_re_and_adj_net_incm_or_loss_amt
                + self.gain_or_loss_from_dispos_amt
                + self.cfc_and_pfic_invst_incm_changes_amt
                + self.other_investment_income_or_loss_amt;

        // Part II
        let line9d_ok = self.investment_expense_amt
            == self.investment_interest_amt
                + self.state_local_foreign_income_tax_amt
                + self.misc_investment_expense_amt;
        let line11_ok = self.total_deduction_modification_amt
            == self.investment_expense_amt + self.additional_modification_amt;

        // Part III
        let line12_ok = self.net_investment_income_amt
            == (self.total_income_amt - self.total_deduction_modification_amt).max(Usd::ZERO);

        // Individuals
        let line15_ok = self.magi_less_threshold_amt
            == (self.modified_agi_amt - self.filing_threshold_amt).max(Usd::ZERO);
        let line16_ok = self.smllr_incm_or_magi_less_thrshld_amt
            == self
                .net_investment_income_amt
                .min(self.magi_less_threshold_amt);
        let line17_ok = self.indiv_net_invst_income_tax_amt
            == Usd::from_cents(
                self.smllr_incm_or_magi_less_thrshld_amt.cents() * niit_bps / 10_000,
            );

        // Estates and Trusts
        let line18c_ok = self.undistributed_net_income_amt
            == (self.adjusted_gross_income_amt - self.income_distribution_deduction_amt)
                .max(Usd::ZERO);
        let line19c_ok = self.smllr_undistr_incm_agi_less_tax_amt
            == (self.agi_less_trust_est_highest_tax_amt - self.trust_est_highest_tax_bracket_amt)
                .max(Usd::ZERO);
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

    fn default_input() -> F8960Input {
        F8960Input {
            filing_status: FilingStatus::Single,
            section6013g_ind: false,
            section6013h_ind: false,
            reg_section1141110g_ind: false,
            taxable_interest_amt: Usd::ZERO,
            ordinary_dividends_amt: Usd::ZERO,
            annuites_from_non_qlf_plans_amt: Usd::ZERO,
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
            income_distribution_deduction_amt: Usd::ZERO,
            adjusted_gross_income_amt: Usd::ZERO,
        }
    }

    // ── must_file ────────────────────────────────────────────────────

    #[test]
    fn must_file_single_above_threshold() {
        let mut input = default_input();
        input.modified_agi_amt = Usd::from_dollars(250_000);
        assert!(Output8960::must_file(&input));
    }

    #[test]
    fn must_file_single_at_threshold() {
        let mut input = default_input();
        input.modified_agi_amt = Usd::from_dollars(200_000);
        assert!(!Output8960::must_file(&input));
    }

    #[test]
    fn must_file_mfj_threshold() {
        let mut input = default_input();
        input.filing_status = FilingStatus::MarriedFilingJointly;
        input.modified_agi_amt = Usd::from_dollars(250_001);
        assert!(Output8960::must_file(&input));
    }

    #[test]
    fn must_file_mfs_threshold() {
        let mut input = default_input();
        input.filing_status = FilingStatus::MarriedFilingSeparately;
        input.modified_agi_amt = Usd::from_dollars(125_001);
        assert!(Output8960::must_file(&input));
    }

    #[test]
    fn must_file_qss_uses_mfj_threshold() {
        let mut input = default_input();
        input.filing_status = FilingStatus::QualifyingSurvivingSpouse;
        // QSS uses MFJ threshold ($250k), not single ($200k)
        input.modified_agi_amt = Usd::from_dollars(225_000);
        assert!(!Output8960::must_file(&input));
    }

    // ── Part I — Investment Income ───────────────────────────────────

    #[test]
    fn part_i_total_investment_income() {
        let mut input = default_input();
        input.taxable_interest_amt = Usd::from_dollars(10_000);
        input.ordinary_dividends_amt = Usd::from_dollars(5_000);
        input.net_rental_income_or_loss_amt = Usd::from_dollars(20_000);
        input.property_dispos_gain_or_loss_amt = Usd::from_dollars(15_000);
        input.modified_agi_amt = Usd::from_dollars(300_000);
        let form = Output8960::try_new(input).unwrap();
        // line 8: 10k + 5k + 0 + 20k + 15k + 0 + 0 = 50k
        assert_eq!(form.total_income_amt, Usd::from_dollars(50_000));
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_line_4c_combines_4a_4b() {
        let mut input = default_input();
        input.net_rental_income_or_loss_amt = Usd::from_dollars(30_000);
        input.adj_net_incm_or_loss_non_sect1411_amt = Usd::from_cents(-1_000_000); // -$10,000
        input.modified_agi_amt = Usd::from_dollars(250_000);
        let form = Output8960::try_new(input).unwrap();
        assert_eq!(
            form.rental_re_and_adj_net_incm_or_loss_amt,
            Usd::from_dollars(20_000)
        );
        assert!(form.is_valid());
    }

    // ── Part III — Individual Tax Computation ────────────────────────

    #[test]
    fn individual_basic_niit() {
        let mut input = default_input();
        input.taxable_interest_amt = Usd::from_dollars(50_000);
        input.modified_agi_amt = Usd::from_dollars(250_000);
        let form = Output8960::try_new(input).unwrap();
        // line 12: 50,000
        assert_eq!(form.net_investment_income_amt, Usd::from_dollars(50_000));
        // line 14: $200,000 (Single threshold)
        assert_eq!(form.filing_threshold_amt, Usd::from_dollars(200_000));
        // line 15: 250,000 - 200,000 = 50,000
        assert_eq!(form.magi_less_threshold_amt, Usd::from_dollars(50_000));
        // line 16: min(50,000, 50,000) = 50,000
        assert_eq!(
            form.smllr_incm_or_magi_less_thrshld_amt,
            Usd::from_dollars(50_000)
        );
        // line 17: 50,000 * 3.8% = 1,900
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(1_900)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn individual_magi_excess_less_than_nii() {
        let mut input = default_input();
        input.taxable_interest_amt = Usd::from_dollars(100_000);
        input.modified_agi_amt = Usd::from_dollars(230_000);
        let form = Output8960::try_new(input).unwrap();
        // line 12: 100,000
        // line 15: 230,000 - 200,000 = 30,000
        // line 16: min(100,000, 30,000) = 30,000
        // line 17: 30,000 * 3.8% = 1,140
        assert_eq!(
            form.smllr_incm_or_magi_less_thrshld_amt,
            Usd::from_dollars(30_000)
        );
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(1_140)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn individual_below_threshold_no_tax() {
        let mut input = default_input();
        input.taxable_interest_amt = Usd::from_dollars(50_000);
        input.modified_agi_amt = Usd::from_dollars(180_000);
        let form = Output8960::try_new(input).unwrap();
        assert_eq!(form.magi_less_threshold_amt, Usd::ZERO);
        assert_eq!(form.smllr_incm_or_magi_less_thrshld_amt, Usd::ZERO);
        assert_eq!(form.indiv_net_invst_income_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn individual_expenses_reduce_nii() {
        let mut input = default_input();
        input.taxable_interest_amt = Usd::from_dollars(60_000);
        input.investment_interest_amt = Usd::from_dollars(10_000);
        input.modified_agi_amt = Usd::from_dollars(250_000);
        let form = Output8960::try_new(input).unwrap();
        // line 8: 60,000; line 11: 10,000; line 12: 50,000
        assert_eq!(form.net_investment_income_amt, Usd::from_dollars(50_000));
        // line 16: min(50,000, 50,000) = 50,000
        // line 17: 50,000 * 3.8% = 1,900
        assert_eq!(
            form.indiv_net_invst_income_tax_amt,
            Usd::from_dollars(1_900)
        );
        assert!(form.is_valid());
    }

    // ── Part III — Estate/Trust Tax Computation ──────────────────────

    #[test]
    fn estate_trust_basic() {
        let mut input = default_input();
        input.taxable_interest_amt = Usd::from_dollars(50_000);
        input.adjusted_gross_income_amt = Usd::from_dollars(50_000);
        input.income_distribution_deduction_amt = Usd::from_dollars(10_000);
        let form = Output8960::try_new(input).unwrap();
        // line 18a: 50,000 (net investment income)
        // line 18c: 50,000 - 10,000 = 40,000
        assert_eq!(form.undistributed_net_income_amt, Usd::from_dollars(40_000));
        // line 19a: 50,000; line 19b: 15,650
        // line 19c: 50,000 - 15,650 = 34,350
        assert_eq!(
            form.smllr_undistr_incm_agi_less_tax_amt,
            Usd::from_dollars(34_350)
        );
        // line 20: min(40,000, 34,350) = 34,350
        assert_eq!(
            form.est_trust_net_invst_income_tax_amt,
            Usd::from_dollars(34_350)
        );
        assert!(form.is_valid());
    }

    // ── Zero everything ──────────────────────────────────────────────

    #[test]
    fn zero_everything() {
        let form = Output8960::try_new(default_input()).unwrap();
        assert_eq!(form.total_income_amt, Usd::ZERO);
        assert_eq!(form.net_investment_income_amt, Usd::ZERO);
        assert_eq!(form.indiv_net_invst_income_tax_amt, Usd::ZERO);
        assert_eq!(form.est_trust_net_invst_income_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
