use crate::Usd;

/// Compute-relevant fields from IRS Form 1099-G (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, and payer metadata.
#[derive(Debug, Clone, Default)]
pub struct Core1099G {
    /// Box 1: unemployment compensation
    pub unemployment_comp_amt: Usd,
    /// Box 2: state or local income tax refunds, credits, or offsets
    pub state_lcl_refund_credit_offset_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: RTAA payments
    pub rtaa_payments_amt: Usd,
    /// Box 6: taxable grants
    pub taxable_grants_amt: Usd,
    /// Box 7: agriculture payments
    pub agriculture_payments_amt: Usd,
    /// Box 8: trade or business income
    pub trade_or_business_income_ind: bool,
    /// Box 9: market gain
    pub market_gain_amt: Usd,
    /// Box 11: state tax withheld
    pub state_tax: Vec<Usd>,
}
