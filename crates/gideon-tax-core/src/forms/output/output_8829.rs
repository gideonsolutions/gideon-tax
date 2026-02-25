use crate::Usd;

/// Output fields for IRS Form 8829 (2025) — Expenses for Business Use of Your Home.
#[derive(Debug, Clone, Default)]
pub struct Output8829 {
    // -----------------------------------------------------------------------
    // Top-of-form — Proprietor information
    // -----------------------------------------------------------------------
    /// Name of proprietor(s)
    pub name_of_proprietor: String,
    /// Proprietor name
    pub proprietor_nm: String,
    /// Your social security number
    pub ssn: String,
    /// Social security number
    pub social_security_number: String,
    /// Form or Schedule number (e.g., Schedule C)
    pub form_schedule_number: String,

    // -----------------------------------------------------------------------
    // Part I — Part of Your Home Used for Business
    // -----------------------------------------------------------------------
    /// Line 1: Area used regularly and exclusively for business, regularly for daycare, or for storage of inventory or product samples (square feet)
    pub business_use_square_feet_cnt: u32,
    /// Line 2: Total area of home (square feet)
    pub total_area_of_home_cnt: u32,
    /// Line 3: Divide line 1 by line 2. Enter the result as a percentage (business square feet percentage)
    pub business_square_feet_pct: String,
    /// Line 4: Multiply days used for daycare during year by hours used per day (hours)
    pub business_use_hours_cnt: u32,
    /// Line 5: Total hours available for use during the year (if you started or stopped using your home for daycare during the year, see instructions; otherwise, enter 8,760)
    pub total_hours_available_cnt: u32,
    /// Line 7: Business percentage. For daycare facilities not used exclusively for business, multiply line 6 by line 3. All others, enter the amount from line 3
    pub business_pct: String,

