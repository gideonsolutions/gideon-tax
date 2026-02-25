use crate::Usd;

/// Output fields for IRS Form 2555 (2025) — Foreign Earned Income.
#[derive(Debug, Clone, Default)]
pub struct Output2555 {
    // -----------------------------------------------------------------------
    // Part I — General Information (Lines 1–9)
    // -----------------------------------------------------------------------
    /// Line 7: Of what country are you a citizen/national?
    pub citizen_country_nm: String,
    /// Are you claiming the housing exclusion or deduction? (Part V, line 27)
    pub claiming_housing_excl_or_ded_ind: bool,
    /// Line 22a: Cost of living and overseas differential
    pub cost_of_living_and_overseas_diff_amt: Usd,
    /// Country code
    pub country_cd: String,
    /// Line 44: Deductions allocable to the excluded income
    pub deduction_alloc_to_excluded_inc_amt: Usd,
    /// Line 22c: Education allowance
    pub education_allowance_amt: Usd,
    /// Line 4b: Employer's foreign address
    pub employer_foreign_address: String,
    /// Line 5a: Employer is a foreign entity indicator
    pub employer_foreign_entity_ind: bool,
    /// Line 11d: Quarters furnished by employer indicator
    pub employer_furnished_quarters_ind: bool,
    /// Line 3: Employer's name (line 1)
    pub business_name_line1_txt: String,
    /// Line 3: Employer's name (line 2)
    pub business_name_line2_txt: String,
    /// Employer-provided housing exclusion percentage
    pub employer_prov_housing_excl_pct: String,
    /// Line 34: Enter employer-provided amounts
    pub employer_provided_housing_amt: Usd,
    /// Line 4a: Employer's U.S. address
    pub employer_us_address: String,
    /// Line 5b: Employer is a U.S. company indicator
    pub employer_united_states_company_ind: bool,
    /// Line 15a: Employment contract terms description
    pub employment_contract_terms_desc: String,
    /// Line 22b: Family allowance
    pub family_allowance_amt: Usd,
    /// Line 12a: Did any of your family live with you abroad during any part of the tax year?
    pub family_lived_abroad_ind: bool,
    /// Line 12b: If "Yes," who and for what period?
    pub family_living_with_taxpayer_abroad: String,
    /// Line 1: Your foreign address (including country)
    pub foreign_address: String,
    /// Line 5d: Employer is a foreign affiliate of a U.S. company indicator
    pub foreign_affiliate_employer_ind: bool,
    /// Line 20a: Allowable share of income for personal services in a business
    pub foreign_business_income_share_amt: Usd,
    /// Line 6c: Have you ever revoked either of the exclusions?
    pub foreign_earn_inc_excl_revoked_ind: bool,
    /// Line 38/39: Number of qualifying days in the tax year
    pub foreign_earn_incm_excl_qlfy_days_cnt: u32,
    /// Line 42: Foreign earned income exclusion
    pub foreign_earned_inc_exclusion_amt: Usd,
    /// Foreign earned income exclusion percentage
    pub foreign_earned_inc_exclusion_pct: String,
    /// Line 26: 2025 foreign earned income (subtract line 25 from line 24)
    pub foreign_earned_income_amt: Usd,
    /// Line 19: Total wages, salaries, bonuses, commissions, etc.
    pub foreign_earned_total_wages_inc_amt: Usd,
    /// Line 45: Subtract line 44 from line 43 (foreign income less housing exclusion)
    pub foreign_inc_less_housing_excl_amt: Usd,
    /// Line 20b: In a partnership — share of income
    pub foreign_partnership_inc_share_amt: Usd,
    /// Line 22d: Home leave allowance
    pub home_leave_allowance_amt: Usd,
    /// Line 50: Housing deduction (add lines 48 and 49)
    pub housing_deduction_amt: Usd,
    /// Line 49: Housing deduction carryover from 2024
    pub housing_deduction_carryover_amt: Usd,
    /// Line 43: Add lines 36 and 42 (housing deduction/exclusion combined)
    pub housing_deduction_exclusion_amt: Usd,
    /// Line 28: Qualified housing expenses for the tax year
    pub housing_deduction_expense_amt: Usd,
    /// Line 48: Enter the smaller of line 46 or line 47 — tentative housing deduction
    pub housing_deduction_tentative_amt: Usd,
    /// Line 36: Housing exclusion (multiply line 33 by line 35, but don't enter more than line 34)
    pub housing_exclusion_amt: Usd,
    /// Line 29b: Limit on housing expenses
    pub housing_expense_limit_amt: Usd,
    /// Line 29a: Enter location where housing expenses incurred
    pub housing_expense_location_desc: String,
    /// Line 33: Subtract line 32 from line 30 (housing expenses over the maximum)
    pub housing_expenses_over_max_amt: Usd,
    /// Line 32: Multiply $56.99 by the number of days on line 31 (or $20,800 if 365)
    pub housing_maximum_allowed_amt: Usd,
    /// Line 31: Number of days in your qualifying period that fall within your 2025 tax year
    pub housing_qualified_days_cnt: u32,
    /// Line 28: Qualified housing expenses for the tax year
    pub housing_qualified_expense_amt: Usd,
    /// Line 6a: Last year foreign earned income exclusion was claimed
    pub last_frgn_earn_inc_excl_claimed_yr: u16,
    /// Line 15d: Did you maintain a home in the United States while living abroad?
    pub maintained_house_in_us_ind: bool,
    /// Name shown on Form 1040 or 1040-SR
    pub name_line1_txt: String,
    /// Line 6b: Didn't previously file Form 2555 or Form 2555-EZ indicator
    pub no_frgn_earn_inc_excl_prev_filed_ind: bool,
    /// No travel explanation code
    pub no_travel_explanation_cd: String,
    /// Line 21c: Noncash income — Car
    pub non_cash_car_income_amt: Usd,
    /// Line 21a: Noncash income — Home (lodging)
    pub non_cash_lodging_income_amt: Usd,
    /// Line 21b: Noncash income — Meals
    pub non_cash_meal_income_amt: Usd,
    /// Line 2: Your occupation
    pub occupation_txt: String,
    /// Line 5e: Other (specify) employer description
    pub other_employer_desc: String,
    /// Line 5e: Other employer indicator
    pub other_employer_ind: bool,
    /// Line 22f: For any other purpose — amount
    pub amt: Usd,
    /// Line 22f: For any other purpose — list type and amount description
    pub desc: String,

