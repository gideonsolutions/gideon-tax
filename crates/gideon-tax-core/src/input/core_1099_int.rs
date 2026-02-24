use crate::Usd;

use super::StateTax;

/// Compute-relevant fields from IRS Form 1099-INT (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, payer metadata,
/// and CUSIP numbers.
#[derive(Debug, Clone, Default)]
pub struct Core1099Int {
    /// Box 1: interest income
    pub interest_income_amt: Usd,
    /// Box 2: early withdrawal penalty
    pub early_withdrawal_penalty_amt: Usd,
    /// Box 3: interest on U.S. savings bonds and Treasury obligations
    pub us_savings_bonds_treas_oblig_int_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: investment expenses
    pub investment_expense_amt: Usd,
    /// Box 6: foreign tax paid
    pub foreign_taxes_paid_amt: Usd,
    /// Box 8: tax-exempt interest
    pub tax_exempt_interest_amt: Usd,
    /// Box 9: specified private activity bond interest
    pub spcfd_prvt_acty_bond_interest_amt: Usd,
    /// Box 10: market discount
    pub market_discount_amt: Usd,
    /// Box 11: bond premium
    pub bond_premium_amt: Usd,
    /// Box 12: bond premium on U.S. Treasury obligations
    pub treasury_oblig_bond_premium_amt: Usd,
    /// Box 13: bond premium on tax-exempt bonds
    pub tax_exempt_bond_premium_amt: Usd,
    /// Boxes 15 & 17: state tax withheld and state income
    pub state_tax: Vec<StateTax>,
}
