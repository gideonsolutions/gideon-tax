use crate::Usd;

/// Compute-relevant fields from IRS Form 1099-DIV (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, and payer metadata.
#[derive(Debug, Clone, Default)]
pub struct Core1099Div {
    /// Box 1a: total ordinary dividends
    pub total_ordinary_dividends_amt: Usd,
    /// Box 1b: qualified dividends
    pub qualified_dividends_amt: Usd,
    /// Box 2a: total capital gain distributions
    pub total_capital_distributions_amt: Usd,
    /// Box 2b: unrecaptured section 1250 gain
    pub unrecaptured_section_1250_gain_amt: Usd,
    /// Box 2c: section 1202 gain
    pub capital_gain_sect_1202_amt: Usd,
    /// Box 2d: collectibles (28%) gain
    pub collectibles_28_percent_gain_amt: Usd,
    /// Box 2e: section 897 ordinary dividends
    pub section_897_ordinary_dividends_amt: Usd,
    /// Box 2f: section 897 capital gain
    pub section_897_capital_gain_amt: Usd,
    /// Box 3: nondividend distributions
    pub nondividend_distributions_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: section 199A dividends
    pub section_199a_dividends_amt: Usd,
    /// Box 6: investment expenses
    pub investment_expense_amt: Usd,
    /// Box 7: foreign tax paid
    pub foreign_taxes_paid_amt: Usd,
    /// Box 9: cash liquidation distributions
    pub cash_liquidation_distri_amt: Usd,
    /// Box 10: noncash liquidation distributions
    pub noncash_liquidation_distri_amt: Usd,
    /// Box 12: exempt-interest dividends
    pub exempt_interest_dividends_amt: Usd,
    /// Box 13: specified private activity bond interest dividends
    pub private_activity_bond_int_div_amt: Usd,
    /// Box 16: state tax withheld
    pub state_tax: Vec<Usd>,
}
