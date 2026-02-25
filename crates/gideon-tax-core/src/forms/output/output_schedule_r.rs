use crate::Usd;

/// Output fields for IRS Schedule R (Form 1040) — Credit for the Elderly or
/// the Disabled (2025).
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleR {
    // -----------------------------------------------------------------------
    // Part I — Check the Box for Your Filing Status and Age
    // -----------------------------------------------------------------------
    /// Box 1: You were 65 or older
    pub primary_65_or_older_ind: bool,
    /// Box 2: You were under 65 and you retired on permanent and total
    /// disability
    pub und_65_rtd_permnnt_tot_dsblty_ind: bool,
    /// Box 3: Both spouses were 65 or older
    pub both_spouses_65_or_older_ind: bool,
    /// Box 4: Both spouses were under 65, but only one spouse retired on
    /// permanent and total disability
    pub both_under_65_one_rtd_dsblty_ind: bool,
    /// Box 5: Both spouses were under 65, and both retired on permanent and
    /// total disability
    pub both_under_65_both_rtd_dsblty_ind: bool,
    /// Box 6: One spouse was 65 or older, and the other spouse was under 65 and
    /// retired on permanent and total disability
    pub one_65_or_older_other_rtd_dsblty_ind: bool,
    /// Box 7: One spouse was 65 or older, and the other spouse was under 65 and
    /// not retired on permanent and total disability
    pub one_65_or_older_other_not_rtd_ind: bool,
    /// Box 8: You were 65 or older and you lived apart from your spouse for all
    /// of 2025
    pub age_65_or_oldr_not_lvng_together_ind: bool,
    /// Box 9: You were under 65, you retired on permanent and total disability,
    /// and you lived apart from your spouse for all of 2025
    pub under_65_did_not_live_together_ind: bool,

    // -----------------------------------------------------------------------
    // Part II — Statement of Permanent and Total Disability
    // -----------------------------------------------------------------------
    /// Prior year statement indicator
    pub prior_year_statement_ind: bool,
    /// Person first name (for prior year statement)
    pub prior_year_person_first_nm: String,
    /// Spouse name (for prior year statement)
    pub prior_year_spouse_nm: String,

    // -----------------------------------------------------------------------
    // Part III — Figure Your Credit
    // -----------------------------------------------------------------------
    /// Line 10: Initial amount based on filing status and age
    pub filing_status_amt: Usd,
    /// Line 11: Taxable disability income
    pub taxable_disability_amt: Usd,
    /// Line 12: If you completed line 11, enter the smaller of line 10 or
    /// line 11. All others, enter the amount from line 10
    pub smaller_of_fs_or_taxable_amt: Usd,
    /// Line 13a: Nontaxable part of social security benefits and nontaxable
    /// part of railroad retirement benefits treated as social security (see
    /// instructions)
    pub nontx_soc_sec_and_rlrd_benefits_amt: Usd,
    /// Line 13b: Nontaxable veterans' pensions and any other pension, annuity,
    /// or disability benefit that is excluded from income under any other
    /// provision of law (see instructions)
    pub nontaxable_other_amt: Usd,
    /// Line 13c: Add lines 13a and 13b
    pub total_nontaxable_amt: Usd,
    /// Line 14: Enter the amount from Form 1040 or 1040-SR, line 11a
    pub tax_return_agi_amt: Usd,
    /// Line 15: Amount based on filing status
    pub adjusted_gross_income_amt: Usd,
    /// Line 16: Subtract line 15 from line 14. If zero or less, enter -0-
    pub exemption_amt: Usd,
    /// Line 17: Enter one-half of line 16
    pub half_agi_amt: Usd,
    /// Line 18: Add lines 13c and 17
    pub adjusted_credit_amt: Usd,
    /// Line 19: Subtract line 18 from line 12. If zero or less, stop; you
    /// can't take the credit. Otherwise, go to line 20
    pub net_credit_amt: Usd,
    /// Line 20: Multiply line 19 by 15% (0.15)
    pub calculated_amount_of_net_credit_amt: Usd,
    /// Line 21: Tax liability limit. Enter the amount from the Credit Limit
    /// Worksheet in the instructions
    pub total_tax_less_credits_amt: Usd,
    /// Line 22: Credit for the elderly or the disabled. Enter the smaller of
    /// line 20 or line 21. Also enter this amount on Schedule 3 (Form 1040),
    /// line 6d
    pub credit_for_elderly_or_disabled_amt: Usd,
}
