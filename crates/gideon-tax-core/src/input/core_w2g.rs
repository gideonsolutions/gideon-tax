use crate::Usd;

use super::StateLocalTax;

/// Compute-relevant fields from IRS Form W-2G (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, payer metadata,
/// and gambling event descriptions/dates.
#[derive(Debug, Clone, Default)]
pub struct CoreW2G {
    /// Box 1: reportable winnings
    pub gambling_reportable_winning_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 7: winnings from identical wagers
    pub gambling_win_from_idntcl_wagers_amt: Usd,
    /// Boxes 14–17: state/local winnings and tax withheld
    pub state_local_tax: Vec<StateLocalTax>,
}
