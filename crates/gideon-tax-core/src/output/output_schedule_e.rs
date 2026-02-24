use crate::Usd;

/// Output fields for IRS Schedule E (Form 1040) 2025 — Supplemental Income and Loss.
///
/// Covers rental real estate, royalties, partnerships, S corporations,
/// estates, trusts, and REMICs.
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleE {
    // -----------------------------------------------------------------------
    // Header questions
    // -----------------------------------------------------------------------
    /// Line A: Did you make any payments in 2025 that would require you to file Form(s) 1099? See instructions
    pub payment_rqr_filing_form_1099_ind: bool,
    /// Line B: If "Yes," did you or will you file required Form(s) 1099?
    pub required_forms_1099_filed_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Income or Loss From Rental Real Estate and Royalties
    // -----------------------------------------------------------------------
    /// Line 3: Rents received
    pub rents_received_amt: Usd,
    /// Line 4: Royalties received
    pub total_royalties_received_amt: Usd,
    /// Line 5: Advertising
    pub advertising_amt: Usd,
    /// Line 6: Auto and travel (see instructions)
    pub auto_and_travel_amt: Usd,
    /// Line 7: Cleaning and maintenance
    pub cleaning_and_maintenance_amt: Usd,
    /// Line 8: Commissions
    pub commissions_amt: Usd,
    /// Line 9: Insurance
    pub insurance_amt: Usd,
    /// Line 10: Legal and other professional fees
    pub legal_and_other_prof_fees_amt: Usd,
    /// Line 11: Management fees
    pub management_fees_amt: Usd,
    /// Line 12: Mortgage interest paid to banks, etc. (see instructions)
    pub mortgage_interest_paid_banks_amt: Usd,
    /// Line 13: Other interest
    pub mortgage_interest_paid_other_amt: Usd,
    /// Line 14: Repairs
    pub repairs_amt: Usd,
    /// Line 15: Supplies
    pub supplies_amt: Usd,
    /// Line 16: Taxes
    pub taxes_amt: Usd,
    /// Line 17: Utilities
    pub utilities_amt: Usd,
    /// Line 18: Depreciation expense or depletion
    pub deprec_expense_or_depletion_amt: Usd,
    /// Line 19: Other (list)
    pub other_expense_amt: Usd,
    /// Line 20: Total expenses. Add lines 5 through 19
    pub total_expenses_amt: Usd,
    /// Line 21: Subtract line 20 from line 3 (rents) and/or 4 (royalties). If
    /// result is a (loss), see instructions to find out if you must file Form 6198
    pub net_rental_income_or_loss_amt: Usd,
    /// Line 22: Deductible rental real estate loss after limitation, if any,
    /// on Form 8582 (see instructions)
    pub ded_rental_real_estate_loss_amt: Usd,
    /// Line 23a: Total of all amounts reported on line 3 for all rental properties
    pub tot_all_payments_all_rental_prop_amt: Usd,
    /// Line 23b: Total of all amounts reported on line 4 for all royalty properties
    pub tot_all_payments_all_rlty_prop_amt: Usd,
    /// Line 23c: Total of all amounts reported on line 12 for all properties
    pub total_mortgage_interest_paid_amt: Usd,
    /// Line 23d: Total of all amounts reported on line 18 for all properties
    pub total_depreciation_amt: Usd,
    /// Line 23e: Total of all amounts reported on line 20 for all properties
    pub total_all_prop_total_expenses_amt: Usd,
    /// Line 24: Income. Add positive amounts shown on line 21. Do not include any losses
    pub income_amt: Usd,
    /// Line 25: Losses. Add royalty losses from line 21 and rental real estate losses from line 22. Enter total losses here
    pub losses_amt: Usd,
    /// Line 26: Total rental real estate and royalty income or (loss). Combine lines 24 and 25. Enter the result
    /// here. If Parts II, III, and IV, and line 40 on page 2 do not apply to you, also enter this amount on
    /// Schedule 1 (Form 1040), line 5. Otherwise, include this amount in the total on line 41 on page 2
    pub total_supp_income_or_loss_amt: Usd,
    /// Property description text
    pub property_desc: String,

    // -----------------------------------------------------------------------
    // Part II — Income or Loss From Partnerships and S Corporations
    // -----------------------------------------------------------------------
    /// Line 27: Are you reporting any loss not allowed in a prior year due to the at-risk or basis limitations,
    /// a prior year unallowed loss from a passive activity (if that loss was not reported on Form 8582),
    /// or unreimbursed partnership expenses?
    pub prior_years_losses_ind: bool,
    /// Line 30: Add columns (h) and (k) of line 29a
    pub total_prtshp_s_corp_income_amt: Usd,
    /// Line 31: Add columns (g), (i), and (j) of line 29b
    pub total_prtshp_s_corp_loss_amt: Usd,
    /// Line 32: Total partnership and S corporation income or (loss). Combine lines 30 and 31
    pub net_prtshp_s_corp_income_or_loss_amt: Usd,
    /// Part II: Total nonpassive income from partnerships and S corporations
    pub bus_total_nonpassive_income_amt: Usd,
    /// Part II: Total nonpassive loss from partnerships and S corporations
    pub total_nonpassive_loss_amt: Usd,
    /// Part II: Total passive income from partnerships and S corporations
    pub total_passive_income_amt: Usd,
    /// Part II: Total passive deduction or loss allowed from partnerships and S corporations
    pub tot_passive_ded_or_loss_allowed_amt: Usd,
    /// Part II: Total section 179 expense deduction from partnerships and S corporations
    pub tot_sect_179_expense_deduction_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Income or Loss From Estates and Trusts
    // -----------------------------------------------------------------------
    /// Line 35: Add columns (d) and (f) of line 34a
    pub total_estate_or_trust_income_amt: Usd,
    /// Line 36: Add columns (c) and (e) of line 34b
    pub total_estate_or_trust_loss_amt: Usd,
    /// Line 37: Total estate and trust income or (loss). Combine lines 35 and 36
    pub tot_estate_and_trust_inc_or_loss_amt: Usd,
    /// Part III: Total passive income from estates and trusts
    pub estate_and_trust_tot_pssv_incm_amt: Usd,
    /// Part III: Total passive loss allowed from estates and trusts
    pub total_passive_loss_allowed_amt: Usd,
    /// Part III: Total deduction or loss from estates and trusts
    pub total_deduction_or_loss_amt: Usd,
    /// Part III: Total other income from estates and trusts
    pub total_other_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Income or Loss From Real Estate Mortgage Investment Conduits (REMICs) — Residual Holder
    // -----------------------------------------------------------------------
    /// Line 39: Combine columns (d) and (e) only. Enter the result here and include in the total on line 41 below
    pub total_remic_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part V — Summary
    // -----------------------------------------------------------------------
    /// Line 40: Net farm rental income or (loss) from Form 4835. Also, complete line 42 below
    pub net_farm_rental_income_or_loss_amt: Usd,
    /// Line 41: Total income or (loss). Combine lines 26, 32, 37, 39, and 40. Enter the result here and on Schedule
    /// 1 (Form 1040), line 5
    pub total_income_or_loss_amt: Usd,
    /// Line 42: Reconciliation of farming and fishing income. Enter your gross
    /// farming and fishing income reported on Form 4835, line 7; Schedule K-1
    /// (Form 1065), box 14, code B; Schedule K-1 (Form 1120-S), box 17, code
    /// AN; and Schedule K-1 (Form 1041), box 14, code F. See instructions
    pub farming_and_fishing_income_amt: Usd,
    /// Line 43: Reconciliation for real estate professionals. If you were a real estate
    /// professional (see instructions), enter the net income or (loss) you
    /// reported anywhere on Form 1040, Form 1040-SR, or Form 1040-NR
    /// from all rental real estate activities in which you materially participated
    /// under the passive activity loss rules
    pub recncl_for_re_professionals_amt: Usd,
}
