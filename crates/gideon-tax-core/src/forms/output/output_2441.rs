use crate::Usd;

/// Output fields for IRS Form 2441 (2025) — Child and Dependent Care Expenses.
#[derive(Debug, Clone, Default)]
pub struct Output2441 {
    // -----------------------------------------------------------------------
    // Part III — Dependent Care Benefits (Lines 12–26)
    // -----------------------------------------------------------------------
    /// Part III: Adjusted dependent care benefits amount
    pub adjusted_depd_care_benefits_amt: Usd,
    /// Line 7: Enter the amount from Form 1040, 1040-SR, or 1040-NR, line 11a (AGI)
    pub adjusted_gross_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Credit for Child and Dependent Care Expenses (Lines 2–11)
    // -----------------------------------------------------------------------
    /// Allowed amount based on number of persons cared for
    pub allowed_cared_for_amt: Usd,
    /// CPYE credit amount
    pub cpye_credit_amt: Usd,
    /// Calculated tentative expense amount
    pub calculated_tentative_expense_amt: Usd,
    /// Line 8: Decimal amount from the table based on line 7 AGI
    pub care_expenses_decimal_amt: Usd,
    /// Line 13: Carryover amount from 2024 used in 2025 during the grace period
    pub carryover_amt: Usd,
    /// Line 11: Credit for child and dependent care expenses (smaller of line 9c or line 10)
    pub credit_for_child_and_depd_care_amt: Usd,
    /// Line 24: Deductible benefits (smallest of line 20, 21, or 22)
    pub deductible_benefits_amt: Usd,
    /// Line 12: Total amount of dependent care benefits received in 2025
    pub dependent_care_benefits_amt: Usd,
    /// Line 18: Enter your earned income
    pub earned_income_amt: Usd,
    /// Eligibility requirement met indicator (filing status check box A or B)
    pub eligibility_requirement_met_ind: bool,
    /// Line 25: Excluded benefits (subtract line 24 from the smaller of line 20 or line 21)
    pub excluded_benefits_amt: Usd,
    /// Line 14: Forfeited or carried forward to 2026 amount
    pub forfeited_amt: Usd,
    /// Form 1040-A filed indicator
    pub form1040_a_filed_ind: bool,
    /// Part I: Check this box if you have more than three care providers
    pub more_than_three_care_providers_ind: bool,
    /// Line 2: Check this box if you have more than three qualifying persons
    pub more_than_three_qlfy_persons_ind: bool,
    /// Line 29: Subtract line 28 from line 27. Net allowable amount
    pub net_allowable_amt: Usd,
    /// Line 4: Enter your earned income (primary taxpayer)
    pub primary_earned_income_amt: Usd,
    /// Line 23: Subtract line 22 from line 15 (sole proprietorship/partnership less adjusted benefits)
    pub propshp_prtshp_less_adj_bnft_amt: Usd,
    /// Line 16: Total amount of qualified expenses incurred in 2025 for qualifying person(s)
    pub qualified_expenses_amt: Usd,
    /// Smaller of adjusted or qualified amount
    pub smaller_of_adj_or_qualified_amt: Usd,
    /// Line 6: Enter the smallest of line 3, 4, or 5. If zero or less, enter -0-
    pub smaller_of_expenses_or_income_amt: Usd,
    /// Line 31: Smaller of line 29 or line 30
    pub smaller_of_total_qlfy_expenses_amt: Usd,
    /// Line 22: Is any amount on line 12 or 13 from your sole proprietorship or partnership (Yes amount)
    pub sole_propshp_prtshp_amt: Usd,
    /// Line 27: Enter $3,000 ($6,000 if two or more qualifying persons)
    pub specified_amt: Usd,
    /// Line 5: Spouse's earned income (if married filing jointly)
    pub spouse_earned_income_amt: Usd,
    /// Line 19: Spouse income amount (for student or disabled spouse)
    pub spouse_income_amt: Usd,
    /// Student or disabled spouse indicator (for deemed income on line 5)
    pub student_or_disabled_ind: bool,
    /// Line 28: Add lines 24 and 25 (sum of deductible and excluded benefits)
    pub sum_of_ded_and_excluded_benefits_amt: Usd,
    /// Line 10: Tax liability limit from the Credit Limit Worksheet in the instructions
    pub tax_liab_lmt_from_cr_lmt_wrksht_amt: Usd,
    /// Line 26: Taxable benefits (subtract line 25 from line 23). Enter on Form 1040, line 1e
    pub taxable_benefits_amt: Usd,
    /// Tentative exclusion amount
    pub tentative_exclusion_amt: Usd,
    /// Total eligible CDCC amount
    pub total_elig_cdcc_amt: Usd,
    /// Line 3: Add the amounts in column (d) of line 2. Total qualified expenses or limit
    pub total_qlfd_expenses_or_limit_amt: Usd,
    /// Line 30: Complete line 2 on page 1. Add the amounts in column (d) — total qualified expenses
    pub total_qualified_expenses_amt: Usd,
}
