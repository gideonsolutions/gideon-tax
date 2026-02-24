use crate::Usd;

use super::StateTax;

/// Compute-relevant fields from IRS Form 1099-MISC (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, and payer metadata.
#[derive(Debug, Clone, Default)]
pub struct Core1099Misc {
    /// Box 1: rents
    pub rent_amt: Usd,
    /// Box 2: royalties
    pub royalty_amt: Usd,
    /// Box 3: other income
    pub other_income_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: fishing boat proceeds
    pub fishing_boat_proceeds_amt: Usd,
    /// Box 6: medical and health care payments
    pub medical_health_care_payments_amt: Usd,
    /// Box 8: substitute payments in lieu of dividends or interest
    pub substitute_payments_amt: Usd,
    /// Box 9: crop insurance proceeds
    pub crop_insurance_proceeds_amt: Usd,
    /// Box 10: gross proceeds paid to an attorney
    pub attorney_gross_proceeds_paid_amt: Usd,
    /// Box 11: fish purchased for resale
    pub fish_purchased_for_resale_amt: Usd,
    /// Box 12: section 409A deferrals
    pub section_409a_deferrals_amt: Usd,
    /// Box 15: nonqualified deferred compensation
    pub nonqlfy_deferred_compensation_amt: Usd,
    /// Boxes 16 & 18: state tax withheld and state income
    pub state_tax: Vec<StateTax>,
}
