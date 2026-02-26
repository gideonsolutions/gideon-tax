use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Schedule R filing box
// =========================================================================

/// Which Part I box the filer checked on Schedule R.
///
/// Exactly one box must be checked. The box determines the initial amount
/// (Line 10), the AGI threshold (Line 15), and whether Line 11 (taxable
/// disability income) applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleRBox {
    /// Box 1: You were 65 or older (Single, HoH, or QSS)
    Box1,
    /// Box 2: You were under 65 and you retired on permanent and total
    /// disability (Single, HoH, or QSS)
    Box2,
    /// Box 3: Both spouses were 65 or older (MFJ)
    Box3,
    /// Box 4: Both spouses were under 65, but only one spouse retired on
    /// permanent and total disability (MFJ)
    Box4,
    /// Box 5: Both spouses were under 65, and both retired on permanent and
    /// total disability (MFJ)
    Box5,
    /// Box 6: One spouse was 65 or older, and the other spouse was under 65
    /// and retired on permanent and total disability (MFJ)
    Box6,
    /// Box 7: One spouse was 65 or older, and the other spouse was under 65
    /// and not retired on permanent and total disability (MFJ)
    Box7,
    /// Box 8: You were 65 or older and you lived apart from your spouse for
    /// all of the tax year (MFS)
    Box8,
    /// Box 9: You were under 65, you retired on permanent and total
    /// disability, and you lived apart from your spouse for all of the tax
    /// year (MFS)
    Box9,
}

impl ScheduleRBox {
    /// Line 10 initial amount based on the checked box.
    const fn filing_status_amt(self) -> Usd {
        match self {
            Self::Box1 | Self::Box2 | Self::Box4 | Self::Box7 => Usd::from_dollars(5_000),
            Self::Box3 | Self::Box5 | Self::Box6 => Usd::from_dollars(7_500),
            Self::Box8 | Self::Box9 => Usd::from_dollars(3_750),
        }
    }

    /// Line 15 AGI threshold based on the checked box.
    const fn agi_threshold(self) -> Usd {
        match self {
            Self::Box1 | Self::Box2 => Usd::from_dollars(7_500),
            Self::Box3 | Self::Box4 | Self::Box5 | Self::Box6 | Self::Box7 => {
                Usd::from_dollars(10_000)
            }
            Self::Box8 | Self::Box9 => Usd::from_dollars(5_000),
        }
    }

    /// Whether Line 11 (taxable disability income) applies for this box.
    const fn needs_line_11(self) -> bool {
        matches!(
            self,
            Self::Box2 | Self::Box4 | Self::Box5 | Self::Box6 | Self::Box9
        )
    }
}

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Schedule R.
///
/// AGI (line 14) comes from Form 1040 line 11; the corresponding
/// dependency is declared in [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct ScheduleRInput {
    /// Part I box checked (determines filing status category)
    pub schedule_r_box: ScheduleRBox,
    /// Line 11: Taxable disability income. Only used when the checked box
    /// requires it (boxes 2, 4, 5, 6, or 9); ignored otherwise.
    pub taxable_disability_amt: Usd,
    /// Line 13a: Nontaxable part of social security benefits and nontaxable
    /// part of railroad retirement benefits treated as social security
    pub nontx_soc_sec_and_rlrd_benefits_amt: Usd,
    /// Line 13b: Nontaxable veterans' pensions and any other pension,
    /// annuity, or disability benefit excluded from income
    pub nontaxable_other_amt: Usd,
    /// Line 14: AGI from Form 1040 or 1040-SR, line 11
    pub tax_return_agi_amt: Usd,
    /// Line 21: Tax liability limit from the Credit Limit Worksheet
    pub total_tax_less_credits_amt: Usd,
    /// Part II: Prior year statement indicator
    pub prior_year_statement_ind: bool,
    /// Part II: Person first name (for prior year statement)
    pub prior_year_person_first_nm: String,
    /// Part II: Spouse name (for prior year statement)
    pub prior_year_spouse_nm: String,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Schedule R (Form 1040) — Credit for the Elderly or
