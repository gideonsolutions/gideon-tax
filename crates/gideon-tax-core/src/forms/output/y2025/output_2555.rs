use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::rules::TaxYearRules;
use crate::rules::y2025::Rules2025;
use crate::{GideonTaxError, Usd};

// =========================================================================
// Line 14 row
// =========================================================================

/// Per-entry row from Form 2555, Line 14 (columns a–d).
///
/// Each row records a period when the taxpayer was present in the United
/// States or its territories during the tax year.
#[derive(Debug, Clone)]
pub struct F2555Line14 {
    /// Column (a): Date arrived in U.S.
    pub arrival_dt: String,
    /// Column (b): Date left U.S.
    pub departure_dt: String,
    /// Column (c): Number of days in U.S. on business
    pub business_days_in_us_cnt: u32,
    /// Column (d): Income earned in U.S. on business (attach computation)
    pub us_business_income_amt: Usd,
}

// =========================================================================
// Line 18 row
// =========================================================================

/// Per-entry row from Form 2555, Line 18 (columns a–f).
///
/// Each row records travel abroad during the 12-month physical presence
/// test period.
#[derive(Debug, Clone)]
pub struct F2555Line18 {
    /// Column (a): Name of country (including U.S.)
    pub country_nm: String,
    /// Column (b): Date arrived
    pub arrival_dt: String,
    /// Column (c): Date left
    pub departure_dt: String,
    /// Column (d): Full days present in country
    pub days_present_in_country_cnt: u32,
    /// Column (e): Number of days in U.S. on business
    pub business_days_in_us_cnt: u32,
    /// Column (f): Income earned in U.S. on business (attach computation)
    pub us_business_income_amt: Usd,
}

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 2555.
///
/// Lines that are computed (22g, 24, 26, 27, 30, 32, 33, 35, 36, 37, 39,
/// 40, 41, 42, 43, 45, 46, 47, 48, 50) are omitted from the input and
/// produced by [`OutputForm::try_new`].
#[derive(Debug, Clone)]
pub struct F2555Input {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name shown on Form 1040 or 1040-SR
    pub name_line1_txt: String,
    /// Your social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — General Information (Lines 1–9)
    // -----------------------------------------------------------------------
    /// Line 1: Your foreign address (including country)
    pub foreign_address: String,
    /// Line 2: Your occupation
    pub occupation_txt: String,
    /// Line 3: Employer's name (line 1)
    pub business_name_line1_txt: String,
    /// Line 3: Employer's name (line 2)
    pub business_name_line2_txt: String,
    /// Line 4a: Employer's U.S. address
    pub employer_us_address: String,
    /// Line 4b: Employer's foreign address
    pub employer_foreign_address: String,
    /// Line 5a: Employer is a foreign entity
    pub employer_foreign_entity_ind: bool,
    /// Line 5b: Employer is a U.S. company
    pub employer_united_states_company_ind: bool,
    /// Line 5c: Self-employed
    pub self_employment_ind: bool,
    /// Line 5d: Employer is a foreign affiliate of a U.S. company
    pub foreign_affiliate_employer_ind: bool,
    /// Line 5e: Other employer indicator
    pub other_employer_ind: bool,
    /// Line 5e: Other employer description
    pub other_employer_desc: String,
    /// Line 6a: Last year Form 2555 or 2555-EZ was filed
    pub last_frgn_earn_inc_excl_claimed_yr: u16,
    /// Line 6b: Didn't previously file Form 2555 or 2555-EZ
    pub no_frgn_earn_inc_excl_prev_filed_ind: bool,
    /// Line 6c: Have you ever revoked either of the exclusions?
    pub foreign_earn_inc_excl_revoked_ind: bool,
    /// Line 6d: Type of exclusion and tax year for which revocation was effective
    pub claim_frgn_earn_inc_waiver_cd: String,
    /// Line 7: Of what country are you a citizen/national?
    pub citizen_country_nm: String,
    /// Line 8a: Did you maintain a separate foreign residence for your family?
    pub separate_foreign_residence_ind: bool,
    /// Line 8b: City and country of the separate foreign residence
    pub separate_foreign_res_location_txt: String,
    /// Line 8b: Number of days during your tax year at that address
    pub separate_foreign_residence_day_cnt: u32,
    /// Line 9: Tax home(s) during your tax year and date(s) established
    pub tax_home_group: String,

    // -----------------------------------------------------------------------
    // Part II — Bona Fide Residence Test (Lines 10–15)
    // -----------------------------------------------------------------------
    /// Line 10: Date bona fide residence began
    pub bona_fide_residence_begin_dt: String,
    /// Line 10: Date bona fide residence ended
    pub bona_fide_residence_end_dt: String,
    /// Line 11a: Purchased house
    pub purchased_house_ind: bool,
    /// Line 11b: Rented house or apartment
    pub rented_house_ind: bool,
    /// Line 11c: Rented room
    pub rented_room_ind: bool,
    /// Line 11d: Quarters furnished by employer
    pub employer_furnished_quarters_ind: bool,
    /// Line 12a: Did any of your family live with you abroad during any part of the tax year?
    pub family_lived_abroad_ind: bool,
    /// Line 12b: If "Yes," who and for what period?
    pub family_living_with_taxpayer_abroad: String,
    /// Line 13a: Have you submitted a statement to the foreign country authorities?
    pub submitted_non_resident_stmt_ind: bool,
    /// Line 13b: Are you required to pay income tax to the foreign country?
    pub required_to_pay_income_tax_ind: bool,
    /// Line 14: Presence in the U.S. during the tax year (table rows)
    pub line14: Vec<F2555Line14>,
    /// Line 15a: Employment contract terms description
    pub employment_contract_terms_desc: String,
    /// Line 15b: Type of visa under which you entered the foreign country
    pub visa_type_desc: String,
    /// Line 15c: Did your visa limit the length of your stay or employment?
    pub visa_limit_stay_or_employment_ind: bool,
    /// Line 15d: Did you maintain a home in the United States while living abroad?
    pub maintained_house_in_us_ind: bool,
    /// Line 15e: Address of U.S. home while living abroad
    pub us_home_while_living_abroad: String,

    // -----------------------------------------------------------------------
    // Part III — Physical Presence Test (Lines 16–18)
    // -----------------------------------------------------------------------
    /// Line 16: Physical presence 12-month period — from date
    pub physical_presence_begin_dt: String,
    /// Line 16: Physical presence 12-month period — through date
    pub physical_presence_end_dt: String,
    /// Line 17: Principal country of employment during your tax year
    pub principal_employment_country_nm: String,
    /// Line 18: Travel abroad during the 12-month period (table rows)
    pub line18: Vec<F2555Line18>,

