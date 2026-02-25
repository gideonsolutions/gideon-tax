use crate::Usd;

/// Output fields for IRS Form 8880 (2025) — Credit for Qualified Retirement Savings Contributions.
#[derive(Debug, Clone, Default)]
pub struct Output8880 {
    /// Line 1: Traditional and Roth IRA contributions, and ABLE account contributions (a) You
    pub primary_roth_ira_for_current_yr_amt: Usd,
    /// Line 1: Traditional and Roth IRA contributions, and ABLE account contributions (b) Your spouse
    pub spouse_roth_ira_for_current_yr_amt: Usd,
    /// Line 2: Elective deferrals to a 401(k) or other qualified employer plan (a) You
    pub primary_contributions_amt: Usd,
    /// Line 2: Elective deferrals to a 401(k) or other qualified employer plan (b) Your spouse
    pub spouse_contributions_amt: Usd,
    /// Line 3: Add lines 1 and 2 (a) You
    pub add_prim_roth_ira_to_cy_contri_amt: Usd,
    /// Line 3: Add lines 1 and 2 (b) Your spouse
    pub add_sp_roth_ira_to_cy_contri_amt: Usd,
    /// Line 4: Certain distributions received after 2022 and before the due date (a) You
    pub prim_taxable_distributions_amt: Usd,
    /// Line 4: Certain distributions received after 2022 and before the due date (b) Your spouse
    pub sps_taxable_distributions_amt: Usd,
    /// Line 5: Subtract line 4 from line 3. If zero or less, enter -0- (a) You
    pub calculate_prim_distrib_from_tot_amt: Usd,
    /// Line 5: Subtract line 4 from line 3. If zero or less, enter -0- (b) Your spouse
    pub calculate_sps_distrib_from_tot_amt: Usd,
    /// Line 6: In each column, enter the smaller of line 5 or $2,000 (a) You
    pub prim_smaller_of_calculation_amt: Usd,
    /// Line 6: In each column, enter the smaller of line 5 or $2,000 (b) Your spouse
    pub sps_smaller_of_calculation_amt: Usd,
    /// Line 7: Add the amounts on line 6
    pub total_calculated_amt: Usd,
    /// Line 8: Enter the amount from Form 1040, 1040-SR, or 1040-NR, line 11a
    pub tax_return_agi_amt: Usd,
    /// Line 9: Enter the applicable decimal amount from the table
    pub qlfy_retirement_sav_decimal_amt: Usd,
    /// Line 10: Multiply line 7 by line 9
    pub calculated_amt_by_decimal_amt: Usd,
    /// Line 11: Limitation based on tax liability from the Credit Limit Worksheet
    pub calculated_credits_from_tax_amt: Usd,
    /// Line 12: Credit for qualified retirement savings contributions. Enter the smaller of line 10 or line 11
    pub cr_qualified_retirement_sav_amt: Usd,
}