    // -----------------------------------------------------------------------
    // Part III — Taxpayers Qualifying Under Physical Presence Test (Lines 16–18)
    // -----------------------------------------------------------------------
    /// Line 18: Physical presence — country group (travel details)
    pub physical_presence_country_group: String,
    /// Line 18(b): Date arrived
    pub arrival_dt: String,
    /// Line 14/18(e): Number of days in U.S. on business
    pub business_days_in_us_cnt: u32,
    /// Line 18(a): Name of country (including U.S.)
    pub country_nm: String,
    /// Line 18(d): Full days present in country
    pub days_present_in_country_cnt: u32,
    /// Line 18(c): Date left
    pub departure_dt: String,
    /// Line 14(d)/18(f): Income earned in U.S. on business
    pub us_business_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Taxpayers Qualifying Under Bona Fide Residence Test (Lines 10–15)
    // -----------------------------------------------------------------------
    /// Line 14: Presence in the U.S. group (dates arrived/left, days, income)
    pub presence_in_the_us_group: String,
    /// Line 17: Principal country of employment during your tax year
    pub principal_employment_country_nm: String,
    /// Line 11a: Purchased house indicator
    pub purchased_house_ind: bool,
    /// Line 22e: Quarters allowance
    pub quarters_allowance_amt: Usd,
    /// Line 11b: Rented house or apartment indicator
    pub rented_house_ind: bool,
    /// Line 11c: Rented room indicator
    pub rented_room_ind: bool,
    /// Line 13b: Are you required to pay income tax to the country?
    pub required_to_pay_income_tax_ind: bool,
    /// Your social security number
    pub ssn: String,
    /// Line 5c: Self-employment indicator
    pub self_employment_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Separate Foreign Residence (Lines 8a–8b)
    // -----------------------------------------------------------------------
    /// Line 8b: City and country of the separate foreign residence
    pub separate_foreign_res_location_txt: String,
    /// Line 8b: Number of days during your tax year at that address
    pub separate_foreign_residence_day_cnt: u32,
    /// Line 8a: Did you maintain a separate foreign residence for your family?
    pub separate_foreign_residence_ind: bool,

    // -----------------------------------------------------------------------
    // Part VI — Taxpayers Claiming the Housing Exclusion and/or Deduction (Lines 28–36)
    // -----------------------------------------------------------------------
    /// Line 30: Enter the smaller of line 28 or line 29b
    pub smaller_qualified_or_limit_amt: Usd,
    /// Line 13a: Have you submitted a statement to authorities of the foreign country?
    pub submitted_non_resident_stmt_ind: bool,
    /// Line 9: List your tax home(s) during your tax year and date(s) established
    pub tax_home_group: String,

    // -----------------------------------------------------------------------
    // Part VII — Taxpayers Claiming the Foreign Earned Income Exclusion (Lines 37–42)
    // -----------------------------------------------------------------------
    /// Line 40: Multiply line 37 by line 39 — tentative foreign earned income exclusion
    pub tent_foreign_earned_income_excl_amt: Usd,
    /// Line 41: Subtract line 36 from line 27 — tentative income exclusion
    pub tentative_income_exclusion_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — All Taxpayers (Lines 19–26)
    // -----------------------------------------------------------------------
    /// Line 22g: Add lines 22a through 22f — total allowances paid on your behalf
    pub total_allowances_paid_on_behalf_amt: Usd,
    /// Line 42 or Line 43: Total foreign earned income exclusion
    pub total_foreign_earned_incm_excl_amt: Usd,
    /// Line 24: Add lines 19 through 21d, line 22g, and line 23 — total foreign earned income
    pub total_foreign_earned_income_amt: Usd,
    /// Total income exclusion amount
    pub total_income_exclusion_amt: Usd,
    /// Prior year foreign earned income exclusion amount
    pub foreign_earned_inc_py_exclusion_amt: Usd,
    /// Prior year foreign earned income exclusion code
    pub foreign_earned_inc_py_exclusion_cd: String,
    /// Line 21d: Other property or facilities — noncash income total
    pub total_non_cash_other_property_amt: Usd,
    /// Line 23: Other foreign earned income — list type and amount
    pub total_other_foreign_income_amt: Usd,
    /// Line 22f: Total other purpose allowance
    pub total_other_purpose_allowance_amt: Usd,
    /// Line 15e: U.S. home address while living abroad
    pub us_home_while_living_abroad: String,
    /// Line 15c: Did your visa limit the length of your stay or employment?
    pub visa_limit_stay_or_employment_ind: bool,
    /// Line 15b: Enter the type of visa under which you entered the foreign country
    pub visa_type_desc: String,

    // -----------------------------------------------------------------------
    // Waiver / Relief Claims (Line 6)
    // -----------------------------------------------------------------------
    /// Line 6d: Type of exclusion and tax year for which revocation was effective
    pub claim_frgn_earn_inc_waiver_cd: String,
    /// Claim waiver relief code
    pub claim_waiver_relief_cd: String,
}
