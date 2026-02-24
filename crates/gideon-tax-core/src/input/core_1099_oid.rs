use crate::Usd;

/// Compute-relevant fields from IRS Form 1099-OID (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, payer metadata,
/// and CUSIP numbers.
#[derive(Debug, Clone, Default)]
pub struct Core1099Oid {
    /// Box 1: original issue discount
    pub original_issue_discount_amt: Usd,
    /// Box 2: other periodic interest
    pub other_periodic_interest_amt: Usd,
    /// Box 3: early withdrawal penalty
    pub early_withdrawal_penalty_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: market discount
    pub market_discount_amt: Usd,
    /// Box 6: acquisition premium
    pub acquisition_premium_amt: Usd,
    /// Box 8: OID on U.S. Treasury obligations
    pub treasury_obligation_oid_amt: Usd,
    /// Box 9: investment expenses
    pub investment_expense_amt: Usd,
    /// Box 10: bond premium
    pub bond_premium_amt: Usd,
    /// Box 11: tax-exempt OID
    pub tax_exempt_oid_amt: Usd,
    /// Box 14: state tax withheld
    pub state_tax: Vec<Usd>,
}
