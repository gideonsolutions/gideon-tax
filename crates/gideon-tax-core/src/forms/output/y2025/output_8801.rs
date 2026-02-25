use crate::Usd;

/// Output fields for IRS Form 8801 (2025) — Credit for Prior Year Minimum Tax — Individuals, Estates, and Trusts.
#[derive(Debug, Clone, Default)]
pub struct Output8801 {
    // -----------------------------------------------------------------------
    // Part I — Net Minimum Tax on Exclusion Items
    // -----------------------------------------------------------------------
    /// Line 1: Combine lines 1 and 2e of your 2024 Form 6251
    pub net_min_tax_taxable_income_loss_amt: Usd,
    /// Line 2: Enter adjustments and preferences treated as exclusion items
    pub net_min_tax_exclusion_items_amt: Usd,
    /// Line 3: Minimum tax credit net operating loss deduction
    pub min_tax_credit_net_opr_loss_ded_amt: Usd,
    /// Line 4: Combine lines 1, 2, and 3
    pub sum_min_tax_credit_loss_and_ded_amt: Usd,
    /// Line 5: Enter exemption amount ($133,300 MFJ/$85,700 single/$66,650 MFS; estates and trusts $29,900)
    pub min_tax_credit_exemption_amt: Usd,
    /// Line 6: Enter phase-out threshold ($1,218,700 MFJ/$609,350 single/MFS; estates and trusts $99,700)
    pub filing_threshold_amt: Usd,
    /// Line 7: Subtract line 6 from line 4. If zero or less, enter -0-
    pub flng_thrshld_less_theshold_sum_amt: Usd,
    /// Line 8: Multiply line 7 by 25% (0.25)
    pub min_tax_credit_phase_out_amt: Usd,
    /// Line 9: Subtract line 8 from line 5. If zero or less, enter -0-
    pub net_min_tax_cr_minus_phase_out_amt: Usd,
    /// Line 10: Subtract line 9 from line 4. If zero or less, enter -0-
    pub net_min_tax_minus_exemption_amt: Usd,
    /// Line 11: Tax on line 10 amount (see instructions for rate computation)
    pub net_min_tax_times_tax_rate_amt: Usd,
    /// Line 12: Minimum tax foreign tax credit on exclusion items
    pub min_tax_foreign_tax_cr_excl_items_amt: Usd,
    /// Line 13: Tentative minimum tax on exclusion items. Subtract line 12 from line 11
    pub tentative_min_tax_on_excl_items_amt: Usd,
    /// Line 14: Enter the amount from your 2024 Form 6251, line 10, or 2024 Schedule I (Form 1041), line 53
    pub py_min_tax_applicable_rtn_tax_amt: Usd,
    /// Line 15: Net minimum tax on exclusion items. Subtract line 14 from line 13. If zero or less, enter -0-
    pub net_min_tax_on_exclusion_items_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Minimum Tax Credit and Carryforward to 2026
    // -----------------------------------------------------------------------
    /// Line 16: Enter the amount from your 2024 Form 6251, line 11, or 2024 Schedule I (Form 1041), line 54
    pub py_alternative_minimum_tax_amt: Usd,
    /// Line 17: Enter the amount from line 15
    pub net_alternative_minimum_tax_amt: Usd,
    /// Line 18: Subtract line 17 from line 16. If less than zero, enter as a negative amount
    pub tent_min_tax_minus_reg_tax_liab_amt: Usd,
    /// Line 19: 2024 credit carryforward. Enter the amount from your 2024 Form 8801, line 26
    pub amt_prior_year_carryforward_amt: Usd,
    /// Line 20: Enter your 2024 unallowed qualified electric vehicle credit
    pub qlfy_elec_veh_py_unallowed_cr_amt: Usd,
    /// Line 21: Combine lines 18 through 20
    pub amt_carryforward_plus_negative_amt: Usd,
    /// Line 22: Enter your 2025 regular income tax liability minus allowable credits
    pub cy_reg_tax_liabi_minus_allwbl_cr_amt: Usd,
    /// Line 23: Enter the amount from your 2025 Form 6251, line 9, or 2025 Schedule I (Form 1041), line 52
    pub cy_tentative_minimum_tax_amt: Usd,
    /// Line 24: Subtract line 23 from line 22. If zero or less, enter -0-
    pub excess_of_sum_amt: Usd,
    /// Line 25: Minimum tax credit. Enter the smaller of line 21 or line 24
    pub min_amt_cr_amt: Usd,
    /// Line 26: Credit carryforward to 2026. Subtract line 25 from line 21
    pub amt_cr_carryforward_to_next_year_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Tax Computation Using Maximum Capital Gains Rates
    // -----------------------------------------------------------------------
    /// Line 27: Enter the amount from Form 8801, line 10
    pub net_min_tax_less_deductions_amt: Usd,
    /// Line 28: Enter the amount from line 4 of your 2024 Qualified Dividends and Capital Gain Tax Worksheet or Schedule D Tax Worksheet
    pub py_min_tax_applicable_cap_gain_amt: Usd,
    /// Line 29: Enter the amount from line 19 of your 2024 Schedule D (Form 1040), or line 18b, column (2), of the 2024 Schedule D (Form 1041)
    pub py_unrecaptured_s1250_gain_amt: Usd,
    /// Line 30: Add lines 28 and 29, and enter the smaller of that result or the amount from line 10 of your 2024 Schedule D Tax Worksheet
    pub sum_threshold_applcbl_wrksht_amt: Usd,
    /// Line 31: Enter the smaller of line 27 or line 30
    pub smaller_net_amt_or_gain_amt: Usd,
    /// Line 32: Subtract line 31 from line 27
    pub gain_minus_smaller_net_amt: Usd,
    /// Line 33: Tax on line 32 amount (26% or 28% rate computation)
    pub net_adj_amt_txbl_inc_times_pct_amt: Usd,
    /// Line 34: Enter applicable threshold amount based on filing status
    pub max_cap_gains_applicable_limit_amt: Usd,
    /// Line 35: Enter the amount from line 5 of your 2024 Qualified Dividends and Capital Gain Tax Worksheet or line 14 of Schedule D Tax Worksheet
    pub amt_prior_year_applicable_gain_amt: Usd,
    /// Line 36: Subtract line 35 from line 34. If zero or less, enter -0-
    pub max_cap_gain_minus_applcbl_limit_amt: Usd,
    /// Line 37: Enter the smaller of line 27 or line 28
    pub smllr_net_min_tax_or_applcbl_gain_amt: Usd,
    /// Line 38: Enter the smaller of line 36 or line 37
    pub smaller_calculated_net_or_gain_amt: Usd,
    /// Line 39: Subtract line 38 from line 37
    pub net_smaller_sch_d_or_adj_net_gain_amt: Usd,
    /// Line 40: Enter applicable amount based on filing status ($518,900 single, etc.)
    pub applcbl_cap_gains_or_sch_d_wrksht_amt: Usd,
    /// Line 41: Enter the amount from line 36
    pub net_min_tax_less_loss_and_ded_amt: Usd,
    /// Line 42: Enter the amount from line 5 of your 2024 Qualified Dividends and Capital Gain Tax Worksheet or line 21 of Schedule D Tax Worksheet
    pub smaller_py_sch_d_gain_or_wrksht_amt: Usd,
    /// Line 43: Add lines 41 and 42
    pub total_net_amt: Usd,
    /// Line 44: Subtract line 43 from line 40. If zero or less, enter -0-
    pub smllr_adj_net_gain_or_txbl_inc_amt: Usd,
    /// Line 45: Enter the smaller of line 39 or line 44
    pub smaller_gain_or_loss_amt: Usd,
    /// Line 46: Multiply line 45 by 15% (0.15)
    pub net_alt_min_taxable_inc_times_pct_amt: Usd,
    /// Line 47: Subtract line 45 from line 39
    pub excess_of_sum_times_pct_amt: Usd,
    /// Line 48: Multiply line 47 by 20% (0.20)
    pub net_alt_min_txbl_inc_times_fs_pct_amt: Usd,
    /// Line 49: Enter the smaller of line 27 or line 31. If line 31 is blank, enter -0-
    pub sum_of_smllr_amt: Usd,
    /// Line 50: Enter the smaller of line 38 or line 49
    pub net_min_tax_cr_times_decimal_amt: Usd,
    /// Line 51: Subtract line 50 from line 49
    pub net_sch_d_or_adj_net_gain_times_pct_amt: Usd,
    /// Line 52: Multiply line 51 by 25% (0.25)
    pub tax_on_alternative_minimum_gain_amt: Usd,
    /// Line 53: Add lines 33, 46, 48, and 52
    pub sum_of_alt_min_tax_percentages_amt: Usd,
}
