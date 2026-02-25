use crate::Usd;

/// Output fields for IRS Form 965-A (2025) — Individual Report of Net 965 Tax Liability.
#[derive(Debug, Clone, Default)]
pub struct Output965A {
    // -----------------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------------
    /// Check this box if this is an amended report
    pub amended_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Report of Net 965 Tax Liability and Election To Pay in Installments
    // -----------------------------------------------------------------------
    /// Part I, Column (d): Net 965 Tax Liability (subtract column (c) from column (b))
    pub net_section965_tax_liab_paid_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Record of Amount of Net 965 Tax Liability Paid by the Taxpayer
    // -----------------------------------------------------------------------
    /// Part II, Column (j): Net 965 Tax Liability Remaining Unpaid (see instructions)
    pub net_section965_tax_liab_unpaid_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — S Corporation Shareholder: Report of Calculation of Net Tax Liability
    //            Related to 965 Amounts Allocated From an S Corporation and
    //            Election To Defer Such Net 965 Tax Liability
    // -----------------------------------------------------------------------
    /// Part III, Column (g): Deferred Net 965 Tax Liability (if column (f) is "Yes," enter amount from column (e))
    pub net_sect965_deferred_tax_liab_amt: Usd,
    /// Part I, Column (e): S Corporation Shareholder Total Deferred Net 965 Tax Liability
    /// (line total from Part III, column (g), see instructions)
    pub tot_s_corp_defrd_net965_tax_liab_amt: Usd,
}
