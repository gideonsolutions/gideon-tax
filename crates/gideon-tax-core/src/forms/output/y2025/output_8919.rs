use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::rules::TaxYearRules;
use crate::rules::y2025::Rules2025;
use crate::{GideonTaxError, Usd};

// =========================================================================
// Reason code
// =========================================================================

/// Reason code for filing Form 8919, entered in column (c) for each firm.
///
/// If none of the reason codes apply but the filer believes they should have
/// been treated as an employee, they enter reason code G and must file
/// Form SS-8 on or before the date they file their tax return.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F8919ReasonCode {
    /// A — Filed Form SS-8 and received a determination letter stating
    /// the filer is an employee of this firm.
    A,
    /// C — Received other correspondence from the IRS stating the filer
    /// is an employee.
    C,
    /// G — Filed Form SS-8 with the IRS and has not received a reply.
    G,
    /// H — Received both a Form W-2 and a Form 1099-MISC/1099-NEC from
    /// this firm. The 1099 amount should have been included as wages on
    /// the W-2. (Do not file Form SS-8 when using this code.)
    H,
}

// =========================================================================
// Lines 1–5 row
// =========================================================================

/// Per-firm row from Form 8919, Lines 1–5 (columns a through f).
///
/// Each row describes one firm that failed to withhold the filer's share of
/// social security and Medicare taxes.
#[derive(Debug, Clone)]
pub struct F8919Lines1To5 {
    /// Column (a): Name of firm
    pub firm_nm: String,
    /// Column (b): Firm's federal identification number (EIN or SSN)
    pub firm_ein: String,
    /// Column (c): Reason code for filing (A, C, G, or H)
    pub reason_cd: F8919ReasonCode,
    /// Column (d): Date of IRS determination or correspondence (MM/DD/YYYY).
    /// Required only when reason code is A or C; empty otherwise.
    pub determination_dt: String,
    /// Column (e): `true` if a Form 1099-MISC and/or 1099-NEC was received
    /// from this firm
    pub received_1099_ind: bool,
    /// Column (f): Total wages received with no social security or Medicare
    /// tax withholding and not reported on Form W-2
    pub wages_with_no_withholding_amt: Usd,
}

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8919.
///
/// W-2 data (boxes 3 and 7), RRTA compensation, and unreported tips from
/// Form 4137 line 10 feed into Line 8; the corresponding dependencies are
/// declared in [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct F8919Input {
    /// Name of person who must file this form
    pub person_nm: String,
    /// Social security number
    pub ssn: String,
    /// Per-firm rows (Lines 1–5). If more than five firms, the IRS
    /// requires additional Forms 8919 with lines 1–5 completed, but
    /// lines 6–13 only on one copy. We accept an unbounded vec.
    pub firms: Vec<F8919Lines1To5>,
    /// Line 8 component: Social security wages from Form(s) W-2, box 3
    pub w2_social_security_wages_amt: Usd,
    /// Line 8 component: Social security tips from Form(s) W-2, box 7
    pub w2_social_security_tips_amt: Usd,
    /// Line 8 component: Railroad retirement (RRTA) compensation
    /// subject to the 6.2% rate. Capped at the social security wage
    /// base when computing Line 8.
    pub rrta_compensation_amt: Usd,
    /// Line 8 component: Unreported tips subject to social security tax
    /// from Form 4137, line 10
    pub f4137_unreported_tips_subj_to_soc_sec_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8919 (2025) — Uncollected Social Security and Medicare Tax on Wages.
