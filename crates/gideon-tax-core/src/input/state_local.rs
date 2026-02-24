use crate::Usd;

/// State earnings and withholding for a single state entry.
///
/// Used by forms that report state income and state tax withheld
/// but have no local tax boxes (e.g. 1099-INT, 1099-MISC, 1099-NEC).
#[derive(Debug, Clone, Default)]
pub struct StateTax {
    pub state_income_amt: Usd,
    pub state_tax_withheld_amt: Usd,
}

/// State/local earnings and withholding for a single state/locality entry.
///
/// Used by forms that report both state and local income and tax withheld
/// (e.g. W-2, W-2G, 1099-R).
#[derive(Debug, Clone, Default)]
pub struct StateLocalTax {
    pub state_income_amt: Usd,
    pub state_tax_withheld_amt: Usd,
    pub local_income_amt: Usd,
    pub local_tax_withheld_amt: Usd,
}
