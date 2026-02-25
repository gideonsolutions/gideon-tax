use crate::Usd;

/// Output fields for IRS Schedule 1-A (Form 1040) 2025 — Additional Deductions.
#[derive(Debug, Clone, Default)]
pub struct OutputSchedule1A {
    // -----------------------------------------------------------------------
    // Part I — Modified Adjusted Gross Income (MAGI) Amount
    // -----------------------------------------------------------------------
    /// Line 1: Enter the amount from Form 1040, 1040-SR, or 1040-NR, line 11b
    pub adjusted_gross_income_amt: Usd,
    /// Line 2a: Enter any income from Puerto Rico that you excluded
    pub excld_sect_933_puerto_rico_incm_amt: Usd,
    /// Line 2b: Enter the amount from Form 2555, line 45
    pub gross_income_exclusion_amt: Usd,
    /// Line 2c: Enter the amount from Form 2555, line 50
    pub housing_deduction_amt: Usd,
    /// Line 2d: Enter the amount from Form 4563, line 15
    pub net_operating_loss_deduction_amt: Usd,
    /// Line 2e: Add lines 2a, 2b, 2c, and 2d
    pub total_income_exclusion_amt: Usd,
    /// Line 3: Add lines 1 and 2e
    pub modified_agi_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — No Tax on Tips
    // -----------------------------------------------------------------------
    /// Line 4a: Enter qualified tips included on Form W-2, box 7, but see the instructions if
    /// Form W-2, box 5 is more than $176,100 or you received tips that are not
    /// subject to social security and Medicare taxes
    pub qualified_tips_wages_amt: Usd,
    /// Line 4b: Qualified tips included on Form 4137, line 1, row A, column (c). If Form 4137 is
    /// not filed, enter -0-
    pub qualified_tips_form_4137_amt: Usd,
    /// Line 4c: If you only received qualified tips as an employee with respect to employment with one
    /// employer, enter the larger of line 4a or line 4b. Otherwise, see the instructions to determine
    /// the amount to enter on line 4c
    pub qualified_tips_employee_amt: Usd,
    /// Line 5: Qualified tips received in the course of a trade or business
    pub qualified_tips_trade_or_bus_amt: Usd,
    /// Line 6: Add lines 4c and 5
    pub total_qualified_tips_amt: Usd,
    /// Line 7: Enter the smaller of the amount on line 6 or $25,000
    pub smaller_tips_or_max_ded_amt: Usd,
    /// Line 9: Enter $150,000 ($300,000 if married filing jointly)
    pub tips_filing_status_thrshld_amt: Usd,
    /// Line 10: Subtract line 9 from line 8. If zero or less, enter the amount from line 7 on line 13
    pub tips_magi_less_thrshld_amt: Usd,
    /// Line 11: Divide line 10 by $1,000. If the resulting number isn't a whole number, decrease the
    /// result to the next lower whole number
    pub tips_magi_less_thrshld_divide_num: String,
    /// Line 12: Multiply line 11 by $100
    pub tips_magi_less_thrshld_red_amt: Usd,
    /// Line 13: Qualified tips deduction. Subtract line 12 from line 7. If zero or less, enter -0-
    pub qualified_tips_deduction_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — No Tax on Overtime
    // -----------------------------------------------------------------------
    /// Line 14a: Qualified overtime compensation included in Form W-2, box 1. If you received
    /// qualified overtime compensation not reported on Form W-2, box 1, see instructions
    pub qualified_overtime_wages_amt: Usd,
    /// Line 14b: Qualified overtime compensation included in Form 1099-NEC, box 1, or Form
    /// 1099-MISC, box 3 (see instructions)
    pub qualified_overtime_form_1099_amt: Usd,
    /// Line 14c: Add lines 14a and 14b
    pub total_qualified_overtime_amt: Usd,
    /// Line 15: Enter the smaller of the amount on line 14c or $12,500 ($25,000 if married filing
    /// jointly)
    pub smaller_overtime_or_max_ded_amt: Usd,
    /// Line 17: Enter $150,000 ($300,000 if married filing jointly)
    pub overtime_filing_status_thrshld_amt: Usd,
    /// Line 18: Subtract line 17 from line 16. If zero or less, enter the amount from line 15 on
    /// line 21
    pub ot_magi_less_thrshld_amt: Usd,
    /// Line 19: Divide line 18 by $1,000. If the resulting number isn't a whole number, decrease the
    /// result to the next lower whole number
    pub ot_magi_less_thrshld_divide_num: String,
    /// Line 20: Multiply line 19 by $100
    pub ot_magi_less_thrshld_red_amt: Usd,
    /// Line 21: Qualified overtime compensation deduction. Subtract line 20 from line 15. If zero or
    /// less, enter -0-
    pub qualified_overtime_comp_ded_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — No Tax on Car Loan Interest
    // -----------------------------------------------------------------------
    /// Line 22a: Schedule 1-A column (iii) amount
    pub qualified_car_loan_interest_amt: Usd,
    /// Line 22a: Deducted on Schedule C, Schedule E, or Schedule F — column (ii) amount
    pub qualified_car_loan_int_ded_sch_amt: Usd,
    /// Line 23: Add lines 22a and 22b, column (iii)
    pub tot_qualified_car_loan_interest_amt: Usd,
    /// Line 24: Enter the smaller of the amount on line 23 or $10,000
    pub smaller_car_loan_int_or_max_ded_amt: Usd,
    /// Line 26: Enter $100,000 ($200,000 if married filing jointly)
    pub car_ln_int_filing_status_thrshld_amt: Usd,
    /// Line 27: Subtract line 26 from line 25. If zero or less, enter the amount from line 24 on
    /// line 30
    pub car_ln_int_magi_less_thrshld_amt: Usd,
    /// Line 28: Divide line 27 by $1,000. If the resulting number isn't a whole number, increase the
    /// result to the next higher whole number
    pub car_ln_int_magi_less_thrshld_div_num: String,
    /// Line 29: Multiply line 28 by $200
    pub car_ln_int_magi_less_thrshld_red_amt: Usd,
    /// Line 30: Qualified passenger vehicle loan interest deduction. Subtract line 29 from line 24.
    /// If zero or less, enter -0-
    pub qualified_car_loan_interest_ded_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Enhanced Deduction for Seniors
    // -----------------------------------------------------------------------
    /// Line 32: Enter $75,000 ($150,000 if married filing jointly)
    pub enhnc_sr_ded_fs_thrshld_amt: Usd,
    /// Line 33: Subtract line 32 from line 31. If zero or less, enter $6,000 on line 35
    pub enhnc_sr_ded_magi_less_thrshld_amt: Usd,
    /// Line 34: Multiply line 33 by 6% (0.06)
    pub specfied_dol_less_thrshld_red_amt: Usd,
    /// Line 35: Subtract line 34 from $6,000. If zero or less, enter -0-
    pub enhn_sr_ded_magi_less_thrshld_red_amt: Usd,
    /// Line 36a: If you have a valid social security number (see instructions) and were born before
    /// January 2, 1961, enter the amount from line 35
    pub primary_enhanced_senior_ded_amt: Usd,
    /// Line 36b: If you are married filing jointly, your spouse has a valid social security number
    /// (see instructions), and your spouse was born before January 2, 1961, enter the amount from
    /// line 35
    pub spouse_enhanced_senior_ded_amt: Usd,
    /// Line 37: Enhanced deduction for seniors. Add lines 36a and 36b
    pub enhanced_senior_deduction_amt: Usd,

    // -----------------------------------------------------------------------
    // Part VI — Total Additional Deductions
    // -----------------------------------------------------------------------
    /// Line 38: Add lines 13, 21, 30, and 37. Enter here and on Form 1040 or 1040-SR, line 13b,
    /// or on Form 1040-NR, line 13c
    pub total_additional_deductions_amt: Usd,
    /// Total exclusions and deductions added back
    pub total_exclusions_deduction_amt: Usd,
}
