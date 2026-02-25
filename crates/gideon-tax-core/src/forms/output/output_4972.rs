use crate::Usd;

/// Output fields for IRS Form 4972 (2025) — Tax on Lump-Sum Distributions.
#[derive(Debug, Clone, Default)]
pub struct Output4972 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name of recipient of distribution
    pub person_nm: String,
    /// Identifying number (SSN)
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — Complete this part to see if you can use Form 4972
    // -----------------------------------------------------------------------
    /// Line 1: Was this a distribution of a plan participant's entire balance? If "No," don't use this form
    pub distribution_of_qualified_plan_ind: bool,
    /// Line 2: Did you roll over any part of the distribution? If "Yes," don't use this form
    pub rollover_ind: bool,
    /// Line 3: Was this distribution paid to you as a beneficiary of a plan participant who was born before January 2, 1936?
    pub beneficiary_distribution_ind: bool,
    /// Line 4: Were you (a) a plan participant who received this distribution, (b) born before January 2, 1936, and (c) a participant in the plan for at least 5 years?
    pub employee_beneficiary_distri_ind: bool,
    /// Line 5a: Did you use Form 4972 after 1986 for a previous distribution from your own plan?
    pub prior_year_distribution_ind: bool,
    /// Line 5b: If you are receiving this distribution as a beneficiary of a plan participant who died, did you use Form 4972 for a previous distribution received as a beneficiary of that participant after 1986?
    pub qualifying_age5_year_member_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Complete this part to choose the 20% capital gain elections
    // -----------------------------------------------------------------------
    /// Line 6: Capital gain part from Form 1099-R, box 3
    pub capital_gain_election_amt: Usd,
    /// Line 7: Multiply line 6 by 20% (0.20)
    pub capital_gain_times_election_pct_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Complete this part to choose the 10-year tax option
    // -----------------------------------------------------------------------
    /// Line 8: If you completed Part II, enter the amount from Form 1099-R, box 2a, minus box 3. If you didn't complete Part II, enter the amount from box 2a. Multiple recipients — include NUA in taxable income
    pub lump_sum_distri_ordinary_incm_amt: Usd,
    /// Line 9: Death benefit exclusion for a beneficiary of a plan participant who died before August 21, 1996
    pub lump_sum_distri_death_bnft_excl_amt: Usd,
    /// Line 10: Total taxable amount. Subtract line 9 from line 8
    pub lump_sum_distri_total_taxable_amt: Usd,
    /// Line 11: Current actuarial value of annuity from Form 1099-R, box 8. If none, enter -0-
    pub annuity_actuarial_value_amt: Usd,
    /// Line 12: Adjusted total taxable amount. Add lines 10 and 11. If $70,000 or more, skip lines 13 through 16, enter this amount on line 17, and go to line 18
    pub lump_sum_distri_adj_tot_taxable_amt: Usd,
    /// Line 13: Multiply line 12 by 50% (0.50), but don't enter more than $10,000
    pub lump_sum_distri50_pct_total_txbl_amt: Usd,
    /// Line 14: Subtract $20,000 from line 12. If line 12 is $20,000 or less, enter -0-
    pub lump_sum_distri_net_taxable_amt: Usd,
    /// Line 15: Multiply line 14 by 20% (0.20)
    pub lump_sum_distri_pct_adj_txbl_amt: Usd,
    /// Line 16: Minimum distribution allowance. Subtract line 15 from line 13
    pub lump_sum_min_distri_allowance_amt: Usd,
    /// Line 17: Subtract line 16 from line 12
    pub lump_sum_distri_allowable_txbl_amt: Usd,
    /// Line 18: Federal estate tax attributable to lump-sum distribution
    pub lump_distrib_federal_estate_tax_amt: Usd,
    /// Line 19: Subtract line 18 from line 17. If line 11 is zero, skip lines 20 through 22 and go to line 23
    pub lump_sum_distri_txbl_adj_actrl_amt: Usd,
    /// Line 20: Divide line 11 by line 12 and enter the result as a decimal (rounded to at least three places)
    pub lump_sum_distri_actuarial_adj_pct: String,
    /// Line 21: Multiply line 16 by the decimal on line 20
    pub lump_sum_distri_min_allw_percent_amt: Usd,
    /// Line 22: Subtract line 21 from line 11
    pub adjusted_actuarial_amt: Usd,
    /// Line 23: Multiply line 19 by 10% (0.10)
    pub lump_sum_distri_prorated_txbl_amt: Usd,
    /// Line 24: Tax on amount on line 23. Use the Tax Rate Schedule in the instructions
    pub lump_sum_distri_tax_on_percent_amt: Usd,
    /// Line 25: Multiply line 24 by 10.0. If line 11 is zero, skip lines 26 through 28, enter this amount on line 29, and go to line 30
    pub lump_sum_distri_tent_avg_tax_amt: Usd,
    /// Line 26: Multiply line 22 by 10% (0.10)
    pub lump_sum_distri_adj_actuarial_amt: Usd,
    /// Line 27: Tax on amount on line 26. Use the Tax Rate Schedule in the instructions
    pub lump_sum_rsdl_annuity_avg_tax_amt: Usd,
    /// Line 28: Multiply line 27 by 10.0
    pub lump_sum_distri_adj_average_tax_amt: Usd,
    /// Line 29: Subtract line 28 from line 25. Multiple recipients, see instructions
    pub lump_sum_net_adj_total_taxable_amt: Usd,
    /// Line 29: Multiple recipients code
    pub lump_sum_distri_mult_recipients_cd: String,
    /// Line 30: Tax on lump-sum distribution. Add lines 7 and 29. Also, include this amount in the total on Form 1040, 1040-SR, or 1040-NR, line 16
    pub lump_sum_distribution_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Additional fields
    // -----------------------------------------------------------------------
    /// Net unrealized appreciation amount (Form 1099-R, box 6)
    pub net_unrealized_appreciation_amt: Usd,
    /// Net unrealized appreciation code
    pub net_unrealized_appreciation_cd: String,
    /// Capital gain election NUA amount
    pub capital_gain_election_nua_amt: Usd,
    /// Capital gain election NUA code
    pub capital_gain_election_nua_cd: String,
    /// Waived literal regular method code
    pub waived_literal_regular_method_cd: String,
    /// Waived regular method amount
    pub waived_regular_method_amt: Usd,
    /// Waived short method amount
    pub waived_short_method_amt: Usd,
}
