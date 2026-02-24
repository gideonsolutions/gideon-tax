use crate::Usd;

use super::StateTax;

/// Compute-relevant fields from IRS Form 1099-NEC (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, and payer metadata.
#[derive(Debug, Clone, Default)]
pub struct Core1099Nec {
    /// Box 1: nonemployee compensation
    pub nonemployee_compensation_amt: Usd,
    /// Box 3: excess golden parachute payments
    pub excess_parachute_payment_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Boxes 5 & 7: state tax withheld and state income
    pub state_tax: Vec<StateTax>,
}
