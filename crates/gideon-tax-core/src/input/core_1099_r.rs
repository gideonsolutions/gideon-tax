use crate::Usd;

use super::StateLocalTax;

/// Compute-relevant fields from IRS Form 1099-R (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, payer metadata,
/// distribution codes (enum), and percentage fields (decimal).
#[derive(Debug, Clone, Default)]
pub struct Core1099R {
    /// Box 1: gross distribution
    pub gross_distribution_amt: Usd,
    /// Box 2a: taxable amount
    pub taxable_amt: Usd,
    /// Box 2b: taxable amount not determined
    pub txbl_amount_not_determined_ind: bool,
    /// Box 2b: total distribution
    pub total_distribution_ind: bool,
    /// Box 3: capital gain (included in box 2a)
    pub capital_gain_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: employee contributions/Designated Roth contributions or insurance premiums
    pub employee_contributions_amt: Usd,
    /// Box 6: net unrealized appreciation in employer's securities
    pub net_unrlzd_securities_apprcn_amt: Usd,
    /// Box 7: IRA/SEP/SIMPLE
    pub ira_sep_simple_ind: bool,
    /// Box 8: other
    pub other_distribution_amt: Usd,
    /// Box 9b: total employee contributions
    pub total_employee_contributions_amt: Usd,
    /// Box 10: amount allocable to IRR within 5 years
    pub irr_allocated_amt: Usd,
    /// Boxes 14, 16–17 & 19: state/local tax withheld and distributions
    pub state_local_tax: Vec<StateLocalTax>,
}
