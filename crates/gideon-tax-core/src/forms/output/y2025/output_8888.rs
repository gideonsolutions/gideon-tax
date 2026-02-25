use crate::Usd;

/// Output fields for IRS Form 8888 (2025) — Allocation of Refund.
#[derive(Debug, Clone, Default)]
pub struct Output8888 {
    // -----------------------------------------------------------------------
    // Direct Deposit
    // -----------------------------------------------------------------------
    /// Line 1a: Amount to be deposited in first account
    pub direct_deposit_refund_amt: Usd,
    /// Line 1b: Routing number for first account
    pub routing_transit_num: String,
    /// Line 1c: Account type (Checking or Savings) for first account
    pub bank_account_type_cd: String,
    /// Line 1d: Account number for first account
    pub depositor_account_num: String,

    // -----------------------------------------------------------------------
    // Total Allocation of Refund
    // -----------------------------------------------------------------------
    /// Line 5: Total allocation of refund (add lines 1a, 2a, and 3a)
    pub total_allocation_of_refund_amt: Usd,
}