    // -----------------------------------------------------------------------
    // Part II — Figure Your Allowable Deduction
    // -----------------------------------------------------------------------
    /// Line 8: Enter the amount from Schedule C, line 29, plus any gain derived from the business use of your home, minus any loss from the trade or business not derived from the business use of your home
    pub home_business_income_amt: Usd,
    /// Line 8: Home business gain or (loss) amount
    pub home_business_gain_or_loss_amt: Usd,
    /// Line 9: Casualty losses (see instructions) — (a) Direct expenses
    pub casualty_losses_direct_amt: Usd,
    /// Line 9: Casualty losses (see instructions) — (b) Indirect expenses
    pub casualty_losses_indirect_amt: Usd,
    /// Line 10: Deductible mortgage interest (see instructions) — (a) Direct expenses
    pub deductible_mortgage_int_direct_amt: Usd,
    /// Line 10: Deductible mortgage interest (see instructions) — (b) Indirect expenses
    pub deductible_mortgage_int_indr_amt: Usd,
    /// Line 11: Real estate taxes (see instructions) — (a) Direct expenses
    pub real_estate_taxes_direct_amt: Usd,
    /// Line 11: Real estate taxes (see instructions) — (b) Indirect expenses
    pub real_estate_taxes_indirect_amt: Usd,
    /// Line 13: Multiply line 12, column (b), by line 7
    pub direct_and_indirect_expns_subtl_amt: Usd,
    /// Line 14: Add line 12, column (a), and line 13
    pub direct_deducted_expnss_subtl_amt: Usd,
    /// Line 15: Subtract line 14 from line 8. If zero or less, enter -0-
    pub nondeductible_net_expenses_amt: Usd,
    /// Line 16: Excess mortgage interest (see instructions) — (a) Direct expenses
    pub excess_mortgage_int_direct_amt: Usd,
    /// Line 16: Excess mortgage interest (see instructions) — (b) Indirect expenses
    pub excess_mortgage_int_indirect_amt: Usd,
    /// Line 17: Excess real estate taxes (see instructions) — (a) Direct expenses
    pub excess_real_estate_txs_direct_amt: Usd,
    /// Line 17: Excess real estate taxes (see instructions) — (b) Indirect expenses
    pub excess_real_estate_txs_indirect_amt: Usd,
    /// Line 18: Insurance — (a) Direct expenses
    pub insurance_direct_amt: Usd,
    /// Line 18: Insurance — (b) Indirect expenses
    pub insurance_indirect_amt: Usd,
    /// Line 19: Rent — (a) Direct expenses
    pub rent_direct_amt: Usd,
    /// Line 19: Rent — (b) Indirect expenses
    pub rent_indirect_amt: Usd,
    /// Line 20: Repairs and maintenance — (a) Direct expenses
    pub repairs_and_maint_direct_amt: Usd,
    /// Line 20: Repairs and maintenance — (b) Indirect expenses
    pub repairs_and_maint_indirect_amt: Usd,
    /// Line 21: Utilities — (a) Direct expenses
    pub utilities_direct_amt: Usd,
    /// Line 21: Utilities — (b) Indirect expenses
    pub utilities_indirect_amt: Usd,
    /// Line 22: Other expenses (see instructions) — (a) Direct expenses
    pub other_expenses_direct_amt: Usd,
    /// Line 22: Other expenses (see instructions) — (b) Indirect expenses
    pub other_expenses_indirect_amt: Usd,
    /// Line 24: Multiply line 23, column (b), by line 7
    pub indirect_deducted_expnss_subtl_amt: Usd,
    /// Line 25: Carryover of prior year operating expenses (see instructions)
    pub operating_expenses_carryover_amt: Usd,
    /// Line 26: Add line 23, column (a), line 24, and line 25
    pub operating_expenses_amt: Usd,
    /// Line 27: Allowable operating expenses. Enter the smaller of line 15 or line 26
    pub allowable_operating_expenses_amt: Usd,
    /// Line 28: Limit on excess casualty losses and depreciation. Subtract line 27 from line 15
    pub cslty_losses_and_deprec_limit_amt: Usd,
    /// Line 29: Excess casualty losses (see instructions) — (a) Direct expenses
    pub excess_casualty_losses_amt: Usd,
    /// Line 30: Depreciation of your home from line 42 below
    pub allowable_home_depreciation_amt: Usd,
    /// Line 31: Carryover of prior year excess casualty losses and depreciation (see instructions)
    pub cyov_ex_cslty_losses_and_deprec_amt: Usd,
    /// Line 32: Add lines 29 through 31
    pub casualty_losses_and_deprec_net_amt: Usd,
    /// Line 33: Allowable excess casualty losses and depreciation. Enter the smaller of line 28 or line 32
    pub allwbl_ex_cslty_losses_deprec_amt: Usd,
    /// Line 34: Add lines 14, 27, and 33
    pub total_allowable_expenses_amt: Usd,
    /// Line 35: Casualty loss portion, if any, from lines 14 and 33. Carry amount to Form 4684. See instructions
    pub casualty_loss_portion_amt: Usd,
    /// Line 36: Allowable expenses for business use of your home. Subtract line 35 from line 34. Enter here and on Schedule C, line 30
    pub allowable_home_bus_expnss_sch_c_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Depreciation of Your Home
    // -----------------------------------------------------------------------
    /// Line 37: Enter the smaller of your home's adjusted basis or its fair market value. See instructions
    pub home_adj_basis_or_fair_market_amt: Usd,
    /// Line 38: Value of land included on line 37
    pub value_of_land_amt: Usd,
    /// Line 39: Basis of building. Subtract line 38 from line 37
    pub basis_of_building_amt: Usd,
    /// Line 40: Business basis of building. Multiply line 39 by line 7
    pub business_basis_of_building_amt: Usd,
    /// Line 41: Depreciation percentage (see instructions)
    pub depreciation_pct: String,

    // -----------------------------------------------------------------------
    // Part IV — Carryover of Unallowed Expenses to 2026
    // -----------------------------------------------------------------------
    /// Line 43: Operating expenses. Subtract line 27 from line 26. If less than zero, enter -0-
    pub allowable_indr_deducted_expnss_amt: Usd,
    /// Line 43: Indirect nondeducted operating expenses subtotal
    pub indirect_nondeducted_subtotal_amt: Usd,
    /// Line 43: Direct nondeducted subtotal
    pub direct_nondeducted_subtotal_amt: Usd,
    /// Line 43: Allowable indirect nondeducted expenses
    pub allwbl_indr_nondeducted_expnss_amt: Usd,
    /// Line 44: Excess casualty losses and depreciation. Subtract line 33 from line 32. If less than zero, enter -0-
    pub excess_cslty_losses_and_deprec_amt: Usd,
}
