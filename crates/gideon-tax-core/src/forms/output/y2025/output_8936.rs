use crate::Usd;

/// Output fields for IRS Form 8936 (2025) — Clean Vehicle Credits.
#[derive(Debug, Clone, Default)]
pub struct Output8936 {
    // -----------------------------------------------------------------------
    // Part I — Modified Adjusted Gross Income (MAGI) Amount
    // -----------------------------------------------------------------------
    /// Line 1a: Amount from line 11a of your 2025 Form 1040, 1040-SR, or 1040-NR
    pub adjusted_gross_income_amt: Usd,
    /// Line 1b: Any income from Puerto Rico you excluded
    pub excld_sect933_puerto_rico_incm_amt: Usd,
    /// Line 1c: Any amount from Form 2555, line 45
    pub gross_income_exclusion_amt: Usd,
    /// Line 1d: Any amount from Form 2555, line 50
    pub housing_deduction_amt: Usd,
    /// Line 1e: Any amount from Form 4563, line 15
    pub total_income_exclusion_amt: Usd,
    /// Line 2: Add lines 1a through 1e (current year MAGI)
    pub net_income_amt: Usd,
    /// Line 5: Prior year filing status code (from 2024 return)
    pub py_indiv_return_filing_status_cd: String,

    // -----------------------------------------------------------------------
    // Part II — Credit for Business/Investment Use Part of New Clean Vehicles
    // -----------------------------------------------------------------------
    /// Line 6: Total credit amount from Part II of Schedule(s) A (Form 8936)
    pub business_investment_use_amt: Usd,
    /// Line 7: New clean vehicle credit from partnerships and S corporations
    pub new_clean_veh_cr_prtshp_s_corp_amt: Usd,
    /// Line 8: Business/investment use part of credit (add lines 6 and 7)
    pub business_invst_use_part_of_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Credit for Personal Use Part of New Clean Vehicles
    // -----------------------------------------------------------------------
    /// Line 9: Total credit amount from Part III of Schedule(s) A (Form 8936)
    pub prsnl_use_new_clean_vehicle_cr_amt: Usd,
    /// Line 10: Amount from Form 1040, 1040-SR, or 1040-NR, line 18
    pub total_tax_before_cr_and_oth_taxes_amt: Usd,
    /// Line 11: Personal credits from Form 1040, 1040-SR, or 1040-NR
    pub personal_tax_credits_amt: Usd,
    /// Line 12: Subtract line 11 from line 10 (if zero or less, enter -0-)
    pub adjusted_personal_tax_credits_amt: Usd,
    /// Line 13: Personal use part of credit (smaller of line 9 or line 12)
    pub clean_veh_prsnl_use_part_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Credit for Previously Owned Clean Vehicles
    // -----------------------------------------------------------------------
    /// Line 14: Total credit amount from Part IV of Schedule(s) A (Form 8936)
    pub max_prev_owned_clean_veh_cr_amt: Usd,
    /// Line 18: Previously owned clean vehicle credit (smaller of line 14 or line 17)
    pub prev_owned_clean_veh_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Credit for Qualified Commercial Clean Vehicles
    // -----------------------------------------------------------------------
    /// Line 19: Total credit amount from Part V of Schedule(s) A (Form 8936)
    pub qlfy_cmrcl_clean_vehicle_cr_amt: Usd,
    /// Line 20: Qualified commercial clean vehicle credit from partnerships and S corporations
    pub cmrcl_clean_veh_cr_prtshp_s_corp_amt: Usd,
    /// Line 21: Add lines 19 and 20 (total qualified commercial clean vehicle credit)
    pub total_qlfy_cmrcl_clean_veh_cr_amt: Usd,
}
