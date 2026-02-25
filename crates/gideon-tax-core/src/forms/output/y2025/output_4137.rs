use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::rules::TaxYearRules;
use crate::rules::y2025::Rules2025;

// =========================================================================
// Line 1 row
// =========================================================================

/// Per-employer row from Form 4137, Line 1 (columns a through d).
#[derive(Debug, Clone)]
pub struct F4137Line1 {
    /// Column (a): Name of employer to whom you were required to,
    /// but did not, report all tips
    pub employer_nm: String,
    /// Column (b): Employer identification number
    pub employer_ein: String,
    /// Column (c): Total cash and charge tips you received
    /// (including unreported tips)
    pub total_tips_received_amt: Usd,
    /// Column (d): Total cash and charge tips you reported to your employer
    pub total_tips_reported_amt: Usd,
}

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 4137.
///
/// W-2 data (boxes 3 and 7) and RRTA compensation feed into Line 8;
/// the corresponding [`DynForm::W2`] dependency is declared in
/// [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct F4137Input {
    /// Name of person who received tips
    pub person_nm: String,
    /// Social security number
    pub ssn: String,
    /// Per-employer unreported tip rows (Line 1, rows A–E)
    pub employers: Vec<F4137Line1>,
    /// Line 5: Cash and charge tips not reported because the total
    /// was less than $20 in a calendar month
    pub incidental_cash_and_tips_amt: Usd,
    /// Line 8 component: Social security wages from Form(s) W-2, box 3
    pub w2_social_security_wages_amt: Usd,
    /// Line 8 component: Social security tips from Form(s) W-2, box 7
    pub w2_social_security_tips_amt: Usd,
    /// Line 8 component: Railroad retirement (RRTA) compensation
    /// subject to the 6.2% rate. Capped at the social security wage
    /// base when computing Line 8.
    pub rrta_compensation_amt: Usd,
    /// Amount of unreported tips subject only to the 1.45% Medicare
    /// tax (not the 6.2% social security tax). Applies to federal,
    /// state, or local government employees who have SS-exempt tips.
    /// Zero for most filers.
    pub government_employee_145_tips_amt: Usd,
    /// Total allocated tips from Form(s) W-2, box 8. When positive
    /// the filer must file Form 4137.
    pub w2_allocated_tips_amt: Usd,
    /// `true` if, for at least one employer in at least one month,
    /// the filer received $20 or more in cash and charge tips and
    /// did not report all of them to that employer. When `true` the
    /// filer must file Form 4137. (Monthly granularity is not
    /// available in the per-employer totals, so this is supplied as
    /// a pre-computed flag.)
    pub has_unreported_tips_in_any_month: bool,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 4137 (2025) — Social Security and Medicare Tax on Unreported Tip Income.
