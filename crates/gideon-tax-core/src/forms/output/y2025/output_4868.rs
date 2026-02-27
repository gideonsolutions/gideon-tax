use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 4868.
///
/// Lines 4, 5, 7 are dollar amounts supplied by the filer;
/// Line 6 (balance due) is computed in [`OutputForm::try_new`].
/// Lines 8 and 9 are checkbox indicators.
#[derive(Debug, Clone)]
pub struct F4868Input {
    // -----------------------------------------------------------------------
    // Part I — Identification
    // -----------------------------------------------------------------------
    /// Line 1: Your name(s). If you plan to file a joint return, include both
    /// spouses' names in the order in which they will appear on the return.
    pub name_line1_txt: String,
    /// Address
    pub address: String,
    /// City, town, or post office
    pub city: String,
    /// State
    pub state: String,
    /// ZIP code
    pub zip_code: String,
    /// Line 2: Your social security number
    pub ssn: String,
    /// Line 3: Spouse's social security number
    pub spouse_ssn: String,

    // -----------------------------------------------------------------------
    // Part II — Individual Income Tax
    // -----------------------------------------------------------------------
    /// Line 4: Estimate of total tax liability for 2025
    pub total_tax_liability_amt: Usd,
    /// Line 5: Total 2025 payments
    pub total_payments_amt: Usd,
    /// Line 7: Amount you're paying
    pub taxpayer_is_paying_amt: Usd,
    /// Line 8: Check here if you're "out of the country" and a U.S. citizen or resident
    pub taxpayer_abroad_ind: bool,
    /// Line 9: Check here if you file Form 1040-NR and didn't receive wages
    /// as an employee subject to U.S. income tax withholding
    pub nonres_with_no_wages_subj_to_wh_ind: bool,

    // -----------------------------------------------------------------------
    // Filing decision
    // -----------------------------------------------------------------------
    /// Whether the filer needs an automatic extension of time to file
    pub need_extension: bool,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 4868 (2025) — Application for Automatic Extension
/// of Time To File U.S. Individual Income Tax Return.
#[derive(Debug, Clone)]
pub struct Output4868 {
    // -----------------------------------------------------------------------
    // Part I — Identification
    // -----------------------------------------------------------------------
    /// Line 1: Your name(s). If you plan to file a joint return, include both
    /// spouses' names in the order in which they will appear on the return.
    pub name_line1_txt: String,
    /// Address
    pub address: String,
    /// City, town, or post office
    pub city: String,
    /// State
    pub state: String,
    /// ZIP code
    pub zip_code: String,
    /// Line 2: Your social security number
    pub ssn: String,
    /// Line 3: Spouse's social security number
    pub spouse_ssn: String,

