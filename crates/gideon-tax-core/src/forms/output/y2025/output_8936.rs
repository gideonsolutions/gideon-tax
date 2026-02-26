use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8936.
///
/// AGI, foreign earned income exclusions, and prior-year filing status feed
/// into Part I (MAGI).  Schedule A (Form 8936) credit amounts feed into
/// Parts II–V.  Tax-liability figures come from Form 1040.
#[derive(Debug, Clone)]
pub struct F8936Input {
    // -- Part I --
    /// Line 1a: Adjusted gross income (Form 1040 line 11a)
    pub adjusted_gross_income_amt: Usd,
    /// Line 1b: Excluded Puerto Rico income (section 933)
    pub excld_sect933_puerto_rico_incm_amt: Usd,
    /// Line 1c: Foreign earned income exclusion (Form 2555 line 45)
    pub gross_income_exclusion_amt: Usd,
    /// Line 1d: Housing deduction (Form 2555 line 50)
    pub housing_deduction_amt: Usd,
    /// Line 1e: Income exclusion (Form 4563 line 15)
    pub total_income_exclusion_amt: Usd,
    /// Line 5: Prior year filing status code
    pub py_indiv_return_filing_status_cd: String,

    // -- Part II --
    /// Line 6: Credit from Schedule A Part II (business/investment use)
    pub business_investment_use_amt: Usd,
    /// Line 7: New clean vehicle credit from partnerships/S corps
    pub new_clean_veh_cr_prtshp_s_corp_amt: Usd,

    // -- Part III --
    /// Line 9: Credit from Schedule A Part III (personal use)
    pub prsnl_use_new_clean_vehicle_cr_amt: Usd,
    /// Line 10: Total tax before credits (Form 1040 line 18)
    pub total_tax_before_cr_and_oth_taxes_amt: Usd,
    /// Line 11: Personal credits from Form 1040
    pub personal_tax_credits_amt: Usd,

    // -- Part IV --
    /// Line 14: Max credit from Schedule A Part IV (previously owned)
    pub max_prev_owned_clean_veh_cr_amt: Usd,
    /// Line 18: Previously owned vehicle credit (from Credit Limit Worksheet)
    pub prev_owned_clean_veh_credit_amt: Usd,

    // -- Part V --
    /// Line 19: Credit from Schedule A Part V (commercial)
    pub qlfy_cmrcl_clean_vehicle_cr_amt: Usd,
    /// Line 20: Commercial clean vehicle credit from partnerships/S corps
    pub cmrcl_clean_veh_cr_prtshp_s_corp_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8936 (2025) — Clean Vehicle Credits.
#[derive(Debug, Clone, Default)]
pub struct Output8936 {
    // -----------------------------------------------------------------------
    // Part I — Modified Adjusted Gross Income (MAGI) Amount
    // -----------------------------------------------------------------------
    /// Line 1a: Amount from line 11a of your 2025 Form 1040, 1040-SR, or 1040-NR
    pub adjusted_gross_income_amt: Usd,
    /// Line 1b: Any income from Puerto Rico you excluded
    pub excld_sect933_puerto_rico_incm_amt: Usd,
    /// Line 1c: Any amount from Form 2555, line 45
    pub gross_income_exclusion_amt: Usd,
    /// Line 1d: Any amount from Form 2555, line 50
    pub housing_deduction_amt: Usd,
    /// Line 1e: Any amount from Form 4563, line 15
    pub total_income_exclusion_amt: Usd,
    /// Line 2: Add lines 1a through 1e (current year MAGI)
    pub net_income_amt: Usd,
    /// Line 5: Prior year filing status code (from 2024 return)
    pub py_indiv_return_filing_status_cd: String,

