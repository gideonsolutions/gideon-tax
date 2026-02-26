use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 3903.
///
/// This form is available only to members of the Armed Forces on active
/// duty who move pursuant to a military order related to a permanent
/// change of station. The W-2 dependency arises because employer
/// reimbursements (box 12, code P) feed into Line 4.
#[derive(Debug, Clone)]
pub struct F3903Input {
    /// Checkbox: Certify that you are a Member of the Armed Forces on
    /// active duty with a permanent change of station
    pub eligibility_requirement_met_ind: bool,
    /// Military move code (type of permanent change of station)
    pub military_move_cd: String,
    /// Line 1: Transportation and storage of household goods and
    /// personal effects
    pub transport_household_goods_amt: Usd,
    /// Line 2: Travel (including lodging) from your old home to your
    /// new home. Do not include the cost of meals
    pub travel_and_lodging_expense_amt: Usd,
    /// Line 4: Enter the total amount the government paid you for the
    /// expenses listed on lines 1 and 2 that is not included in box 1
    /// of your Form W-2 (shown in box 12 with code P)
    pub total_employer_expenses_paid_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 3903 (2025) — Moving Expenses.
#[derive(Debug, Clone)]
pub struct Output3903 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Checkbox: Certify that you are a Member of the Armed Forces on active duty with a
    /// permanent change of station
    pub eligibility_requirement_met_ind: bool,
    /// Military move code (type of permanent change of station)
    pub military_move_cd: String,
    /// Indicator: Whether moving expenses are deductible (line 3 is more than line 4)
    pub moving_expenses_deductible_ind: bool,