    // -----------------------------------------------------------------------
    // Part IV — All Taxpayers (Lines 19–25)
    // -----------------------------------------------------------------------
    /// Line 19: Total wages, salaries, bonuses, commissions, etc.
    pub foreign_earned_total_wages_inc_amt: Usd,
    /// Line 20a: Allowable share of income for personal services in a business
    pub foreign_business_income_share_amt: Usd,
    /// Line 20b: In a partnership — share of income
    pub foreign_partnership_inc_share_amt: Usd,
    /// Line 21a: Noncash income — Home (lodging)
    pub non_cash_lodging_income_amt: Usd,
    /// Line 21b: Noncash income — Meals
    pub non_cash_meal_income_amt: Usd,
    /// Line 21c: Noncash income — Car
    pub non_cash_car_income_amt: Usd,
    /// Line 21d: Noncash income — Other property or facilities
    pub total_non_cash_other_property_amt: Usd,
    /// Line 22a: Cost of living and overseas differential
    pub cost_of_living_and_overseas_diff_amt: Usd,
    /// Line 22b: Family allowance
    pub family_allowance_amt: Usd,
    /// Line 22c: Education allowance
    pub education_allowance_amt: Usd,
    /// Line 22d: Home leave allowance
    pub home_leave_allowance_amt: Usd,
    /// Line 22e: Quarters allowance
    pub quarters_allowance_amt: Usd,
    /// Line 22f: For any other purpose — amount
    pub other_purpose_allowance_amt: Usd,
    /// Line 22f: For any other purpose — description
    pub other_purpose_allowance_desc: String,
    /// Line 23: Other foreign earned income
    pub total_other_foreign_income_amt: Usd,
    /// Line 25: Total excludable meals and lodging included on line 24
    pub excludable_meals_lodging_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — All Taxpayers
    // -----------------------------------------------------------------------
    /// Are you claiming the housing exclusion or housing deduction?
    pub claiming_housing_excl_or_ded_ind: bool,

    // -----------------------------------------------------------------------
    // Part VI — Housing Exclusion and/or Deduction (Lines 28–34)
    // -----------------------------------------------------------------------
    /// Line 28: Qualified housing expenses for the tax year
    pub housing_qualified_expense_amt: Usd,
    /// Line 29a: Location where housing expenses incurred
    pub housing_expense_location_desc: String,
    /// Line 29b: Limit on housing expenses
    pub housing_expense_limit_amt: Usd,
    /// Line 31: Number of days in your qualifying period that fall within your 2025 tax year
    pub housing_qualified_days_cnt: u32,
    /// Line 34: Employer-provided amounts
    pub employer_provided_housing_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VII — Foreign Earned Income Exclusion (Line 38)
    // -----------------------------------------------------------------------
    /// Line 38: Number of qualifying days that fall within your 2025 tax year
    pub foreign_earn_incm_excl_qlfy_days_cnt: u32,

    // -----------------------------------------------------------------------
    // Part VIII (Line 44)
    // -----------------------------------------------------------------------
    /// Line 44: Deductions allocable to the excluded income
    pub deduction_alloc_to_excluded_inc_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IX — Housing Deduction (Line 49)
    // -----------------------------------------------------------------------
    /// Line 49: Housing deduction carryover from 2024
    pub housing_deduction_carryover_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 2555 (2025) — Foreign Earned Income.
#[derive(Debug, Clone)]
pub struct Output2555 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name shown on Form 1040 or 1040-SR
    pub name_line1_txt: String,
    /// Your social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — General Information (Lines 1–9)
    // -----------------------------------------------------------------------
    /// Line 1: Your foreign address (including country)
    pub foreign_address: String,
    /// Line 2: Your occupation
    pub occupation_txt: String,
    /// Line 3: Employer's name (line 1)
    pub business_name_line1_txt: String,
    /// Line 3: Employer's name (line 2)
    pub business_name_line2_txt: String,
    /// Line 4a: Employer's U.S. address
    pub employer_us_address: String,
    /// Line 4b: Employer's foreign address
    pub employer_foreign_address: String,
    /// Line 5a: Employer is a foreign entity
    pub employer_foreign_entity_ind: bool,
    /// Line 5b: Employer is a U.S. company
    pub employer_united_states_company_ind: bool,
    /// Line 5c: Self-employed
    pub self_employment_ind: bool,
    /// Line 5d: Employer is a foreign affiliate of a U.S. company
    pub foreign_affiliate_employer_ind: bool,
    /// Line 5e: Other employer indicator
    pub other_employer_ind: bool,
    /// Line 5e: Other employer description
    pub other_employer_desc: String,
    /// Line 6a: Last year Form 2555 or 2555-EZ was filed
    pub last_frgn_earn_inc_excl_claimed_yr: u16,
    /// Line 6b: Didn't previously file Form 2555 or 2555-EZ
    pub no_frgn_earn_inc_excl_prev_filed_ind: bool,
    /// Line 6c: Have you ever revoked either of the exclusions?
    pub foreign_earn_inc_excl_revoked_ind: bool,
    /// Line 6d: Type of exclusion and tax year for which revocation was effective
    pub claim_frgn_earn_inc_waiver_cd: String,
    /// Line 7: Of what country are you a citizen/national?
    pub citizen_country_nm: String,
    /// Line 8a: Did you maintain a separate foreign residence for your family?
    pub separate_foreign_residence_ind: bool,
    /// Line 8b: City and country of the separate foreign residence
    pub separate_foreign_res_location_txt: String,
    /// Line 8b: Number of days during your tax year at that address
    pub separate_foreign_residence_day_cnt: u32,
    /// Line 9: Tax home(s) during your tax year and date(s) established
    pub tax_home_group: String,