/// the Disabled (2025).
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleR {
    // -----------------------------------------------------------------------
    // Part I — Check the Box for Your Filing Status and Age
    // -----------------------------------------------------------------------
    /// Box 1: You were 65 or older
    pub primary_65_or_older_ind: bool,
    /// Box 2: You were under 65 and you retired on permanent and total
    /// disability
    pub und_65_rtd_permnnt_tot_dsblty_ind: bool,
    /// Box 3: Both spouses were 65 or older
    pub both_spouses_65_or_older_ind: bool,
    /// Box 4: Both spouses were under 65, but only one spouse retired on
    /// permanent and total disability
    pub both_under_65_one_rtd_dsblty_ind: bool,
    /// Box 5: Both spouses were under 65, and both retired on permanent and
    /// total disability
    pub both_under_65_both_rtd_dsblty_ind: bool,
    /// Box 6: One spouse was 65 or older, and the other spouse was under 65 and
    /// retired on permanent and total disability
    pub one_65_or_older_other_rtd_dsblty_ind: bool,
    /// Box 7: One spouse was 65 or older, and the other spouse was under 65 and
    /// not retired on permanent and total disability
    pub one_65_or_older_other_not_rtd_ind: bool,
    /// Box 8: You were 65 or older and you lived apart from your spouse for all
    /// of 2025
    pub age_65_or_oldr_not_lvng_together_ind: bool,
    /// Box 9: You were under 65, you retired on permanent and total disability,
    /// and you lived apart from your spouse for all of 2025
    pub under_65_did_not_live_together_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Statement of Permanent and Total Disability
    // -----------------------------------------------------------------------
    /// Prior year statement indicator
    pub prior_year_statement_ind: bool,
    /// Person first name (for prior year statement)
    pub prior_year_person_first_nm: String,
    /// Spouse name (for prior year statement)
    pub prior_year_spouse_nm: String,

    // -----------------------------------------------------------------------
    // Part III — Figure Your Credit
    // -----------------------------------------------------------------------
    /// Line 10: Initial amount based on filing status and age
    pub filing_status_amt: Usd,
    /// Line 11: Taxable disability income
    pub taxable_disability_amt: Usd,
    /// Line 12: If you completed line 11, enter the smaller of line 10 or
    /// line 11. All others, enter the amount from line 10
    pub smaller_of_fs_or_taxable_amt: Usd,
    /// Line 13a: Nontaxable part of social security benefits and nontaxable
    /// part of railroad retirement benefits treated as social security (see
    /// instructions)
    pub nontx_soc_sec_and_rlrd_benefits_amt: Usd,
    /// Line 13b: Nontaxable veterans' pensions and any other pension, annuity,
    /// or disability benefit that is excluded from income under any other
    /// provision of law (see instructions)
    pub nontaxable_other_amt: Usd,
    /// Line 13c: Add lines 13a and 13b
    pub total_nontaxable_amt: Usd,
    /// Line 14: Enter the amount from Form 1040 or 1040-SR, line 11a
    pub tax_return_agi_amt: Usd,
    /// Line 15: Amount based on filing status
    pub adjusted_gross_income_amt: Usd,
    /// Line 16: Subtract line 15 from line 14. If zero or less, enter -0-
    pub exemption_amt: Usd,
    /// Line 17: Enter one-half of line 16
    pub half_agi_amt: Usd,
    /// Line 18: Add lines 13c and 17
    pub adjusted_credit_amt: Usd,
    /// Line 19: Subtract line 18 from line 12. If zero or less, stop; you
    /// can't take the credit. Otherwise, go to line 20
    pub net_credit_amt: Usd,
    /// Line 20: Multiply line 19 by 15% (0.15)
    pub calculated_amount_of_net_credit_amt: Usd,
    /// Line 21: Tax liability limit. Enter the amount from the Credit Limit
    /// Worksheet in the instructions
    pub total_tax_less_credits_amt: Usd,
    /// Line 22: Credit for the elderly or the disabled. Enter the smaller of
    /// line 20 or line 21. Also enter this amount on Schedule 3 (Form 1040),
    /// line 6d
    pub credit_for_elderly_or_disabled_amt: Usd,
}

// =========================================================================
// Helpers
// =========================================================================