#[derive(Debug, Clone, Default)]
pub struct Output4137 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of person who received tips
    pub person_nm: String,
    /// Social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Line 1 — Employer-level detail (per-employer rows A through E)
    // -----------------------------------------------------------------------
    /// Line 1: Unreported tip income per employer (table rows A-E with columns a-d)
    pub unreported_tip_income_per_employer: Vec<F4137Line1>,

    // -----------------------------------------------------------------------
    // Lines 2-13
    // -----------------------------------------------------------------------
    /// Line 2: Total cash and charge tips you received in 2025 (sum of line 1, column (c))
    pub total_tips_received_amt: Usd,
    /// Line 3: Total cash and charge tips you reported to your employer(s) in 2025 (sum of
    /// line 1, column (d))
    pub total_tips_reported_amt: Usd,
    /// Line 4: Subtract line 3 from line 2. Include as income on Form 1040, 1040-SR, or
    /// 1040-NR, line 1c
    pub total_tips_received_minus_rpt_amt: Usd,
    /// Line 5: Cash and charge tips you received but did not report to your employer because
    /// the total was less than $20 in a calendar month
    pub incidental_cash_and_tips_amt: Usd,
    /// Line 6: Unreported tips subject to Medicare tax. Subtract line 5 from line 4
    pub net_unreported_minus_incdntl_amt: Usd,
    /// Line 8: Total social security wages and social security tips (total of Form(s) W-2,
    /// boxes 3 and 7) and railroad retirement (RRTA) compensation (subject to 6.2% rate)
    pub social_security_wages_and_tips_amt: Usd,
    /// Line 9: Subtract line 8 from line 7. If line 8 is more than line 7, enter -0-
    pub net_wage_subject_to_soc_sec_tax_amt: Usd,
    /// Line 10: Unreported tips subject to social security tax. Enter the smaller of line 6
    /// or line 9
    pub unreported_tips_subj_to_soc_sec_amt: Usd,
    /// Line 10: If you received tips as a federal, state, or local government employee
    /// whose position is not covered by social security, code and amount for "1.45% tips".
    /// Not all government employees qualify — only those with SS-exempt tips.
    pub government_employee_tip_cd: String,
    /// Line 10: Government employee tip amount subject only to 1.45% Medicare tax
    /// (applicable only when the employee has SS-exempt tips)
    pub government_employee_145_tip_amt: Usd,
    /// Line 11: Multiply line 10 by 0.062 (social security tax rate)
    pub social_security_tax_tip_amt: Usd,
    /// Line 12: Multiply line 6 by 0.0145 (Medicare tax rate)
    pub medicare_tax_tips_amt: Usd,
    /// Line 13: Add lines 11 and 12. Include as tax on Schedule 2 (Form 1040), line 5
    pub soc_sec_medicare_tax_unrptd_tip_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output4137 {
    fn name() -> &'static str {
        "Form 4137"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output4137 {
    type Input = F4137Input;

    fn must_file(input: &Self::Input) -> bool {
        input.w2_allocated_tips_amt > Usd::ZERO || input.has_unreported_tips_in_any_month
    }

    fn new(input: Self::Input) -> Option<Self> {
        let rules = Rules2025;

        // Line 2: sum of column (c)
        let line2 = input
            .employers
            .iter()
            .map(|e| e.total_tips_received_amt)
            .sum::<Usd>();

        // Line 3: sum of column (d)
        let line3 = input
            .employers
            .iter()
            .map(|e| e.total_tips_reported_amt)
            .sum::<Usd>();

        // Line 4: line 2 − line 3
        let line4 = line2 - line3;

        // Line 5
        let line5 = input.incidental_cash_and_tips_amt;

        // Line 6: line 4 − line 5 (min 0)
        let line6 = (line4 - line5).max(Usd::ZERO);

        // Line 7: social security wage base (from rules)
        let ss_wage_base = rules.social_security_wage_base();

        // Line 8: W-2 box 3 + W-2 box 7 + RRTA (RRTA capped at wage base)
        let rrta_capped = input.rrta_compensation_amt.min(ss_wage_base);
        let line8 =
            input.w2_social_security_wages_amt + input.w2_social_security_tips_amt + rrta_capped;

        // Line 9: line 7 − line 8 (min 0)
        let line9 = (ss_wage_base - line8).max(Usd::ZERO);

        // Government 1.45%-only tips: remove from line 6 before SS comparison
        let gov_145_amt = input.government_employee_145_tips_amt;
        let gov_cd = if gov_145_amt > Usd::ZERO {
            "1.45% TIPS".to_string()
        } else {
            String::new()
        };

        // Revised line 6 excluding SS-exempt tips, for SS tax purposes
        let revised_line6 = (line6 - gov_145_amt).max(Usd::ZERO);

        // Line 10: min(revised_line6, line 9)
        let line10 = revised_line6.min(line9);

        let ss_bps = rules.social_security_rate_bps() as i64;
        let med_bps = rules.medicare_rate_bps() as i64;

        // Line 11: line 10 × SS rate (only tips subject to SS)
        let line11 = Usd::from_cents(line10.cents() * ss_bps / 10_000);

        // Line 12: line 6 × Medicare rate
        let line12 = Usd::from_cents(line6.cents() * med_bps / 10_000);

        // Line 13: line 11 + line 12
        let line13 = line11 + line12;

        Some(Output4137 {
            person_nm: input.person_nm,
            ssn: input.ssn,
            unreported_tip_income_per_employer: input.employers,
            total_tips_received_amt: line2,
            total_tips_reported_amt: line3,
            total_tips_received_minus_rpt_amt: line4,
            incidental_cash_and_tips_amt: line5,
            net_unreported_minus_incdntl_amt: line6,
            social_security_wages_and_tips_amt: line8,
            net_wage_subject_to_soc_sec_tax_amt: line9,
            unreported_tips_subj_to_soc_sec_amt: line10,
            government_employee_tip_cd: gov_cd,
            government_employee_145_tip_amt: gov_145_amt,
            social_security_tax_tip_amt: line11,
            medicare_tax_tips_amt: line12,
            soc_sec_medicare_tax_unrptd_tip_amt: line13,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::W2]
    }

    fn is_valid(&self) -> bool {
        let rules = Rules2025;
        let ss_wage_base = rules.social_security_wage_base();
        let med_bps = rules.medicare_rate_bps() as i64;

        // Line 4 = Line 2 − Line 3
        let line4_ok = self.total_tips_received_minus_rpt_amt
            == self.total_tips_received_amt - self.total_tips_reported_amt;

        // Line 6 = max(Line 4 − Line 5, 0)
        let line6_ok = self.net_unreported_minus_incdntl_amt
            == (self.total_tips_received_minus_rpt_amt - self.incidental_cash_and_tips_amt)
                .max(Usd::ZERO);

        // Line 9 = max(Line 7 − Line 8, 0)
        let line9_ok = self.net_wage_subject_to_soc_sec_tax_amt
            == (ss_wage_base - self.social_security_wages_and_tips_amt).max(Usd::ZERO);

        // Line 10 = min(revised_line6, Line 9)
        // revised_line6 = Line 6 − gov 1.45%-only tips
        let revised_line6 = (self.net_unreported_minus_incdntl_amt
            - self.government_employee_145_tip_amt)
            .max(Usd::ZERO);
        let line10_ok = self.unreported_tips_subj_to_soc_sec_amt
            == revised_line6.min(self.net_wage_subject_to_soc_sec_tax_amt);

        // Line 12 = Line 6 × Medicare rate
        let line12_ok = self.medicare_tax_tips_amt
            == Usd::from_cents(self.net_unreported_minus_incdntl_amt.cents() * med_bps / 10_000);

        // Line 13 = Line 11 + Line 12
        let line13_ok = self.soc_sec_medicare_tax_unrptd_tip_amt
            == self.social_security_tax_tip_amt + self.medicare_tax_tips_amt;

        line4_ok && line6_ok && line9_ok && line10_ok && line12_ok && line13_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_employer(received: i64, reported: i64) -> F4137Line1 {
        F4137Line1 {
            employer_nm: "Test Employer".to_string(),
            employer_ein: "12-3456789".to_string(),
            total_tips_received_amt: Usd::from_dollars(received),
            total_tips_reported_amt: Usd::from_dollars(reported),
        }
    }

    fn basic_input(received: i64, reported: i64) -> F4137Input {
        F4137Input {
            person_nm: "Jane Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            employers: vec![make_employer(received, reported)],
            incidental_cash_and_tips_amt: Usd::ZERO,
            w2_social_security_wages_amt: Usd::from_dollars(40_000),
            w2_social_security_tips_amt: Usd::from_dollars(10_000),
            rrta_compensation_amt: Usd::ZERO,
            government_employee_145_tips_amt: Usd::ZERO,
            w2_allocated_tips_amt: Usd::ZERO,
            has_unreported_tips_in_any_month: true,
        }
    }

    #[test]
    fn must_file_allocated_tips() {
        let mut input = basic_input(5_000, 5_000);
        input.has_unreported_tips_in_any_month = false;
        input.w2_allocated_tips_amt = Usd::from_dollars(500);
        assert!(Output4137::must_file(&input));
    }

    #[test]
    fn must_file_unreported_month() {
        let mut input = basic_input(5_000, 5_000);
        input.has_unreported_tips_in_any_month = true;
        input.w2_allocated_tips_amt = Usd::ZERO;
        assert!(Output4137::must_file(&input));
    }

    #[test]
    fn must_file_neither_trigger() {
        let mut input = basic_input(5_000, 5_000);
        input.has_unreported_tips_in_any_month = false;
        input.w2_allocated_tips_amt = Usd::ZERO;
        assert!(!Output4137::must_file(&input));
    }

    #[test]
    fn basic_unreported_tips() {
        let form = Output4137::new(basic_input(10_000, 7_000)).unwrap();
        // Line 4: 10,000 − 7,000 = 3,000
        assert_eq!(
            form.total_tips_received_minus_rpt_amt,
            Usd::from_dollars(3_000)
        );
        // Line 6: 3,000 − 0 = 3,000
        assert_eq!(
            form.net_unreported_minus_incdntl_amt,
            Usd::from_dollars(3_000)
        );
        // Line 8: 40,000 + 10,000 + 0 = 50,000
        assert_eq!(
            form.social_security_wages_and_tips_amt,
            Usd::from_dollars(50_000)
        );
        // Line 9: 176,100 − 50,000 = 126,100
        assert_eq!(
            form.net_wage_subject_to_soc_sec_tax_amt,
            Usd::from_dollars(126_100)
        );
        // Line 10: min(3,000, 126,100) = 3,000
        assert_eq!(
            form.unreported_tips_subj_to_soc_sec_amt,
            Usd::from_dollars(3_000)
        );
        // Line 11: 3,000 × 0.062 = 186
        assert_eq!(form.social_security_tax_tip_amt, Usd::from_dollars(186));
        // Line 12: 3,000 × 0.0145 = 43.50
        assert_eq!(form.medicare_tax_tips_amt, Usd::from_cents(4_350));
        // Line 13: 186.00 + 43.50 = 229.50
        assert_eq!(
            form.soc_sec_medicare_tax_unrptd_tip_amt,
            Usd::from_cents(22_950)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn ss_wage_base_cap() {
        let mut input = basic_input(10_000, 7_000);
        input.w2_social_security_wages_amt = Usd::from_dollars(170_000);
        input.w2_social_security_tips_amt = Usd::from_dollars(5_000);
        let form = Output4137::new(input).unwrap();
        // Line 8: 170,000 + 5,000 + 0 = 175,000
        // Line 9: 176,100 − 175,000 = 1,100
        assert_eq!(
            form.net_wage_subject_to_soc_sec_tax_amt,
            Usd::from_dollars(1_100)
        );
        // Line 10: min(3,000, 1,100) = 1,100
        assert_eq!(
            form.unreported_tips_subj_to_soc_sec_amt,
            Usd::from_dollars(1_100)
        );
        // Line 11: 1,100 × 0.062 = 68.20
        assert_eq!(form.social_security_tax_tip_amt, Usd::from_cents(6_820));
        // Line 12: 3,000 × 0.0145 = 43.50 (Medicare still on full line 6)
        assert_eq!(form.medicare_tax_tips_amt, Usd::from_cents(4_350));
        assert!(form.is_valid());
    }

    #[test]
    fn incidental_tips_reduce_line6() {
        let mut input = basic_input(10_000, 7_000);
        input.incidental_cash_and_tips_amt = Usd::from_dollars(500);
        let form = Output4137::new(input).unwrap();
        // Line 6: 3,000 − 500 = 2,500
        assert_eq!(
            form.net_unreported_minus_incdntl_amt,
            Usd::from_dollars(2_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn government_employee_all_ss_exempt() {
        let mut input = basic_input(10_000, 7_000);
        // All 3,000 of unreported tips are SS-exempt
        input.government_employee_145_tips_amt = Usd::from_dollars(3_000);
        let form = Output4137::new(input).unwrap();
        // revised_line6 = 3,000 − 3,000 = 0 → line 10 = 0 → no SS tax
        assert_eq!(form.unreported_tips_subj_to_soc_sec_amt, Usd::ZERO);
        assert_eq!(form.social_security_tax_tip_amt, Usd::ZERO);
        assert_eq!(form.government_employee_tip_cd, "1.45% TIPS");
        assert_eq!(
            form.government_employee_145_tip_amt,
            Usd::from_dollars(3_000)
        );
        // Medicare tax still applies on full line 6
        assert_eq!(form.medicare_tax_tips_amt, Usd::from_cents(4_350));
        assert!(form.is_valid());
    }

    #[test]
    fn government_employee_partial_ss_exempt() {
        let mut input = basic_input(10_000, 7_000);
        // Only 1,000 of the 3,000 unreported tips are SS-exempt
        input.government_employee_145_tips_amt = Usd::from_dollars(1_000);
        let form = Output4137::new(input).unwrap();
        // revised_line6 = 3,000 − 1,000 = 2,000
        // line 10 = min(2,000, 126,100) = 2,000
        assert_eq!(
            form.unreported_tips_subj_to_soc_sec_amt,
            Usd::from_dollars(2_000)
        );
        // Line 11: 2,000 × 0.062 = 124
        assert_eq!(form.social_security_tax_tip_amt, Usd::from_dollars(124));
        assert_eq!(form.government_employee_tip_cd, "1.45% TIPS");
        assert_eq!(
            form.government_employee_145_tip_amt,
            Usd::from_dollars(1_000)
        );
        // Medicare tax still on full line 6 = 3,000
        assert_eq!(form.medicare_tax_tips_amt, Usd::from_cents(4_350));
        assert!(form.is_valid());
    }

    #[test]
    fn multiple_employers() {
        let input = F4137Input {
            person_nm: "Jane Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            employers: vec![make_employer(5_000, 3_000), make_employer(4_000, 2_000)],
            incidental_cash_and_tips_amt: Usd::ZERO,
            w2_social_security_wages_amt: Usd::from_dollars(40_000),
            w2_social_security_tips_amt: Usd::from_dollars(10_000),
            rrta_compensation_amt: Usd::ZERO,
            government_employee_145_tips_amt: Usd::ZERO,
            w2_allocated_tips_amt: Usd::ZERO,
            has_unreported_tips_in_any_month: true,
        };
        let form = Output4137::new(input).unwrap();
        assert_eq!(form.total_tips_received_amt, Usd::from_dollars(9_000));
        assert_eq!(form.total_tips_reported_amt, Usd::from_dollars(5_000));
        assert_eq!(
            form.total_tips_received_minus_rpt_amt,
            Usd::from_dollars(4_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn exceeded_ss_wage_base_zero_ss_tax() {
        let mut input = basic_input(10_000, 7_000);
        input.w2_social_security_wages_amt = Usd::from_dollars(200_000);
        input.w2_social_security_tips_amt = Usd::ZERO;
        let form = Output4137::new(input).unwrap();
        assert_eq!(form.net_wage_subject_to_soc_sec_tax_amt, Usd::ZERO);
        assert_eq!(form.unreported_tips_subj_to_soc_sec_amt, Usd::ZERO);
        assert_eq!(form.social_security_tax_tip_amt, Usd::ZERO);
        assert!(form.medicare_tax_tips_amt > Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn rrta_capped_at_wage_base() {
        let mut input = basic_input(10_000, 7_000);
        input.w2_social_security_wages_amt = Usd::ZERO;
        input.w2_social_security_tips_amt = Usd::ZERO;
        // RRTA of $200,000 should be capped at $176,100
        input.rrta_compensation_amt = Usd::from_dollars(200_000);
        let form = Output4137::new(input).unwrap();
        // Line 8: 0 + 0 + min(200,000, 176,100) = 176,100
        assert_eq!(
            form.social_security_wages_and_tips_amt,
            Usd::from_dollars(176_100)
        );
        // Line 9: 176,100 − 176,100 = 0
        assert_eq!(form.net_wage_subject_to_soc_sec_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn rrta_below_wage_base_not_capped() {
        let mut input = basic_input(10_000, 7_000);
        input.w2_social_security_wages_amt = Usd::from_dollars(30_000);
        input.w2_social_security_tips_amt = Usd::ZERO;
        input.rrta_compensation_amt = Usd::from_dollars(10_000);
        let form = Output4137::new(input).unwrap();
        // Line 8: 30,000 + 0 + min(10,000, 176,100) = 40,000
        assert_eq!(
            form.social_security_wages_and_tips_amt,
            Usd::from_dollars(40_000)
        );
        // Line 9: 176,100 − 40,000 = 136,100
        assert_eq!(
            form.net_wage_subject_to_soc_sec_tax_amt,
            Usd::from_dollars(136_100)
        );
        assert!(form.is_valid());
    }
}