    // -----------------------------------------------------------------------
    // Part II — Bona Fide Residence Test (Lines 10–15)
    // -----------------------------------------------------------------------
    /// Line 10: Date bona fide residence began
    pub bona_fide_residence_begin_dt: String,
    /// Line 10: Date bona fide residence ended
    pub bona_fide_residence_end_dt: String,
    /// Line 11a: Purchased house
    pub purchased_house_ind: bool,
    /// Line 11b: Rented house or apartment
    pub rented_house_ind: bool,
    /// Line 11c: Rented room
    pub rented_room_ind: bool,
    /// Line 11d: Quarters furnished by employer
    pub employer_furnished_quarters_ind: bool,
    /// Line 12a: Did any of your family live with you abroad during any part of the tax year?
    pub family_lived_abroad_ind: bool,
    /// Line 12b: If "Yes," who and for what period?
    pub family_living_with_taxpayer_abroad: String,
    /// Line 13a: Have you submitted a statement to the foreign country authorities?
    pub submitted_non_resident_stmt_ind: bool,
    /// Line 13b: Are you required to pay income tax to the foreign country?
    pub required_to_pay_income_tax_ind: bool,
    /// Line 14: Presence in the U.S. during the tax year (table rows)
    pub line14: Vec<F2555Line14>,
    /// Line 15a: Employment contract terms description
    pub employment_contract_terms_desc: String,
    /// Line 15b: Type of visa under which you entered the foreign country
    pub visa_type_desc: String,
    /// Line 15c: Did your visa limit the length of your stay or employment?
    pub visa_limit_stay_or_employment_ind: bool,
    /// Line 15d: Did you maintain a home in the United States while living abroad?
    pub maintained_house_in_us_ind: bool,
    /// Line 15e: Address of U.S. home while living abroad
    pub us_home_while_living_abroad: String,

    // -----------------------------------------------------------------------
    // Part III — Physical Presence Test (Lines 16–18)
    // -----------------------------------------------------------------------
    /// Line 16: Physical presence 12-month period — from date
    pub physical_presence_begin_dt: String,
    /// Line 16: Physical presence 12-month period — through date
    pub physical_presence_end_dt: String,
    /// Line 17: Principal country of employment during your tax year
    pub principal_employment_country_nm: String,
    /// Line 18: Travel abroad during the 12-month period (table rows)
    pub line18: Vec<F2555Line18>,

    // -----------------------------------------------------------------------
    // Part IV — All Taxpayers (Lines 19–26)
    // -----------------------------------------------------------------------
    /// Line 19: Total wages, salaries, bonuses, commissions, etc.
    pub foreign_earned_total_wages_inc_amt: Usd,
    /// Line 20a: Allowable share of income for personal services in a business
    pub foreign_business_income_share_amt: Usd,
    /// Line 20b: In a partnership — share of income
    pub foreign_partnership_inc_share_amt: Usd,
    /// Line 21a: Noncash income — Home (lodging)
    pub non_cash_lodging_income_amt: Usd,
    /// Line 21b: Noncash income — Meals
    pub non_cash_meal_income_amt: Usd,
    /// Line 21c: Noncash income — Car
    pub non_cash_car_income_amt: Usd,
    /// Line 21d: Noncash income — Other property or facilities
    pub total_non_cash_other_property_amt: Usd,
    /// Line 22a: Cost of living and overseas differential
    pub cost_of_living_and_overseas_diff_amt: Usd,
    /// Line 22b: Family allowance
    pub family_allowance_amt: Usd,
    /// Line 22c: Education allowance
    pub education_allowance_amt: Usd,
    /// Line 22d: Home leave allowance
    pub home_leave_allowance_amt: Usd,
    /// Line 22e: Quarters allowance
    pub quarters_allowance_amt: Usd,
    /// Line 22f: For any other purpose — amount
    pub other_purpose_allowance_amt: Usd,
    /// Line 22f: For any other purpose — description
    pub other_purpose_allowance_desc: String,
    /// Line 22g: Add lines 22a through 22f
    pub total_allowances_paid_on_behalf_amt: Usd,
    /// Line 23: Other foreign earned income
    pub total_other_foreign_income_amt: Usd,
    /// Line 24: Add lines 19 through 21d, line 22g, and line 23
    pub total_foreign_earned_income_amt: Usd,
    /// Line 25: Total excludable meals and lodging included on line 24
    pub excludable_meals_lodging_amt: Usd,
    /// Line 26: Subtract line 25 from line 24 — 2025 foreign earned income
    pub foreign_earned_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — All Taxpayers (Line 27)
    // -----------------------------------------------------------------------
    /// Line 27: Enter the amount from line 26
    pub line_27_foreign_earned_income_amt: Usd,
    /// Are you claiming the housing exclusion or housing deduction?
    pub claiming_housing_excl_or_ded_ind: bool,