    // -----------------------------------------------------------------------
    // Part II — Individual Income Tax
    // -----------------------------------------------------------------------
    /// Line 4: Estimate of total tax liability for 2025
    pub total_tax_liability_amt: Usd,
    /// Line 5: Total 2025 payments
    pub total_payments_amt: Usd,
    /// Line 6: Balance due. Subtract line 5 from line 4. If line 5 is more than line 4, enter 0.
    pub balance_due_amt: Usd,
    /// Line 7: Amount you're paying
    pub taxpayer_is_paying_amt: Usd,
    /// Line 8: Check here if you're "out of the country" and a U.S. citizen or resident
    pub taxpayer_abroad_ind: bool,
    /// Line 9: Check here if you file Form 1040-NR and didn't receive wages
    /// as an employee subject to U.S. income tax withholding
    pub nonres_with_no_wages_subj_to_wh_ind: bool,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output4868 {
    fn name() -> &'static str {
        "Form 4868"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output4868 {
    type Input = F4868Input;

    fn must_file(input: &Self::Input) -> bool {
        input.need_extension
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Line 6: Subtract line 5 from line 4. If line 5 is more than line 4, enter 0.
        let balance_due = if input.total_payments_amt >= input.total_tax_liability_amt {
            Usd::ZERO
        } else {
            input.total_tax_liability_amt - input.total_payments_amt
        };

        Ok(Output4868 {
            // Part I
            name_line1_txt: input.name_line1_txt,
            address: input.address,
            city: input.city,
            state: input.state,
            zip_code: input.zip_code,
            ssn: input.ssn,
            spouse_ssn: input.spouse_ssn,

            // Part II
            total_tax_liability_amt: input.total_tax_liability_amt,
            total_payments_amt: input.total_payments_amt,
            balance_due_amt: balance_due,
            taxpayer_is_paying_amt: input.taxpayer_is_paying_amt,
            taxpayer_abroad_ind: input.taxpayer_abroad_ind,
            nonres_with_no_wages_subj_to_wh_ind: input.nonres_with_no_wages_subj_to_wh_ind,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        // Form 4868 is filed before the return; the estimates on lines 4–5
        // are supplied directly by the filer, not pulled from other forms.
        &[]
    }

    fn is_valid(&self) -> bool {
        let expected_balance = if self.total_payments_amt >= self.total_tax_liability_amt {
            Usd::ZERO
        } else {
            self.total_tax_liability_amt - self.total_payments_amt
        };

        self.balance_due_amt == expected_balance
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input() -> F4868Input {
        F4868Input {
            name_line1_txt: "John Doe and Jane Doe".to_string(),
            address: "123 Main St".to_string(),
            city: "Anytown".to_string(),
            state: "CA".to_string(),
            zip_code: "90210".to_string(),
            ssn: "123-45-6789".to_string(),
            spouse_ssn: "987-65-4321".to_string(),
            total_tax_liability_amt: Usd::from_dollars(10_000),
            total_payments_amt: Usd::from_dollars(8_000),
            taxpayer_is_paying_amt: Usd::from_dollars(1_000),
            taxpayer_abroad_ind: false,
            nonres_with_no_wages_subj_to_wh_ind: false,
            need_extension: true,
        }
    }

    #[test]
    fn must_file_when_extension_needed() {
        let input = basic_input();
        assert!(Output4868::must_file(&input));
    }

    #[test]
    fn must_file_no_extension() {
        let mut input = basic_input();
        input.need_extension = false;
        assert!(!Output4868::must_file(&input));
    }

    #[test]
    fn balance_due_basic() {
        let form = Output4868::try_new(basic_input()).unwrap();
        // 10,000 - 8,000 = 2,000
        assert_eq!(form.balance_due_amt, Usd::from_dollars(2_000));
        assert!(form.is_valid());
    }

    #[test]
    fn balance_due_zero_when_overpaid() {
        let mut input = basic_input();
        input.total_payments_amt = Usd::from_dollars(12_000);
        let form = Output4868::try_new(input).unwrap();
        assert_eq!(form.balance_due_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn balance_due_zero_when_exactly_paid() {
        let mut input = basic_input();
        input.total_payments_amt = Usd::from_dollars(10_000);
        let form = Output4868::try_new(input).unwrap();
        assert_eq!(form.balance_due_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn balance_due_equals_liability_when_no_payments() {
        let mut input = basic_input();
        input.total_payments_amt = Usd::ZERO;
        let form = Output4868::try_new(input).unwrap();
        assert_eq!(form.balance_due_amt, Usd::from_dollars(10_000));
        assert!(form.is_valid());
    }

    #[test]
    fn zero_liability_zero_payments() {
        let mut input = basic_input();
        input.total_tax_liability_amt = Usd::ZERO;
        input.total_payments_amt = Usd::ZERO;
        input.taxpayer_is_paying_amt = Usd::ZERO;
        let form = Output4868::try_new(input).unwrap();
        assert_eq!(form.balance_due_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn passthrough_fields_preserved() {
        let form = Output4868::try_new(basic_input()).unwrap();
        assert_eq!(form.name_line1_txt, "John Doe and Jane Doe");
        assert_eq!(form.address, "123 Main St");
        assert_eq!(form.city, "Anytown");
        assert_eq!(form.state, "CA");
        assert_eq!(form.zip_code, "90210");
        assert_eq!(form.ssn, "123-45-6789");
        assert_eq!(form.spouse_ssn, "987-65-4321");
        assert_eq!(form.total_tax_liability_amt, Usd::from_dollars(10_000));
        assert_eq!(form.total_payments_amt, Usd::from_dollars(8_000));
        assert_eq!(form.taxpayer_is_paying_amt, Usd::from_dollars(1_000));
        assert!(!form.taxpayer_abroad_ind);
        assert!(!form.nonres_with_no_wages_subj_to_wh_ind);
    }

    #[test]
    fn taxpayer_abroad_preserved() {
        let mut input = basic_input();
        input.taxpayer_abroad_ind = true;
        let form = Output4868::try_new(input).unwrap();
        assert!(form.taxpayer_abroad_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn nonres_no_wages_preserved() {
        let mut input = basic_input();
        input.nonres_with_no_wages_subj_to_wh_ind = true;
        let form = Output4868::try_new(input).unwrap();
        assert!(form.nonres_with_no_wages_subj_to_wh_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn cents_precision_balance_due() {
        let mut input = basic_input();
        input.total_tax_liability_amt = Usd::from_cents(1_000_099);
        input.total_payments_amt = Usd::from_cents(800_050);
        let form = Output4868::try_new(input).unwrap();
        // 1,000,099 - 800,050 = 200,049 cents
        assert_eq!(form.balance_due_amt, Usd::from_cents(200_049));
        assert!(form.is_valid());
    }

    #[test]
    fn form_name_and_year() {
        assert_eq!(Output4868::name(), "Form 4868");
        let form = Output4868::try_new(basic_input()).unwrap();
        assert_eq!(form.year(), TaxYear::Y2025);
        assert_eq!(<Output4868 as Form>::form_type(), FormType::Output);
    }

    #[test]
    fn no_dependencies() {
        assert!(Output4868::dependencies().is_empty());
    }
}
