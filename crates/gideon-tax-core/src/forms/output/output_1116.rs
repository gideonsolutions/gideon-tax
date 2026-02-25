use crate::Usd;

/// Output fields for IRS Form 1116 (2025) — Foreign Tax Credit.
#[derive(Debug, Clone, Default)]
pub struct Output1116 {
    // -----------------------------------------------------------------------
    // Top-of-form — Category of Income / Election
    // -----------------------------------------------------------------------
    /// Line 1a(b): Check if alternative basis of compensation used to determine source
    pub alt_basis_compensation_source_ind: bool,
    /// Category checkbox: Alternative minimum tax code
    pub alternative_minimum_tax_cd: String,

    // -----------------------------------------------------------------------
    // Part III — Figuring the Credit (Lines 9–24)
    // -----------------------------------------------------------------------
    /// Line 24: Enter the smaller of line 14 or line 23. Credit limitation amount
    pub credit_limitation_amt: Usd,
    /// Part IV, Line 26: Credit for taxes on foreign branch category income
    pub foreign_branch_income_cr_amt: Usd,
    /// Category checkbox (b): Foreign branch category income
    pub foreign_branch_income_ind: bool,
    /// Part IV, Line 28: Credit for taxes on general category income
    pub foreign_general_inc_tax_credit_amt: Usd,
    /// Part II, Line 8: Total foreign taxes paid or accrued (add lines A–C, column (u))
    pub foreign_gross_tax_paid_or_accr_amt: Usd,
    /// Category checkbox (d): General category income
    pub foreign_inc_general_category_ind: bool,
    /// Line 13: Taxes reclassified under high tax kickout — adjustment amount
    pub foreign_inc_high_tax_kick_out_adj_amt: Usd,
    /// Line 13: Taxes reclassified under high tax kickout — code
    pub foreign_inc_high_taxed_kick_out_cd: String,
    /// Part IV, Line 31: Credit for taxes on lump-sum distributions
    pub foreign_inc_lump_sum_distrib_cr_amt: Usd,
    /// Category checkbox (g): Lump-sum distributions
    pub foreign_inc_lump_sum_distrib_ind: bool,
    /// Category checkbox (c): Passive category income
    pub foreign_inc_passive_category_ind: bool,
    /// Category checkbox (f): Certain income re-sourced by treaty
    pub foreign_inc_resourced_treaty_ind: bool,
    /// Part IV, Line 30: Credit for taxes on certain income re-sourced by treaty
    pub foreign_inc_rsrcd_treaty_tax_cr_amt: Usd,
    /// Category checkbox (e): Section 901(j) income
    pub foreign_inc_section901j_ind: bool,
    /// Part IV, Line 29: Credit for taxes on section 901(j) income
    pub foreign_incm_section901j_cr_amt: Usd,
    /// Part IV, Line 25: Credit for taxes on section 951A category income
    pub foreign_incm_section951_a_cr_amt: Usd,
    /// Category checkbox (a): Section 951A category income
    pub foreign_incm_section951_a_ind: bool,
    /// Part I: Foreign income net adjustment amount
    pub foreign_income_net_adjustment_amt: Usd,
    /// Line 15: Taxable income or (loss) from sources outside the U.S.
    pub foreign_net_taxable_income_amt: Usd,
    /// Part IV, Line 27: Credit for taxes on passive category income
    pub foreign_passive_inc_tax_credit_amt: Usd,
    /// Regulated investment company code
    pub foreign_regulated_investmt_comp_cd: String,
    /// Line 14: Total amount of foreign taxes available for credit (combine lines 11, 12, and 13)
    pub foreign_tax_available_for_cr_red_amt: Usd,
    /// Line 13: High tax kickout adjustment code
    pub foreign_inc_high_tax_kick_out_adj_cd: String,
    /// Line 10: Carryover of foreign taxes (from Schedule B, line 3, column (xiv)) plus any carrybacks
    pub foreign_tax_cr_carryback_or_over_amt: Usd,
    /// Line 35: Foreign tax credit (subtract line 34 from line 33)
    pub foreign_tax_credit_amt: Usd,
    /// Foreign tax credit source description
    pub foreign_tax_credit_source: String,
    /// Line 12: Reduction in foreign taxes
    pub foreign_tax_reduction_amt: Usd,
    /// Line 19: Divide line 17 by line 18. If line 17 is more than line 18, enter "1"
    pub foreign_taxable_inc_bf_exempt_amt: Usd,
    /// Line 19: Rate/ratio
    pub foreign_taxable_inc_bf_exempt_rt: String,
    /// Line 7: Subtract line 6 from line 1a (taxable income or loss from foreign sources)
    pub foreign_taxable_income_or_loss_amt: Usd,
    /// Credit claimed: Taxes accrued indicator (k)
    pub foreign_taxes_accrued_credit_ind: bool,
    /// Credit claimed: Taxes paid indicator (j)
    pub foreign_taxes_paid_credit_ind: bool,
    /// Line 11: Add lines 9 and 10 (gross foreign tax credit)
    pub gross_foreign_tax_credit_amt: Usd,
    /// Category checkbox (g): Lump-sum distributions code
    pub foreign_income_lump_sum_distrib_cd: String,
    /// Schedule B attachment
    pub irs1116_schedule_b: String,
    /// Part I (i): Name of foreign country or U.S. territory (income from foreign source text)
    pub income_from_foreign_source_txt: String,
    /// Income from foreign source type code
    pub income_from_foreign_source_type_cd: String,
    /// Line 22: Increase in limitation (section 960(c))
    pub increase_limitation_sect960c_amt: Usd,
    /// Line 34: Reduction of credit for international boycott operations
    pub intl_boycott_credit_reduction_amt: Usd,
    /// Line 21: Multiply line 20 by line 19 (maximum amount of credit)
    pub max_allowed_foreign_tax_credit_amt: Usd,
    /// Line 17: Net foreign source taxable income (combine lines 15 and 16)
    pub net_foreign_taxable_income_loss_amt: Usd,
    /// Line h: Resident of (name of country)
    pub residence_foreign_country_cd: String,
    /// Schedule B not required indicator
    pub schedule_b_not_required_ind: bool,
    /// Line 33: Enter the smaller of line 20 or line 32
    pub smllr_of_rtn_tax_or_foreign_tax_cr_amt: Usd,
    /// Line 20: Enter the total of Form 1040/1040-SR/1040-NR, line 16, and Schedule 2, line 1z
    pub tax_from_tax_return_amt: Usd,
    /// Line 23: Add lines 21 and 22 (tentative foreign tax credit)
    pub tentative_foreign_tax_credit_amt: Usd,
    /// Line 6: Total deductions or losses (add lines 2, 3g, 4a, 4b, and 5)
    pub total_deduction_or_loss_amt: Usd,
    /// Line 1a: Gross income from sources within the foreign country
    pub total_foreign_gross_income_amt: Usd,
    /// Line 9: Total foreign taxes paid or accrued (from Part II, line 8)
    pub total_foreign_taxes_paid_or_accr_amt: Usd,
}
