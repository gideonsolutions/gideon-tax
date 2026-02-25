use crate::Usd;

/// Output fields for IRS Form 3800 (2025) — General Business Credit.
#[derive(Debug, Clone, Default)]
pub struct Output3800 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Line A: Corporate Alternative Minimum Tax (CAMT) and Base Erosion Anti-Abuse Tax
    /// (BEAT) indicator — Are you both an "applicable corporation" and an "applicable
    /// taxpayer"?
    pub camt_and_beat_ind: bool,
    /// Line B(ii): Number of transfer election statements attached to your return
    pub transfer_election_statement_cnt: u32,
    /// Credit transfer election indicator (line B(i))
    pub credit_transfer_election_ind: bool,
    /// Section 383 or 384 indicator code
    pub section383_or384_indicator_cd: String,

    // -----------------------------------------------------------------------
    // Part I — Credits Not Allowed Against Tentative Minimum Tax (TMT)
    // -----------------------------------------------------------------------
    /// Line 1: Credits not subject to the passive activity limit from Part III, line 2,
    /// column (e) with non-passive amounts from column (f)
    pub general_bus_cr_from_nn_pssv_acty_amt: Usd,
    /// Line 2: Credits subject to the passive activity limit (Part III, line 2, column (d),
    /// and passive amounts in line 2, column (f); and Part IV, line 6, column (d))
    pub cr_subj_to_passive_acty_lmt_amt: Usd,
    /// Line 3: Enter the portion of line 2 allowed for 2025
    pub passive_acty_allowed_for_ty_amt: Usd,
    /// Line 4: Enter the portion of Part IV, line 6, column (f), that is from carryforwards
    /// to 2025
    pub cy_general_bus_cr_carryforward_amt: Usd,
    /// Line 4 checkbox: Check this box if the carryforward was changed or revised from the
    /// original reported amount
    pub carryforward_chgd_or_revs_ind: bool,
    /// Line 5: Enter the portion of Part IV, line 6, column (f), that is from carrybacks
    /// from 2026
    pub carry_back_general_business_cr_amt: Usd,
    /// Line 6: Add lines 1, 3, 4, and 5
    pub cy_credits_not_allw_against_tmt_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Figuring Credit Allowed After Limitations
    // Section A — Figuring Credit Allowed After Section 38(c)(1)
    //              Limitation Based on Amount of Tax
    // -----------------------------------------------------------------------
    /// Line 7: Regular tax before credits
    pub regular_tax_before_credits_amt: Usd,
    /// Line 8: Alternative minimum tax
    pub alternative_minimum_tax_amt: Usd,
    /// Line 9: Add lines 7 and 8
    pub adjusted_reg_tax_before_credit_amt: Usd,
    /// Line 10a: Foreign tax credit
    pub foreign_tax_credit_amt: Usd,
    /// Line 10b: Certain allowable credits (see instructions)
    pub certain_allowable_credits_amt: Usd,
    /// Line 10c: Add lines 10a and 10b
    pub total_tax_credits_amt: Usd,
    /// Line 11: Net income tax. Subtract line 10c from line 9. If zero, skip lines 12
    /// through 15 and enter -0- on line 16
    pub net_income_tax_amt: Usd,
    /// Line 12: Net regular tax. Subtract line 10c from line 7. If zero or less, enter -0-
    pub net_regular_tax_amt: Usd,
    /// Line 13: Enter 25% (0.25) of the excess, if any, of line 12 (line 11 for
    /// corporations) over $25,000
    pub excess_net_regular_tax_amt: Usd,
    /// Line 14: Tentative minimum tax
    pub tentative_minimum_tax_amt: Usd,
    /// Line 15: Enter the greater of line 13 or line 14
    pub greater_excess_or_times_pct_amt: Usd,
    /// Line 16: Subtract line 15 from line 11. If zero or less, enter -0-
    pub net_incm_tax_less_greater_excess_amt: Usd,
    /// Line 17: Enter the smaller of line 6 or line 16. This is the amount of your credit
    /// allowed after the limitation of section 38(c)(1)
    pub smllr_gen_bus_cr_or_tot_gen_elig_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Section B — Figuring Section 38(c)(2) Empowerment Zone and Renewal
    //              Community Employment Credit Allowed
    // -----------------------------------------------------------------------
    /// Line 18: Multiply line 14 by 75% (0.75)
    pub tentative_minimun_tax_times_pct_amt: Usd,
    /// Line 19: Enter the greater of line 13 or line 18
    pub adjusted_excess_net_regular_tax_amt: Usd,
    /// Line 20: Subtract line 19 from line 11. If zero or less, enter -0-
    pub net_income_tax_less_pct_excess_amt: Usd,
    /// Line 21: Subtract line 17 from line 20. If zero or less, enter -0-
    pub sub_smllr_from_net_less_greater_amt: Usd,
    /// Line 22: Combine the amounts from Part III, line 3, column (e), with the amount from
    /// Part IV, line 3, column (f)
    pub empwr_zone_and_com_employment_cr_amt: Usd,
    /// Line 23: Passive activity credit from Part III, line 3, column (d), plus the amount
    /// from Part IV, line 3, column (d)
    pub gbc_from_pssv_acty_all_parts_amt: Usd,
    /// Line 24: Enter the applicable passive activity credits allowed for 2025
    pub pssv_acty_for_gen_bus_cr_allowed_amt: Usd,
    /// Line 25: Add lines 22 and 24
    pub sum_smllr_empwr_zn_emplmn_cr_amt: Usd,
    /// Line 26: Empowerment zone and renewal community employment credit allowed. Enter the
    /// smaller of line 21 or line 25
    pub net_smllr_and_empwr_zn_emplmn_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Section C — Figuring the Specified Credit Amount Allowed Under
    //              Section 38(c)(4)
    // -----------------------------------------------------------------------
    /// Line 27: Subtract line 13 from line 11. If zero or less, enter -0-
    pub adjusted_net_income_tax_amt: Usd,
    /// Line 28: Add lines 17 and 26
    pub tot_empwr_zone_gen_bus_credits_amt: Usd,
    /// Line 29: Subtract line 28 from line 27. If zero or less, enter -0-
    pub current_year_credit_allowed_amt: Usd,
    /// Line 30: Enter the general business credit from line 5 of Part III: combine column
    /// (e) with non-passive amounts in column (f)
    pub allw_gen_bus_cr_from_non_pssv_acty_amt: Usd,
    /// Line 32: Passive activity credits from line 5 of Part III: combine column (d) with
    /// passive amounts in column (f). Also include passive specified credit carryovers from
    /// Part IV, line 5, column (d)
    pub total_passive_activity_credit_amt: Usd,
    /// Line 33: Enter the applicable passive activity credits allowed for 2025
    pub gen_bus_elig_smll_bus_pssv_acty_cr_amt: Usd,
    /// Line 34: Carryforward of business credit to 2025 (from Part IV)
    pub allw_gen_and_elig_smll_bus_cfwd_cr_amt: Usd,
    /// Line 35: Carryback of business credit from 2026 (from Part IV)
    pub allw_gen_and_elig_smll_bus_cybk_cr_amt: Usd,
    /// Line 36: Add lines 30, 33, 34, and 35
    pub tot_allw_gen_and_elig_smll_bus_cr_amt: Usd,
    /// Line 37: Enter the smaller of line 29 or line 36. This is the amount allowed for
    /// specified credits
    pub other_specified_allw_gen_bus_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Section D — Credits Allowed After Limitations
    // -----------------------------------------------------------------------
    /// Line 38: Credit allowed for the current year. Add lines 28 and 37
    pub smllr_cy_not_allw_tmt_or_tot_adj_amt: Usd,
}
