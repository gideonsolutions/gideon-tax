use crate::Usd;

/// Output fields for IRS Form 8689 (2025) — Allocation of Individual Income Tax to the U.S. Virgin Islands.
#[derive(Debug, Clone, Default)]
pub struct Output8689 {
    // -----------------------------------------------------------------------
    // Part I — Income From the U.S. Virgin Islands (USVI)
    // -----------------------------------------------------------------------
    /// Line 1: Wages, salaries, tips, etc.
    pub wages_salaries_and_tips_amt: Usd,
    /// Line 2: Taxable interest
    pub taxable_interest_amt: Usd,
    /// Line 3: Ordinary dividends
    pub ordinary_dividends_amt: Usd,
    /// Line 4: Taxable refunds, credits, or offsets of local USVI income taxes
    pub usvi_tax_ref_credits_offset_amt: Usd,
    /// Line 5: Alimony received
    pub alimony_received_amt: Usd,
    /// Line 6: Business income or (loss)
    pub business_income_loss_amt: Usd,
    /// Line 7: Capital gain or (loss)
    pub capital_gain_loss_amt: Usd,
    /// Line 8: Other gains or (losses)
    pub other_gain_loss_amt: Usd,
    /// Line 9: IRA distributions (taxable amount)
    pub ira_distributions_amt: Usd,
    /// Line 10: Pensions and annuities (taxable amount)
    pub txbl_pension_and_annuities_amt: Usd,
    /// Line 11: Rental real estate, royalties, partnerships, S corporations, trusts, etc.
    pub rntl_rylts_prtshp_s_corp_tr_etc_amt: Usd,
    /// Line 12: Farm income or (loss)
    pub farm_income_or_loss_amt: Usd,
    /// Line 13: Unemployment compensation
    pub unemployment_comp_amt: Usd,
    /// Line 14: Social security benefits (taxable amount)
    pub taxable_soc_sec_amt: Usd,
    /// Line 15: Other income (list type and amount)
    pub total_other_usvi_income_amt: Usd,
    /// Line 16: Total USVI income (add lines 1 through 15)
    pub total_usvi_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Adjusted Gross Income From the USVI
    // -----------------------------------------------------------------------
    /// Line 17: Educator expenses
    pub educator_expenses_amt: Usd,
    /// Line 18: Certain business expenses of reservists, performing artists, and fee-basis government officials
    pub bus_expns_reservists_and_others_amt: Usd,
    /// Line 19: Health savings account deduction
    pub health_savings_account_ded_amt: Usd,
    /// Line 20: Moving expenses for members of the armed forces
    pub moving_expense_amt: Usd,
    /// Line 21: Deductible part of self-employment tax
    pub one_half_self_employment_tax_amt: Usd,
    /// Line 22: Self-employed SEP, SIMPLE, and qualified plans
    pub self_empld_sep_simple_qlfy_plans_amt: Usd,
    /// Line 23: Self-employed health insurance deduction
    pub self_empld_health_ins_ded_amt: Usd,
    /// Line 24: Penalty on early withdrawal of savings
    pub pnlty_on_erly_wthdrw_of_savings_amt: Usd,
    /// Line 25: IRA deduction
    pub ira_deduction_amt: Usd,
    /// Line 26: Student loan interest deduction
    pub student_loan_interest_ded_amt: Usd,
    /// Line 29: Total deductions attributable to USVI income (add lines 17 through 28)
    pub total_ded_attrbl_to_usvi_income_amt: Usd,
    /// Line 30: USVI adjusted gross income (subtract line 29 from line 16)
    pub virgin_islands_agi_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — Allocation of Tax to the USVI
    // -----------------------------------------------------------------------
    /// Line 31: Total tax from tax return
    pub form1040_total_tax_amt: Usd,
    /// Line 32: Total of certain amounts from tax return (see instructions)
    pub virgin_islands_tax_adjustment_amt: Usd,
    /// Line 33: Subtract line 32 from line 31 (adjusted tax)
    pub adjusted_tax_amt: Usd,
    /// Line 34: Adjusted gross income from tax return
    pub form1040_adjusted_gross_income_amt: Usd,
    /// Line 35: Divide line 30 by line 34 (USVI AGI / Form 1040 AGI ratio)
    pub usviagi_divided_by1040_agi_pct: String,
    /// Line 36: Tax allocated to the USVI (multiply line 33 by line 35)
    pub virgin_islands_tax_allocated_amt: Usd,

    // -----------------------------------------------------------------------
    // Part IV — Payments of Income Tax to the USVI
    // -----------------------------------------------------------------------
    /// Line 37: Income tax withheld by the USVI
    pub usvi_withholding_tax_amt: Usd,
    /// Line 38: 2025 estimated tax payments and amount applied from 2024 return
    pub est_tax_and_credit_elect_pymt_amt: Usd,
    /// Line 39: Amount paid with Form 4868 (extension request)
    pub pymt_made_with_extension_amt: Usd,
    /// Line 40: Total payments to the USVI (add lines 37 through 39)
    pub total_payments_amt: Usd,
    /// Line 41: Smaller of line 36 or line 40 (enter on Schedule 3, "Other refundable credits")
    pub smaller_alloc_tax_or_tot_payment_amt: Usd,
    /// Line 42: Overpayment to the USVI (line 40 minus line 36, if line 40 is more)
    pub overpaid_to_usvi_amt: Usd,
    /// Line 43: Amount of line 42 you want refunded to you
    pub refund_amt: Usd,
    /// Line 44: Amount of line 42 you want applied to your 2026 estimated tax
    pub applied_to_es_tax_amt: Usd,
    /// Line 45: Amount you owe to the USVI (line 36 minus line 40, if line 36 is more)
    pub amount_owed_to_usvi_amt: Usd,
    /// Line 46: Amount from line 45 to pay when you file (enter on Schedule 3, "Other refundable credits")
    pub usvi_payment_amt: Usd,

    // -----------------------------------------------------------------------
    // Additional
    // -----------------------------------------------------------------------
    /// Child name control
    pub child_name_control: String,
}
