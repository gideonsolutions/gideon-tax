use us_tax_brackets::{FilingStatus, TaxYear};

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::rules::TaxYearRules;
use crate::rules::y2025::Rules2025;
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8959.
///
/// W-2 data (box 5 Medicare wages, box 6 Medicare withholding), unreported
/// tips from Form 4137 line 6, wages from Form 8919 line 6,
/// self-employment income from Schedule SE, and RRTA compensation feed
/// into the computation; the corresponding dependencies are declared in
/// [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct F8959Input {
    /// Filing status (determines the Additional Medicare Tax threshold)
    pub filing_status: FilingStatus,
    /// Line 1: Medicare wages and tips from Form(s) W-2, box 5
    pub w2_medicare_wages_and_tips_amt: Usd,
    /// Line 2: Unreported tips from Form 4137, line 6
    pub unreported_medicare_tips_amt: Usd,
    /// Line 3: Wages from Form 8919, line 6
    pub wages_with_no_withholding_amt: Usd,
    /// Line 8: Self-employment income from Schedule SE, Part I, line 6
    pub self_employment_income_amt: Usd,
    /// Line 14: Railroad retirement (RRTA) compensation and tips
    pub railroad_retirement_comp_amt: Usd,
    /// Line 19: Medicare tax withheld from Form(s) W-2, box 6
    pub w2_medicare_tax_withheld_amt: Usd,
    /// Line 23: Additional Medicare Tax withholding on RRTA compensation
    pub w2_addl_rrt_tax_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8959 (2025) — Additional Medicare Tax.