#[derive(Debug, Clone)]
pub struct Output8919 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of person who must file this form
    pub person_nm: String,
    /// Social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Firm-level detail (Lines 1–5, columns a–f)
    // -----------------------------------------------------------------------
    /// Lines 1–5: Per-firm detail
    pub uncollected_soc_sec_med_tax_per_firm: Vec<F8919Lines1To5>,

    // -----------------------------------------------------------------------
    // Totals (Lines 6–13)
    // -----------------------------------------------------------------------
    /// Line 6: Total wages — combine lines 1 through 5 in column (f).
    /// Also entered on Form 1040/1040-SR/1040-NR line 1g and Form 8959 line 3.
    pub total_wages_with_no_withholding_amt: Usd,
    /// Line 8: Total social security wages and social security tips (W-2
    /// boxes 3 + 7), railroad retirement (RRTA) compensation (subject to
    /// the 6.2% rate), and unreported tips from Form 4137 line 10.
    pub total_wages_and_unreported_tips_amt: Usd,
    /// Line 9: Line 7 minus line 8 (if line 8 > line 7, enter -0-)
    pub net_wages_subject_to_soc_sec_tax_amt: Usd,
    /// Line 10: Wages subject to social security tax — smaller of line 6
    /// or line 9
    pub wages_subject_to_sst_amt: Usd,
    /// Line 11: Line 10 × 0.062 (social security tax rate)
    pub uncollected_soc_sec_tax_amt: Usd,
    /// Line 12: Line 6 × 0.0145 (Medicare tax rate)
    pub uncollected_medicare_tax_amt: Usd,
    /// Line 13: Lines 11 + 12. Include as tax on Schedule 2 (Form 1040)
    /// line 6, or Form 1040-SS Part I line 6c.
    pub uncollected_soc_sec_med_tax_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8919 {
    fn name() -> &'static str {
        "Form 8919"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8919 {
    type Input = F8919Input;

    fn must_file(input: &Self::Input) -> bool {
        !input.firms.is_empty()
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Line 6: sum of column (f) across all firms
        let line6 = input
            .firms
            .iter()
            .map(|f| f.wages_with_no_withholding_amt)
            .sum::<Usd>();

        // Line 7: social security wage base (from rules)
        let ss_wage_base = Rules2025::SOCIAL_SECURITY_WAGE_BASE;

        // Line 8: W-2 box 3 + W-2 box 7 + RRTA (capped at wage base)
        //         + Form 4137 line 10
        let rrta_capped = input.rrta_compensation_amt.min(ss_wage_base);
        let line8 = input.w2_social_security_wages_amt
            + input.w2_social_security_tips_amt
            + rrta_capped
            + input.f4137_unreported_tips_subj_to_soc_sec_amt;

        // Line 9: line 7 − line 8 (min 0)
        let line9 = (ss_wage_base - line8).max(Usd::ZERO);

        // Line 10: min(line 6, line 9)
        let line10 = line6.min(line9);

        let ss_bps = Rules2025::SOCIAL_SECURITY_RATE_BPS as i64;
        let med_bps = Rules2025::MEDICARE_RATE_BPS as i64;

        // Line 11: line 10 × SS rate
        let line11 = Usd::from_cents(line10.cents() * ss_bps / 10_000);

        // Line 12: line 6 × Medicare rate
        let line12 = Usd::from_cents(line6.cents() * med_bps / 10_000);

        // Line 13: line 11 + line 12
        let line13 = line11 + line12;

        Ok(Output8919 {
            person_nm: input.person_nm,
            ssn: input.ssn,
            uncollected_soc_sec_med_tax_per_firm: input.firms,
            total_wages_with_no_withholding_amt: line6,
            total_wages_and_unreported_tips_amt: line8,
            net_wages_subject_to_soc_sec_tax_amt: line9,
            wages_subject_to_sst_amt: line10,
            uncollected_soc_sec_tax_amt: line11,
            uncollected_medicare_tax_amt: line12,
            uncollected_soc_sec_med_tax_amt: line13,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::W2, DynForm::F4137]
    }

    fn is_valid(&self) -> bool {
        let ss_wage_base = Rules2025::SOCIAL_SECURITY_WAGE_BASE;
        let ss_bps = Rules2025::SOCIAL_SECURITY_RATE_BPS as i64;
        let med_bps = Rules2025::MEDICARE_RATE_BPS as i64;

        // Line 6 = sum of per-firm wages
        let line6_ok = self.total_wages_with_no_withholding_amt
            == self
                .uncollected_soc_sec_med_tax_per_firm
                .iter()
                .map(|f| f.wages_with_no_withholding_amt)
                .sum::<Usd>();

        // Line 9 = max(Line 7 − Line 8, 0)
        let line9_ok = self.net_wages_subject_to_soc_sec_tax_amt
            == (ss_wage_base - self.total_wages_and_unreported_tips_amt).max(Usd::ZERO);

        // Line 10 = min(Line 6, Line 9)
        let line10_ok = self.wages_subject_to_sst_amt
            == self
                .total_wages_with_no_withholding_amt
                .min(self.net_wages_subject_to_soc_sec_tax_amt);

        // Line 11 = Line 10 × SS rate
        let line11_ok = self.uncollected_soc_sec_tax_amt
            == Usd::from_cents(self.wages_subject_to_sst_amt.cents() * ss_bps / 10_000);

        // Line 12 = Line 6 × Medicare rate
        let line12_ok = self.uncollected_medicare_tax_amt
            == Usd::from_cents(self.total_wages_with_no_withholding_amt.cents() * med_bps / 10_000);

        // Line 13 = Line 11 + Line 12
        let line13_ok = self.uncollected_soc_sec_med_tax_amt
            == self.uncollected_soc_sec_tax_amt + self.uncollected_medicare_tax_amt;

        line6_ok && line9_ok && line10_ok && line11_ok && line12_ok && line13_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn make_firm(wages: i64) -> F8919Lines1To5 {
        F8919Lines1To5 {
            firm_nm: "Test Firm".to_string(),
            firm_ein: "12-3456789".to_string(),
            reason_cd: F8919ReasonCode::G,
            determination_dt: String::new(),
            received_1099_ind: true,
            wages_with_no_withholding_amt: Usd::from_dollars(wages),
        }
    }

    fn basic_input(wages: i64) -> F8919Input {
        F8919Input {
            person_nm: "Jane Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            firms: vec![make_firm(wages)],
            w2_social_security_wages_amt: Usd::from_dollars(40_000),
            w2_social_security_tips_amt: Usd::from_dollars(10_000),
            rrta_compensation_amt: Usd::ZERO,
            f4137_unreported_tips_subj_to_soc_sec_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_with_firms() {
        let input = basic_input(5_000);
        assert!(Output8919::must_file(&input));
    }

    #[test]
    fn must_file_no_firms() {
        let mut input = basic_input(5_000);
        input.firms.clear();
        assert!(!Output8919::must_file(&input));
    }

    #[test]
    fn basic_wages() {
        let form = Output8919::try_new(basic_input(30_000)).unwrap();
        // Line 6: 30,000
        assert_eq!(
            form.total_wages_with_no_withholding_amt,
            Usd::from_dollars(30_000)
        );
        // Line 8: 40,000 + 10,000 + 0 + 0 = 50,000
        assert_eq!(
            form.total_wages_and_unreported_tips_amt,
            Usd::from_dollars(50_000)
        );
        // Line 9: 176,100 − 50,000 = 126,100
        assert_eq!(
            form.net_wages_subject_to_soc_sec_tax_amt,
            Usd::from_dollars(126_100)
        );
        // Line 10: min(30,000, 126,100) = 30,000
        assert_eq!(form.wages_subject_to_sst_amt, Usd::from_dollars(30_000));
        // Line 11: 30,000 × 0.062 = 1,860
        assert_eq!(form.uncollected_soc_sec_tax_amt, Usd::from_dollars(1_860));
        // Line 12: 30,000 × 0.0145 = 435
        assert_eq!(form.uncollected_medicare_tax_amt, Usd::from_dollars(435));
        // Line 13: 1,860 + 435 = 2,295
        assert_eq!(
            form.uncollected_soc_sec_med_tax_amt,
            Usd::from_dollars(2_295)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn ss_wage_base_cap() {
        let mut input = basic_input(30_000);
        input.w2_social_security_wages_amt = Usd::from_dollars(170_000);
        input.w2_social_security_tips_amt = Usd::from_dollars(5_000);
        let form = Output8919::try_new(input).unwrap();
        // Line 8: 170,000 + 5,000 + 0 + 0 = 175,000
        // Line 9: 176,100 − 175,000 = 1,100
        assert_eq!(
            form.net_wages_subject_to_soc_sec_tax_amt,
            Usd::from_dollars(1_100)
        );
        // Line 10: min(30,000, 1,100) = 1,100
        assert_eq!(form.wages_subject_to_sst_amt, Usd::from_dollars(1_100));
        // Line 11: 1,100 × 0.062 = 68.20
        assert_eq!(form.uncollected_soc_sec_tax_amt, Usd::from_cents(6_820));
        // Line 12: 30,000 × 0.0145 = 435 (Medicare still on full line 6)
        assert_eq!(form.uncollected_medicare_tax_amt, Usd::from_dollars(435));
        assert!(form.is_valid());
    }

    #[test]
    fn exceeded_ss_wage_base_zero_ss_tax() {
        let mut input = basic_input(30_000);
        input.w2_social_security_wages_amt = Usd::from_dollars(200_000);
        input.w2_social_security_tips_amt = Usd::ZERO;
        let form = Output8919::try_new(input).unwrap();
        // Line 9: 176,100 − 200,000 → 0
        assert_eq!(form.net_wages_subject_to_soc_sec_tax_amt, Usd::ZERO);
        // Line 10: min(30,000, 0) = 0
        assert_eq!(form.wages_subject_to_sst_amt, Usd::ZERO);
        // Line 11: 0
        assert_eq!(form.uncollected_soc_sec_tax_amt, Usd::ZERO);
        // Line 12: 30,000 × 0.0145 = 435 (Medicare still applies)
        assert_eq!(form.uncollected_medicare_tax_amt, Usd::from_dollars(435));
        assert!(form.is_valid());
    }

    #[test]
    fn multiple_firms() {
        let input = F8919Input {
            person_nm: "Jane Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            firms: vec![make_firm(10_000), make_firm(20_000)],
            w2_social_security_wages_amt: Usd::from_dollars(40_000),
            w2_social_security_tips_amt: Usd::from_dollars(10_000),
            rrta_compensation_amt: Usd::ZERO,
            f4137_unreported_tips_subj_to_soc_sec_amt: Usd::ZERO,
        };
        let form = Output8919::try_new(input).unwrap();
        // Line 6: 10,000 + 20,000 = 30,000
        assert_eq!(
            form.total_wages_with_no_withholding_amt,
            Usd::from_dollars(30_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn rrta_capped_at_wage_base() {
        let mut input = basic_input(30_000);
        input.w2_social_security_wages_amt = Usd::ZERO;
        input.w2_social_security_tips_amt = Usd::ZERO;
        // RRTA of $200,000 should be capped at $176,100
        input.rrta_compensation_amt = Usd::from_dollars(200_000);
        let form = Output8919::try_new(input).unwrap();
        // Line 8: 0 + 0 + min(200,000, 176,100) + 0 = 176,100
        assert_eq!(
            form.total_wages_and_unreported_tips_amt,
            Usd::from_dollars(176_100)
        );
        // Line 9: 176,100 − 176,100 = 0
        assert_eq!(form.net_wages_subject_to_soc_sec_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn rrta_below_wage_base_not_capped() {
        let mut input = basic_input(30_000);
        input.w2_social_security_wages_amt = Usd::from_dollars(30_000);
        input.w2_social_security_tips_amt = Usd::ZERO;
        input.rrta_compensation_amt = Usd::from_dollars(10_000);
        let form = Output8919::try_new(input).unwrap();
        // Line 8: 30,000 + 0 + min(10,000, 176,100) + 0 = 40,000
        assert_eq!(
            form.total_wages_and_unreported_tips_amt,
            Usd::from_dollars(40_000)
        );
        // Line 9: 176,100 − 40,000 = 136,100
        assert_eq!(
            form.net_wages_subject_to_soc_sec_tax_amt,
            Usd::from_dollars(136_100)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn f4137_tips_flow_into_line8() {
        let mut input = basic_input(30_000);
        input.f4137_unreported_tips_subj_to_soc_sec_amt = Usd::from_dollars(5_000);
        let form = Output8919::try_new(input).unwrap();
        // Line 8: 40,000 + 10,000 + 0 + 5,000 = 55,000
        assert_eq!(
            form.total_wages_and_unreported_tips_amt,
            Usd::from_dollars(55_000)
        );
        // Line 9: 176,100 − 55,000 = 121,100
        assert_eq!(
            form.net_wages_subject_to_soc_sec_tax_amt,
            Usd::from_dollars(121_100)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn reason_code_a_with_determination_date() {
        let input = F8919Input {
            person_nm: "Jane Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            firms: vec![F8919Lines1To5 {
                firm_nm: "Acme Corp".to_string(),
                firm_ein: "98-7654321".to_string(),
                reason_cd: F8919ReasonCode::A,
                determination_dt: "03/15/2025".to_string(),
                received_1099_ind: false,
                wages_with_no_withholding_amt: Usd::from_dollars(50_000),
            }],
            w2_social_security_wages_amt: Usd::from_dollars(40_000),
            w2_social_security_tips_amt: Usd::ZERO,
            rrta_compensation_amt: Usd::ZERO,
            f4137_unreported_tips_subj_to_soc_sec_amt: Usd::ZERO,
        };
        let form = Output8919::try_new(input).unwrap();
        assert_eq!(
            form.uncollected_soc_sec_med_tax_per_firm[0].reason_cd,
            F8919ReasonCode::A
        );
        assert_eq!(
            form.uncollected_soc_sec_med_tax_per_firm[0].determination_dt,
            "03/15/2025"
        );
        assert!(form.is_valid());
    }

    #[test]
    fn zero_wages_zero_tax() {
        let mut input = basic_input(0);
        input.firms = vec![make_firm(0)];
        let form = Output8919::try_new(input).unwrap();
        assert_eq!(form.total_wages_with_no_withholding_amt, Usd::ZERO);
        assert_eq!(form.uncollected_soc_sec_tax_amt, Usd::ZERO);
        assert_eq!(form.uncollected_medicare_tax_amt, Usd::ZERO);
        assert_eq!(form.uncollected_soc_sec_med_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
