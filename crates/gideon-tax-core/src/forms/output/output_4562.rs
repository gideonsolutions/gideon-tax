use crate::Usd;

/// Output fields for IRS Form 4562 (2025) — Depreciation and Amortization.
#[derive(Debug, Clone, Default)]
pub struct Output4562 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Business or activity to which this form relates
    pub business_or_activity_txt: String,

    // -----------------------------------------------------------------------
    // Part I — Election To Expense Certain Property Under Section 179
    // -----------------------------------------------------------------------
    /// Line 1: Maximum amount (see instructions)
    pub maximum_dollar_limitation_amt: Usd,
    /// Line 2: Total cost of section 179 property placed in service (see instructions)
    pub total_cost_of_section179_prop_amt: Usd,
    /// Line 3: Threshold cost of section 179 property before reduction in limitation
    pub threshold_cost_of_sect179_prop_amt: Usd,
    /// Line 4: Reduction in limitation. Subtract line 3 from line 2. If zero or less,
    /// enter -0-
    pub reduction_in_limitation_amt: Usd,
    /// Line 5: Dollar limitation for tax year. Subtract line 4 from line 1. If zero or
    /// less, enter -0-. If married filing separately, see instructions
    pub dollar_limitation_for_tax_year_amt: Usd,
    /// Line 6: Elected property (table with description, cost, and elected cost)
    pub elected_property: String,
    /// Line 7: Listed property. Enter the amount from line 29
    pub total_listed_depreciation_amt: Usd,
    /// Line 8: Total elected cost of section 179 property. Add amounts in column (c),
    /// lines 6 and 7
    pub total_elected_cost_sect179_prop_amt: Usd,
    /// Line 9: Tentative deduction. Enter the smaller of line 5 or line 8
    pub tentative_deduction_amt: Usd,
    /// Line 10: Carryover of disallowed deduction from line 13 of your 2024 Form 4562
    pub disallowed_deduction_cyov_amt: Usd,
    /// Line 11: Business income limitation. Enter the smaller of business income (not less
    /// than zero) or line 5
    pub business_income_limitation_amt: Usd,
    /// Line 12: Section 179 expense deduction. Add lines 9 and 10, but do not enter more
    /// than line 11
    pub section179_expense_deduction_amt: Usd,
    /// Line 12: Section 179 expense deduction summary code
    pub section179_expns_ded_summary_cd: String,
    /// Line 13: Carryover of disallowed deduction to 2026. Add lines 9 and 10, less line 12
    pub next_year_carryover_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Special Depreciation Allowance and Other Depreciation
    //           (Do not include listed property)
    // -----------------------------------------------------------------------
    /// Line 14: Special depreciation allowance for qualified property (other than listed
    /// property) placed in service during the tax year
    pub special_allowance_amt: Usd,
    /// Line 15: Property subject to section 168(f)(1) election
    pub section168f1_elected_property_amt: Usd,
    /// Line 16: Other depreciation (including ACRS)
    pub other_depreciation_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — MACRS Depreciation (Do not include listed property)
    // Section A
    // -----------------------------------------------------------------------
    /// Line 17: MACRS deductions for assets placed in service in tax years beginning
    /// before 2025
    pub macrs_ded_for_ast_in_srvc_bfr_py_amt: Usd,
    /// Line 18: If you are electing to group any assets placed in service during the tax
    /// year into one or more general asset accounts, check here
    pub general_asset_account_election_ind: bool,

    // -----------------------------------------------------------------------
    // Section B — Assets Placed in Service During 2025 Tax Year Using the
    //              General Depreciation System
    // -----------------------------------------------------------------------
    /// Line 19a: GDS 3-year property
    pub gds3_year_property: String,
    /// Line 19b: GDS 5-year property
    pub gds5_year_property: String,
    /// Line 19c: GDS 7-year property
    pub gds7_year_property: String,
    /// Line 19d: GDS 10-year property
    pub gds10_year_property: String,
    /// Line 19e: GDS 15-year property
    pub gds15_year_property: String,
    /// Line 19f: GDS 20-year property
    pub gds20_year_property: String,
    /// Line 19g: GDS 25-year property
    pub gds25_year_property: String,
    /// Line 19h: GDS 50-year property
    pub general_depreciation_system: String,
    /// Line 19i: GDS Residential rental property
    pub gds_residential_rental_property: String,
    /// Line 19j: GDS Nonresidential real property
    pub gds_non_rsdntl_real_prop: String,
    /// Line 19j: GDS Nonresidential real property — specify recovery period
    pub gds_non_rsdntl_real_prop_specify: String,

    // -----------------------------------------------------------------------
    // Section C — Assets Placed in Service During 2025 Tax Year Using the
    //              Alternative Depreciation System
    // -----------------------------------------------------------------------
    /// Line 20a: ADS Class life property
    pub ads_class_life_property: String,
    /// Line 20b: ADS 12-year property
    pub ads12_year_property: String,
    /// Line 20c: ADS 30-year property
    pub ads30_year_property: String,
    /// Line 20d: ADS 40-year property
    pub ads40_year_property: String,
    /// Line 20e: ADS 50-year property
    pub ads50_year_property: String,
    /// Alternative depreciation system (general/summary field)
    pub alternative_depreciation_system: String,

    // -----------------------------------------------------------------------
    // Part IV — Summary
    // -----------------------------------------------------------------------
    /// Line 21: Listed property. Enter amount from line 28
    pub total_special_deprec_allwnc_amt: Usd,
    /// Line 22: Total. Add amounts from line 12, lines 14 through 17, lines 19 and 20 in
    /// column (g), and line 21. Enter here and on the appropriate lines of your return
    pub total_depreciation_amt: Usd,
    /// Line 23a: For assets shown in Part III that are placed in service during the current
    /// tax year, and have costs capitalized under section 263A, enter the amount of the
    /// basis attributable to interest costs capitalized under section 263A(f)
    pub cap_sect263_af_int_costs_ast_amt: Usd,
    /// Line 23b: For assets shown in Part III that are placed in service during the current
    /// tax year, and have costs capitalized under section 263A, enter the amount of the
    /// basis attributable to costs other than interest costs capitalized under section 263A(f)
    pub all_oth_cap_sect263_a_costs_ast_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Listed Property (automobiles, certain other vehicles, certain
    //           aircraft, and property used for entertainment, recreation, or
    //           amusement)
    // Section A — Depreciation and Other Information
    // -----------------------------------------------------------------------
    /// Line 24a: Do you have evidence to support the business/investment use claimed?
    pub evidence_to_support_deduction_ind: bool,
    /// Line 24b: If "Yes," is the evidence written?
    pub evidence_written_ind: bool,
    /// Line 24c: Do you own, lease, or charter an aircraft? — Own
    pub own_ind: bool,
    /// Line 24c: Do you own, lease, or charter an aircraft? — Lease
    pub lease_ind: bool,
    /// Line 24c: Do you own, lease, or charter an aircraft? — Charter
    pub charter_ind: bool,
    /// Lines 25-26: Property used more than 50% in a qualified business use (table data)
    pub more_than_half_business_use_prop: String,
    /// Line 25: Special depreciation allowance for qualified listed property placed in
    /// service during the tax year and used more than 50% in a qualified business use
    pub total_section179_expense_amt: Usd,
    /// Line 27: Property used 50% or less in a qualified business use (table data)
    pub less_than_half_business_use_prop: String,
    /// Line 28: Add amounts in column (h), lines 25 through 27. Enter here and on line 21
    pub total_amortization_amt: Usd,
    /// Line 29: Add amounts in column (i), line 26. Enter here and on line 7
    pub amortization_costs_before_ty_amt: Usd,
    /// Extensions under announcement code
    pub extns_under_announcement_cd: String,

    // -----------------------------------------------------------------------
    // Section B — Information on Use of Vehicles
    // -----------------------------------------------------------------------
    /// Vehicle usage (table data for vehicles 1-6)
    pub vehicle_usage: String,
    /// Line 30: Total business/investment miles driven during the year (do not include
    /// commuting miles)
    pub business_miles_cnt: u32,
    /// Line 31: Total commuting miles driven during the year
    pub commuting_miles_cnt: u32,
    /// Line 32: Total other personal (noncommuting) miles driven
    pub other_personal_miles_cnt: u32,
    /// Line 33: Total miles driven during the year. Add lines 30 through 32
    pub total_miles_cnt: u32,
    /// Line 34: Was the vehicle available for personal use during off-duty hours?
    pub vehicle_available_off_duty_hrs_ind: bool,
    /// Line 35: Was the vehicle used primarily by a more than 5% owner or related person?
    pub used_primarily_by_owner_ind: bool,
    /// Line 36: Is another vehicle available for personal use?
    pub another_vehicle_for_prsnl_use_ind: bool,

    // -----------------------------------------------------------------------
    // Section C — Questions for Employers Who Provide Vehicles for Use by
    //              Their Employees
    // -----------------------------------------------------------------------
    /// Do you maintain a written policy statement that prohibits all personal use of
    /// vehicles, including commuting, by your employees?
    pub policy_no_prsnl_or_cmmtng_use_ind: bool,
    /// Do you maintain a written policy statement that prohibits personal use of vehicles,
    /// except commuting, by your employees?
    pub policy_no_prsnl_exc_cmmtng_use_ind: bool,
    /// Do you treat all use of vehicles by employees as personal use?
    pub treat_all_veh_use_as_prsnl_use_ind: bool,
    /// Do you provide more than five vehicles to your employees, obtain information from
    /// your employees about the use of the vehicles, and retain the information received?
    pub provide_over_num_veh_and_have_rec_ind: bool,
    /// Do you meet the requirements concerning qualified automobile demonstration use?
    pub meet_rqr_for_auto_demo_use_ind: bool,

    // -----------------------------------------------------------------------
    // Part VI — Amortization
    // -----------------------------------------------------------------------
    /// Amortization information table (description, date, amount, code section, period,
    /// and current year amortization)
    pub amortization_info_table: String,
}
