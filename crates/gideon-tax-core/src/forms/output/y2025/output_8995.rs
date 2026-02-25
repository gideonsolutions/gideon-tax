use crate::Usd;

/// Output fields for IRS Form 8995 (2025) — Qualified Business Income Deduction Simplified Computation.
#[derive(Debug, Clone, Default)]
pub struct Output8995 {
    // -----------------------------------------------------------------------
    // Lines 1-5 — Qualified Business Income
    // -----------------------------------------------------------------------
    /// Line 1 (column c): Qualified business income or (loss) per trade/business (lines i-v)
    pub tot_qlfy_business_income_or_loss_amt: Usd,
    /// Line 2: Total qualified business income or (loss). Combine lines 1i through 1v, column (c)
    pub tot_qualified_business_income_amt: Usd,
    /// Line 3: Qualified business net (loss) carryforward from the prior year
    pub py_qlfy_business_net_loss_cfwd_amt: Usd,
    /// Line 4: Total qualified business income. Combine lines 2 and 3. If zero or less, enter -0-
    pub qbi_component_amt: Usd,
    /// Line 5: Qualified business income component. Multiply line 4 by 20% (0.20)
    pub qbi_ded_bfr_income_limitation_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 6-9 — REIT Dividends and PTP Income
    // -----------------------------------------------------------------------
    /// Line 6: Qualified REIT dividends and publicly traded partnership (PTP) income or (loss)
    pub qlfy_reit_div_ptp_income_loss_amt: Usd,
    /// Line 7: Qualified REIT dividends and qualified PTP (loss) carryforward from the prior year
    pub py_qlfy_reit_div_ptp_loss_cfwd_amt: Usd,
    /// Line 8: Total qualified REIT dividends and PTP income. Combine lines 6 and 7. If zero or less, enter -0-
    pub tot_qlfy_reit_div_ptp_income_amt: Usd,
    /// Line 9: REIT and PTP component. Multiply line 8 by 20% (0.20)
    pub reitptp_component_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 10-15 — Income Limitation and QBI Deduction
    // -----------------------------------------------------------------------
    /// Line 10: Qualified business income deduction before the income limitation. Add lines 5 and 9
    pub qualified_business_income_ded_amt: Usd,
    /// Line 11: Taxable income before qualified business income deduction (see instructions)
    pub taxable_income_before_qbi_ded_amt: Usd,
    /// Line 12: Enter your net capital gain, if any, increased by any qualified dividends (see instructions)
    pub net_capital_gain_amt: Usd,
    /// Line 13: Subtract line 12 from line 11. If zero or less, enter -0-
    pub adjusted_taxable_income_amt: Usd,
    /// Line 14: Income limitation. Multiply line 13 by 20% (0.20)
    pub income_limitation_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 16-17 — Loss Carryforwards
    // -----------------------------------------------------------------------
    /// Line 16: Total qualified business (loss) carryforward. Combine lines 2 and 3. If greater than zero, enter -0-
    pub tot_qlfy_bus_loss_carryforward_amt: Usd,
    /// Line 17: Total qualified REIT dividends and PTP (loss) carryforward. Combine lines 6 and 7. If greater than zero, enter -0-
    pub tot_qlfy_reit_div_ptp_loss_cfwd_amt: Usd,
}
