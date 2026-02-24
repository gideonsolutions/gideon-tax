use crate::Usd;

/// Compute-relevant fields from IRS Form 1099-K (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, payer/PSE metadata,
/// monthly breakdowns, and merchant category codes.
#[derive(Debug, Clone, Default)]
pub struct Core1099K {
    /// Box 1a: gross amount of payment card/third party network transactions
    pub gross_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 8: state tax withheld
    pub state_tax: Vec<Usd>,
}