    // -----------------------------------------------------------------------
    // Part VI — Housing Exclusion and/or Deduction (Lines 28–36)
    // -----------------------------------------------------------------------
    /// Line 28: Qualified housing expenses for the tax year
    pub housing_qualified_expense_amt: Usd,
    /// Line 29a: Location where housing expenses incurred
    pub housing_expense_location_desc: String,
    /// Line 29b: Limit on housing expenses
    pub housing_expense_limit_amt: Usd,
    /// Line 30: Enter the smaller of line 28 or line 29b
    pub smaller_qualified_or_limit_amt: Usd,
    /// Line 31: Number of days in your qualifying period that fall within your 2025 tax year
    pub housing_qualified_days_cnt: u32,
    /// Line 32: Multiply $56.99 by the number of days on line 31 (or $20,800 if 365)
    pub housing_maximum_allowed_amt: Usd,
    /// Line 33: Subtract line 32 from line 30
    pub housing_expenses_over_max_amt: Usd,
    /// Line 34: Employer-provided amounts
    pub employer_provided_housing_amt: Usd,
    /// Line 35: Divide line 34 by line 27 (decimal, rounded to at least three places, max "1.000")
    pub employer_prov_housing_excl_pct: String,
    /// Line 36: Housing exclusion — multiply line 33 by line 35, but not more than line 34
    pub housing_exclusion_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VII — Foreign Earned Income Exclusion (Lines 37–42)
    // -----------------------------------------------------------------------
    /// Line 37: Maximum foreign earned income exclusion ($130,000 for 2025)
    pub max_foreign_earned_inc_exclusion_amt: Usd,
    /// Line 38: Number of qualifying days that fall within your 2025 tax year
    pub foreign_earn_incm_excl_qlfy_days_cnt: u32,
    /// Line 39: Line 38 / 365 (decimal, "1.000" if line 38 is 365)
    pub foreign_earned_inc_exclusion_pct: String,
    /// Line 40: Multiply line 37 by line 39
    pub tent_foreign_earned_income_excl_amt: Usd,
    /// Line 41: Subtract line 36 from line 27
    pub tentative_income_exclusion_amt: Usd,
    /// Line 42: Foreign earned income exclusion — smaller of line 40 or line 41
    pub foreign_earned_inc_exclusion_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VIII — (Lines 43–45)
    // -----------------------------------------------------------------------
    /// Line 43: Add lines 36 and 42
    pub housing_and_income_exclusion_amt: Usd,
    /// Line 44: Deductions allocable to the excluded income
    pub deduction_alloc_to_excluded_inc_amt: Usd,
    /// Line 45: Subtract line 44 from line 43 — enter on Schedule 1 (Form 1040), line 8d
    pub total_income_exclusion_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IX — Housing Deduction (Lines 46–50)
    // -----------------------------------------------------------------------
    /// Line 46: Subtract line 36 from line 33
    pub housing_expense_less_exclusion_amt: Usd,
    /// Line 47: Subtract line 43 from line 27
    pub foreign_income_less_total_exclusion_amt: Usd,
    /// Line 48: Enter the smaller of line 46 or line 47
    pub housing_deduction_tentative_amt: Usd,
    /// Line 49: Housing deduction carryover from 2024
    pub housing_deduction_carryover_amt: Usd,
    /// Line 50: Housing deduction — add lines 48 and 49. Enter on Schedule 1 (Form 1040), line 24j
    pub housing_deduction_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output2555 {
    fn name() -> &'static str {
        "Form 2555"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output2555 {
    type Input = F2555Input;

    fn must_file(input: &Self::Input) -> bool {
        let amounts = [
            input.foreign_earned_total_wages_inc_amt,
            input.foreign_business_income_share_amt,
            input.foreign_partnership_inc_share_amt,
            input.non_cash_lodging_income_amt,
            input.non_cash_meal_income_amt,
            input.non_cash_car_income_amt,
            input.total_non_cash_other_property_amt,
            input.cost_of_living_and_overseas_diff_amt,
            input.family_allowance_amt,
            input.education_allowance_amt,
            input.home_leave_allowance_amt,
            input.quarters_allowance_amt,
            input.other_purpose_allowance_amt,
            input.total_other_foreign_income_amt,
        ];
        amounts.iter().any(|&a| a > Usd::ZERO)
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        let rules = Rules2025;
        let days_in_year = rules.days_in_tax_year();

        // -- Validation --
        if input.housing_qualified_days_cnt > days_in_year {
            return Err(GideonTaxError::OutOfBounds(format!(
                "housing_qualified_days_cnt ({}) exceeds {}",
                input.housing_qualified_days_cnt, days_in_year
            )));
        }
        if input.foreign_earn_incm_excl_qlfy_days_cnt > days_in_year {
            return Err(GideonTaxError::OutOfBounds(format!(
                "foreign_earn_incm_excl_qlfy_days_cnt ({}) exceeds {}",
                input.foreign_earn_incm_excl_qlfy_days_cnt, days_in_year
            )));
        }

        // ================================================================
        // Part IV — Lines 22g, 24, 25, 26
        // ================================================================
        let line22g = input.cost_of_living_and_overseas_diff_amt
            + input.family_allowance_amt
            + input.education_allowance_amt
            + input.home_leave_allowance_amt
            + input.quarters_allowance_amt
            + input.other_purpose_allowance_amt;

        let line24 = input.foreign_earned_total_wages_inc_amt
            + input.foreign_business_income_share_amt
            + input.foreign_partnership_inc_share_amt
            + input.non_cash_lodging_income_amt
            + input.non_cash_meal_income_amt
            + input.non_cash_car_income_amt
            + input.total_non_cash_other_property_amt
            + line22g
            + input.total_other_foreign_income_amt;

        let line25 = input.excludable_meals_lodging_amt;
        let line26 = line24 - line25;

        // ================================================================
        // Part V — Line 27
        // ================================================================
        let line27 = line26;

        // ================================================================
        // Part VI — Lines 28–36 (only when claiming housing)
        // ================================================================
        let claiming_housing = input.claiming_housing_excl_or_ded_ind;
        let days31 = input.housing_qualified_days_cnt;

        let (line30, line32, line33, line35_str, line36) = if claiming_housing {
            let l30 = input
                .housing_qualified_expense_amt
                .min(input.housing_expense_limit_amt);

            let l32 = if days31 == days_in_year {
                rules.f2555_housing_full_year()
            } else {
                Usd::from_cents(rules.f2555_housing_per_day_cents() * days31 as i64)
            };

            let l33 = (l30 - l32).max(Usd::ZERO);

            // Lines 34-36 only apply when line 33 > 0
            if l33 > Usd::ZERO && line27 > Usd::ZERO {
                let thousandths =
                    (input.employer_provided_housing_amt.cents() * 1000 / line27.cents()).min(1000);
                let l35_str = if thousandths >= 1000 {
                    "1.000".to_string()
                } else {
                    format!("0.{:03}", thousandths)
                };
                let l36 = Usd::from_cents(
                    l33.cents() * input.employer_provided_housing_amt.cents() / line27.cents(),
                )
                .min(input.employer_provided_housing_amt);
                (l30, l32, l33, l35_str, l36)
            } else {
                (l30, l32, l33, "0.000".to_string(), Usd::ZERO)
            }
        } else {
            (
                Usd::ZERO,
                Usd::ZERO,
                Usd::ZERO,
                "0.000".to_string(),
                Usd::ZERO,
            )
        };

        // ================================================================
        // Part VII — Lines 37–42
        // ================================================================
        let line37 = rules.f2555_max_foreign_earned_income_exclusion();
        let days38 = input.foreign_earn_incm_excl_qlfy_days_cnt;

        let line39_str = if days38 == days_in_year {
            "1.000".to_string()
        } else if days38 == 0 {
            "0.000".to_string()
        } else {
            let thousandths = days38 as i64 * 1000 / days_in_year as i64;
            format!("0.{:03}", thousandths)
        };

        let line40 = if days38 == days_in_year {
            line37
        } else {
            Usd::from_cents(line37.cents() * days38 as i64 / days_in_year as i64)
        };

        let line41 = (line27 - line36).max(Usd::ZERO);
        let line42 = line40.min(line41);

        // ================================================================
        // Part VIII — Lines 43–45
        // ================================================================
        let line43 = line36 + line42;
        let line44 = input.deduction_alloc_to_excluded_inc_amt;
        let line45 = line43 - line44;

        // ================================================================
        // Part IX — Lines 46–50
        // Only if (a) line 33 > line 36 AND (b) line 27 > line 43.
        // ================================================================
        let (line46, line47, line48, line50) = if line33 > line36 && line27 > line43 {
            let l46 = line33 - line36;
            let l47 = line27 - line43;
            let l48 = l46.min(l47);
            let l50 = l48 + input.housing_deduction_carryover_amt;
            (l46, l47, l48, l50)
        } else {
            (Usd::ZERO, Usd::ZERO, Usd::ZERO, Usd::ZERO)
        };

        Ok(Output2555 {
            // Top-of-form
            name_line1_txt: input.name_line1_txt,
            ssn: input.ssn,

            // Part I
            foreign_address: input.foreign_address,
            occupation_txt: input.occupation_txt,
            business_name_line1_txt: input.business_name_line1_txt,
            business_name_line2_txt: input.business_name_line2_txt,
            employer_us_address: input.employer_us_address,
            employer_foreign_address: input.employer_foreign_address,
            employer_foreign_entity_ind: input.employer_foreign_entity_ind,
            employer_united_states_company_ind: input.employer_united_states_company_ind,
            self_employment_ind: input.self_employment_ind,
            foreign_affiliate_employer_ind: input.foreign_affiliate_employer_ind,
            other_employer_ind: input.other_employer_ind,
            other_employer_desc: input.other_employer_desc,
            last_frgn_earn_inc_excl_claimed_yr: input.last_frgn_earn_inc_excl_claimed_yr,
            no_frgn_earn_inc_excl_prev_filed_ind: input.no_frgn_earn_inc_excl_prev_filed_ind,
            foreign_earn_inc_excl_revoked_ind: input.foreign_earn_inc_excl_revoked_ind,
            claim_frgn_earn_inc_waiver_cd: input.claim_frgn_earn_inc_waiver_cd,
            citizen_country_nm: input.citizen_country_nm,
            separate_foreign_residence_ind: input.separate_foreign_residence_ind,
            separate_foreign_res_location_txt: input.separate_foreign_res_location_txt,
            separate_foreign_residence_day_cnt: input.separate_foreign_residence_day_cnt,
            tax_home_group: input.tax_home_group,

            // Part II
            bona_fide_residence_begin_dt: input.bona_fide_residence_begin_dt,
            bona_fide_residence_end_dt: input.bona_fide_residence_end_dt,
            purchased_house_ind: input.purchased_house_ind,
            rented_house_ind: input.rented_house_ind,
            rented_room_ind: input.rented_room_ind,
            employer_furnished_quarters_ind: input.employer_furnished_quarters_ind,
            family_lived_abroad_ind: input.family_lived_abroad_ind,
            family_living_with_taxpayer_abroad: input.family_living_with_taxpayer_abroad,
            submitted_non_resident_stmt_ind: input.submitted_non_resident_stmt_ind,
            required_to_pay_income_tax_ind: input.required_to_pay_income_tax_ind,
            line14: input.line14,
            employment_contract_terms_desc: input.employment_contract_terms_desc,
            visa_type_desc: input.visa_type_desc,
            visa_limit_stay_or_employment_ind: input.visa_limit_stay_or_employment_ind,
            maintained_house_in_us_ind: input.maintained_house_in_us_ind,
            us_home_while_living_abroad: input.us_home_while_living_abroad,

            // Part III
            physical_presence_begin_dt: input.physical_presence_begin_dt,
            physical_presence_end_dt: input.physical_presence_end_dt,
            principal_employment_country_nm: input.principal_employment_country_nm,
            line18: input.line18,

            // Part IV
            foreign_earned_total_wages_inc_amt: input.foreign_earned_total_wages_inc_amt,
            foreign_business_income_share_amt: input.foreign_business_income_share_amt,
            foreign_partnership_inc_share_amt: input.foreign_partnership_inc_share_amt,
            non_cash_lodging_income_amt: input.non_cash_lodging_income_amt,
            non_cash_meal_income_amt: input.non_cash_meal_income_amt,
            non_cash_car_income_amt: input.non_cash_car_income_amt,
            total_non_cash_other_property_amt: input.total_non_cash_other_property_amt,
            cost_of_living_and_overseas_diff_amt: input.cost_of_living_and_overseas_diff_amt,
            family_allowance_amt: input.family_allowance_amt,
            education_allowance_amt: input.education_allowance_amt,
            home_leave_allowance_amt: input.home_leave_allowance_amt,
            quarters_allowance_amt: input.quarters_allowance_amt,
            other_purpose_allowance_amt: input.other_purpose_allowance_amt,
            other_purpose_allowance_desc: input.other_purpose_allowance_desc,
            total_allowances_paid_on_behalf_amt: line22g,
            total_other_foreign_income_amt: input.total_other_foreign_income_amt,
            total_foreign_earned_income_amt: line24,
            excludable_meals_lodging_amt: line25,
            foreign_earned_income_amt: line26,

            // Part V
            line_27_foreign_earned_income_amt: line27,
            claiming_housing_excl_or_ded_ind: claiming_housing,

            // Part VI
            housing_qualified_expense_amt: input.housing_qualified_expense_amt,
            housing_expense_location_desc: input.housing_expense_location_desc,
            housing_expense_limit_amt: input.housing_expense_limit_amt,
            smaller_qualified_or_limit_amt: line30,
            housing_qualified_days_cnt: days31,
            housing_maximum_allowed_amt: line32,
            housing_expenses_over_max_amt: line33,
            employer_provided_housing_amt: input.employer_provided_housing_amt,
            employer_prov_housing_excl_pct: line35_str,
            housing_exclusion_amt: line36,

            // Part VII
            max_foreign_earned_inc_exclusion_amt: line37,
            foreign_earn_incm_excl_qlfy_days_cnt: days38,
            foreign_earned_inc_exclusion_pct: line39_str,
            tent_foreign_earned_income_excl_amt: line40,
            tentative_income_exclusion_amt: line41,
            foreign_earned_inc_exclusion_amt: line42,

            // Part VIII
            housing_and_income_exclusion_amt: line43,
            deduction_alloc_to_excluded_inc_amt: line44,
            total_income_exclusion_amt: line45,

            // Part IX
            housing_expense_less_exclusion_amt: line46,
            foreign_income_less_total_exclusion_amt: line47,
            housing_deduction_tentative_amt: line48,
            housing_deduction_carryover_amt: input.housing_deduction_carryover_amt,
            housing_deduction_amt: line50,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[]
    }

    fn is_valid(&self) -> bool {
        // Line 22g
        let line22g = self.cost_of_living_and_overseas_diff_amt
            + self.family_allowance_amt
            + self.education_allowance_amt
            + self.home_leave_allowance_amt
            + self.quarters_allowance_amt
            + self.other_purpose_allowance_amt;
        let line22g_ok = self.total_allowances_paid_on_behalf_amt == line22g;

        // Line 24
        let line24 = self.foreign_earned_total_wages_inc_amt
            + self.foreign_business_income_share_amt
            + self.foreign_partnership_inc_share_amt
            + self.non_cash_lodging_income_amt
            + self.non_cash_meal_income_amt
            + self.non_cash_car_income_amt
            + self.total_non_cash_other_property_amt
            + line22g
            + self.total_other_foreign_income_amt;
        let line24_ok = self.total_foreign_earned_income_amt == line24;

        // Line 26
        let line26_ok =
            self.foreign_earned_income_amt == line24 - self.excludable_meals_lodging_amt;

        // Line 27
        let line27_ok = self.line_27_foreign_earned_income_amt == self.foreign_earned_income_amt;

        // Line 43
        let line43_ok = self.housing_and_income_exclusion_amt
            == self.housing_exclusion_amt + self.foreign_earned_inc_exclusion_amt;

        // Line 45
        let line45_ok = self.total_income_exclusion_amt
            == self.housing_and_income_exclusion_amt - self.deduction_alloc_to_excluded_inc_amt;

        line22g_ok && line24_ok && line26_ok && line27_ok && line43_ok && line45_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: basic full-year single filer with foreign wages only, no housing.
    fn basic_input() -> F2555Input {
        F2555Input {
            name_line1_txt: "Jane Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            foreign_address: "10 Rue de Paris, 75001 Paris, France".to_string(),
            occupation_txt: "Software Engineer".to_string(),
            business_name_line1_txt: "Acme Corp".to_string(),
            business_name_line2_txt: String::new(),
            employer_us_address: "100 Main St, New York, NY 10001".to_string(),
            employer_foreign_address: "10 Rue de Paris, Paris, France".to_string(),
            employer_foreign_entity_ind: false,
            employer_united_states_company_ind: true,
            self_employment_ind: false,
            foreign_affiliate_employer_ind: false,
            other_employer_ind: false,
            other_employer_desc: String::new(),
            last_frgn_earn_inc_excl_claimed_yr: 2024,
            no_frgn_earn_inc_excl_prev_filed_ind: false,
            foreign_earn_inc_excl_revoked_ind: false,
            claim_frgn_earn_inc_waiver_cd: String::new(),
            citizen_country_nm: "United States".to_string(),
            separate_foreign_residence_ind: false,
            separate_foreign_res_location_txt: String::new(),
            separate_foreign_residence_day_cnt: 0,
            tax_home_group: "Paris, France, 01/01/2024".to_string(),
            bona_fide_residence_begin_dt: "01/01/2024".to_string(),
            bona_fide_residence_end_dt: "12/31/2025".to_string(),
            purchased_house_ind: false,
            rented_house_ind: true,
            rented_room_ind: false,
            employer_furnished_quarters_ind: false,
            family_lived_abroad_ind: false,
            family_living_with_taxpayer_abroad: String::new(),
            submitted_non_resident_stmt_ind: false,
            required_to_pay_income_tax_ind: true,
            line14: vec![],
            employment_contract_terms_desc: "Indefinite".to_string(),
            visa_type_desc: "Work visa".to_string(),
            visa_limit_stay_or_employment_ind: false,
            maintained_house_in_us_ind: false,
            us_home_while_living_abroad: String::new(),
            physical_presence_begin_dt: String::new(),
            physical_presence_end_dt: String::new(),
            principal_employment_country_nm: "France".to_string(),
            line18: vec![],
            foreign_earned_total_wages_inc_amt: Usd::from_dollars(150_000),
            foreign_business_income_share_amt: Usd::ZERO,
            foreign_partnership_inc_share_amt: Usd::ZERO,
            non_cash_lodging_income_amt: Usd::ZERO,
            non_cash_meal_income_amt: Usd::ZERO,
            non_cash_car_income_amt: Usd::ZERO,
            total_non_cash_other_property_amt: Usd::ZERO,
            cost_of_living_and_overseas_diff_amt: Usd::ZERO,
            family_allowance_amt: Usd::ZERO,
            education_allowance_amt: Usd::ZERO,
            home_leave_allowance_amt: Usd::ZERO,
            quarters_allowance_amt: Usd::ZERO,
            other_purpose_allowance_amt: Usd::ZERO,
            other_purpose_allowance_desc: String::new(),
            total_other_foreign_income_amt: Usd::ZERO,
            excludable_meals_lodging_amt: Usd::ZERO,
            claiming_housing_excl_or_ded_ind: false,
            housing_qualified_expense_amt: Usd::ZERO,
            housing_expense_location_desc: String::new(),
            housing_expense_limit_amt: Usd::ZERO,
            housing_qualified_days_cnt: 0,
            employer_provided_housing_amt: Usd::ZERO,
            foreign_earn_incm_excl_qlfy_days_cnt: 365,
            deduction_alloc_to_excluded_inc_amt: Usd::ZERO,
            housing_deduction_carryover_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_with_wages() {
        assert!(Output2555::must_file(&basic_input()));
    }

    #[test]
    fn must_file_no_income() {
        let mut input = basic_input();
        input.foreign_earned_total_wages_inc_amt = Usd::ZERO;
        assert!(!Output2555::must_file(&input));
    }

    #[test]
    fn basic_full_year_exclusion_no_housing() {
        // Wages $150,000, full year, no housing
        // Line 22g = 0, line 24 = 150,000, line 26 = 150,000
        // Line 27 = 150,000
        // Line 36 = 0 (not claiming housing)
        // Line 37 = 130,000, line 38 = 365, line 39 = 1.000
        // Line 40 = 130,000
        // Line 41 = 150,000 - 0 = 150,000
        // Line 42 = min(130,000, 150,000) = 130,000
        // Line 43 = 0 + 130,000 = 130,000
        // Line 45 = 130,000 - 0 = 130,000
        let form = Output2555::try_new(basic_input()).unwrap();
        assert_eq!(
            form.total_foreign_earned_income_amt,
            Usd::from_dollars(150_000)
        );
        assert_eq!(form.foreign_earned_income_amt, Usd::from_dollars(150_000));
        assert_eq!(
            form.line_27_foreign_earned_income_amt,
            Usd::from_dollars(150_000)
        );
        assert_eq!(
            form.max_foreign_earned_inc_exclusion_amt,
            Usd::from_dollars(130_000)
        );
        assert_eq!(form.foreign_earned_inc_exclusion_pct, "1.000");
        assert_eq!(
            form.tent_foreign_earned_income_excl_amt,
            Usd::from_dollars(130_000)
        );
        assert_eq!(
            form.tentative_income_exclusion_amt,
            Usd::from_dollars(150_000)
        );
        assert_eq!(
            form.foreign_earned_inc_exclusion_amt,
            Usd::from_dollars(130_000)
        );
        assert_eq!(
            form.housing_and_income_exclusion_amt,
            Usd::from_dollars(130_000)
        );
        assert_eq!(form.total_income_exclusion_amt, Usd::from_dollars(130_000));
        assert!(form.is_valid());
    }

    #[test]
    fn income_below_max_exclusion() {
        // Wages $100,000 < max exclusion $130,000
        // Line 42 = min(130,000, 100,000) = 100,000
        let mut input = basic_input();
        input.foreign_earned_total_wages_inc_amt = Usd::from_dollars(100_000);
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(
            form.foreign_earned_inc_exclusion_amt,
            Usd::from_dollars(100_000)
        );
        assert_eq!(form.total_income_exclusion_amt, Usd::from_dollars(100_000));
        assert!(form.is_valid());
    }

    #[test]
    fn partial_year_pro_rata() {
        // 200 days qualifying, full-year wages $150,000, no housing
        // Line 39 = 200/365 = 0.547
        // Line 40 = 130,000 * 200 / 365 = 71,232.876... => from_cents(13_000_000 * 200 / 365) = from_cents(7_123_287)
        let mut input = basic_input();
        input.foreign_earn_incm_excl_qlfy_days_cnt = 200;
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(form.foreign_earned_inc_exclusion_pct, "0.547");
        let expected_line40 = Usd::from_cents(13_000_000 * 200 / 365);
        assert_eq!(form.tent_foreign_earned_income_excl_amt, expected_line40);
        assert_eq!(form.foreign_earned_inc_exclusion_amt, expected_line40);
        assert!(form.is_valid());
    }

    #[test]
    fn line22g_sums_allowances() {
        let mut input = basic_input();
        input.cost_of_living_and_overseas_diff_amt = Usd::from_dollars(5_000);
        input.family_allowance_amt = Usd::from_dollars(3_000);
        input.education_allowance_amt = Usd::from_dollars(2_000);
        input.home_leave_allowance_amt = Usd::from_dollars(1_000);
        input.quarters_allowance_amt = Usd::from_dollars(4_000);
        input.other_purpose_allowance_amt = Usd::from_dollars(500);
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(
            form.total_allowances_paid_on_behalf_amt,
            Usd::from_dollars(15_500)
        );
        // Line 24 = 150,000 + 15,500 = 165,500
        assert_eq!(
            form.total_foreign_earned_income_amt,
            Usd::from_dollars(165_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn excludable_meals_lodging_reduces_line26() {
        let mut input = basic_input();
        input.excludable_meals_lodging_amt = Usd::from_dollars(10_000);
        let form = Output2555::try_new(input).unwrap();
        // Line 26 = 150,000 - 10,000 = 140,000
        assert_eq!(form.foreign_earned_income_amt, Usd::from_dollars(140_000));
        assert_eq!(
            form.line_27_foreign_earned_income_amt,
            Usd::from_dollars(140_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn housing_exclusion_full_year() {
        // Claiming housing, full year (365 days)
        // Line 28 = $30,000 housing expenses
        // Line 29b = $40,000 limit
        // Line 30 = min(30,000, 40,000) = 30,000
        // Line 32 = $20,800 (365 days)
        // Line 33 = 30,000 - 20,800 = 9,200
        // Line 34 = $50,000 employer-provided
        // Line 27 = $150,000
        // Line 35 = 50,000/150,000 = 0.333
        // Line 36 = min(9,200 * 50,000/150,000, 50,000) = min(3,066.66, 50,000) = 3,066.66
        let mut input = basic_input();
        input.claiming_housing_excl_or_ded_ind = true;
        input.housing_qualified_expense_amt = Usd::from_dollars(30_000);
        input.housing_expense_limit_amt = Usd::from_dollars(40_000);
        input.housing_qualified_days_cnt = 365;
        input.employer_provided_housing_amt = Usd::from_dollars(50_000);
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(
            form.smaller_qualified_or_limit_amt,
            Usd::from_dollars(30_000)
        );
        assert_eq!(
            form.housing_maximum_allowed_amt,
            Rules2025.f2555_housing_full_year()
        );
        assert_eq!(form.housing_expenses_over_max_amt, Usd::from_dollars(9_200));
        assert_eq!(form.employer_prov_housing_excl_pct, "0.333");
        // line 36 = 920_000 * 5_000_000 / 15_000_000 = 306_666 cents
        let expected_line36 = Usd::from_cents(920_000 * 5_000_000 / 15_000_000);
        assert_eq!(form.housing_exclusion_amt, expected_line36);
        assert!(form.is_valid());
    }

    #[test]
    fn housing_partial_year_line32() {
        // 200 days qualifying
        // Line 32 = $56.99 * 200 = $11,398
        let mut input = basic_input();
        input.claiming_housing_excl_or_ded_ind = true;
        input.housing_qualified_expense_amt = Usd::from_dollars(20_000);
        input.housing_expense_limit_amt = Usd::from_dollars(30_000);
        input.housing_qualified_days_cnt = 200;
        input.employer_provided_housing_amt = Usd::from_dollars(20_000);
        input.foreign_earn_incm_excl_qlfy_days_cnt = 200;
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(
            form.housing_maximum_allowed_amt,
            Usd::from_cents(Rules2025.f2555_housing_per_day_cents() * 200)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn housing_expenses_below_base_no_exclusion() {
        // Line 28 = $15,000 < line 32 = $20,800 => line 33 = 0 => no housing exclusion
        let mut input = basic_input();
        input.claiming_housing_excl_or_ded_ind = true;
        input.housing_qualified_expense_amt = Usd::from_dollars(15_000);
        input.housing_expense_limit_amt = Usd::from_dollars(40_000);
        input.housing_qualified_days_cnt = 365;
        input.employer_provided_housing_amt = Usd::from_dollars(50_000);
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(form.housing_expenses_over_max_amt, Usd::ZERO);
        assert_eq!(form.housing_exclusion_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn housing_deduction_part_ix() {
        // Set up so Part IX applies: line 33 > line 36 AND line 27 > line 43
        // Wages = $200,000, employer-provided housing = $10,000
        // Housing expenses = $50,000, limit = $60,000, 365 days
        // Line 30 = min(50,000, 60,000) = 50,000
        // Line 32 = $20,800
        // Line 33 = 50,000 - 20,800 = 29,200
        // Line 35 = 10,000/200,000 = 0.050
        // Line 36 = min(29,200 * 10,000/200,000, 10,000) = min(1,460, 10,000) = 1,460
        // Line 40 = 130,000 (full year)
        // Line 41 = 200,000 - 1,460 = 198,540
        // Line 42 = min(130,000, 198,540) = 130,000
        // Line 43 = 1,460 + 130,000 = 131,460
        // line 33 (29,200) > line 36 (1,460) ✓
        // line 27 (200,000) > line 43 (131,460) ✓ => Part IX applies
        // Line 46 = 29,200 - 1,460 = 27,740
        // Line 47 = 200,000 - 131,460 = 68,540
        // Line 48 = min(27,740, 68,540) = 27,740
        // Line 50 = 27,740 + 0 = 27,740
        let mut input = basic_input();
        input.foreign_earned_total_wages_inc_amt = Usd::from_dollars(200_000);
        input.claiming_housing_excl_or_ded_ind = true;
        input.housing_qualified_expense_amt = Usd::from_dollars(50_000);
        input.housing_expense_limit_amt = Usd::from_dollars(60_000);
        input.housing_qualified_days_cnt = 365;
        input.employer_provided_housing_amt = Usd::from_dollars(10_000);
        let form = Output2555::try_new(input).unwrap();

        let expected_line36 = Usd::from_cents(2_920_000 * 1_000_000 / 20_000_000);
        assert_eq!(form.housing_exclusion_amt, expected_line36);

        let expected_line43 = expected_line36 + Usd::from_dollars(130_000);
        assert_eq!(form.housing_and_income_exclusion_amt, expected_line43);

        let expected_line46 = Usd::from_dollars(29_200) - expected_line36;
        assert_eq!(form.housing_expense_less_exclusion_amt, expected_line46);

        let expected_line47 = Usd::from_dollars(200_000) - expected_line43;
        assert_eq!(
            form.foreign_income_less_total_exclusion_amt,
            expected_line47
        );

        let expected_line48 = expected_line46.min(expected_line47);
        assert_eq!(form.housing_deduction_tentative_amt, expected_line48);
        assert_eq!(form.housing_deduction_amt, expected_line48);
        assert!(form.is_valid());
    }

    #[test]
    fn housing_deduction_with_carryover() {
        let mut input = basic_input();
        input.foreign_earned_total_wages_inc_amt = Usd::from_dollars(200_000);
        input.claiming_housing_excl_or_ded_ind = true;
        input.housing_qualified_expense_amt = Usd::from_dollars(50_000);
        input.housing_expense_limit_amt = Usd::from_dollars(60_000);
        input.housing_qualified_days_cnt = 365;
        input.employer_provided_housing_amt = Usd::from_dollars(10_000);
        input.housing_deduction_carryover_amt = Usd::from_dollars(5_000);
        let form = Output2555::try_new(input).unwrap();
        // Line 50 = line 48 + 5,000
        assert_eq!(
            form.housing_deduction_amt,
            form.housing_deduction_tentative_amt + Usd::from_dollars(5_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn part_ix_does_not_apply_when_no_excess() {
        // Not claiming housing => line 33 = 0, line 36 = 0
        // line 33 is NOT > line 36 => Part IX doesn't apply
        let form = Output2555::try_new(basic_input()).unwrap();
        assert_eq!(form.housing_expense_less_exclusion_amt, Usd::ZERO);
        assert_eq!(form.foreign_income_less_total_exclusion_amt, Usd::ZERO);
        assert_eq!(form.housing_deduction_tentative_amt, Usd::ZERO);
        assert_eq!(form.housing_deduction_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn deductions_reduce_line45() {
        let mut input = basic_input();
        input.deduction_alloc_to_excluded_inc_amt = Usd::from_dollars(5_000);
        let form = Output2555::try_new(input).unwrap();
        // Line 45 = 130,000 - 5,000 = 125,000
        assert_eq!(form.total_income_exclusion_amt, Usd::from_dollars(125_000));
        assert!(form.is_valid());
    }

    #[test]
    fn line14_entries_preserved() {
        let mut input = basic_input();
        input.line14 = vec![
            F2555Line14 {
                arrival_dt: "06/01/2025".to_string(),
                departure_dt: "06/15/2025".to_string(),
                business_days_in_us_cnt: 10,
                us_business_income_amt: Usd::from_dollars(5_000),
            },
            F2555Line14 {
                arrival_dt: "11/20/2025".to_string(),
                departure_dt: "11/30/2025".to_string(),
                business_days_in_us_cnt: 8,
                us_business_income_amt: Usd::from_dollars(3_000),
            },
        ];
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(form.line14.len(), 2);
        assert_eq!(form.line14[0].arrival_dt, "06/01/2025");
        assert_eq!(form.line14[0].business_days_in_us_cnt, 10);
        assert_eq!(
            form.line14[1].us_business_income_amt,
            Usd::from_dollars(3_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn line18_entries_preserved() {
        let mut input = basic_input();
        input.line18 = vec![
            F2555Line18 {
                country_nm: "France".to_string(),
                arrival_dt: "01/01/2025".to_string(),
                departure_dt: "06/01/2025".to_string(),
                days_present_in_country_cnt: 151,
                business_days_in_us_cnt: 0,
                us_business_income_amt: Usd::ZERO,
            },
            F2555Line18 {
                country_nm: "United States".to_string(),
                arrival_dt: "06/01/2025".to_string(),
                departure_dt: "06/15/2025".to_string(),
                days_present_in_country_cnt: 14,
                business_days_in_us_cnt: 10,
                us_business_income_amt: Usd::from_dollars(5_000),
            },
        ];
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(form.line18.len(), 2);
        assert_eq!(form.line18[0].country_nm, "France");
        assert_eq!(form.line18[0].days_present_in_country_cnt, 151);
        assert_eq!(form.line18[1].country_nm, "United States");
        assert_eq!(form.line18[1].business_days_in_us_cnt, 10);
        assert!(form.is_valid());
    }

    #[test]
    fn qualifying_days_exceeds_365_returns_error() {
        let mut input = basic_input();
        input.foreign_earn_incm_excl_qlfy_days_cnt = 366;
        let err = Output2555::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn housing_days_exceeds_365_returns_error() {
        let mut input = basic_input();
        input.housing_qualified_days_cnt = 400;
        let err = Output2555::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn zero_qualifying_days_zero_exclusion() {
        let mut input = basic_input();
        input.foreign_earn_incm_excl_qlfy_days_cnt = 0;
        let form = Output2555::try_new(input).unwrap();
        assert_eq!(form.foreign_earned_inc_exclusion_pct, "0.000");
        assert_eq!(form.tent_foreign_earned_income_excl_amt, Usd::ZERO);
        assert_eq!(form.foreign_earned_inc_exclusion_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn form_name_and_year() {
        assert_eq!(Output2555::name(), "Form 2555");
        let form = Output2555::try_new(basic_input()).unwrap();
        assert_eq!(form.year(), TaxYear::Y2025);
        assert_eq!(<Output2555 as Form>::form_type(), FormType::Output);
    }

    #[test]
    fn no_dependencies() {
        assert!(Output2555::dependencies().is_empty());
    }
}