    // -----------------------------------------------------------------------
    // Part II — Credit for Business/Investment Use Part of New Clean Vehicles
    // -----------------------------------------------------------------------
    /// Line 6: Total credit amount from Part II of Schedule(s) A (Form 8936)
    pub business_investment_use_amt: Usd,
    /// Line 7: New clean vehicle credit from partnerships and S corporations
    pub new_clean_veh_cr_prtshp_s_corp_amt: Usd,
    /// Line 8: Business/investment use part of credit (add lines 6 and 7)
    pub business_invst_use_part_of_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Credit for Personal Use Part of New Clean Vehicles
    // -----------------------------------------------------------------------
    /// Line 9: Total credit amount from Part III of Schedule(s) A (Form 8936)
    pub prsnl_use_new_clean_vehicle_cr_amt: Usd,
    /// Line 10: Amount from Form 1040, 1040-SR, or 1040-NR, line 18
    pub total_tax_before_cr_and_oth_taxes_amt: Usd,
    /// Line 11: Personal credits from Form 1040, 1040-SR, or 1040-NR
    pub personal_tax_credits_amt: Usd,
    /// Line 12: Subtract line 11 from line 10 (if zero or less, enter -0-)
    pub adjusted_personal_tax_credits_amt: Usd,
    /// Line 13: Personal use part of credit (smaller of line 9 or line 12)
    pub clean_veh_prsnl_use_part_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Credit for Previously Owned Clean Vehicles
    // -----------------------------------------------------------------------
    /// Line 14: Total credit amount from Part IV of Schedule(s) A (Form 8936)
    pub max_prev_owned_clean_veh_cr_amt: Usd,
    /// Line 18: Previously owned clean vehicle credit (smaller of line 14 or line 17)
    pub prev_owned_clean_veh_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Credit for Qualified Commercial Clean Vehicles
    // -----------------------------------------------------------------------
    /// Line 19: Total credit amount from Part V of Schedule(s) A (Form 8936)
    pub qlfy_cmrcl_clean_vehicle_cr_amt: Usd,
    /// Line 20: Qualified commercial clean vehicle credit from partnerships and S corporations
    pub cmrcl_clean_veh_cr_prtshp_s_corp_amt: Usd,
    /// Line 21: Add lines 19 and 20 (total qualified commercial clean vehicle credit)
    pub total_qlfy_cmrcl_clean_veh_cr_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8936 {
    fn name() -> &'static str {
        "Form 8936"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8936 {
    type Input = F8936Input;

    fn must_file(input: &Self::Input) -> bool {
        // Must file if any vehicle credits exist across any part
        input.business_investment_use_amt > Usd::ZERO
            || input.new_clean_veh_cr_prtshp_s_corp_amt > Usd::ZERO
            || input.prsnl_use_new_clean_vehicle_cr_amt > Usd::ZERO
            || input.max_prev_owned_clean_veh_cr_amt > Usd::ZERO
            || input.prev_owned_clean_veh_credit_amt > Usd::ZERO
            || input.qlfy_cmrcl_clean_vehicle_cr_amt > Usd::ZERO
            || input.cmrcl_clean_veh_cr_prtshp_s_corp_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // -- Part I: MAGI --
        // Line 2 = 1a + 1b + 1c + 1d + 1e
        let line2 = input.adjusted_gross_income_amt
            + input.excld_sect933_puerto_rico_incm_amt
            + input.gross_income_exclusion_amt
            + input.housing_deduction_amt
            + input.total_income_exclusion_amt;

        // -- Part II: Business/Investment Use --
        // Line 8 = Line 6 + Line 7
        let line8 = input.business_investment_use_amt + input.new_clean_veh_cr_prtshp_s_corp_amt;

        // -- Part III: Personal Use --
        // Line 12 = max(Line 10 - Line 11, 0)
        let line12 = (input.total_tax_before_cr_and_oth_taxes_amt
            - input.personal_tax_credits_amt)
            .max(Usd::ZERO);
        // Line 13 = min(Line 9, Line 12)
        let line13 = input.prsnl_use_new_clean_vehicle_cr_amt.min(line12);

        // -- Part IV: Previously Owned --
        // Line 18: input (from Credit Limit Worksheet), but cannot exceed
        // Line 14
        let line18 = input
            .prev_owned_clean_veh_credit_amt
            .min(input.max_prev_owned_clean_veh_cr_amt);

        // -- Part V: Commercial --
        // Line 21 = Line 19 + Line 20
        let line21 =
            input.qlfy_cmrcl_clean_vehicle_cr_amt + input.cmrcl_clean_veh_cr_prtshp_s_corp_amt;

        Ok(Output8936 {
            // Part I
            adjusted_gross_income_amt: input.adjusted_gross_income_amt,
            excld_sect933_puerto_rico_incm_amt: input.excld_sect933_puerto_rico_incm_amt,
            gross_income_exclusion_amt: input.gross_income_exclusion_amt,
            housing_deduction_amt: input.housing_deduction_amt,
            total_income_exclusion_amt: input.total_income_exclusion_amt,
            net_income_amt: line2,
            py_indiv_return_filing_status_cd: input.py_indiv_return_filing_status_cd,
            // Part II
            business_investment_use_amt: input.business_investment_use_amt,
            new_clean_veh_cr_prtshp_s_corp_amt: input.new_clean_veh_cr_prtshp_s_corp_amt,
            business_invst_use_part_of_cr_amt: line8,
            // Part III
            prsnl_use_new_clean_vehicle_cr_amt: input.prsnl_use_new_clean_vehicle_cr_amt,
            total_tax_before_cr_and_oth_taxes_amt: input.total_tax_before_cr_and_oth_taxes_amt,
            personal_tax_credits_amt: input.personal_tax_credits_amt,
            adjusted_personal_tax_credits_amt: line12,
            clean_veh_prsnl_use_part_cr_amt: line13,
            // Part IV
            max_prev_owned_clean_veh_cr_amt: input.max_prev_owned_clean_veh_cr_amt,
            prev_owned_clean_veh_credit_amt: line18,
            // Part V
            qlfy_cmrcl_clean_vehicle_cr_amt: input.qlfy_cmrcl_clean_vehicle_cr_amt,
            cmrcl_clean_veh_cr_prtshp_s_corp_amt: input.cmrcl_clean_veh_cr_prtshp_s_corp_amt,
            total_qlfy_cmrcl_clean_veh_cr_amt: line21,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040, DynForm::F2555, DynForm::F8936ScheduleA]
    }

    fn is_valid(&self) -> bool {
        // Part I: Line 2 = 1a + 1b + 1c + 1d + 1e
        let line2_ok = self.net_income_amt
            == self.adjusted_gross_income_amt
                + self.excld_sect933_puerto_rico_incm_amt
                + self.gross_income_exclusion_amt
                + self.housing_deduction_amt
                + self.total_income_exclusion_amt;

        // Part II: Line 8 = Line 6 + Line 7
        let line8_ok = self.business_invst_use_part_of_cr_amt
            == self.business_investment_use_amt + self.new_clean_veh_cr_prtshp_s_corp_amt;

        // Part III: Line 12 = max(Line 10 - Line 11, 0)
        let line12_ok = self.adjusted_personal_tax_credits_amt
            == (self.total_tax_before_cr_and_oth_taxes_amt - self.personal_tax_credits_amt)
                .max(Usd::ZERO);

        // Part III: Line 13 = min(Line 9, Line 12)
        let line13_ok = self.clean_veh_prsnl_use_part_cr_amt
            == self
                .prsnl_use_new_clean_vehicle_cr_amt
                .min(self.adjusted_personal_tax_credits_amt);

        // Part IV: Line 18 <= Line 14
        let line18_ok = self.prev_owned_clean_veh_credit_amt <= self.max_prev_owned_clean_veh_cr_amt;

        // Part V: Line 21 = Line 19 + Line 20
        let line21_ok = self.total_qlfy_cmrcl_clean_veh_cr_amt
            == self.qlfy_cmrcl_clean_vehicle_cr_amt + self.cmrcl_clean_veh_cr_prtshp_s_corp_amt;

        line2_ok && line8_ok && line12_ok && line13_ok && line18_ok && line21_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F8936Input {
        F8936Input {
            adjusted_gross_income_amt: Usd::from_dollars(100_000),
            excld_sect933_puerto_rico_incm_amt: Usd::ZERO,
            gross_income_exclusion_amt: Usd::ZERO,
            housing_deduction_amt: Usd::ZERO,
            total_income_exclusion_amt: Usd::ZERO,
            py_indiv_return_filing_status_cd: "1".to_string(),
            business_investment_use_amt: Usd::ZERO,
            new_clean_veh_cr_prtshp_s_corp_amt: Usd::ZERO,
            prsnl_use_new_clean_vehicle_cr_amt: Usd::ZERO,
            total_tax_before_cr_and_oth_taxes_amt: Usd::from_dollars(15_000),
            personal_tax_credits_amt: Usd::from_dollars(2_000),
            max_prev_owned_clean_veh_cr_amt: Usd::ZERO,
            prev_owned_clean_veh_credit_amt: Usd::ZERO,
            qlfy_cmrcl_clean_vehicle_cr_amt: Usd::ZERO,
            cmrcl_clean_veh_cr_prtshp_s_corp_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_no_credits() {
        let input = basic_input();
        assert!(!Output8936::must_file(&input));
    }

    #[test]
    fn must_file_with_business_credit() {
        let mut input = basic_input();
        input.business_investment_use_amt = Usd::from_dollars(3_750);
        assert!(Output8936::must_file(&input));
    }

    #[test]
    fn must_file_with_personal_credit() {
        let mut input = basic_input();
        input.prsnl_use_new_clean_vehicle_cr_amt = Usd::from_dollars(7_500);
        assert!(Output8936::must_file(&input));
    }

    #[test]
    fn must_file_with_prev_owned_credit() {
        let mut input = basic_input();
        input.max_prev_owned_clean_veh_cr_amt = Usd::from_dollars(4_000);
        assert!(Output8936::must_file(&input));
    }

    #[test]
    fn must_file_with_commercial_credit() {
        let mut input = basic_input();
        input.qlfy_cmrcl_clean_vehicle_cr_amt = Usd::from_dollars(7_500);
        assert!(Output8936::must_file(&input));
    }

    #[test]
    fn part1_magi_computation() {
        let mut input = basic_input();
        input.adjusted_gross_income_amt = Usd::from_dollars(80_000);
        input.excld_sect933_puerto_rico_incm_amt = Usd::from_dollars(5_000);
        input.gross_income_exclusion_amt = Usd::from_dollars(10_000);
        input.housing_deduction_amt = Usd::from_dollars(3_000);
        input.total_income_exclusion_amt = Usd::from_dollars(2_000);
        let form = Output8936::try_new(input).unwrap();
        // Line 2: 80,000 + 5,000 + 10,000 + 3,000 + 2,000 = 100,000
        assert_eq!(form.net_income_amt, Usd::from_dollars(100_000));
        assert!(form.is_valid());
    }

    #[test]
    fn part2_business_investment_credit() {
        let mut input = basic_input();
        input.business_investment_use_amt = Usd::from_dollars(3_750);
        input.new_clean_veh_cr_prtshp_s_corp_amt = Usd::from_dollars(1_250);
        let form = Output8936::try_new(input).unwrap();
        // Line 8: 3,750 + 1,250 = 5,000
        assert_eq!(
            form.business_invst_use_part_of_cr_amt,
            Usd::from_dollars(5_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part3_personal_use_credit_limited_by_tax() {
        let mut input = basic_input();
        input.prsnl_use_new_clean_vehicle_cr_amt = Usd::from_dollars(7_500);
        input.total_tax_before_cr_and_oth_taxes_amt = Usd::from_dollars(15_000);
        input.personal_tax_credits_amt = Usd::from_dollars(2_000);
        let form = Output8936::try_new(input).unwrap();
        // Line 12: max(15,000 - 2,000, 0) = 13,000
        assert_eq!(
            form.adjusted_personal_tax_credits_amt,
            Usd::from_dollars(13_000)
        );
        // Line 13: min(7,500, 13,000) = 7,500
        assert_eq!(
            form.clean_veh_prsnl_use_part_cr_amt,
            Usd::from_dollars(7_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part3_personal_use_credit_capped_by_liability() {
        let mut input = basic_input();
        input.prsnl_use_new_clean_vehicle_cr_amt = Usd::from_dollars(7_500);
        input.total_tax_before_cr_and_oth_taxes_amt = Usd::from_dollars(5_000);
        input.personal_tax_credits_amt = Usd::from_dollars(2_000);
        let form = Output8936::try_new(input).unwrap();
        // Line 12: max(5,000 - 2,000, 0) = 3,000
        assert_eq!(
            form.adjusted_personal_tax_credits_amt,
            Usd::from_dollars(3_000)
        );
        // Line 13: min(7,500, 3,000) = 3,000
        assert_eq!(
            form.clean_veh_prsnl_use_part_cr_amt,
            Usd::from_dollars(3_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part3_personal_credits_exceed_tax() {
        let mut input = basic_input();
        input.prsnl_use_new_clean_vehicle_cr_amt = Usd::from_dollars(7_500);
        input.total_tax_before_cr_and_oth_taxes_amt = Usd::from_dollars(1_000);
        input.personal_tax_credits_amt = Usd::from_dollars(3_000);
        let form = Output8936::try_new(input).unwrap();
        // Line 12: max(1,000 - 3,000, 0) = 0
        assert_eq!(form.adjusted_personal_tax_credits_amt, Usd::ZERO);
        // Line 13: min(7,500, 0) = 0
        assert_eq!(form.clean_veh_prsnl_use_part_cr_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn part4_previously_owned_credit() {
        let mut input = basic_input();
        input.max_prev_owned_clean_veh_cr_amt = Usd::from_dollars(4_000);
        input.prev_owned_clean_veh_credit_amt = Usd::from_dollars(3_500);
        let form = Output8936::try_new(input).unwrap();
        // Line 18: min(3,500, 4,000) = 3,500
        assert_eq!(
            form.prev_owned_clean_veh_credit_amt,
            Usd::from_dollars(3_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part4_previously_owned_credit_capped_at_line14() {
        let mut input = basic_input();
        input.max_prev_owned_clean_veh_cr_amt = Usd::from_dollars(4_000);
        input.prev_owned_clean_veh_credit_amt = Usd::from_dollars(5_000);
        let form = Output8936::try_new(input).unwrap();
        // Line 18: min(5,000, 4,000) = 4,000
        assert_eq!(
            form.prev_owned_clean_veh_credit_amt,
            Usd::from_dollars(4_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part5_commercial_credit() {
        let mut input = basic_input();
        input.qlfy_cmrcl_clean_vehicle_cr_amt = Usd::from_dollars(7_500);
        input.cmrcl_clean_veh_cr_prtshp_s_corp_amt = Usd::from_dollars(2_500);
        let form = Output8936::try_new(input).unwrap();
        // Line 21: 7,500 + 2,500 = 10,000
        assert_eq!(
            form.total_qlfy_cmrcl_clean_veh_cr_amt,
            Usd::from_dollars(10_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn all_parts_combined() {
        let input = F8936Input {
            adjusted_gross_income_amt: Usd::from_dollars(80_000),
            excld_sect933_puerto_rico_incm_amt: Usd::from_dollars(5_000),
            gross_income_exclusion_amt: Usd::from_dollars(10_000),
            housing_deduction_amt: Usd::from_dollars(3_000),
            total_income_exclusion_amt: Usd::from_dollars(2_000),
            py_indiv_return_filing_status_cd: "2".to_string(),
            business_investment_use_amt: Usd::from_dollars(3_750),
            new_clean_veh_cr_prtshp_s_corp_amt: Usd::from_dollars(1_250),
            prsnl_use_new_clean_vehicle_cr_amt: Usd::from_dollars(7_500),
            total_tax_before_cr_and_oth_taxes_amt: Usd::from_dollars(20_000),
            personal_tax_credits_amt: Usd::from_dollars(5_000),
            max_prev_owned_clean_veh_cr_amt: Usd::from_dollars(4_000),
            prev_owned_clean_veh_credit_amt: Usd::from_dollars(4_000),
            qlfy_cmrcl_clean_vehicle_cr_amt: Usd::from_dollars(7_500),
            cmrcl_clean_veh_cr_prtshp_s_corp_amt: Usd::from_dollars(2_500),
        };
        let form = Output8936::try_new(input).unwrap();
        // Part I: MAGI = 80,000 + 5,000 + 10,000 + 3,000 + 2,000 = 100,000
        assert_eq!(form.net_income_amt, Usd::from_dollars(100_000));
        // Part II: Line 8 = 3,750 + 1,250 = 5,000
        assert_eq!(
            form.business_invst_use_part_of_cr_amt,
            Usd::from_dollars(5_000)
        );
        // Part III: Line 12 = max(20,000 - 5,000, 0) = 15,000
        assert_eq!(
            form.adjusted_personal_tax_credits_amt,
            Usd::from_dollars(15_000)
        );
        // Part III: Line 13 = min(7,500, 15,000) = 7,500
        assert_eq!(
            form.clean_veh_prsnl_use_part_cr_amt,
            Usd::from_dollars(7_500)
        );
        // Part IV: Line 18 = min(4,000, 4,000) = 4,000
        assert_eq!(
            form.prev_owned_clean_veh_credit_amt,
            Usd::from_dollars(4_000)
        );
        // Part V: Line 21 = 7,500 + 2,500 = 10,000
        assert_eq!(
            form.total_qlfy_cmrcl_clean_veh_cr_amt,
            Usd::from_dollars(10_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn zero_everything() {
        let mut input = basic_input();
        input.adjusted_gross_income_amt = Usd::ZERO;
        input.total_tax_before_cr_and_oth_taxes_amt = Usd::ZERO;
        input.personal_tax_credits_amt = Usd::ZERO;
        let form = Output8936::try_new(input).unwrap();
        assert_eq!(form.net_income_amt, Usd::ZERO);
        assert_eq!(form.business_invst_use_part_of_cr_amt, Usd::ZERO);
        assert_eq!(form.adjusted_personal_tax_credits_amt, Usd::ZERO);
        assert_eq!(form.clean_veh_prsnl_use_part_cr_amt, Usd::ZERO);
        assert_eq!(form.prev_owned_clean_veh_credit_amt, Usd::ZERO);
        assert_eq!(form.total_qlfy_cmrcl_clean_veh_cr_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn filing_status_passthrough() {
        let mut input = basic_input();
        input.py_indiv_return_filing_status_cd = "4".to_string();
        let form = Output8936::try_new(input).unwrap();
        assert_eq!(form.py_indiv_return_filing_status_cd, "4");
        assert!(form.is_valid());
    }

    #[test]
    fn dependencies_are_correct() {
        let deps = Output8936::dependencies();
        assert!(deps.contains(&DynForm::F1040));
        assert!(deps.contains(&DynForm::F2555));
        assert!(deps.contains(&DynForm::F8936ScheduleA));
        assert_eq!(deps.len(), 3);
    }

    #[test]
    fn form_name() {
        assert_eq!(Output8936::name(), "Form 8936");
    }
}
