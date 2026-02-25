use crate::Usd;

/// Output fields for IRS Schedule C (Form 1040) — Profit or Loss From Business (2025).
///
/// Fields are ordered by line number as they appear on the form.
/// Covers header fields (A--J), Part I (Income), Part II (Expenses),
/// Part III (Cost of Goods Sold), and Part V total (line 48).
/// Excludes Part IV (Information on Your Vehicle) and Part V individual
/// line items, which are variable-length groups.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleC {
    // -----------------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------------
    /// Name of proprietor
    pub proprietor_nm: String,
    /// Social security number (SSN)
    pub ssn: String,
    /// Line A: Principal business or profession, including product or service (see instructions)
    pub principal_business_activity_desc: String,
    /// Line B: Enter code from instructions
    pub principal_business_activity_cd: String,
    /// Unclassified establishment code
    pub unclassified_establishment_cd: String,
    /// Line C: Business name. If no separate business name, leave blank.
    pub business_name_line_1_txt: String,
    /// Line D: Employer ID number (EIN) (see instr.)
    pub ein: String,
    /// Line E: Business address (including suite or room no.) -- US address line 1
    pub business_us_address_line_1_txt: String,
    /// Line E: Business address -- US address line 2
    pub business_us_address_line_2_txt: String,
    /// City, town or post office, state, and ZIP code -- city
    pub business_us_city_nm: String,
    /// City, town or post office, state, and ZIP code -- state abbreviation
    pub business_us_state_abbreviation_cd: String,
    /// City, town or post office, state, and ZIP code -- ZIP code
    pub business_us_zip_cd: String,
    /// Line E: Business address -- foreign address line 1
    pub business_foreign_address_line_1_txt: String,
    /// Line E: Business address -- foreign address line 2
    pub business_foreign_address_line_2_txt: String,
    /// Business foreign address -- city
    pub business_foreign_city_nm: String,
    /// Business foreign address -- province or state name
    pub business_foreign_province_or_state_nm: String,
    /// Business foreign address -- country code
    pub business_foreign_country_cd: String,
    /// Business foreign address -- foreign postal code
    pub business_foreign_postal_cd: String,
    /// Line F(1): Accounting method -- Cash
    pub method_of_accounting_cash_ind: bool,
    /// Line F(2): Accounting method -- Accrual
    pub method_of_accounting_accrual_ind: bool,
    /// Line F(3): Accounting method -- Other (specify)
    pub method_of_accounting_other_ind: bool,
    /// Line F(3): Other accounting method description
    pub method_of_accounting_other_desc: String,
    /// Line G: Did you "materially participate" in the operation of this business during 2025? If "No," see instructions for limit on losses
    pub material_participation_in_cy_ind: bool,
    /// Line H: If you started or acquired this business during 2025, check here
    pub new_business_in_current_year_ind: bool,
    /// Line I: Did you make any payments in 2025 that would require you to file Form(s) 1099? See instructions
    pub payment_rqr_filing_form_1099_ind: bool,
    /// Line J: If "Yes," did you or will you file required Form(s) 1099?
    pub required_form_1099_filed_ind: bool,
    /// Line 1 checkbox: Check the box if this income was reported to you on Form W-2 and the "Statutory employee" box on that form was checked
    pub statutory_employee_from_w2_ind: bool,

    // -----------------------------------------------------------------------
    // Part I -- Income
    // -----------------------------------------------------------------------
    /// Line 1: Gross receipts or sales. See instructions for line 1 and check the box if this income was reported to you on Form W-2 and the "Statutory employee" box on that form was checked
    pub total_gross_receipts_amt: Usd,
    /// Line 2: Returns and allowances
    pub returns_and_allowances_amt: Usd,
    /// Line 3: Subtract line 2 from line 1
    pub net_gross_receipts_amt: Usd,
    /// Line 4: Cost of goods sold (from line 42)
    pub cost_of_goods_sold_amt: Usd,
    /// Line 5: Gross profit. Subtract line 4 from line 3
    pub gross_profit_amt: Usd,
    /// Line 6: Other income, including federal and state gasoline or fuel tax credit or refund (see instructions)
    pub other_income_amt: Usd,
    /// Line 7: Gross income. Add lines 5 and 6
    pub gross_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II -- Expenses
    // -----------------------------------------------------------------------
    /// Line 8: Advertising
    pub advertising_amt: Usd,
    /// Line 9: Car and truck expenses (see instructions)
    pub car_and_truck_expenses_amt: Usd,
    /// Line 10: Commissions and fees
    pub commissions_and_fees_amt: Usd,
    /// Line 11: Contract labor (see instructions)
    pub contract_labor_amt: Usd,
    /// Line 12: Depletion
    pub depletion_amt: Usd,
    /// Line 13: Depreciation and section 179 expense deduction (not included in Part III) (see instructions)
    pub deprec_and_sect_179_expns_ded_amt: Usd,
    /// Line 14: Employee benefit programs (other than on line 19)
    pub employee_benefit_program_amt: Usd,
    /// Line 15: Insurance (other than health)
    pub insurance_amt: Usd,
    /// Line 16a: Mortgage (paid to banks, etc.)
    pub mortgage_interest_paid_banks_amt: Usd,
    /// Line 16b: Other
    pub mortgage_interest_paid_other_amt: Usd,
    /// Line 17: Legal and professional services
    pub legal_and_professional_service_amt: Usd,
    /// Line 18: Office expense (see instructions)
    pub office_expenses_amt: Usd,
    /// Line 19: Pension and profit-sharing plans
    pub pension_profit_sharing_plans_amt: Usd,
    /// Line 20a: Vehicles, machinery, and equipment
    pub machinery_and_equipment_rent_amt: Usd,
    /// Line 20b: Other business property
    pub other_business_property_rent_amt: Usd,
    /// Line 21: Repairs and maintenance
    pub repairs_and_maintenance_amt: Usd,
    /// Line 22: Supplies (not included in Part III)
    pub supplies_amt: Usd,
    /// Line 23: Taxes and licenses
    pub taxes_and_licenses_amt: Usd,
    /// Line 24a: Travel
    pub travel_amt: Usd,
    /// Line 24b: Deductible meals (see instructions)
    pub meals_and_entertainment_amt: Usd,
    /// Line 25: Utilities
    pub utilities_amt: Usd,
    /// Line 26: Wages (less employment credits)
    pub wages_less_employment_credits_amt: Usd,
    /// Line 27a: Energy efficient commercial bldgs deduction (attach Form 7205)
    pub energy_effcnt_cmrcl_bldg_ded_amt: Usd,
    /// Line 27b: Other expenses (from line 48)
    pub total_other_expenses_amt: Usd,
    /// Line 27b: Other expense code
    pub other_expense_cd: String,
    /// Line 28: Total expenses before expenses for business use of home. Add lines 8 through 27b
    pub total_expenses_amt: Usd,
    /// Line 29: Tentative profit or (loss). Subtract line 28 from line 7
    pub tentative_profit_or_loss_amt: Usd,
    /// Line 30: Expenses for business use of your home. Do not report these expenses elsewhere. Attach Form 8829 unless using the simplified method. See instructions.
    pub home_business_expense_amt: Usd,
    /// Simplified method filers only: Enter the total square footage of (a) your home
    pub total_area_of_home_cnt: u32,
    /// Simplified method filers only: (b) the part of your home used for business
    pub home_business_use_square_feet_cnt: u32,
    /// Line 31: Net profit or (loss). Subtract line 30 from line 29
    pub net_profit_or_loss_amt: Usd,
    /// Line 31: Passive activity loss amount
    pub passive_activity_loss_amt: Usd,
    /// Line 31: Passive activity loss literal code
    pub passive_activity_loss_literal_cd: String,

    // -----------------------------------------------------------------------
    // Line 32 -- At-risk indicators
    // -----------------------------------------------------------------------
    /// Line 32a: All investment is at risk
    pub all_investment_is_at_risk_ind: bool,
    /// Line 32b: Some investment is not at risk
    pub some_investment_is_not_at_risk_ind: bool,

    // -----------------------------------------------------------------------
    // Part III -- Cost of Goods Sold
    // -----------------------------------------------------------------------
    /// Line 33a: Method(s) used to value closing inventory -- Cost
    pub closing_inventory_cost_method_ind: bool,
    /// Line 33b: Method(s) used to value closing inventory -- Lower of cost or market
    pub lower_of_cost_or_market_method_ind: bool,
    /// Line 33c: Method(s) used to value closing inventory -- Other (attach explanation)
    pub other_closing_inventory_method_ind: bool,
    /// Line 34: Was there any change in determining quantities, costs, or valuations between opening and closing inventory?
    pub change_in_valuations_ind: bool,
    /// Line 35: Inventory at beginning of year. If different from last year's closing inventory, attach explanation
    pub beginning_of_year_inventory_amt: Usd,
    /// Line 36: Purchases less cost of items withdrawn for personal use
    pub purchases_less_personal_items_amt: Usd,
    /// Line 37: Cost of labor. Do not include any amounts paid to yourself
    pub cost_of_labor_amt: Usd,
    /// Line 38: Materials and supplies
    pub materials_and_supplies_amt: Usd,
    /// Line 39: Other costs
    pub other_costs_amt: Usd,
    /// Line 40: Add lines 35 through 39
    pub total_costs_amt: Usd,
    /// Line 41: Inventory at end of year
    pub end_of_year_inventory_amt: Usd,
}
