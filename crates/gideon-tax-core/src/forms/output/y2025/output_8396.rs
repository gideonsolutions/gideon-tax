use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8396 — Mortgage Interest Credit.
///
/// The certificate credit rate is passed as a string (e.g., "0.20") because the
/// IRS form displays it verbatim.  The product of line 1 and line 2 is
/// pre-computed by the caller and provided as `mortgage_interest_credit_amt`
/// (line 3), so we do not need to parse the rate.
#[derive(Debug, Clone)]
pub struct F8396Input {
    // ── Header ──────────────────────────────────────────────────────────
    /// Name of issuer of Mortgage Credit Certificate
    pub mortg_sbsdy_cert_issuer_agency_nm: String,
    /// Mortgage Credit Certificate number
    pub mortgage_credit_certificate_num: String,
    /// Issue date
    pub mortg_cr_certificate_issue_dt: String,
    /// Address of main home to which the qualified mortgage certificate relates
    pub qlfy_mortgage_cert_us_address: String,

    // ── Part I ──────────────────────────────────────────────────────────
    /// Line 1: Interest paid on the certified indebtedness amount
    pub certified_mortgage_int_cr_pd_amt: Usd,
    /// Line 2: Certificate credit rate shown on Mortgage Credit Certificate
    pub mortgage_credit_certificate_rt: String,
    /// Line 3: Mortgage interest credit amount (line 1 × line 2, pre-computed)
    pub mortgage_interest_credit_amt: Usd,
    /// Line 4: 2022 credit carryforward (3-year-old)
    pub mortg_int_previous3_yr_cfwd_cr_amt: Usd,
    /// Line 5: 2023 credit carryforward (2-year-old)
    pub mortg_int_previous2_yr_cfwd_cr_amt: Usd,
    /// Line 6: 2024 credit carryforward (prior year)
    pub mortg_int_py_carryforward_cr_amt: Usd,
    /// Line 8: Tax liability limit (from Credit Limit Worksheet)
    pub tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8396 (2025) — Mortgage Interest Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8396 {
    // -----------------------------------------------------------------------
    // Header Information
    // -----------------------------------------------------------------------
    /// Header: Name of issuer of Mortgage Credit Certificate
    pub mortg_sbsdy_cert_issuer_agency_nm: String,
    /// Header: Mortgage Credit Certificate number
    pub mortgage_credit_certificate_num: String,
    /// Header: Issue date
    pub mortg_cr_certificate_issue_dt: String,
    /// Header: Address of main home to which the qualified mortgage certificate relates
    pub qlfy_mortgage_cert_us_address: String,

    // -----------------------------------------------------------------------
    // Part I — Current-Year Mortgage Interest Credit
    // -----------------------------------------------------------------------
    /// Line 1: Interest paid on the certified indebtedness amount
    pub certified_mortgage_int_cr_pd_amt: Usd,
    /// Line 2: Certificate credit rate shown on Mortgage Credit Certificate
    pub mortgage_credit_certificate_rt: String,
    /// Line 3: Mortgage interest credit amount (line 1 times line 2, or limited amount)
    pub mortgage_interest_credit_amt: Usd,
    /// Line 4: 2022 credit carryforward from line 16 of 2024 Form 8396
    pub mortg_int_previous3_yr_cfwd_cr_amt: Usd,
    /// Line 5: 2023 credit carryforward from line 14 of 2024 Form 8396
    pub mortg_int_previous2_yr_cfwd_cr_amt: Usd,
    /// Line 6: 2024 credit carryforward from line 17 of 2024 Form 8396
    pub mortg_int_py_carryforward_cr_amt: Usd,
    /// Line 7: Sum of lines 3 through 6
    pub larger_of_mortg_int_cr_or_cfwd_amt: Usd,
    /// Line 8: Limitation based on tax liability (from Credit Limit Worksheet line 3)
    pub tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd,
    /// Line 9: Current-year mortgage interest credit (smaller of line 7 or line 8)
    pub mortgage_interest_reduction_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Mortgage Interest Credit Carryforward to 2026
    // -----------------------------------------------------------------------
    /// Line 10: Add lines 3 and 4
    pub mortg_int_red_plus_oldest_cfwd_cr_amt: Usd,
    /// Line 11: Enter the amount from line 7
    pub mortg_int_tot_previous_cfwd_cr_amt: Usd,
    /// Line 12: Enter the larger of line 9 or line 10
    pub mortg_int_tent3_year_cfwd_cr_amt: Usd,
    /// Line 13: Subtract line 12 from line 11
    pub mortg_int_tent_two_year_cfwd_cr_amt: Usd,
    /// Line 14: 2024 credit carryforward to 2026 (smaller of line 6 or line 13)
    pub mortg_int_next_years_py_cfwd_cr_amt: Usd,
    /// Line 15: Subtract line 14 from line 13
    pub mortg_int_next_years2_yr_cfwd_cr_amt: Usd,
    /// Line 16: 2023 credit carryforward to 2026 (smaller of line 5 or line 15)
    pub mortg_int_next_years3_yr_cfwd_cr_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8396 {
    fn name() -> &'static str {
        "Form 8396"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8396 {
    type Input = F8396Input;

    fn must_file(input: &Self::Input) -> bool {
        input.certified_mortgage_int_cr_pd_amt > Usd::ZERO
            || input.mortg_int_previous3_yr_cfwd_cr_amt > Usd::ZERO
            || input.mortg_int_previous2_yr_cfwd_cr_amt > Usd::ZERO
            || input.mortg_int_py_carryforward_cr_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        let line3 = input.mortgage_interest_credit_amt;
        let line4 = input.mortg_int_previous3_yr_cfwd_cr_amt;
        let line5 = input.mortg_int_previous2_yr_cfwd_cr_amt;
        let line6 = input.mortg_int_py_carryforward_cr_amt;

        // Line 7: line 3 + line 4 + line 5 + line 6
        let line7 = line3 + line4 + line5 + line6;

        let line8 = input.tax_liab_lmt_from_cr_lmt_wrksht_amt;

        // Line 9: min(line 7, line 8)
        let line9 = line7.min(line8);

        // ── Part II ─────────────────────────────────────────────────────
        // Line 10: line 3 + line 4
        let line10 = line3 + line4;

        // Line 11: same as line 7
        let line11 = line7;

        // Line 12: max(line 9, line 10)
        let line12 = line9.max(line10);

        // Line 13: line 11 - line 12
        let line13 = (line11 - line12).max(Usd::ZERO);

        // Line 14: min(line 6, line 13)
        let line14 = line6.min(line13);

        // Line 15: line 13 - line 14
        let line15 = line13 - line14;

        // Line 16: min(line 5, line 15)
        let line16 = line5.min(line15);

        Ok(Output8396 {
            // Header
            mortg_sbsdy_cert_issuer_agency_nm: input.mortg_sbsdy_cert_issuer_agency_nm,
            mortgage_credit_certificate_num: input.mortgage_credit_certificate_num,
            mortg_cr_certificate_issue_dt: input.mortg_cr_certificate_issue_dt,
            qlfy_mortgage_cert_us_address: input.qlfy_mortgage_cert_us_address,
            // Part I
            certified_mortgage_int_cr_pd_amt: input.certified_mortgage_int_cr_pd_amt,
            mortgage_credit_certificate_rt: input.mortgage_credit_certificate_rt,
            mortgage_interest_credit_amt: line3,
            mortg_int_previous3_yr_cfwd_cr_amt: line4,
            mortg_int_previous2_yr_cfwd_cr_amt: line5,
            mortg_int_py_carryforward_cr_amt: line6,
            larger_of_mortg_int_cr_or_cfwd_amt: line7,
            tax_liab_lmt_from_cr_lmt_wrksht_amt: line8,
            mortgage_interest_reduction_amt: line9,
            // Part II
            mortg_int_red_plus_oldest_cfwd_cr_amt: line10,
            mortg_int_tot_previous_cfwd_cr_amt: line11,
            mortg_int_tent3_year_cfwd_cr_amt: line12,
            mortg_int_tent_two_year_cfwd_cr_amt: line13,
            mortg_int_next_years_py_cfwd_cr_amt: line14,
            mortg_int_next_years2_yr_cfwd_cr_amt: line15,
            mortg_int_next_years3_yr_cfwd_cr_amt: line16,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040]
    }

    fn is_valid(&self) -> bool {
        let line3 = self.mortgage_interest_credit_amt;
        let line4 = self.mortg_int_previous3_yr_cfwd_cr_amt;
        let line5 = self.mortg_int_previous2_yr_cfwd_cr_amt;
        let line6 = self.mortg_int_py_carryforward_cr_amt;
        let line7 = self.larger_of_mortg_int_cr_or_cfwd_amt;
        let line8 = self.tax_liab_lmt_from_cr_lmt_wrksht_amt;
        let line9 = self.mortgage_interest_reduction_amt;
        let line10 = self.mortg_int_red_plus_oldest_cfwd_cr_amt;
        let line11 = self.mortg_int_tot_previous_cfwd_cr_amt;
        let line12 = self.mortg_int_tent3_year_cfwd_cr_amt;
        let line13 = self.mortg_int_tent_two_year_cfwd_cr_amt;
        let line14 = self.mortg_int_next_years_py_cfwd_cr_amt;
        let line15 = self.mortg_int_next_years2_yr_cfwd_cr_amt;
        let line16 = self.mortg_int_next_years3_yr_cfwd_cr_amt;

        // Line 7 = line 3 + line 4 + line 5 + line 6
        let line7_ok = line7 == line3 + line4 + line5 + line6;

        // Line 9 = min(line 7, line 8)
        let line9_ok = line9 == line7.min(line8);

        // Line 10 = line 3 + line 4
        let line10_ok = line10 == line3 + line4;

        // Line 11 = line 7
        let line11_ok = line11 == line7;

        // Line 12 = max(line 9, line 10)
        let line12_ok = line12 == line9.max(line10);

        // Line 13 = line 11 - line 12
        let line13_ok = line13 == (line11 - line12).max(Usd::ZERO);

        // Line 14 = min(line 6, line 13)
        let line14_ok = line14 == line6.min(line13);

        // Line 15 = line 13 - line 14
        let line15_ok = line15 == line13 - line14;

        // Line 16 = min(line 5, line 15)
        let line16_ok = line16 == line5.min(line15);

        line7_ok
            && line9_ok
            && line10_ok
            && line11_ok
            && line12_ok
            && line13_ok
            && line14_ok
            && line15_ok
            && line16_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F8396Input {
        F8396Input {
            mortg_sbsdy_cert_issuer_agency_nm: "City Housing Authority".to_string(),
            mortgage_credit_certificate_num: "MCC-2025-001".to_string(),
            mortg_cr_certificate_issue_dt: "01/15/2025".to_string(),
            qlfy_mortgage_cert_us_address: "123 Main St, Anytown, US 12345".to_string(),
            certified_mortgage_int_cr_pd_amt: Usd::from_dollars(10_000),
            mortgage_credit_certificate_rt: "0.20".to_string(),
            mortgage_interest_credit_amt: Usd::from_dollars(2_000),
            mortg_int_previous3_yr_cfwd_cr_amt: Usd::ZERO,
            mortg_int_previous2_yr_cfwd_cr_amt: Usd::ZERO,
            mortg_int_py_carryforward_cr_amt: Usd::ZERO,
            tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd::from_dollars(5_000),
        }
    }

    #[test]
    fn must_file_with_interest() {
        let input = basic_input();
        assert!(Output8396::must_file(&input));
    }

    #[test]
    fn must_file_with_carryforward_only() {
        let mut input = basic_input();
        input.certified_mortgage_int_cr_pd_amt = Usd::ZERO;
        input.mortgage_interest_credit_amt = Usd::ZERO;
        input.mortg_int_previous2_yr_cfwd_cr_amt = Usd::from_dollars(100);
        assert!(Output8396::must_file(&input));
    }

    #[test]
    fn must_file_no_credit_no_carryforward() {
        let mut input = basic_input();
        input.certified_mortgage_int_cr_pd_amt = Usd::ZERO;
        input.mortgage_interest_credit_amt = Usd::ZERO;
        input.mortg_int_previous3_yr_cfwd_cr_amt = Usd::ZERO;
        input.mortg_int_previous2_yr_cfwd_cr_amt = Usd::ZERO;
        input.mortg_int_py_carryforward_cr_amt = Usd::ZERO;
        assert!(!Output8396::must_file(&input));
    }

    #[test]
    fn basic_credit_no_carryforward() {
        let form = Output8396::try_new(basic_input()).unwrap();
        // Line 3: 2,000
        assert_eq!(form.mortgage_interest_credit_amt, Usd::from_dollars(2_000));
        // Line 7: 2,000 + 0 + 0 + 0 = 2,000
        assert_eq!(
            form.larger_of_mortg_int_cr_or_cfwd_amt,
            Usd::from_dollars(2_000)
        );
        // Line 9: min(2,000, 5,000) = 2,000
        assert_eq!(
            form.mortgage_interest_reduction_amt,
            Usd::from_dollars(2_000)
        );
        // Line 10: 2,000 + 0 = 2,000
        assert_eq!(
            form.mortg_int_red_plus_oldest_cfwd_cr_amt,
            Usd::from_dollars(2_000)
        );
        // Line 11: 2,000
        assert_eq!(
            form.mortg_int_tot_previous_cfwd_cr_amt,
            Usd::from_dollars(2_000)
        );
        // Line 12: max(2,000, 2,000) = 2,000
        assert_eq!(
            form.mortg_int_tent3_year_cfwd_cr_amt,
            Usd::from_dollars(2_000)
        );
        // Line 13: 2,000 - 2,000 = 0
        assert_eq!(form.mortg_int_tent_two_year_cfwd_cr_amt, Usd::ZERO);
        // Line 14: min(0, 0) = 0
        assert_eq!(form.mortg_int_next_years_py_cfwd_cr_amt, Usd::ZERO);
        // Line 15: 0 - 0 = 0
        assert_eq!(form.mortg_int_next_years2_yr_cfwd_cr_amt, Usd::ZERO);
        // Line 16: min(0, 0) = 0
        assert_eq!(form.mortg_int_next_years3_yr_cfwd_cr_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn tax_liability_limits_credit() {
        let mut input = basic_input();
        input.tax_liab_lmt_from_cr_lmt_wrksht_amt = Usd::from_dollars(1_000);
        let form = Output8396::try_new(input).unwrap();
        // Line 7: 2,000
        assert_eq!(
            form.larger_of_mortg_int_cr_or_cfwd_amt,
            Usd::from_dollars(2_000)
        );
        // Line 9: min(2,000, 1,000) = 1,000
        assert_eq!(
            form.mortgage_interest_reduction_amt,
            Usd::from_dollars(1_000)
        );
        // Line 10: 2,000
        // Line 12: max(1,000, 2,000) = 2,000
        assert_eq!(
            form.mortg_int_tent3_year_cfwd_cr_amt,
            Usd::from_dollars(2_000)
        );
        // Line 13: 2,000 - 2,000 = 0
        assert_eq!(form.mortg_int_tent_two_year_cfwd_cr_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn carryforwards_all_years() {
        let mut input = basic_input();
        input.mortg_int_previous3_yr_cfwd_cr_amt = Usd::from_dollars(500);
        input.mortg_int_previous2_yr_cfwd_cr_amt = Usd::from_dollars(300);
        input.mortg_int_py_carryforward_cr_amt = Usd::from_dollars(200);
        input.tax_liab_lmt_from_cr_lmt_wrksht_amt = Usd::from_dollars(2_500);
        let form = Output8396::try_new(input).unwrap();
        // Line 7: 2,000 + 500 + 300 + 200 = 3,000
        assert_eq!(
            form.larger_of_mortg_int_cr_or_cfwd_amt,
            Usd::from_dollars(3_000)
        );
        // Line 9: min(3,000, 2,500) = 2,500
        assert_eq!(
            form.mortgage_interest_reduction_amt,
            Usd::from_dollars(2_500)
        );
        // Line 10: 2,000 + 500 = 2,500
        assert_eq!(
            form.mortg_int_red_plus_oldest_cfwd_cr_amt,
            Usd::from_dollars(2_500)
        );
        // Line 11: 3,000
        assert_eq!(
            form.mortg_int_tot_previous_cfwd_cr_amt,
            Usd::from_dollars(3_000)
        );
        // Line 12: max(2,500, 2,500) = 2,500
        assert_eq!(
            form.mortg_int_tent3_year_cfwd_cr_amt,
            Usd::from_dollars(2_500)
        );
        // Line 13: 3,000 - 2,500 = 500
        assert_eq!(
            form.mortg_int_tent_two_year_cfwd_cr_amt,
            Usd::from_dollars(500)
        );
        // Line 14: min(200, 500) = 200
        assert_eq!(
            form.mortg_int_next_years_py_cfwd_cr_amt,
            Usd::from_dollars(200)
        );
        // Line 15: 500 - 200 = 300
        assert_eq!(
            form.mortg_int_next_years2_yr_cfwd_cr_amt,
            Usd::from_dollars(300)
        );
        // Line 16: min(300, 300) = 300
        assert_eq!(
            form.mortg_int_next_years3_yr_cfwd_cr_amt,
            Usd::from_dollars(300)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn carryforward_limited_by_remaining() {
        let mut input = basic_input();
        input.mortgage_interest_credit_amt = Usd::from_dollars(1_000);
        input.mortg_int_previous3_yr_cfwd_cr_amt = Usd::from_dollars(200);
        input.mortg_int_previous2_yr_cfwd_cr_amt = Usd::from_dollars(800);
        input.mortg_int_py_carryforward_cr_amt = Usd::from_dollars(500);
        // tax liability limit allows only 1,500 of the 2,500 total
        input.tax_liab_lmt_from_cr_lmt_wrksht_amt = Usd::from_dollars(1_500);
        let form = Output8396::try_new(input).unwrap();
        // Line 7: 1,000 + 200 + 800 + 500 = 2,500
        assert_eq!(
            form.larger_of_mortg_int_cr_or_cfwd_amt,
            Usd::from_dollars(2_500)
        );
        // Line 9: min(2,500, 1,500) = 1,500
        assert_eq!(
            form.mortgage_interest_reduction_amt,
            Usd::from_dollars(1_500)
        );
        // Line 10: 1,000 + 200 = 1,200
        assert_eq!(
            form.mortg_int_red_plus_oldest_cfwd_cr_amt,
            Usd::from_dollars(1_200)
        );
        // Line 12: max(1,500, 1,200) = 1,500
        assert_eq!(
            form.mortg_int_tent3_year_cfwd_cr_amt,
            Usd::from_dollars(1_500)
        );
        // Line 13: 2,500 - 1,500 = 1,000
        assert_eq!(
            form.mortg_int_tent_two_year_cfwd_cr_amt,
            Usd::from_dollars(1_000)
        );
        // Line 14: min(500, 1,000) = 500
        assert_eq!(
            form.mortg_int_next_years_py_cfwd_cr_amt,
            Usd::from_dollars(500)
        );
        // Line 15: 1,000 - 500 = 500
        assert_eq!(
            form.mortg_int_next_years2_yr_cfwd_cr_amt,
            Usd::from_dollars(500)
        );
        // Line 16: min(800, 500) = 500
        assert_eq!(
            form.mortg_int_next_years3_yr_cfwd_cr_amt,
            Usd::from_dollars(500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn zero_credit_zero_carryforward() {
        let mut input = basic_input();
        input.certified_mortgage_int_cr_pd_amt = Usd::ZERO;
        input.mortgage_interest_credit_amt = Usd::ZERO;
        input.tax_liab_lmt_from_cr_lmt_wrksht_amt = Usd::ZERO;
        let form = Output8396::try_new(input).unwrap();
        assert_eq!(form.larger_of_mortg_int_cr_or_cfwd_amt, Usd::ZERO);
        assert_eq!(form.mortgage_interest_reduction_amt, Usd::ZERO);
        assert_eq!(form.mortg_int_tent_two_year_cfwd_cr_amt, Usd::ZERO);
        assert_eq!(form.mortg_int_next_years_py_cfwd_cr_amt, Usd::ZERO);
        assert_eq!(form.mortg_int_next_years2_yr_cfwd_cr_amt, Usd::ZERO);
        assert_eq!(form.mortg_int_next_years3_yr_cfwd_cr_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn header_passthrough() {
        let input = basic_input();
        let form = Output8396::try_new(input).unwrap();
        assert_eq!(form.mortg_sbsdy_cert_issuer_agency_nm, "City Housing Authority");
        assert_eq!(form.mortgage_credit_certificate_num, "MCC-2025-001");
        assert_eq!(form.mortg_cr_certificate_issue_dt, "01/15/2025");
        assert_eq!(
            form.qlfy_mortgage_cert_us_address,
            "123 Main St, Anytown, US 12345"
        );
        assert_eq!(form.mortgage_credit_certificate_rt, "0.20");
        assert!(form.is_valid());
    }

    #[test]
    fn dependencies_include_f1040() {
        assert_eq!(Output8396::dependencies(), &[DynForm::F1040]);
    }

    #[test]
    fn carryforward_only_no_current_year_credit() {
        let mut input = basic_input();
        input.certified_mortgage_int_cr_pd_amt = Usd::ZERO;
        input.mortgage_interest_credit_amt = Usd::ZERO;
        input.mortg_int_previous3_yr_cfwd_cr_amt = Usd::from_dollars(100);
        input.mortg_int_previous2_yr_cfwd_cr_amt = Usd::from_dollars(200);
        input.mortg_int_py_carryforward_cr_amt = Usd::from_dollars(300);
        input.tax_liab_lmt_from_cr_lmt_wrksht_amt = Usd::from_dollars(400);
        let form = Output8396::try_new(input).unwrap();
        // Line 7: 0 + 100 + 200 + 300 = 600
        assert_eq!(
            form.larger_of_mortg_int_cr_or_cfwd_amt,
            Usd::from_dollars(600)
        );
        // Line 9: min(600, 400) = 400
        assert_eq!(
            form.mortgage_interest_reduction_amt,
            Usd::from_dollars(400)
        );
        // Line 10: 0 + 100 = 100
        assert_eq!(
            form.mortg_int_red_plus_oldest_cfwd_cr_amt,
            Usd::from_dollars(100)
        );
        // Line 12: max(400, 100) = 400
        assert_eq!(
            form.mortg_int_tent3_year_cfwd_cr_amt,
            Usd::from_dollars(400)
        );
        // Line 13: 600 - 400 = 200
        assert_eq!(
            form.mortg_int_tent_two_year_cfwd_cr_amt,
            Usd::from_dollars(200)
        );
        // Line 14: min(300, 200) = 200
        assert_eq!(
            form.mortg_int_next_years_py_cfwd_cr_amt,
            Usd::from_dollars(200)
        );
        // Line 15: 200 - 200 = 0
        assert_eq!(form.mortg_int_next_years2_yr_cfwd_cr_amt, Usd::ZERO);
        // Line 16: min(200, 0) = 0
        assert_eq!(form.mortg_int_next_years3_yr_cfwd_cr_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
