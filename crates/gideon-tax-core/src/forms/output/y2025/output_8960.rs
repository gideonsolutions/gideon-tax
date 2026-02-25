use crate::Usd;

/// Output fields for IRS Form 8960 (2025) — Net Investment Income Tax — Individuals, Estates, and Trusts.
#[derive(Debug, Clone, Default)]
pub struct Output8960 {
    // -----------------------------------------------------------------------
    // Part I — Investment Income (checkboxes)
    // -----------------------------------------------------------------------
    /// Section 6013(g) election indicator
    pub section6013g_ind: bool,
    /// Section 6013(h) election indicator
    pub section6013h_ind: bool,
    /// Regulations section 1.1411-10(g) election indicator
    pub reg_section1141110g_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Investment Income (lines 1-8)
    // -----------------------------------------------------------------------
    /// Line 1: Taxable interest
    pub taxable_interest_amt: Usd,
    /// Line 2: Ordinary dividends
    pub ordinary_dividends_amt: Usd,
    /// Line 3: Annuities from nonqualified plans
    pub annuites_from_non_qlf_plans_amt: Usd,
    /// Line 4a: Rental real estate, royalties, partnerships, S corporations, trusts, trades or businesses
    pub net_rental_income_or_loss_amt: Usd,
    /// Line 4b: Adjustment for net income or loss derived in the ordinary course of a non-section 1411 trade or business
    pub adj_net_incm_or_loss_non_sect1411_amt: Usd,
    /// Line 4c: Combine lines 4a and 4b
    pub rental_re_and_adj_net_incm_or_loss_amt: Usd,
    /// Line 5a: Net gain or loss from disposition of property
    pub property_dispos_gain_or_loss_amt: Usd,
    /// Line 5b: Net gain or loss from disposition of property not subject to net investment income tax
    pub non_niit_prop_dispos_gain_or_loss_amt: Usd,
    /// Line 5c: Adjustment from disposition of partnership interest or S corporation stock
    pub adj_from_dispos_of_stock_amt: Usd,
    /// Line 5d: Combine lines 5a through 5c
    pub gain_or_loss_from_dispos_amt: Usd,
    /// Line 6: Adjustments to investment income for certain CFCs and PFICs
    pub cfc_and_pfic_invst_incm_changes_amt: Usd,
    /// Line 7: Other modifications to investment income
    pub other_investment_income_or_loss_amt: Usd,
    /// Line 8: Total investment income (combine lines 1, 2, 3, 4c, 5d, 6, and 7)
    pub total_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Investment Expenses Allocable to Investment Income and Modifications
    // -----------------------------------------------------------------------
    /// Line 9a: Investment interest expenses
    pub investment_interest_amt: Usd,
    /// Line 9b: State, local, and foreign income tax
    pub state_local_foreign_income_tax_amt: Usd,
    /// Line 9c: Miscellaneous investment expenses
    pub misc_investment_expense_amt: Usd,
    /// Line 9d: Add lines 9a, 9b, and 9c
    pub investment_expense_amt: Usd,
    /// Line 10: Additional modifications
    pub additional_modification_amt: Usd,
    /// Line 11: Total deductions and modifications (add lines 9d and 10)
    pub total_deduction_modification_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Tax Computation
    // -----------------------------------------------------------------------
    /// Line 12: Net investment income (subtract Part II, line 11 from Part I, line 8)
    pub net_investment_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Tax Computation — Individuals (lines 13-17)
    // -----------------------------------------------------------------------
    /// Line 13: Modified adjusted gross income
    pub modified_agi_amt: Usd,
    /// Line 14: Threshold based on filing status
    pub filing_threshold_amt: Usd,
    /// Line 15: Subtract line 14 from line 13 (if zero or less, enter -0-)
    pub magi_less_threshold_amt: Usd,
    /// Line 16: Smaller of line 12 or line 15
    pub smllr_incm_or_magi_less_thrshld_amt: Usd,
    /// Line 17: Net investment income tax for individuals (multiply line 16 by 3.8%)
    pub indiv_net_invst_income_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Tax Computation — Estates and Trusts (lines 18a-21)
    // -----------------------------------------------------------------------
    /// Line 18a: Net investment income (line 12 above)
    pub adjusted_gross_income_amt: Usd,
    /// Line 18b: Deductions for distributions of net investment income and charitable deductions
    pub income_distribution_deduction_amt: Usd,
    /// Line 18c: Undistributed net investment income (subtract line 18b from line 18a)
    pub undistributed_net_income_amt: Usd,
    /// Line 19a: Adjusted gross income
    pub agi_less_trust_est_highest_tax_amt: Usd,
    /// Line 19b: Highest tax bracket for estates and trusts for the year
    pub trust_est_highest_tax_bracket_amt: Usd,
    /// Line 19c: Subtract line 19b from line 19a (if zero or less, enter -0-)
    /// (AGI less highest tax bracket threshold)
    pub smllr_undistr_incm_agi_less_tax_amt: Usd,
    /// Line 20: Smaller of line 18c or line 19c
    pub est_trust_net_invst_income_tax_amt: Usd,
    // Line 21: Net investment income tax for estates and trusts (multiply line 20 by 3.8%)
    // (not stored separately; use est_trust_net_invst_income_tax_amt for the base)
}
