use crate::Usd;

/// Output fields for IRS Form 4952 (2025) — Investment Interest Expense Deduction.
#[derive(Debug, Clone, Default)]
pub struct Output4952 {
    // -----------------------------------------------------------------------
    // Part I — Total Investment Interest Expense
    // -----------------------------------------------------------------------
    /// Line 1: Investment interest expense paid or accrued in 2025 (see instructions)
    pub investment_interest_expense_amt: Usd,
    /// Line 2: Disallowed investment interest expense from 2024 Form 4952, line 7
    pub prior_yr_disallow_invsmt_int_exp_amt: Usd,
    /// Line 3: Total investment interest expense. Add lines 1 and 2
    pub total_investment_interest_exp_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Net Investment Income
    // -----------------------------------------------------------------------
    /// Line 4a: Gross income from property held for investment (excluding any net gain from the disposition of property held for investment)
    pub investment_prop_gross_income_amt: Usd,
    /// Line 4b: Qualified dividends included on line 4a
    pub investment_prop_qual_dividends_amt: Usd,
    /// Line 4c: Subtract line 4b from line 4a
    pub investment_prop_net_gross_inc_amt: Usd,
    /// Line 4d: Net gain from the disposition of property held for investment
    pub investment_prop_net_disp_gain_amt: Usd,
    /// Line 4e: Enter the smaller of line 4d or your net capital gain from the disposition of property held for investment
    pub investment_net_gain_less_small_amt: Usd,
    /// Line 4f: Subtract line 4e from line 4d
    pub property_dspstn_cap_gain_inv_inc_amt: Usd,
    /// Line 4g: Enter the amount from lines 4b and 4e that you elect to include in investment income (see instructions)
    pub investment_prop_gain_elected_amt: Usd,
    /// Line 4g: Election code
    pub investment_prop_gain_elected_cd: String,
    /// Line 4h: Investment income. Add lines 4c, 4f, and 4g
    pub investment_income_amt: Usd,
    /// Line 5: Investment expenses (see instructions)
    pub investment_expense_amt: Usd,
    /// Line 6: Net investment income. Subtract line 5 from line 4h. If zero or less, enter -0-
    pub net_investment_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Investment Interest Expense Deduction
    // -----------------------------------------------------------------------
    /// Line 7: Disallowed investment interest expense to be carried forward to 2026. Subtract line 6 from line 3. If zero or less, enter -0-
    pub disallowed_carry_forward_exp_amt: Usd,
    /// Line 8: Investment interest expense deduction. Enter the smaller of line 3 or line 6
    pub investment_interest_exp_deduct_amt: Usd,
    /// Investment income election amount (amount elected to treat as investment income)
    pub investment_income_election_amt: Usd,
}