    // -----------------------------------------------------------------------
    // Lines 1-5
    // -----------------------------------------------------------------------
    /// Line 1: Transportation and storage of household goods and personal effects
    pub transport_household_goods_amt: Usd,
    /// Line 2: Travel (including lodging) from your old home to your new home. Do not include
    /// the cost of meals
    pub travel_and_lodging_expense_amt: Usd,
    /// Line 3: Add lines 1 and 2
    pub total_moving_expense_amt: Usd,
    /// Line 4: Enter the total amount the government paid you for the expenses listed on
    /// lines 1 and 2 that is not included in box 1 of your Form W-2 (shown in box 12 with
    /// code P)
    pub total_employer_expenses_paid_amt: Usd,
    /// Line 5: Subtract line 4 from line 3. This is your moving expense deduction
    pub moving_deduction_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output3903 {
    fn name() -> &'static str {
        "Form 3903"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output3903 {
    type Input = F3903Input;

    fn must_file(input: &Self::Input) -> bool {
        input.transport_household_goods_amt > Usd::ZERO
            || input.travel_and_lodging_expense_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Line 3: Line 1 + Line 2
        let line3 = input.transport_household_goods_amt + input.travel_and_lodging_expense_amt;

        // Line 5: Line 3 - Line 4 (min 0)
        let line5 = (line3 - input.total_employer_expenses_paid_amt).max(Usd::ZERO);

        // Deductible indicator: Line 3 > Line 4
        let deductible = line3 > input.total_employer_expenses_paid_amt;

        Ok(Output3903 {
            eligibility_requirement_met_ind: input.eligibility_requirement_met_ind,
            military_move_cd: input.military_move_cd,
            moving_expenses_deductible_ind: deductible,
            transport_household_goods_amt: input.transport_household_goods_amt,
            travel_and_lodging_expense_amt: input.travel_and_lodging_expense_amt,
            total_moving_expense_amt: line3,
            total_employer_expenses_paid_amt: input.total_employer_expenses_paid_amt,
            moving_deduction_amt: line5,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::W2]
    }

    fn is_valid(&self) -> bool {
        // Line 3 = Line 1 + Line 2
        let line3_ok = self.total_moving_expense_amt
            == self.transport_household_goods_amt + self.travel_and_lodging_expense_amt;

        // Line 5 = max(Line 3 - Line 4, 0)
        let line5_ok = self.moving_deduction_amt
            == (self.total_moving_expense_amt - self.total_employer_expenses_paid_amt)
                .max(Usd::ZERO);

        // Deductible indicator = Line 3 > Line 4
        let deductible_ok = self.moving_expenses_deductible_ind
            == (self.total_moving_expense_amt > self.total_employer_expenses_paid_amt);

        line3_ok && line5_ok && deductible_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_input(line1: i64, line2: i64) -> F3903Input {
        F3903Input {
            eligibility_requirement_met_ind: true,
            military_move_cd: "A".to_string(),
            transport_household_goods_amt: Usd::from_dollars(line1),
            travel_and_lodging_expense_amt: Usd::from_dollars(line2),
            total_employer_expenses_paid_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_with_transport_expenses() {
        let input = basic_input(5_000, 0);
        assert!(Output3903::must_file(&input));
    }

    #[test]
    fn must_file_with_travel_expenses() {
        let input = basic_input(0, 2_000);
        assert!(Output3903::must_file(&input));
    }

    #[test]
    fn must_file_no_expenses() {
        let input = basic_input(0, 0);
        assert!(!Output3903::must_file(&input));
    }

    #[test]
    fn basic_moving_expenses() {
        let form = Output3903::try_new(basic_input(5_000, 2_000)).unwrap();
        // Line 1: 5,000
        assert_eq!(
            form.transport_household_goods_amt,
            Usd::from_dollars(5_000)
        );
        // Line 2: 2,000
        assert_eq!(
            form.travel_and_lodging_expense_amt,
            Usd::from_dollars(2_000)
        );
        // Line 3: 5,000 + 2,000 = 7,000
        assert_eq!(form.total_moving_expense_amt, Usd::from_dollars(7_000));
        // Line 4: 0
        assert_eq!(form.total_employer_expenses_paid_amt, Usd::ZERO);
        // Line 5: 7,000 - 0 = 7,000
        assert_eq!(form.moving_deduction_amt, Usd::from_dollars(7_000));
        // Deductible: 7,000 > 0 = true
        assert!(form.moving_expenses_deductible_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn partial_reimbursement() {
        let mut input = basic_input(5_000, 2_000);
        input.total_employer_expenses_paid_amt = Usd::from_dollars(3_000);
        let form = Output3903::try_new(input).unwrap();
        // Line 3: 7,000
        assert_eq!(form.total_moving_expense_amt, Usd::from_dollars(7_000));
        // Line 5: 7,000 - 3,000 = 4,000
        assert_eq!(form.moving_deduction_amt, Usd::from_dollars(4_000));
        // Deductible: 7,000 > 3,000 = true
        assert!(form.moving_expenses_deductible_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn full_reimbursement() {
        let mut input = basic_input(5_000, 2_000);
        input.total_employer_expenses_paid_amt = Usd::from_dollars(7_000);
        let form = Output3903::try_new(input).unwrap();
        // Line 3: 7,000
        assert_eq!(form.total_moving_expense_amt, Usd::from_dollars(7_000));
        // Line 5: 7,000 - 7,000 = 0
        assert_eq!(form.moving_deduction_amt, Usd::ZERO);
        // Deductible: 7,000 > 7,000 = false
        assert!(!form.moving_expenses_deductible_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn over_reimbursement_floors_at_zero() {
        let mut input = basic_input(5_000, 2_000);
        input.total_employer_expenses_paid_amt = Usd::from_dollars(10_000);
        let form = Output3903::try_new(input).unwrap();
        // Line 3: 7,000
        assert_eq!(form.total_moving_expense_amt, Usd::from_dollars(7_000));
        // Line 5: max(7,000 - 10,000, 0) = 0
        assert_eq!(form.moving_deduction_amt, Usd::ZERO);
        // Deductible: 7,000 > 10,000 = false
        assert!(!form.moving_expenses_deductible_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn passthrough_fields() {
        let mut input = basic_input(1_000, 500);
        input.eligibility_requirement_met_ind = true;
        input.military_move_cd = "B".to_string();
        let form = Output3903::try_new(input).unwrap();
        assert!(form.eligibility_requirement_met_ind);
        assert_eq!(form.military_move_cd, "B");
        assert!(form.is_valid());
    }

    #[test]
    fn zero_expenses_zero_deduction() {
        let form = Output3903::try_new(basic_input(0, 0)).unwrap();
        assert_eq!(form.total_moving_expense_amt, Usd::ZERO);
        assert_eq!(form.moving_deduction_amt, Usd::ZERO);
        assert!(!form.moving_expenses_deductible_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn only_transport_expenses() {
        let form = Output3903::try_new(basic_input(8_000, 0)).unwrap();
        // Line 3: 8,000 + 0 = 8,000
        assert_eq!(form.total_moving_expense_amt, Usd::from_dollars(8_000));
        // Line 5: 8,000 - 0 = 8,000
        assert_eq!(form.moving_deduction_amt, Usd::from_dollars(8_000));
        assert!(form.moving_expenses_deductible_ind);
        assert!(form.is_valid());
    }

    #[test]
    fn only_travel_expenses() {
        let form = Output3903::try_new(basic_input(0, 3_500)).unwrap();
        // Line 3: 0 + 3,500 = 3,500
        assert_eq!(form.total_moving_expense_amt, Usd::from_dollars(3_500));
        // Line 5: 3,500 - 0 = 3,500
        assert_eq!(form.moving_deduction_amt, Usd::from_dollars(3_500));
        assert!(form.moving_expenses_deductible_ind);
        assert!(form.is_valid());
    }
}