impl OutputScheduleR {
    /// Reconstruct the [`ScheduleRBox`] from the Part I checkbox booleans.
    ///
    /// Assumes exactly one box is checked (verified by [`is_valid`](OutputForm::is_valid)).
    fn checked_box(&self) -> ScheduleRBox {
        if self.primary_65_or_older_ind {
            ScheduleRBox::Box1
        } else if self.und_65_rtd_permnnt_tot_dsblty_ind {
            ScheduleRBox::Box2
        } else if self.both_spouses_65_or_older_ind {
            ScheduleRBox::Box3
        } else if self.both_under_65_one_rtd_dsblty_ind {
            ScheduleRBox::Box4
        } else if self.both_under_65_both_rtd_dsblty_ind {
            ScheduleRBox::Box5
        } else if self.one_65_or_older_other_rtd_dsblty_ind {
            ScheduleRBox::Box6
        } else if self.one_65_or_older_other_not_rtd_ind {
            ScheduleRBox::Box7
        } else if self.age_65_or_oldr_not_lvng_together_ind {
            ScheduleRBox::Box8
        } else {
            ScheduleRBox::Box9
        }
    }
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for OutputScheduleR {
    fn name() -> &'static str {
        "Schedule R"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for OutputScheduleR {
    type Input = ScheduleRInput;

    fn must_file(_input: &Self::Input) -> bool {
        true
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        let b = input.schedule_r_box;

        // Line 10: initial amount from filing status box
        let line10 = b.filing_status_amt();

        // Line 11: taxable disability income (only for applicable boxes)
        let line11 = if b.needs_line_11() {
            input.taxable_disability_amt
        } else {
            Usd::ZERO
        };

        // Line 12: if line 11 applies, min(line 10, line 11); otherwise line 10
        let line12 = if b.needs_line_11() {
            line10.min(line11)
        } else {
            line10
        };

        // Lines 13a–c: nontaxable benefits
        let line13a = input.nontx_soc_sec_and_rlrd_benefits_amt;
        let line13b = input.nontaxable_other_amt;
        let line13c = line13a + line13b;

        // Line 14: AGI
        let line14 = input.tax_return_agi_amt;

        // Line 15: AGI threshold from filing status box
        let line15 = b.agi_threshold();

        // Line 16: line 14 − line 15 (min 0)
        let line16 = (line14 - line15).max(Usd::ZERO);

        // Line 17: one-half of line 16
        let line17 = Usd::from_cents(line16.cents() / 2);

        // Line 18: line 13c + line 17
        let line18 = line13c + line17;

        // Line 19: line 12 − line 18 (min 0; if zero, credit is $0)
        let line19 = (line12 - line18).max(Usd::ZERO);

        // Line 20: line 19 × 15%
        let line20 = Usd::from_cents(line19.cents() * 15 / 100);

        // Line 21: tax liability limit (from input)
        let line21 = input.total_tax_less_credits_amt;

        // Line 22: min(line 20, line 21)
        let line22 = line20.min(line21);

        Ok(OutputScheduleR {
            // Part I
            primary_65_or_older_ind: b == ScheduleRBox::Box1,
            und_65_rtd_permnnt_tot_dsblty_ind: b == ScheduleRBox::Box2,
            both_spouses_65_or_older_ind: b == ScheduleRBox::Box3,
            both_under_65_one_rtd_dsblty_ind: b == ScheduleRBox::Box4,
            both_under_65_both_rtd_dsblty_ind: b == ScheduleRBox::Box5,
            one_65_or_older_other_rtd_dsblty_ind: b == ScheduleRBox::Box6,
            one_65_or_older_other_not_rtd_ind: b == ScheduleRBox::Box7,
            age_65_or_oldr_not_lvng_together_ind: b == ScheduleRBox::Box8,
            under_65_did_not_live_together_ind: b == ScheduleRBox::Box9,
            // Part II
            prior_year_statement_ind: input.prior_year_statement_ind,
            prior_year_person_first_nm: input.prior_year_person_first_nm,
            prior_year_spouse_nm: input.prior_year_spouse_nm,
            // Part III
            filing_status_amt: line10,
            taxable_disability_amt: line11,
            smaller_of_fs_or_taxable_amt: line12,
            nontx_soc_sec_and_rlrd_benefits_amt: line13a,
            nontaxable_other_amt: line13b,
            total_nontaxable_amt: line13c,
            tax_return_agi_amt: line14,
            adjusted_gross_income_amt: line15,
            exemption_amt: line16,
            half_agi_amt: line17,
            adjusted_credit_amt: line18,
            net_credit_amt: line19,
            calculated_amount_of_net_credit_amt: line20,
            total_tax_less_credits_amt: line21,
            credit_for_elderly_or_disabled_amt: line22,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040]
    }

    fn is_valid(&self) -> bool {
        // Exactly one Part I box must be checked
        let box_count = [
            self.primary_65_or_older_ind,
            self.und_65_rtd_permnnt_tot_dsblty_ind,
            self.both_spouses_65_or_older_ind,
            self.both_under_65_one_rtd_dsblty_ind,
            self.both_under_65_both_rtd_dsblty_ind,
            self.one_65_or_older_other_rtd_dsblty_ind,
            self.one_65_or_older_other_not_rtd_ind,
            self.age_65_or_oldr_not_lvng_together_ind,
            self.under_65_did_not_live_together_ind,
        ]
        .iter()
        .filter(|&&b| b)
        .count();

        if box_count != 1 {
            return false;
        }

        let b = self.checked_box();

        // Line 10 matches expected amount for the box
        let line10_ok = self.filing_status_amt == b.filing_status_amt();

        // Line 12
        let line12_ok = if b.needs_line_11() {
            self.smaller_of_fs_or_taxable_amt
                == self.filing_status_amt.min(self.taxable_disability_amt)
        } else {
            self.smaller_of_fs_or_taxable_amt == self.filing_status_amt
        };

        // Line 13c = 13a + 13b
        let line13c_ok = self.total_nontaxable_amt
            == self.nontx_soc_sec_and_rlrd_benefits_amt + self.nontaxable_other_amt;

        // Line 15 matches expected threshold for the box
        let line15_ok = self.adjusted_gross_income_amt == b.agi_threshold();

        // Line 16 = max(line 14 − line 15, 0)
        let line16_ok = self.exemption_amt
            == (self.tax_return_agi_amt - self.adjusted_gross_income_amt).max(Usd::ZERO);

        // Line 17 = line 16 / 2
        let line17_ok =
            self.half_agi_amt == Usd::from_cents(self.exemption_amt.cents() / 2);

        // Line 18 = line 13c + line 17
        let line18_ok =
            self.adjusted_credit_amt == self.total_nontaxable_amt + self.half_agi_amt;

        // Line 19 = max(line 12 − line 18, 0)
        let line19_ok = self.net_credit_amt
            == (self.smaller_of_fs_or_taxable_amt - self.adjusted_credit_amt).max(Usd::ZERO);

        // Line 20 = line 19 × 15%
        let line20_ok = self.calculated_amount_of_net_credit_amt
            == Usd::from_cents(self.net_credit_amt.cents() * 15 / 100);

        // Line 22 = min(line 20, line 21)
        let line22_ok = self.credit_for_elderly_or_disabled_amt
            == self
                .calculated_amount_of_net_credit_amt
                .min(self.total_tax_less_credits_amt);

        line10_ok
            && line12_ok
            && line13c_ok
            && line15_ok
            && line16_ok
            && line17_ok
            && line18_ok
            && line19_ok
            && line20_ok
            && line22_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input(schedule_r_box: ScheduleRBox) -> ScheduleRInput {
        ScheduleRInput {
            schedule_r_box,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::ZERO,
            nontaxable_other_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(20_000),
            total_tax_less_credits_amt: Usd::from_dollars(10_000),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        }
    }

    // ── must_file ────────────────────────────────────────────────────

    #[test]
    fn must_file_always_true() {
        assert!(OutputScheduleR::must_file(&basic_input(ScheduleRBox::Box1)));
    }

    // ── Line 10 / Line 15 amounts per box ────────────────────────────

    #[test]
    fn box1_amounts() {
        let form = OutputScheduleR::try_new(basic_input(ScheduleRBox::Box1)).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(5_000));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(7_500));
        assert!(form.primary_65_or_older_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn box2_amounts() {
        let mut input = basic_input(ScheduleRBox::Box2);
        input.taxable_disability_amt = Usd::from_dollars(6_000);
        let form = OutputScheduleR::try_new(input).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(5_000));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(7_500));
        assert!(form.und_65_rtd_permnnt_tot_dsblty_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn box3_amounts() {
        let form = OutputScheduleR::try_new(basic_input(ScheduleRBox::Box3)).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(7_500));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(10_000));
        assert!(form.both_spouses_65_or_older_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn box4_amounts() {
        let mut input = basic_input(ScheduleRBox::Box4);
        input.taxable_disability_amt = Usd::from_dollars(6_000);
        let form = OutputScheduleR::try_new(input).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(5_000));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(10_000));
        assert!(form.both_under_65_one_rtd_dsblty_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn box5_amounts() {
        let mut input = basic_input(ScheduleRBox::Box5);
        input.taxable_disability_amt = Usd::from_dollars(8_000);
        let form = OutputScheduleR::try_new(input).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(7_500));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(10_000));
        assert!(form.both_under_65_both_rtd_dsblty_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn box6_amounts() {
        let mut input = basic_input(ScheduleRBox::Box6);
        input.taxable_disability_amt = Usd::from_dollars(8_000);
        let form = OutputScheduleR::try_new(input).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(7_500));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(10_000));
        assert!(form.one_65_or_older_other_rtd_dsblty_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn box7_amounts() {
        let form = OutputScheduleR::try_new(basic_input(ScheduleRBox::Box7)).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(5_000));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(10_000));
        assert!(form.one_65_or_older_other_not_rtd_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn box8_amounts() {
        let form = OutputScheduleR::try_new(basic_input(ScheduleRBox::Box8)).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(3_750));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(5_000));
        assert!(form.age_65_or_oldr_not_lvng_together_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn box9_amounts() {
        let mut input = basic_input(ScheduleRBox::Box9);
        input.taxable_disability_amt = Usd::from_dollars(4_000);
        let form = OutputScheduleR::try_new(input).unwrap();
        assert_eq!(form.filing_status_amt, Usd::from_dollars(3_750));
        assert_eq!(form.adjusted_gross_income_amt, Usd::from_dollars(5_000));
        assert!(form.under_65_did_not_live_together_ind);
        assert!(form.is_valid());
    }

    // ── Line 11 / Line 12: disability income cap ─────────────────────

    #[test]
    fn line11_ignored_for_box1() {
        let mut input = basic_input(ScheduleRBox::Box1);
        input.taxable_disability_amt = Usd::from_dollars(99_999);
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 11 ignored → Line 12 = Line 10 = $5,000
        assert_eq!(form.taxable_disability_amt, Usd::ZERO);
        assert_eq!(
            form.smaller_of_fs_or_taxable_amt,
            Usd::from_dollars(5_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn line12_capped_by_disability_income() {
        let mut input = basic_input(ScheduleRBox::Box2);
        input.taxable_disability_amt = Usd::from_dollars(3_000);
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 10 = 5,000; Line 11 = 3,000 → Line 12 = min(5,000, 3,000) = 3,000
        assert_eq!(
            form.smaller_of_fs_or_taxable_amt,
            Usd::from_dollars(3_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn line12_not_capped_when_disability_exceeds_line10() {
        let mut input = basic_input(ScheduleRBox::Box2);
        input.taxable_disability_amt = Usd::from_dollars(8_000);
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 10 = 5,000; Line 11 = 8,000 → Line 12 = min(5,000, 8,000) = 5,000
        assert_eq!(
            form.smaller_of_fs_or_taxable_amt,
            Usd::from_dollars(5_000)
        );
        assert!(form.is_valid());
    }

    // ── Full credit computation ──────────────────────────────────────

    #[test]
    fn basic_credit_computation_box1() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box1,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::from_dollars(1_000),
            nontaxable_other_amt: Usd::from_dollars(500),
            tax_return_agi_amt: Usd::from_dollars(15_000),
            total_tax_less_credits_amt: Usd::from_dollars(10_000),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 10: $5,000
        assert_eq!(form.filing_status_amt, Usd::from_dollars(5_000));
        // Line 12: $5,000 (no line 11)
        assert_eq!(
            form.smaller_of_fs_or_taxable_amt,
            Usd::from_dollars(5_000)
        );
        // Line 13c: 1,000 + 500 = 1,500
        assert_eq!(form.total_nontaxable_amt, Usd::from_dollars(1_500));
        // Line 14: $15,000
        // Line 15: $7,500
        // Line 16: 15,000 − 7,500 = 7,500
        assert_eq!(form.exemption_amt, Usd::from_dollars(7_500));
        // Line 17: 7,500 / 2 = 3,750
        assert_eq!(form.half_agi_amt, Usd::from_dollars(3_750));
        // Line 18: 1,500 + 3,750 = 5,250
        assert_eq!(form.adjusted_credit_amt, Usd::from_dollars(5_250));
        // Line 19: max(5,000 − 5,250, 0) = 0
        assert_eq!(form.net_credit_amt, Usd::ZERO);
        // Line 20: 0 × 15% = 0
        assert_eq!(form.calculated_amount_of_net_credit_amt, Usd::ZERO);
        // Line 22: min(0, 10,000) = 0
        assert_eq!(form.credit_for_elderly_or_disabled_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn positive_credit_box3() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box3,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::ZERO,
            nontaxable_other_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(12_000),
            total_tax_less_credits_amt: Usd::from_dollars(5_000),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 10: $7,500
        // Line 12: $7,500
        // Line 13c: $0
        // Line 16: max(12,000 − 10,000, 0) = 2,000
        assert_eq!(form.exemption_amt, Usd::from_dollars(2_000));
        // Line 17: 2,000 / 2 = 1,000
        assert_eq!(form.half_agi_amt, Usd::from_dollars(1_000));
        // Line 18: 0 + 1,000 = 1,000
        assert_eq!(form.adjusted_credit_amt, Usd::from_dollars(1_000));
        // Line 19: 7,500 − 1,000 = 6,500
        assert_eq!(form.net_credit_amt, Usd::from_dollars(6_500));
        // Line 20: 6,500 × 0.15 = 975
        assert_eq!(
            form.calculated_amount_of_net_credit_amt,
            Usd::from_dollars(975)
        );
        // Line 22: min(975, 5,000) = 975
        assert_eq!(
            form.credit_for_elderly_or_disabled_amt,
            Usd::from_dollars(975)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn credit_capped_by_tax_liability() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box3,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::ZERO,
            nontaxable_other_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(12_000),
            total_tax_less_credits_amt: Usd::from_dollars(500),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 20: $975 (same as above)
        assert_eq!(
            form.calculated_amount_of_net_credit_amt,
            Usd::from_dollars(975)
        );
        // Line 22: min(975, 500) = 500
        assert_eq!(
            form.credit_for_elderly_or_disabled_amt,
            Usd::from_dollars(500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn agi_below_threshold_no_reduction() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box1,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::ZERO,
            nontaxable_other_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(5_000),
            total_tax_less_credits_amt: Usd::from_dollars(10_000),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 16: max(5,000 − 7,500, 0) = 0
        assert_eq!(form.exemption_amt, Usd::ZERO);
        // Line 17: 0
        assert_eq!(form.half_agi_amt, Usd::ZERO);
        // Line 18: 0
        assert_eq!(form.adjusted_credit_amt, Usd::ZERO);
        // Line 19: 5,000 − 0 = 5,000
        assert_eq!(form.net_credit_amt, Usd::from_dollars(5_000));
        // Line 20: 5,000 × 0.15 = 750
        assert_eq!(
            form.calculated_amount_of_net_credit_amt,
            Usd::from_dollars(750)
        );
        // Line 22: min(750, 10,000) = 750
        assert_eq!(
            form.credit_for_elderly_or_disabled_amt,
            Usd::from_dollars(750)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn nontaxable_benefits_reduce_credit() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box1,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::from_dollars(3_000),
            nontaxable_other_amt: Usd::from_dollars(1_000),
            tax_return_agi_amt: Usd::from_dollars(5_000),
            total_tax_less_credits_amt: Usd::from_dollars(10_000),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 13c: 3,000 + 1,000 = 4,000
        assert_eq!(form.total_nontaxable_amt, Usd::from_dollars(4_000));
        // Line 18: 4,000 + 0 = 4,000
        assert_eq!(form.adjusted_credit_amt, Usd::from_dollars(4_000));
        // Line 19: 5,000 − 4,000 = 1,000
        assert_eq!(form.net_credit_amt, Usd::from_dollars(1_000));
        // Line 20: 1,000 × 0.15 = 150
        assert_eq!(
            form.calculated_amount_of_net_credit_amt,
            Usd::from_dollars(150)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn mfs_box8_lower_thresholds() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box8,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::ZERO,
            nontaxable_other_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(5_000),
            total_tax_less_credits_amt: Usd::from_dollars(10_000),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 10: $3,750; Line 15: $5,000
        // Line 16: max(5,000 − 5,000, 0) = 0
        assert_eq!(form.exemption_amt, Usd::ZERO);
        // Line 19: 3,750 − 0 = 3,750
        assert_eq!(form.net_credit_amt, Usd::from_dollars(3_750));
        // Line 20: 3,750 × 0.15 = 562.50
        assert_eq!(
            form.calculated_amount_of_net_credit_amt,
            Usd::from_cents(56_250)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn disability_box2_with_full_computation() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box2,
            taxable_disability_amt: Usd::from_dollars(3_000),
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::from_dollars(500),
            nontaxable_other_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_dollars(10_000),
            total_tax_less_credits_amt: Usd::from_dollars(2_000),
            prior_year_statement_ind: true,
            prior_year_person_first_nm: "John".to_string(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 10: $5,000; Line 11: $3,000
        // Line 12: min(5,000, 3,000) = 3,000
        assert_eq!(
            form.smaller_of_fs_or_taxable_amt,
            Usd::from_dollars(3_000)
        );
        // Line 13c: 500
        // Line 16: max(10,000 − 7,500, 0) = 2,500
        assert_eq!(form.exemption_amt, Usd::from_dollars(2_500));
        // Line 17: 2,500 / 2 = 1,250
        assert_eq!(form.half_agi_amt, Usd::from_dollars(1_250));
        // Line 18: 500 + 1,250 = 1,750
        assert_eq!(form.adjusted_credit_amt, Usd::from_dollars(1_750));
        // Line 19: 3,000 − 1,750 = 1,250
        assert_eq!(form.net_credit_amt, Usd::from_dollars(1_250));
        // Line 20: 1,250 × 0.15 = 187.50
        assert_eq!(
            form.calculated_amount_of_net_credit_amt,
            Usd::from_cents(18_750)
        );
        // Line 22: min(187.50, 2,000) = 187.50
        assert_eq!(
            form.credit_for_elderly_or_disabled_amt,
            Usd::from_cents(18_750)
        );
        // Part II passthrough
        assert!(form.prior_year_statement_ind);
        assert_eq!(form.prior_year_person_first_nm, "John");
        assert!(form.is_valid());
    }

    #[test]
    fn zero_agi_maximum_credit() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box3,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::ZERO,
            nontaxable_other_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::ZERO,
            total_tax_less_credits_amt: Usd::from_dollars(10_000),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 10/12: $7,500; Line 18: $0; Line 19: $7,500
        assert_eq!(form.net_credit_amt, Usd::from_dollars(7_500));
        // Line 20: 7,500 × 0.15 = 1,125
        assert_eq!(
            form.calculated_amount_of_net_credit_amt,
            Usd::from_dollars(1_125)
        );
        // Line 22: min(1,125, 10,000) = 1,125
        assert_eq!(
            form.credit_for_elderly_or_disabled_amt,
            Usd::from_dollars(1_125)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn odd_cent_half_agi() {
        let input = ScheduleRInput {
            schedule_r_box: ScheduleRBox::Box1,
            taxable_disability_amt: Usd::ZERO,
            nontx_soc_sec_and_rlrd_benefits_amt: Usd::ZERO,
            nontaxable_other_amt: Usd::ZERO,
            tax_return_agi_amt: Usd::from_cents(750_001), // $7,500.01
            total_tax_less_credits_amt: Usd::from_dollars(10_000),
            prior_year_statement_ind: false,
            prior_year_person_first_nm: String::new(),
            prior_year_spouse_nm: String::new(),
        };
        let form = OutputScheduleR::try_new(input).unwrap();
        // Line 16: 7,500.01 − 7,500.00 = 0.01
        assert_eq!(form.exemption_amt, Usd::from_cents(1));
        // Line 17: 0.01 / 2 = 0 (integer truncation)
        assert_eq!(form.half_agi_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
