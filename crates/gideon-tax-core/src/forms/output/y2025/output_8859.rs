use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8859.
///
/// The credit carryforward from the prior year and the tax liability limit
/// (from the Tax Liability Limit Worksheet) are the two inputs. The current
/// year credit and next-year carryforward are computed from these.
#[derive(Debug, Clone)]
pub struct F8859Input {
    /// Line 1: Credit carryforward from prior year (2024 Form 8859, line 4)
    pub dc_hm_byr_credit_carryforward_py_amt: Usd,
    /// Line 2: Limitation based on tax liability (from Tax Liability Limit Worksheet)
    pub tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8859 (2025) — Carryforward of the District of Columbia First-Time Homebuyer Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8859 {
    /// Line 1: Credit carryforward from 2024. Enter the amount from line 4 of your 2024 Form 8859
    pub dc_hm_byr_credit_carryforward_py_amt: Usd,
    /// Line 2: Limitation based on tax liability. Enter the amount from the Tax Liability Limit Worksheet
    pub tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd,
    /// Line 3: Current year credit. Enter the smaller of line 1 or line 2
    pub dc_hm_byr_current_year_credit_amt: Usd,
    /// Line 4: Credit carryforward to 2026. Subtract line 3 from line 1
    pub dc_hm_byr_credit_cfwd_next_year_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8859 {
    fn name() -> &'static str {
        "Form 8859"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8859 {
    type Input = F8859Input;

    fn must_file(input: &Self::Input) -> bool {
        input.dc_hm_byr_credit_carryforward_py_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        let line1 = input.dc_hm_byr_credit_carryforward_py_amt;
        let line2 = input.tax_liab_lmt_from_cr_lmt_wrksht_amt;

        // Line 3: current year credit = min(line 1, line 2)
        let line3 = line1.min(line2);

        // Line 4: carryforward to next year = line 1 − line 3
        let line4 = line1 - line3;

        Ok(Output8859 {
            dc_hm_byr_credit_carryforward_py_amt: line1,
            tax_liab_lmt_from_cr_lmt_wrksht_amt: line2,
            dc_hm_byr_current_year_credit_amt: line3,
            dc_hm_byr_credit_cfwd_next_year_amt: line4,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::F1040]
    }

    fn is_valid(&self) -> bool {
        // Line 3 = min(Line 1, Line 2)
        let line3_ok = self.dc_hm_byr_current_year_credit_amt
            == self
                .dc_hm_byr_credit_carryforward_py_amt
                .min(self.tax_liab_lmt_from_cr_lmt_wrksht_amt);

        // Line 4 = Line 1 − Line 3
        let line4_ok = self.dc_hm_byr_credit_cfwd_next_year_amt
            == self.dc_hm_byr_credit_carryforward_py_amt
                - self.dc_hm_byr_current_year_credit_amt;

        line3_ok && line4_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input(carryforward: i64, tax_limit: i64) -> F8859Input {
        F8859Input {
            dc_hm_byr_credit_carryforward_py_amt: Usd::from_dollars(carryforward),
            tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd::from_dollars(tax_limit),
        }
    }

    #[test]
    fn must_file_with_carryforward() {
        let input = basic_input(5_000, 3_000);
        assert!(Output8859::must_file(&input));
    }

    #[test]
    fn must_file_no_carryforward() {
        let input = basic_input(0, 3_000);
        assert!(!Output8859::must_file(&input));
    }

    #[test]
    fn credit_limited_by_tax_liability() {
        let form = Output8859::try_new(basic_input(5_000, 3_000)).unwrap();
        // Line 1: 5,000
        assert_eq!(
            form.dc_hm_byr_credit_carryforward_py_amt,
            Usd::from_dollars(5_000)
        );
        // Line 2: 3,000
        assert_eq!(
            form.tax_liab_lmt_from_cr_lmt_wrksht_amt,
            Usd::from_dollars(3_000)
        );
        // Line 3: min(5,000, 3,000) = 3,000
        assert_eq!(
            form.dc_hm_byr_current_year_credit_amt,
            Usd::from_dollars(3_000)
        );
        // Line 4: 5,000 − 3,000 = 2,000
        assert_eq!(
            form.dc_hm_byr_credit_cfwd_next_year_amt,
            Usd::from_dollars(2_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn credit_limited_by_carryforward() {
        let form = Output8859::try_new(basic_input(2_000, 5_000)).unwrap();
        // Line 3: min(2,000, 5,000) = 2,000
        assert_eq!(
            form.dc_hm_byr_current_year_credit_amt,
            Usd::from_dollars(2_000)
        );
        // Line 4: 2,000 − 2,000 = 0
        assert_eq!(form.dc_hm_byr_credit_cfwd_next_year_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn equal_carryforward_and_limit() {
        let form = Output8859::try_new(basic_input(4_000, 4_000)).unwrap();
        // Line 3: min(4,000, 4,000) = 4,000
        assert_eq!(
            form.dc_hm_byr_current_year_credit_amt,
            Usd::from_dollars(4_000)
        );
        // Line 4: 4,000 − 4,000 = 0
        assert_eq!(form.dc_hm_byr_credit_cfwd_next_year_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn zero_carryforward_zero_credit() {
        let form = Output8859::try_new(basic_input(0, 5_000)).unwrap();
        assert_eq!(form.dc_hm_byr_current_year_credit_amt, Usd::ZERO);
        assert_eq!(form.dc_hm_byr_credit_cfwd_next_year_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn zero_tax_liability_full_carryforward() {
        let form = Output8859::try_new(basic_input(5_000, 0)).unwrap();
        // Line 3: min(5,000, 0) = 0
        assert_eq!(form.dc_hm_byr_current_year_credit_amt, Usd::ZERO);
        // Line 4: 5,000 − 0 = 5,000
        assert_eq!(
            form.dc_hm_byr_credit_cfwd_next_year_amt,
            Usd::from_dollars(5_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn cents_precision() {
        let input = F8859Input {
            dc_hm_byr_credit_carryforward_py_amt: Usd::from_cents(12_345),
            tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd::from_cents(10_000),
        };
        let form = Output8859::try_new(input).unwrap();
        // Line 3: min(123.45, 100.00) = 100.00
        assert_eq!(
            form.dc_hm_byr_current_year_credit_amt,
            Usd::from_cents(10_000)
        );
        // Line 4: 123.45 − 100.00 = 23.45
        assert_eq!(
            form.dc_hm_byr_credit_cfwd_next_year_amt,
            Usd::from_cents(2_345)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn dependencies_includes_f1040() {
        assert_eq!(Output8859::dependencies(), &[DynForm::F1040]);
    }
}
