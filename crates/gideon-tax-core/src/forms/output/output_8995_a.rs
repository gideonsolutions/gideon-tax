use crate::Usd;

/// Output fields for IRS Form 8995-A (2025) — Qualified Business Income Deduction.
#[derive(Debug, Clone, Default)]
pub struct Output8995A {
    // -----------------------------------------------------------------------
    // Part I — Trade, Business, or Aggregation Information
    // -----------------------------------------------------------------------
    /// Part I: Filing status threshold code
    pub filing_status_threshold_cd: String,
    /// Part I: Filing status phase-in range code
    pub filing_status_phase_in_range_cd: String,

    // -----------------------------------------------------------------------
    // Part II — Determine Your Adjusted Qualified Business Income
    // -----------------------------------------------------------------------
    /// Line 2: Qualified business income from the trade, business, or aggregation
    pub qualified_business_income_amt: Usd,
    /// Line 3: Multiply line 2 by 20% (0.20). If taxable income is $197,300 or less ($394,600 if MFJ),
    /// skip lines 4 through 12 and enter the amount from line 3 on line 13
    pub qlfy_business_income20_pct_amt: Usd,
    /// Line 4: Allocable share of W-2 wages from the trade, business, or aggregation
    pub allocable_share_w2_wages_amt: Usd,
    /// Line 5: Multiply line 4 by 50% (0.50)
    pub allocable_share_w2_wages50_pct_amt: Usd,
    /// Line 6: Multiply line 4 by 25% (0.25)
    pub allocable_share_w2_wages25_pct_amt: Usd,
    /// Line 7: Allocable share of the unadjusted basis immediately after acquisition (UBIA) of all qualified property
    pub allocable_share_ubia_qlfy_prop_amt: Usd,
    /// Line 8: Multiply line 7 by 2.5% (0.025)
    pub allcbl_shr_ubia_qlfy_prop025_pct_amt: Usd,
    /// Line 9: Add lines 6 and 8
    pub total_allcbl_w2_wgs_qlfy_prop_pct_amt: Usd,
    /// Line 10: Enter the greater of line 5 or line 9
    pub grtr_allcbl_shr_w2_wage_qlfy_prop_amt: Usd,
    /// Line 11: W-2 wage and UBIA of qualified property limitation. Enter the smaller of line 3 or line 10
    pub w2_wage_qlfy_prop_limitation_amt: Usd,
    /// Line 12: Phased-in reduction. Enter the amount from line 26, if any
    pub total_phase_in_reduction_amt: Usd,
    /// Line 13: Qualified business income deduction before patron reduction. Enter the greater of line 11 or line 12
    pub qbi_ded_before_patron_reduction_amt: Usd,
    /// Line 14: Patron reduction. Enter the amount from Schedule D (Form 8995-A), line 6, if any
    pub patron_reduction_amt: Usd,
    /// Line 15: Qualified business income component. Subtract line 14 from line 13
    pub qbi_component_amt: Usd,
    /// Line 16: Total qualified business income component. Add all amounts reported on line 15
    pub total_qbi_component_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Phased-in Reduction
    // -----------------------------------------------------------------------
    /// Line 17: Enter the amounts from line 3
    pub qbi20_pct_less_grtr_allcbl_share_amt: Usd,
    /// Line 18: Enter the amounts from line 10
    pub qbi_after_phase_in_reduction_amt: Usd,
    /// Line 20: Taxable income before qualified business income deduction
    pub taxable_income_before_qbi_ded_amt: Usd,
    /// Line 21: Threshold. Enter $197,300 ($394,600 if married filing jointly)
    pub txi_bfr_qbi_ded_less_threshold_amt: Usd,
    /// Line 24: Phase-in percentage. Divide line 22 by line 23
    pub phase_in_pct: String,

    // -----------------------------------------------------------------------
    // Part IV — Determine Your Qualified Business Income Deduction
    // -----------------------------------------------------------------------
    /// Line 28: Qualified REIT dividends and publicly traded partnership (PTP) income or (loss)
    pub qlfy_reit_div_ptp_income_loss_amt: Usd,
    /// Line 29: Qualified REIT dividends and PTP (loss) carryforward from prior years
    pub py_qlfy_reit_div_ptp_loss_cfwd_amt: Usd,
    /// Line 30: Total qualified REIT dividends and PTP income. Combine lines 28 and 29. If less than zero, enter -0-
    pub tot_qlfy_reit_div_ptp_income_amt: Usd,
    /// Line 31: REIT and PTP component. Multiply line 30 by 20% (0.20)
    pub reitptp_component_amt: Usd,
    /// Line 32: Qualified business income deduction before the income limitation. Add lines 27 and 31
    pub qbi_ded_bfr_income_limitation_amt: Usd,
    /// Line 33: Taxable income before qualified business income deduction
    pub adjusted_taxable_income_amt: Usd,
    /// Line 34: Enter your net capital gain, if any, increased by any qualified dividends (see instructions)
    pub net_capital_gain_amt: Usd,
    /// Line 36: Income limitation. Multiply line 35 by 20% (0.20)
    pub income_limitation_amt: Usd,
    /// Line 37: Qualified business income deduction before the DPAD under section 199A(g). Enter the smaller of line 32 or line 36
    pub qbi_ded_before_dpad_sect199_ag_amt: Usd,
    /// Line 38: DPAD under section 199A(g) allocated from an agricultural or horticultural cooperative. Don't enter more than line 33 minus line 37
    pub dpad_sect199_ag_alloc_agric_hort_amt: Usd,
    /// Line 39: Total qualified business income deduction. Add lines 37 and 38
    pub qualified_business_income_ded_amt: Usd,
    /// Line 40: Total qualified REIT dividends and PTP (loss) carryforward. Combine lines 28 and 29. If zero or greater, enter -0-
    pub tot_qlfy_reit_div_ptp_loss_cfwd_amt: Usd,
}
