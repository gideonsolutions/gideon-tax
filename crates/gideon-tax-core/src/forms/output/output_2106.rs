use crate::Usd;

/// Output fields for IRS Form 2106 (2025) — Employee Business Expenses.
#[derive(Debug, Clone, Default)]
pub struct Output2106 {
    // -----------------------------------------------------------------------
    // Part II, Section C — Actual Expenses (Lines 23–29)
    // -----------------------------------------------------------------------
    /// Line 29: Add lines 27 and 28 — total actual vehicle expense. Enter total here and on line 1
    pub actual_vehicle_expense_amt: Usd,
    /// Calculated business expense amount
    pub calculated_business_expense_amt: Usd,
    /// Line 28: Depreciation (see instructions)
    pub depreciation_amt: Usd,
    /// Line 24b: Inclusion amount (see instructions)
    pub inclusion_amt: Usd,
    /// Line 24c: Subtract line 24b from line 24a (rental minus inclusion)
    pub rental_minus_inclusion_amt: Usd,
    /// Line 26: Add lines 23, 24c, and 25 — total actual expenses before business-use percentage
    pub total_actual_expense_amt: Usd,
    /// Line 25: Value of employer-provided vehicle (see instructions)
    pub value_emplr_provided_vehicle_amt: Usd,
    /// Line 23: Gasoline, oil, repairs, vehicle insurance, etc.
    pub vehicle_gas_repairs_ins_amt: Usd,
    /// Line 24a: Vehicle rentals
    pub vehicle_rentals_amt: Usd,

    // -----------------------------------------------------------------------
    // Part I — Employee Business Expenses and Reimbursements (Lines 1–10)
    // -----------------------------------------------------------------------
    /// Line 10: Add the amounts on line 9 for both columns — allowable business deduction
    pub allowable_business_deduction_amt: Usd,
    /// Allowable meals deduction amount
    pub allowable_meals_deduction_amt: Usd,
    /// Line 19: Do you (or your spouse) have another vehicle available for personal use?
    pub another_vehicle_for_prsnl_use_ind: bool,
    /// Line 8 (Column A): Business expenses less meals
    pub bus_expnss_less_meals_entrmt_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II, Section D — Depreciation of Vehicles (Lines 30–38)
    // -----------------------------------------------------------------------
    /// Line 34: Multiply line 32 by the percentage on line 33 — calculated depreciation
    pub calculated_depreciation_amt: Usd,
    /// Line 33: Enter depreciation method and percentage
    pub depreciation_method_type_txt: String,
    /// Line 38: Enter the smaller of line 35 or line 37 — depreciation of vehicle
    pub depreciation_of_vehicle_amt: Usd,
    /// Line 32: Multiply line 30 by line 14 (depreciation recovery amount)
    pub depreciation_recovery_amt: Usd,
    /// Line 35: Add lines 31 and 34 — depreciation subtotal
    pub depreciation_subtotal_amt: Usd,
    /// Line 36: Enter the applicable limit explained in the line 36 instructions
    pub limitation_amt: Usd,
    /// Line 37: Multiply line 36 by the percentage on line 14
    pub limitation_multiplied_by_use_amt: Usd,
    /// Line 14: Percent of business use (divide line 13 by line 12)
    pub pct: String,
    /// Line 31: Enter section 179 deduction and special allowance
    pub section179_deduction_amt: Usd,
    /// Line 30: Enter cost or other basis (see instructions)
    pub vehicle_basis_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II, Section A — General Information (Lines 11–21)
    // -----------------------------------------------------------------------
    /// Line 20: Do you have evidence to support your deduction?
    pub evidence_to_support_deduction_ind: bool,
    /// Line 21: If "Yes," is the evidence written?
    pub evidence_written_ind: bool,

    // -----------------------------------------------------------------------
    // Step 1 — Enter Your Expenses (Lines 1–6)
    // -----------------------------------------------------------------------
    /// Line 5: Meals expenses (see instructions)
    pub meals_and_entertainment_amt: Usd,
    /// Meals and entertainment reimbursements not reported on W-2
    pub meals_entrmt_reimb_not_rpt_w2_amt: Usd,
    /// Occupation in which you incurred expenses
    pub occupation_txt: String,
    /// Other reimbursements not reported on W-2
    pub other_reimb_not_rpt_on_w2_amt: Usd,
    /// Line 2: Parking fees, tolls, and transportation (didn't involve overnight travel)
    pub parking_fees_tolls_local_trans_amt: Usd,
    /// Your name
    pub person_nm: String,
    /// Social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part II, Section B — Standard Mileage Rate (Line 22)
    // -----------------------------------------------------------------------
    /// Line 22: Multiply line 13 by 70c (0.70). Enter the result here and on line 1
    pub standard_mileage_deduction_amt: Usd,

    // -----------------------------------------------------------------------
    // Step 3 — Figure Expenses To Deduct (Lines 8–10)
    // -----------------------------------------------------------------------
    /// Line 9 (Column A): Total expenses less meals
    pub tot_expnss_less_meals_entrmt_amt: Usd,
    /// Line 6 (Column B): Total meals and entertainment amount
    pub total_meals_and_entrmt_amt: Usd,
    /// Line 3: Travel expense while away from home overnight (don't include meals)
    pub trav_expns_less_meals_entrmt_amt: Usd,
    /// Line 8: Subtract line 7 from line 6 — unreimbursed employee business expenses
    pub unreim_employee_bus_expns_amt: Usd,
    /// Unreimbursed business expense amount
    pub unreimbursed_business_expense_amt: Usd,
    /// Unreimbursed meals expense amount
    pub unreimbursed_meals_expense_amt: Usd,
    /// Line 18: Was your vehicle available for personal use during off-duty hours?
    pub vehicle_available_off_duty_hrs_ind: bool,
    /// Line 1: Vehicle expense from line 22 or line 29
    pub vehicle_expense_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II, Section A — Vehicle Mileage (Lines 12–17)
    // -----------------------------------------------------------------------
    /// Line 15: Average daily roundtrip commuting distance (miles)
    pub average_distance_cnt: u32,
    /// Line 13: Business miles included on line 12
    pub business_miles_cnt: u32,
    /// Line 16: Commuting miles included on line 12
    pub miles_commuting_cnt: u32,
    /// Line 17: Other personal miles (add lines 13 and 16 and subtract from line 12)
    pub other_personal_miles_cnt: u32,
    /// Line 12: Total miles the vehicle was driven during 2025
    pub total_miles_cnt: u32,
    /// Line 14: Percent of business use (divide line 13 by line 12)
    pub veh_bus_investment_use_pct: String,
    /// Line 11: Date vehicle was placed in service
    pub vehicle_placed_in_service_dt: String,
}