#[derive(Debug, Clone, Default)]
pub struct Output8959 {
    // -----------------------------------------------------------------------
    // Part I — Additional Medicare Tax on Medicare Wages
    // -----------------------------------------------------------------------
    /// Line 1: Medicare wages and tips from Form W-2, box 5
    pub total_w2_medicare_wages_and_tips_amt: Usd,
    /// Line 2: Unreported tips from Form 4137, line 6
    pub total_unreported_medicare_tips_amt: Usd,
    /// Line 3: Wages from Form 8919, line 6
    pub total_wages_with_no_withholding_amt: Usd,
    /// Line 4: Add lines 1 through 3
    pub total_medicare_wages_and_tips_amt: Usd,
    /// Line 5: Filing status threshold amount
    pub filing_status_threshold_amt: Usd,
    /// Line 6: Subtract line 5 from line 4 (if zero or less, enter -0-)
    pub wages_tips_subj_to_addl_medcr_tax_amt: Usd,
    /// Line 7: Additional Medicare Tax on Medicare wages (multiply line 6 by 0.9%)
    pub additional_medicare_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Additional Medicare Tax on Self-Employment Income
    // -----------------------------------------------------------------------
    /// Line 8: Self-employment income from Schedule SE, Part I, line 6
    pub total_self_employment_income_amt: Usd,
    /// Line 9: Filing status threshold amount (same as line 5)
    pub se_filing_status_threshold_amt: Usd,
    /// Line 10: Amount from line 4
    pub se_income_subj_to_add_se_tax_amt: Usd,
    /// Line 11: Subtract line 10 from line 9. If zero or less, enter -0-
    pub se_reduced_threshold_amt: Usd,
    /// Line 12: Subtract line 11 from line 8. If zero or less, enter -0-
    pub se_income_above_threshold_amt: Usd,
    /// Line 13: Additional Medicare Tax on self-employment income (multiply line 12 by 0.9%)
    pub addl_self_employment_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Additional Medicare Tax on Railroad Retirement Tax Act (RRTA) Compensation
    // -----------------------------------------------------------------------
    /// Line 14: Railroad retirement (RRTA) compensation and tips from Form W-2, box 14
    pub total_railroad_retirement_comp_amt: Usd,
    /// Line 15: Filing status threshold amount (same as line 5)
    pub rrta_filing_status_threshold_amt: Usd,
    /// Line 16: Subtract line 15 from line 14 (if zero or less, enter -0-)
    pub rrt_comp_subj_to_add_rrt_tax_amt: Usd,
    /// Line 17: Additional Medicare Tax on RRTA compensation (multiply line 16 by 0.9%)
    pub addl_railroad_retirement_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Total Additional Medicare Tax
    // -----------------------------------------------------------------------
    /// Line 18: Add lines 7, 13, and 17 (total Additional Medicare Tax)
    pub total_amrrt_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Withholding Reconciliation
    // -----------------------------------------------------------------------
    /// Line 19: Medicare tax withheld from Form W-2, box 6
    pub total_w2_medicare_tax_withheld_amt: Usd,
    /// Line 20: Amount from line 1 (Medicare wages and tips)
    pub total_medicare_tax_amt: Usd,
    /// Line 21: Multiply line 20 by 1.45% (regular Medicare tax withholding)
    pub addnl_medicare_tax_withholding_amt: Usd,
    /// Line 22: Subtract line 21 from line 19 (Additional Medicare Tax withholding on Medicare wages)
    pub addl_medcr_rrt_tax_withholding_amt: Usd,
    /// Line 23: Additional Medicare Tax withholding on RRTA compensation from Form W-2, box 14
    pub total_w2_addl_rrt_tax_amt: Usd,
    /// Line 24: Total Additional Medicare Tax withholding (add lines 22 and 23)
    pub total_addl_medcr_tax_withholding_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8959 {
    fn name() -> &'static str {
        "Form 8959"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8959 {
    type Input = F8959Input;

    fn must_file(input: &Self::Input) -> bool {
        let threshold = Rules2025::additional_medicare_threshold(input.filing_status);

        let line4 = input.w2_medicare_wages_and_tips_amt
            + input.unreported_medicare_tips_amt
            + input.wages_with_no_withholding_amt;

        line4 + input.self_employment_income_amt > threshold
            || input.railroad_retirement_comp_amt > threshold
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        let threshold = Rules2025::additional_medicare_threshold(input.filing_status);
        let add_med_bps = Rules2025::ADDITIONAL_MEDICARE_RATE_BPS as i64;
        let med_bps = Rules2025::MEDICARE_RATE_BPS as i64;

        // ── Part I ──────────────────────────────────────────────────
        let line1 = input.w2_medicare_wages_and_tips_amt;
        let line2 = input.unreported_medicare_tips_amt;
        let line3 = input.wages_with_no_withholding_amt;
        let line4 = line1 + line2 + line3;
        let line6 = (line4 - threshold).max(Usd::ZERO);
        let line7 = Usd::from_cents(line6.cents() * add_med_bps / 10_000);

        // ── Part II ─────────────────────────────────────────────────
        let line8 = input.self_employment_income_amt;
        let line10 = line4;
        let line11 = (threshold - line4).max(Usd::ZERO);
        let line12 = (line8 - line11).max(Usd::ZERO);
        let line13 = Usd::from_cents(line12.cents() * add_med_bps / 10_000);

        // ── Part III ────────────────────────────────────────────────
        let line14 = input.railroad_retirement_comp_amt;
        let line16 = (line14 - threshold).max(Usd::ZERO);
        let line17 = Usd::from_cents(line16.cents() * add_med_bps / 10_000);

        // ── Part IV ─────────────────────────────────────────────────
        let line18 = line7 + line13 + line17;

        // ── Part V ──────────────────────────────────────────────────
        let line19 = input.w2_medicare_tax_withheld_amt;
        let line20 = line1;
        let line21 = Usd::from_cents(line20.cents() * med_bps / 10_000);
        let line22 = (line19 - line21).max(Usd::ZERO);
        let line23 = input.w2_addl_rrt_tax_amt;
        let line24 = line22 + line23;

        Ok(Output8959 {
            // Part I
            total_w2_medicare_wages_and_tips_amt: line1,
            total_unreported_medicare_tips_amt: line2,
            total_wages_with_no_withholding_amt: line3,
            total_medicare_wages_and_tips_amt: line4,
            filing_status_threshold_amt: threshold,
            wages_tips_subj_to_addl_medcr_tax_amt: line6,
            additional_medicare_tax_amt: line7,
            // Part II
            total_self_employment_income_amt: line8,
            se_filing_status_threshold_amt: threshold,
            se_income_subj_to_add_se_tax_amt: line10,
            se_reduced_threshold_amt: line11,
            se_income_above_threshold_amt: line12,
            addl_self_employment_tax_amt: line13,
            // Part III
            total_railroad_retirement_comp_amt: line14,
            rrta_filing_status_threshold_amt: threshold,
            rrt_comp_subj_to_add_rrt_tax_amt: line16,
            addl_railroad_retirement_tax_amt: line17,
            // Part IV
            total_amrrt_tax_amt: line18,
            // Part V
            total_w2_medicare_tax_withheld_amt: line19,
            total_medicare_tax_amt: line20,
            addnl_medicare_tax_withholding_amt: line21,
            addl_medcr_rrt_tax_withholding_amt: line22,
            total_w2_addl_rrt_tax_amt: line23,
            total_addl_medcr_tax_withholding_amt: line24,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[
            DynForm::W2,
            DynForm::F4137,
            DynForm::F8919,
            DynForm::ScheduleSe,
        ]
    }

    fn is_valid(&self) -> bool {
        let add_med_bps = Rules2025::ADDITIONAL_MEDICARE_RATE_BPS as i64;
        let med_bps = Rules2025::MEDICARE_RATE_BPS as i64;
        let threshold = self.filing_status_threshold_amt;

        // All three threshold fields must agree
        let thresholds_ok = self.se_filing_status_threshold_amt == threshold
            && self.rrta_filing_status_threshold_amt == threshold;

        // Part I
        let line4 = self.total_w2_medicare_wages_and_tips_amt
            + self.total_unreported_medicare_tips_amt
            + self.total_wages_with_no_withholding_amt;
        let line4_ok = self.total_medicare_wages_and_tips_amt == line4;
        let line6_ok =
            self.wages_tips_subj_to_addl_medcr_tax_amt == (line4 - threshold).max(Usd::ZERO);
        let line7_ok = self.additional_medicare_tax_amt
            == Usd::from_cents(
                self.wages_tips_subj_to_addl_medcr_tax_amt.cents() * add_med_bps / 10_000,
            );

        // Part II
        let line10_ok = self.se_income_subj_to_add_se_tax_amt == line4;
        let line11_ok = self.se_reduced_threshold_amt == (threshold - line4).max(Usd::ZERO);
        let line12_ok = self.se_income_above_threshold_amt
            == (self.total_self_employment_income_amt - self.se_reduced_threshold_amt)
                .max(Usd::ZERO);
        let line13_ok = self.addl_self_employment_tax_amt
            == Usd::from_cents(self.se_income_above_threshold_amt.cents() * add_med_bps / 10_000);

        // Part III
        let line16_ok = self.rrt_comp_subj_to_add_rrt_tax_amt
            == (self.total_railroad_retirement_comp_amt - threshold).max(Usd::ZERO);
        let line17_ok = self.addl_railroad_retirement_tax_amt
            == Usd::from_cents(
                self.rrt_comp_subj_to_add_rrt_tax_amt.cents() * add_med_bps / 10_000,
            );

        // Part IV
        let line18_ok = self.total_amrrt_tax_amt
            == self.additional_medicare_tax_amt
                + self.addl_self_employment_tax_amt
                + self.addl_railroad_retirement_tax_amt;

        // Part V
        let line20_ok = self.total_medicare_tax_amt == self.total_w2_medicare_wages_and_tips_amt;
        let line21_ok = self.addnl_medicare_tax_withholding_amt
            == Usd::from_cents(self.total_medicare_tax_amt.cents() * med_bps / 10_000);
        let line22_ok = self.addl_medcr_rrt_tax_withholding_amt
            == (self.total_w2_medicare_tax_withheld_amt - self.addnl_medicare_tax_withholding_amt)
                .max(Usd::ZERO);
        let line24_ok = self.total_addl_medcr_tax_withholding_amt
            == self.addl_medcr_rrt_tax_withholding_amt + self.total_w2_addl_rrt_tax_amt;

        thresholds_ok
            && line4_ok
            && line6_ok
            && line7_ok
            && line10_ok
            && line11_ok
            && line12_ok
            && line13_ok
            && line16_ok
            && line17_ok
            && line18_ok
            && line20_ok
            && line21_ok
            && line22_ok
            && line24_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn single_input() -> F8959Input {
        F8959Input {
            filing_status: FilingStatus::Single,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(250_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::ZERO,
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::from_cents(362_500), // 250,000 * 1.45%
            w2_addl_rrt_tax_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_wages_exceed_threshold() {
        assert!(Output8959::must_file(&single_input()));
    }

    #[test]
    fn must_file_below_threshold() {
        let mut input = single_input();
        input.w2_medicare_wages_and_tips_amt = Usd::from_dollars(150_000);
        // line4 (150k) + se (0) = 150k <= 200k threshold
        assert!(!Output8959::must_file(&input));
    }

    #[test]
    fn must_file_se_income_exceeds_threshold() {
        let mut input = single_input();
        input.w2_medicare_wages_and_tips_amt = Usd::ZERO;
        // line4 (0) + se (250k) = 250k > 200k threshold
        input.self_employment_income_amt = Usd::from_dollars(250_000);
        assert!(Output8959::must_file(&input));
    }

    #[test]
    fn must_file_wages_plus_se_exceed_threshold() {
        let mut input = single_input();
        // Neither alone exceeds 200k, but combined they do
        input.w2_medicare_wages_and_tips_amt = Usd::from_dollars(120_000);
        input.self_employment_income_amt = Usd::from_dollars(100_000);
        // line4 (120k) + se (100k) = 220k > 200k
        assert!(Output8959::must_file(&input));
    }

    #[test]
    fn must_file_wages_plus_se_below_threshold() {
        let mut input = single_input();
        input.w2_medicare_wages_and_tips_amt = Usd::from_dollars(100_000);
        input.self_employment_income_amt = Usd::from_dollars(50_000);
        // line4 (100k) + se (50k) = 150k <= 200k
        assert!(!Output8959::must_file(&input));
    }

    #[test]
    fn must_file_rrta_exceeds_threshold() {
        let mut input = single_input();
        input.w2_medicare_wages_and_tips_amt = Usd::ZERO;
        input.railroad_retirement_comp_amt = Usd::from_dollars(250_000);
        assert!(Output8959::must_file(&input));
    }

    #[test]
    fn basic_single_wages_only() {
        let form = Output8959::try_new(single_input()).unwrap();
        assert_eq!(
            form.total_medicare_wages_and_tips_amt,
            Usd::from_dollars(250_000)
        );
        // Line 5: Single threshold = $200,000
        assert_eq!(form.filing_status_threshold_amt, Usd::from_dollars(200_000));
        // Line 6: 250,000 - 200,000 = 50,000
        assert_eq!(
            form.wages_tips_subj_to_addl_medcr_tax_amt,
            Usd::from_dollars(50_000)
        );
        // Line 7: 50,000 * 0.9% = 450
        assert_eq!(form.additional_medicare_tax_amt, Usd::from_dollars(450));
        assert_eq!(form.addl_self_employment_tax_amt, Usd::ZERO);
        assert_eq!(form.addl_railroad_retirement_tax_amt, Usd::ZERO);
        assert_eq!(form.total_amrrt_tax_amt, Usd::from_dollars(450));
        // Line 21: 250,000 * 1.45% = 3,625
        assert_eq!(
            form.addnl_medicare_tax_withholding_amt,
            Usd::from_dollars(3_625)
        );
        // Line 22: 3,625 - 3,625 = 0
        assert_eq!(form.addl_medcr_rrt_tax_withholding_amt, Usd::ZERO);
        assert_eq!(form.total_addl_medcr_tax_withholding_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn mfj_threshold() {
        let input = F8959Input {
            filing_status: FilingStatus::MarriedFilingJointly,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(300_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::ZERO,
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::from_cents(435_000),
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.filing_status_threshold_amt, Usd::from_dollars(250_000));
        assert_eq!(
            form.wages_tips_subj_to_addl_medcr_tax_amt,
            Usd::from_dollars(50_000)
        );
        assert_eq!(form.additional_medicare_tax_amt, Usd::from_dollars(450));
        assert!(form.is_valid());
    }

    #[test]
    fn mfs_threshold() {
        let input = F8959Input {
            filing_status: FilingStatus::MarriedFilingSeparately,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(175_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::ZERO,
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::from_cents(253_750),
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.filing_status_threshold_amt, Usd::from_dollars(125_000));
        assert_eq!(
            form.wages_tips_subj_to_addl_medcr_tax_amt,
            Usd::from_dollars(50_000)
        );
        assert_eq!(form.additional_medicare_tax_amt, Usd::from_dollars(450));
        assert!(form.is_valid());
    }

    #[test]
    fn hoh_threshold() {
        let input = F8959Input {
            filing_status: FilingStatus::HeadOfHousehold,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(250_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::ZERO,
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::from_cents(362_500),
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.filing_status_threshold_amt, Usd::from_dollars(200_000));
        assert!(form.is_valid());
    }

    #[test]
    fn qss_threshold() {
        let input = F8959Input {
            filing_status: FilingStatus::QualifyingSurvivingSpouse,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(250_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::ZERO,
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::from_cents(362_500),
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.filing_status_threshold_amt, Usd::from_dollars(200_000));
        assert!(form.is_valid());
    }

    #[test]
    fn wages_below_threshold_zero_part_i() {
        let mut input = single_input();
        input.w2_medicare_wages_and_tips_amt = Usd::from_dollars(150_000);
        input.w2_medicare_tax_withheld_amt = Usd::from_cents(217_500);
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.wages_tips_subj_to_addl_medcr_tax_amt, Usd::ZERO);
        assert_eq!(form.additional_medicare_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn se_income_with_wages() {
        let input = F8959Input {
            filing_status: FilingStatus::Single,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(150_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::from_dollars(100_000),
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::from_cents(217_500),
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.wages_tips_subj_to_addl_medcr_tax_amt, Usd::ZERO);
        assert_eq!(form.additional_medicare_tax_amt, Usd::ZERO);
        // Line 9 = threshold
        assert_eq!(
            form.se_filing_status_threshold_amt,
            Usd::from_dollars(200_000)
        );
        // Line 11: max(200,000 - 150,000, 0) = 50,000
        assert_eq!(form.se_reduced_threshold_amt, Usd::from_dollars(50_000));
        // Line 12: max(100,000 - 50,000, 0) = 50,000
        assert_eq!(
            form.se_income_above_threshold_amt,
            Usd::from_dollars(50_000)
        );
        // Line 13: 50,000 * 0.9% = 450
        assert_eq!(form.addl_self_employment_tax_amt, Usd::from_dollars(450));
        assert_eq!(form.total_amrrt_tax_amt, Usd::from_dollars(450));
        assert!(form.is_valid());
    }

    #[test]
    fn se_income_threshold_fully_consumed_by_wages() {
        let input = F8959Input {
            filing_status: FilingStatus::Single,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(250_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::from_dollars(30_000),
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::from_cents(362_500),
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.se_reduced_threshold_amt, Usd::ZERO);
        assert_eq!(
            form.se_income_above_threshold_amt,
            Usd::from_dollars(30_000)
        );
        assert_eq!(form.addl_self_employment_tax_amt, Usd::from_dollars(270));
        assert!(form.is_valid());
    }

    #[test]
    fn se_income_below_reduced_threshold() {
        let input = F8959Input {
            filing_status: FilingStatus::Single,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(100_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::from_dollars(50_000),
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::from_cents(145_000),
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.se_reduced_threshold_amt, Usd::from_dollars(100_000));
        assert_eq!(form.se_income_above_threshold_amt, Usd::ZERO);
        assert_eq!(form.addl_self_employment_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn rrta_compensation() {
        let input = F8959Input {
            filing_status: FilingStatus::Single,
            w2_medicare_wages_and_tips_amt: Usd::ZERO,
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::ZERO,
            railroad_retirement_comp_amt: Usd::from_dollars(250_000),
            w2_medicare_tax_withheld_amt: Usd::ZERO,
            w2_addl_rrt_tax_amt: Usd::from_dollars(450),
        };
        let form = Output8959::try_new(input).unwrap();
        // Line 15 = threshold
        assert_eq!(
            form.rrta_filing_status_threshold_amt,
            Usd::from_dollars(200_000)
        );
        assert_eq!(
            form.rrt_comp_subj_to_add_rrt_tax_amt,
            Usd::from_dollars(50_000)
        );
        assert_eq!(
            form.addl_railroad_retirement_tax_amt,
            Usd::from_dollars(450)
        );
        assert_eq!(form.total_amrrt_tax_amt, Usd::from_dollars(450));
        assert_eq!(
            form.total_addl_medcr_tax_withholding_amt,
            Usd::from_dollars(450)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn withholding_reconciliation() {
        let input = F8959Input {
            filing_status: FilingStatus::Single,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(300_000),
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::ZERO,
            railroad_retirement_comp_amt: Usd::ZERO,
            // 300,000 * 1.45% + 100,000 * 0.9% = 4,350 + 900 = 5,250
            w2_medicare_tax_withheld_amt: Usd::from_dollars(5_250),
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.total_medicare_tax_amt, Usd::from_dollars(300_000));
        assert_eq!(
            form.addnl_medicare_tax_withholding_amt,
            Usd::from_dollars(4_350)
        );
        assert_eq!(
            form.addl_medcr_rrt_tax_withholding_amt,
            Usd::from_dollars(900)
        );
        assert_eq!(
            form.total_addl_medcr_tax_withholding_amt,
            Usd::from_dollars(900)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn all_parts_combined() {
        let input = F8959Input {
            filing_status: FilingStatus::Single,
            w2_medicare_wages_and_tips_amt: Usd::from_dollars(220_000),
            unreported_medicare_tips_amt: Usd::from_dollars(10_000),
            wages_with_no_withholding_amt: Usd::from_dollars(20_000),
            self_employment_income_amt: Usd::from_dollars(50_000),
            railroad_retirement_comp_amt: Usd::from_dollars(210_000),
            w2_medicare_tax_withheld_amt: Usd::from_cents(319_000), // 220,000 * 1.45%
            w2_addl_rrt_tax_amt: Usd::from_dollars(90),
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(
            form.total_medicare_wages_and_tips_amt,
            Usd::from_dollars(250_000)
        );
        assert_eq!(
            form.wages_tips_subj_to_addl_medcr_tax_amt,
            Usd::from_dollars(50_000)
        );
        assert_eq!(form.additional_medicare_tax_amt, Usd::from_dollars(450));
        assert_eq!(form.se_reduced_threshold_amt, Usd::ZERO);
        assert_eq!(
            form.se_income_above_threshold_amt,
            Usd::from_dollars(50_000)
        );
        assert_eq!(form.addl_self_employment_tax_amt, Usd::from_dollars(450));
        assert_eq!(
            form.rrt_comp_subj_to_add_rrt_tax_amt,
            Usd::from_dollars(10_000)
        );
        assert_eq!(form.addl_railroad_retirement_tax_amt, Usd::from_dollars(90));
        assert_eq!(form.total_amrrt_tax_amt, Usd::from_dollars(990));
        assert_eq!(
            form.addnl_medicare_tax_withholding_amt,
            Usd::from_dollars(3_190)
        );
        assert_eq!(form.addl_medcr_rrt_tax_withholding_amt, Usd::ZERO);
        assert_eq!(
            form.total_addl_medcr_tax_withholding_amt,
            Usd::from_dollars(90)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn withholding_less_than_regular_medicare() {
        let mut input = single_input();
        input.w2_medicare_tax_withheld_amt = Usd::from_dollars(2_000);
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(
            form.addnl_medicare_tax_withholding_amt,
            Usd::from_dollars(3_625)
        );
        assert_eq!(form.addl_medcr_rrt_tax_withholding_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn zero_everything() {
        let input = F8959Input {
            filing_status: FilingStatus::Single,
            w2_medicare_wages_and_tips_amt: Usd::ZERO,
            unreported_medicare_tips_amt: Usd::ZERO,
            wages_with_no_withholding_amt: Usd::ZERO,
            self_employment_income_amt: Usd::ZERO,
            railroad_retirement_comp_amt: Usd::ZERO,
            w2_medicare_tax_withheld_amt: Usd::ZERO,
            w2_addl_rrt_tax_amt: Usd::ZERO,
        };
        let form = Output8959::try_new(input).unwrap();
        assert_eq!(form.total_amrrt_tax_amt, Usd::ZERO);
        assert_eq!(form.total_addl_medcr_tax_withholding_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
