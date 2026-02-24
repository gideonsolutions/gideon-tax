use crate::Usd;

/// Compute-relevant fields from IRS Form 1099-B (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, payer metadata,
/// CUSIP numbers, property descriptions, and dates.
#[derive(Debug, Clone, Default)]
pub struct Core1099B {
    /// Box 1d: proceeds
    pub proceeds_amt: Usd,
    /// Box 1e: cost or other basis
    pub cost_or_other_basis_amt: Usd,
    /// Box 1f: accrued market discount
    pub accrued_market_discount_amt: Usd,
    /// Box 1g: wash sale loss disallowed
    pub nondeductible_wash_sale_loss_amt: Usd,
    /// Box 2: short-term gain or loss
    pub short_term_gain_loss_ind: bool,
    /// Box 2: long-term gain or loss
    pub long_term_gain_loss_ind: bool,
    /// Box 2: ordinary
    pub ordinary_ind: bool,
    /// Box 3: collectibles
    pub collectibles_ind: bool,
    /// Box 3: QOF
    pub qof_ind: bool,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: noncovered security
    pub noncovered_security_ind: bool,
    /// Box 6: gross proceeds reported to IRS
    pub gross_proceeds_ind: bool,
    /// Box 6: net proceeds reported to IRS
    pub net_proceeds_ind: bool,
    /// Box 7: loss not allowed based on amount in 1d
    pub loss_not_allowed_ind: bool,
    /// Box 8: profit or (loss) realized on closed contracts
    pub ty_closed_contract_profit_loss_amt: Usd,
    /// Box 9: unrealized profit or (loss) on open contracts—12/31 prior year
    pub prior_yr_open_cntrct_profit_loss_amt: Usd,
    /// Box 10: unrealized profit or (loss) on open contracts—12/31 current year
    pub cy_open_cntrct_profit_loss_amt: Usd,
    /// Box 11: aggregate profit or (loss) on contracts
    pub cntrct_aggregate_profit_loss_amt: Usd,
    /// Box 12: basis reported to IRS
    pub basis_reported_ind: bool,
    /// Box 13: bartering
    pub barter_amt: Usd,
    /// Box 16: state tax withheld
    pub state_tax: Vec<Usd>,
}
