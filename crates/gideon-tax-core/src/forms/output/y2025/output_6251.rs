use crate::Usd;

/// Output fields for IRS Form 6251 (2025) — Alternative Minimum Tax — Individuals.
#[derive(Debug, Clone, Default)]
pub struct Output6251 {
    // -----------------------------------------------------------------------
    // Part I — Alternative Minimum Taxable Income
    // -----------------------------------------------------------------------
    /// Line 1a: Subtract Schedule 1-A (Form 1040), line 37, from Form 1040, 1040-SR, or
    /// 1040-NR, line 14
    pub agi_less_tot_ded_less_enhnc_sr_ded_amt: Usd,
    /// Line 1b: Subtract line 1a from Form 1040, 1040-SR, or 1040-NR, line 11b (if less than
    /// zero, enter as a negative amount)
    pub tot_ded_less_enhnc_sr_ded_amt: Usd,
    /// Line 2a: If filing Schedule A (Form 1040), enter the taxes from Schedule A, line 7;
    /// otherwise, enter the amount from Form 1040 or 1040-SR, line 12e
    pub schedule_a_taxes_amt: Usd,
    /// Line 2b: Tax refund from Schedule 1 (Form 1040), line 1 or line 8z
    pub total_refund_received_amt: Usd,
    /// Line 2c: Investment interest expense (difference between regular tax and AMT)
    pub investment_interest_amt: Usd,
    /// Line 2d: Depletion (difference between regular tax and AMT)
    pub depletion_amt: Usd,
    /// Line 2e: Net operating loss deduction from Schedule 1 (Form 1040), line 8a. Enter as a positive amount
    pub net_operating_loss_deduction_amt: Usd,
    /// Line 2f: Alternative tax net operating loss deduction
    pub alt_tax_net_operating_loss_ded_amt: Usd,
    /// Line 2g: Interest from specified private activity bonds exempt from the regular tax
    pub exempt_private_activity_bonds_amt: Usd,
    /// Line 2h: Qualified small business stock, see instructions
    pub section1202_exclusion_amt: Usd,
    /// Line 2i: Exercise of incentive stock options (excess of AMT income over regular tax income)
    pub incentive_stock_options_amt: Usd,
    /// Line 2j: Estates and trusts (amount from Schedule K-1 (Form 1041), box 12, code A)
    pub estates_and_trusts_amt: Usd,
    /// Line 2k: Disposition of property (difference between AMT and regular tax gain or loss)
    pub property_disposition_amt: Usd,
    /// Line 2l: Depreciation on assets placed in service after 1986 (difference between regular tax and AMT)
    pub depreciation_amt: Usd,
    /// Line 2m: Passive activities (difference between AMT and regular tax income or loss)
    pub passive_activity_amt: Usd,
    /// Line 2n: Loss limitations (difference between AMT and regular tax income or loss)
    pub loss_limitation_amt: Usd,
    /// Line 2o: Circulation costs (difference between regular tax and AMT)
    pub circulation_cost_amt: Usd,
    /// Line 2p: Long-term contracts (difference between regular tax and AMT)
    pub long_term_contract_amt: Usd,
    /// Line 2q: Mining costs (difference between regular tax and AMT)
    pub mining_costs_amt: Usd,
    /// Line 2r: Research and experimental costs (difference between regular tax and AMT)
    pub research_experimental_cost_amt: Usd,
    /// Line 2s: Income from certain installment sales before January 1, 1987
    pub installment_sale_income_amt: Usd,
    /// Line 2s: Residual interest in REMIC code
    pub residual_interest_in_remic_cd: String,
    /// Line 2t: Intangible drilling costs preference
    pub intangible_drilling_cost_amt: Usd,
    /// Line 3: Other adjustments, including income-based related adjustments
    pub related_adjustment_amt: Usd,
    /// Line 4: Alternative minimum taxable income. Combine lines 1b through 3. (If married filing
    /// separately and line 4 is more than $900,350, see instructions.)
    pub alternative_min_taxable_income_amt: Usd,
    /// RPI special processing code
    pub r_pi_special_processing_cd: String,

