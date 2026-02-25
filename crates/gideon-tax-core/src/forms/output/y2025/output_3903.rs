use crate::Usd;

/// Output fields for IRS Form 3903 (2025) — Moving Expenses.
#[derive(Debug, Clone, Default)]
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
