use crate::Usd;

/// Compute-relevant fields from IRS Form 1099-PATR (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, payer metadata,
/// and CUSIP numbers.
#[derive(Debug, Clone, Default)]
pub struct Core1099Patr {
    /// Box 1: patronage dividends
    pub patronage_dividends_amt: Usd,
    /// Box 2: nonpatronage distributions
    pub nonpatronage_distributions_amt: Usd,
    /// Box 3: per-unit retain allocations
    pub per_unit_retain_allocations_amt: Usd,
    /// Box 4: federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: redeemed nonqualified notices
    pub redeemed_nonqualified_notices_amt: Usd,
    /// Box 6: section 199A(g) deduction
    pub section_199ag_deduction_amt: Usd,
    /// Box 7: qualified payments (section 199A(b)(7))
    pub section_199ab7_qualified_pymt_amt: Usd,
    /// Box 8: section 199A(a) qualified items
    pub section_199aa_qualified_items_amt: Usd,
    /// Box 9: section 199A(a) SSTB items
    pub section_199aa_sstb_items_amt: Usd,
    /// Box 10: investment credit
    pub investment_credit_amt: Usd,
    /// Box 11: work opportunity credit
    pub work_opportunity_credit_amt: Usd,
    /// Box 12: other credits and deductions — EPA sulfur regulations
    pub epa_sulfur_reg_deduction_amt: Usd,
    /// Box 12: other credits and deductions — Form 8844 credit
    pub form_8844_credit_amt: Usd,
    /// Box 12: other credits and deductions — Form 8864 credit
    pub form_8864_credit_amt: Usd,
    /// Box 12: other credits and deductions — Form 8896 credit
    pub form_8896_credit_amt: Usd,
    /// Box 12: other credits and deductions — Form 8932 credit
    pub form_8932_credit_amt: Usd,
    /// Box 12: other credits and deductions — Form 8941 credit
    pub form_8941_credit_amt: Usd,
    /// Box 13: specified cooperative
    pub specified_cooperative_ind: bool,
}