    // -----------------------------------------------------------------------
    // Part II — Alternative Minimum Tax (AMT)
    // -----------------------------------------------------------------------
    /// Line 5: Exemption. Based on filing status and line 4 amount
    pub alternative_minimum_tax_exempt_amt: Usd,
    /// Line 5: Filing status limit amount
    pub filing_status_limit_amt: Usd,
    /// Line 5: Filing threshold amount
    pub filing_threshold_amt: Usd,
    /// Line 5: Income above threshold worksheet amount
    pub income_above_threshold_worksht_amt: Usd,
    /// Line 5: Filing status AMT less income above threshold amount
    pub fs_amt_less_inc_above_threshold_amt: Usd,
    /// Line 5: Filing threshold less threshold sum amount
    pub flng_thrshld_less_theshold_sum_amt: Usd,
    /// Line 6: Subtract line 5 from line 4. If more than zero, go to line 7. If zero or less,
    /// enter -0- here and on lines 7, 9, and 11, and go to line 10
    pub reported_alt_min_taxable_inc_amt: Usd,
    /// Line 7: Tax on AMT taxable income (see instructions for line 7 computation, including
    /// Part III if applicable)
    pub tax_on_alt_min_taxable_inc_amt: Usd,
    /// Line 8: Alternative minimum tax foreign tax credit (see instructions)
    pub amt_foreign_tax_credit_amt: Usd,
    /// Line 9: Tentative minimum tax. Subtract line 8 from line 7
    pub tentative_alternative_min_tax_amt: Usd,
    /// Line 10: Add Form 1040 or 1040-SR, line 16 (minus any tax from Form 4972), and Schedule 2
    /// (Form 1040), line 1z. Subtract from the result Schedule 3 (Form 1040), line 1 and any
    /// negative amount reported on Form 8978, line 14 (treated as a positive number). If zero or
    /// less, enter -0-
    pub adjusted_regular_tax_amt: Usd,
    /// Line 11: AMT. Subtract line 10 from line 9. If zero or less, enter -0-. Enter here and on
    /// Schedule 2 (Form 1040), line 2
    pub alternative_minimum_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Tax Computation Using Maximum Capital Gains Rates
    // -----------------------------------------------------------------------
    /// Line 12: Enter the amount from Form 6251, line 6
    pub adj_alternative_min_taxable_inc_amt: Usd,
    /// Line 13: Enter the amount from line 4 of the Qualified Dividends and Capital Gain Tax
    /// Worksheet or from line 13 of the Schedule D Tax Worksheet
    pub capital_gains_worksheet_amt: Usd,
    /// Line 14: Enter the amount from Schedule D (Form 1040), line 19 (as refigured for the AMT,
    /// if necessary)
    pub unrecaptured_section1250_gain_amt: Usd,
    /// Line 15: If you did not complete a Schedule D Tax Worksheet, enter the amount from line 13.
    /// Otherwise, add lines 13 and 14, and enter the smaller of that result or the amount from
    /// line 10 of the Schedule D Tax Worksheet
    pub smllr_of_adjusted_alt_min_or_sch_d_amt: Usd,
    /// Line 16: Enter the smaller of line 12 or line 15
    pub smllr_adj_net_gain_or_txbl_inc_amt: Usd,
    /// Line 17: Subtract line 16 from line 12
    pub adj_alt_min_taxable_inc_less_gain_amt: Usd,
    /// Line 18: If line 17 is $239,100 or less ($119,550 or less if married filing separately),
    /// multiply line 17 by 26% (0.26). Otherwise, multiply line 17 by 28% (0.28) and subtract
    /// $4,782 ($2,391 if married filing separately) from the result
    pub net_adj_alt_min_txbl_inc_times_pct_amt: Usd,
    /// Line 19: Enter the applicable amount based on filing status
    pub sum_threshold_applcbl_wrksht_amt: Usd,
    /// Line 20: Enter the amount from line 5 of the Qualified Dividends and Capital Gain Tax
    /// Worksheet or from line 14 of the Schedule D Tax Worksheet
    pub applcbl_cap_gains_or_sch_d_wrksht_amt: Usd,
    /// Line 21: Subtract line 20 from line 19. If zero or less, enter -0-
    pub excess_of_sum_amt: Usd,
    /// Line 22: Enter the smaller of line 12 or line 13
    pub smllr_net_adj_alt_min_or_net_gain_amt: Usd,
    /// Line 23: Enter the smaller of line 21 or line 22. This amount is taxed at 0%
    pub net_smaller_sch_d_or_adj_net_gain_amt: Usd,
    /// Line 24: Subtract line 23 from line 22
    pub excess_of_sum_times_pct_amt: Usd,
    /// Line 25: Enter the applicable amount based on filing status
    pub sum_of_smllr_amt: Usd,
    /// Line 26: Enter the amount from line 21
    pub smllr_abv_thrshld_or_alt_min_gain_amt: Usd,
    /// Line 27: Enter the amount from line 5 of the Qualified Dividends and Capital Gain Tax
    /// Worksheet or from line 21 of the Schedule D Tax Worksheet
    pub sum_plus_unrecaptured_sect1250_amt: Usd,
    /// Line 28: Add line 26 and line 27
    pub total_net_amt: Usd,
    /// Line 29: Subtract line 28 from line 25. If zero or less, enter -0-
    pub smaller_of_alt_min_txbl_inc_or_sum_amt: Usd,
    /// Line 30: Enter the smaller of line 24 or line 29
    pub net_alt_min_txbl_inc_times_fs_pct_amt: Usd,
    /// Line 31: Multiply line 30 by 15% (0.15)
    pub net_alt_min_taxable_inc_times_pct_amt: Usd,
    /// Line 32: Add lines 23 and 30
    pub sum_of_alt_min_tax_percentages_amt: Usd,
    /// Line 33: Subtract line 32 from line 22
    pub net_sch_d_or_adj_net_gain_times_pct_amt: Usd,
    /// Line 34: Multiply line 33 by 20% (0.20)
    pub tax_on_alternative_minimum_gain_amt: Usd,
}
