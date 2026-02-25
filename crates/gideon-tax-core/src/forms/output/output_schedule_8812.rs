use crate::Usd;

/// Output fields for IRS Schedule 8812 (Form 1040) 2025 — Credits for Qualifying Children and Other Dependents.
#[derive(Debug, Clone, Default)]
pub struct OutputSchedule8812 {
    // -----------------------------------------------------------------------
    // Part I — Child Tax Credit and Credit for Other Dependents
    // -----------------------------------------------------------------------
    /// Line 1: Enter the amount from line 11a of your Form 1040, 1040-SR, or 1040-NR
    pub adjusted_gross_income_amt: Usd,
    /// Line 2a: Enter income from Puerto Rico that you excluded
    pub excld_sect933_puerto_rico_incm_amt: Usd,
    /// Line 2b: Enter the amounts from lines 45 and 50 of your Form 2555
    pub gross_income_exclusion_amt: Usd,
    /// Line 2c: Enter the amount from line 15 of your Form 4563
    pub exclusion_and_deduction_sum_amt: Usd,
    /// Line 2d: Add lines 2a through 2c
    pub additional_income_adj_amt: Usd,
    /// Line 3: Add lines 1 and 2d
    pub modified_agi_amt: Usd,
    /// Line 4: Number of qualifying children under age 17 with the required social security number
    pub qlfy_child_under_age_ssn_cnt: u32,
    /// Line 5: Multiply line 4 by $2,200
    pub qlfy_child_under_age_ssn_limt_amt: Usd,
    /// Line 6: Number of other dependents, including any qualifying children who are not under age
    /// 17 or who do not have the required social security number
    pub other_dependent_cnt: u32,
    /// Line 7: Multiply line 6 by $500
    pub other_dependent_credit_amt: Usd,
    /// Line 8: Add lines 5 and 7
    pub initial_ctcodc_amt: Usd,
    /// Line 9: Enter the amount shown below for your filing status
    /// (Married filing jointly — $400,000; All other filing statuses — $200,000)
    pub filing_status_threshold_cd: String,
    /// Line 9: Filing status threshold amount
    pub modified_agi_phase_out_amt: Usd,
    /// Line 10: Subtract line 9 from line 3. If more than zero and not a multiple of $1,000,
    /// enter the next multiple of $1,000
    pub excess_adj_gross_income_amt: Usd,
    /// Line 11: Multiply line 10 by 5% (0.05)
    pub ctcodc_amt: Usd,
    /// Line 12: Is the amount on line 8 more than the amount on line 11?
    pub ctcodc_over_agi_limit_ind: bool,
    /// Line 12 (Yes): Subtract line 11 from line 8. Enter the result.
    /// Line 12 (No/Stop): Cannot take the child tax credit, credit for other dependents,
    /// or additional child tax credit
    pub ctcodc_after_agi_limit_amt: Usd,
    /// Line 13: Enter the amount from Credit Limit Worksheet A
    pub actc_tax_liabilty_limit_amt: Usd,
    /// Line 14: Enter the smaller of line 12 or line 13. This is your child tax credit and
    /// credit for other dependents. Enter this amount on Form 1040, 1040-SR, or 1040-NR, line 19
    pub actc_after_limit_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II-A — Additional Child Tax Credit for All Filers
    // -----------------------------------------------------------------------
    /// Line 15: Reserved for future use
    pub from_tax_return_amt: Usd,
    /// Line 16a: Subtract line 14 from line 12. If zero, stop here; you cannot take the
    /// additional child tax credit
    pub actc_before_limit_amt: Usd,
    /// Line 16b: Number of qualifying children under age 17 with the required social security
    /// number, multiplied by $1,700
    pub calculated_difference_amt: Usd,
    /// Line 17: Enter the smaller of line 16a or line 16b
    pub larger_calc_income_or_diff_amt: Usd,
    /// Line 18a: Earned income (see instructions)
    pub total_earned_income_amt: Usd,
    /// Line 18b: Nontaxable combat pay (see instructions)
    pub nontaxable_combat_pay_amt: Usd,
    /// Line 19: Is the amount on line 18a more than $2,500?
    pub earned_incm_more_than_specified_ind: bool,
    /// Line 19 (Yes): Subtract $2,500 from the amount on line 18a. Enter the result
    pub net_earned_income_calculated_amt: Usd,
    /// Line 20: Multiply the amount on line 19 by 15% (0.15) and enter the result
    pub net_total_earned_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II-A (continued) — Three or more qualifying children check
    // -----------------------------------------------------------------------
    /// Line 20 (continued): On line 16b, is the amount $5,100 or more?
    pub three_or_more_qlfy_children_ind: bool,

    // -----------------------------------------------------------------------
    // Part II-B — Certain Filers Who Have Three or More Qualifying Children
    // and Bona Fide Residents of Puerto Rico
    // -----------------------------------------------------------------------
    /// Line 21: Withheld social security, Medicare, and Additional Medicare taxes from Form(s) W-2,
    /// boxes 4 and 6
    pub from_w2_amt: Usd,
    /// Line 22: Enter the total of the amounts from Schedule 1 (Form 1040), line 15;
    /// Schedule 2 (Form 1040), line 5; Schedule 2 (Form 1040), line 6; and
    /// Schedule 2 (Form 1040), line 13
    pub calc_from_w2_and_return_amt: Usd,
    /// Line 23: Add lines 21 and 22
    pub calc_amt_from_ret_plus_tax_whld_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II-C — Additional Child Tax Credit
    // -----------------------------------------------------------------------
    /// Line 27: This is your additional child tax credit. Enter this amount on Form 1040,
    /// 1040-SR, or 1040-NR, line 28
    pub additional_child_tax_credit_amt: Usd,
}
