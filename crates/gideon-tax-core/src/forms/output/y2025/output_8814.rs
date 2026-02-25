use crate::Usd;

/// Output fields for IRS Form 8814 (2025) — Parents' Election To Report Child's Interest and Dividends.
#[derive(Debug, Clone, Default)]
pub struct Output8814 {
    // -----------------------------------------------------------------------
    // Top-of-form — Child identification
    // -----------------------------------------------------------------------
    /// A: Child's name (first, initial, and last)
    pub child_nm: String,
    /// A: Child's name control text
    pub child_name_control_txt: String,
    /// B: Child's social security number
    pub child_ssn: String,
    /// C: If more than one Form 8814 is attached, check here
    pub multiple_form8814_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Child's Interest and Dividends To Report on Your Return
    // -----------------------------------------------------------------------
    /// Line 1a: Enter your child's taxable interest
    pub child_taxable_interest_amt: Usd,
    /// Line 1b: Enter your child's tax-exempt interest (do not include this amount on line 1a)
    pub child_tax_exempt_interest_amt: Usd,
    /// Line 2a: Enter your child's ordinary dividends (including any Alaska Permanent Fund dividends)
    pub child_ordinary_dividend_amt: Usd,
    /// Line 2a: Nominee distribution amount
    pub nominee_distribution_amt: Usd,
    /// Line 2a: Nominee distribution code
    pub nominee_distribution_cd: String,
    /// Line 2b: Enter your child's qualified dividends included on line 2a
    pub child_qualified_dividend_amt: Usd,
    /// Line 3: Enter your child's capital gain distributions
    pub child_capital_gain_distri_amt: Usd,
    /// Line 4: Add lines 1a, 2a, and 3
    pub child_investment_income_amt: Usd,
    /// Line 5: Base amount ($2,700)
    pub child_interest_and_div_tax_basis_amt: Usd,
    /// Line 5: Check if child's tax basis is under specified amount
    pub child_tax_basis_under_spcfd_amt_ind: bool,
    /// Line 6: Subtract line 5 from line 4
    pub child_net_investment_income_amt: Usd,
    /// Line 7: Divide line 2b by line 4 (decimal, rounded to at least three places)
    pub child_qualified_dividend_pct: String,
    /// Line 7: Qualified dividend adjusted amount (line 6 multiplied by line 7 ratio)
    pub child_qualified_dividend_adj_amt: Usd,
    /// Line 8: Divide line 3 by line 4 (decimal, rounded to at least three places)
    pub child_capital_gain_distri_pct: String,
    /// Line 8: Capital gain distribution adjusted amount (line 6 multiplied by line 8 ratio)
    pub child_capital_gain_distri_adj_amt: Usd,
    /// Line 9: Multiply line 6 by line 7. Enter the result here. See the instructions for where to report this amount on your return
    pub child_net_adjusted_income_amt: Usd,
    /// Line 10: Multiply line 6 by line 8. Enter the result here. See the instructions for where to report this amount on your return
    pub child_tax_basis_adjustment_sum_amt: Usd,
    /// Line 11: Add lines 9 and 10
    pub child_interest_and_dividend_tax_amt: Usd,
}
