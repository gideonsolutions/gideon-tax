use crate::Usd;

/// Output fields for IRS Schedule 1 (Form 1040) 2025 — Additional Income and Adjustments to Income.
#[derive(Debug, Clone, Default)]
pub struct OutputSchedule1 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// For 2025, enter the amount reported to you on Form(s) 1099-K that was included in error or
    /// for personal items sold at a loss
    pub form_1099k_rpt_error_or_loss_amt: Usd,

    // -----------------------------------------------------------------------
    // Part I — Additional Income
    // -----------------------------------------------------------------------
    /// Line 1: Taxable refunds, credits, or offsets of state and local income taxes
    pub state_local_income_tax_refund_amt: Usd,
    /// Line 2a: Alimony received
    pub total_alimony_received_amt: Usd,
    /// Line 3: Business income or (loss). Attach Schedule C
    pub business_income_loss_amt: Usd,
    /// Line 4: Other gains or (losses). Check if any from Form(s): 4797
    pub form_4797_ind: bool,
    /// Line 4: Other gains or (losses). Check if any from Form(s): 4684
    pub form_4684_ind: bool,
    /// Line 4: Other gains or (losses)
    pub other_gain_loss_amt: Usd,
    /// Line 5: Rental real estate, royalties, partnerships, S corporations, trusts, etc. Attach Schedule E
    pub rental_real_estate_income_loss_amt: Usd,
    /// Line 6: Farm income or (loss). Attach Schedule F
    pub net_farm_profit_loss_amt: Usd,
    /// Line 7: Unemployment compensation
    pub unemployment_comp_amt: Usd,
    /// Line 7: If you repaid a 2025 overpayment, check here
    pub repaid_overpayment_ind: bool,
    /// Line 7: Enter amount repaid
    pub repayment_amt: Usd,
    /// Line 8a: Net operating loss
    pub net_operating_loss_deduction_amt: Usd,
    /// Line 8b: Gambling
    pub gambling_reportable_winning_amt: Usd,
    /// Line 8c: Cancellation of debt
    pub debt_cancellation_amt: Usd,
    /// Line 8d: Foreign earned income exclusion from Form 2555
    pub total_income_exclusion_amt: Usd,
    /// Line 8e: Income from Form 8853
    pub tot_archer_msa_medcr_ltc_amt: Usd,
    /// Line 8f: Income from Form 8889
    pub tot_hsa_distri_hdhp_amt: Usd,
    /// Line 8g: Alaska Permanent Fund dividends
    pub alaska_permanent_fund_div_amt: Usd,
    /// Line 8h: Jury duty pay
    pub jury_duty_pay_amt: Usd,
    /// Line 8i: Prizes and awards
    pub prizes_awards_amt: Usd,
    /// Line 8j: Activity not engaged in for profit income
    pub activity_not_for_profit_incm_amt: Usd,
    /// Line 8k: Stock options
    pub stock_options_amt: Usd,
    /// Line 8l: Income from the rental of personal property if you engaged in the rental for
    /// profit but were not in the business of renting such property
    pub rental_income_personal_prop_amt: Usd,
    /// Line 8m: Olympic and Paralympic medals and USOC prize money (see instructions)
    pub olympic_paralympic_medal_usoc_amt: Usd,
    /// Line 8n: Section 951(a) inclusion (see instructions)
    pub section_951a_inclusion_amt: Usd,
    /// Line 8o: Section 951A(a) inclusion (see instructions)
    pub section_951_aa_inclusion_amt: Usd,
    /// Line 8p: Section 461(l) excess business loss adjustment
    pub excess_business_loss_amt: Usd,
    /// Line 8q: Taxable distributions from an ABLE account (see instructions)
    pub taxable_able_distributions_amt: Usd,
    /// Line 8r: Scholarship and fellowship grants not reported on Form W-2
    pub grants_or_scholarships_amt: Usd,
    /// Line 8s: Nontaxable amount of Medicaid waiver payments included on Form 1040, line
    /// 1a or 1d
    pub nontx_medicaid_waiver_pymt_amt: Usd,
    /// Line 8t: Pension or annuity from a nonqualified deferred compensation plan or a
    /// nongovernmental section 457 plan
    pub nonqlfy_deferred_compensation_amt: Usd,
    /// Line 8u: Wages earned while incarcerated
    pub certain_penal_instn_wages_amt: Usd,
    /// Line 8v: Digital assets received as ordinary income not reported elsewhere. See
    /// instructions
    pub digital_assets_amt: Usd,
    /// Line 8z: Other income. List type and amount
    pub total_other_income_amt: Usd,
    /// Line 9: Total other income. Add lines 8a through 8z
    pub other_income_total_amt: Usd,
    /// Line 10: Combine lines 1 through 7 and 9. This is your additional income. Enter here and on
    /// Form 1040, 1040-SR, or 1040-NR, line 8
    pub total_additional_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Adjustments to Income
    // -----------------------------------------------------------------------
    /// Line 11: Educator expenses
    pub educator_expenses_amt: Usd,
    /// Line 12: Certain business expenses of reservists, performing artists, and fee-basis government
    /// officials. Attach Form 2106
    pub bus_expns_reservists_and_others_amt: Usd,
    /// Line 13: Health savings account deduction. Attach Form 8889
    pub health_savings_account_ded_amt: Usd,
    /// Line 14: Moving expenses for members of the Armed Forces. Attach Form 3903
    pub moving_expense_amt: Usd,
    /// Line 14: If claiming only storage fees (see instructions), check here
    pub claim_storage_fees_ind: bool,
    /// Line 15: Deductible part of self-employment tax. Attach Schedule SE
    pub deductible_self_employment_tax_amt: Usd,
    /// Line 16: Self-employed SEP, SIMPLE, and qualified plans
    pub self_empld_sep_simple_qlfy_plans_amt: Usd,
    /// Line 17: Self-employed health insurance deduction
    pub self_empld_health_ins_ded_amt: Usd,
    /// Line 18: Penalty on early withdrawal of savings
    pub pnlty_on_erly_wthdrw_of_savings_amt: Usd,
    /// Line 19a: Alimony paid
    pub total_alimony_paid_amt: Usd,
    /// Line 20: IRA deduction
    pub ira_deduction_amt: Usd,
    /// Line 20: If you are married filing separately and lived apart from your spouse for the
    /// entire year (see instructions), check here
    pub mfs_live_apart_entire_yr_ind: bool,
    /// Line 21: Student loan interest deduction
    pub student_loan_interest_ded_amt: Usd,
    /// Line 23: Archer MSA deduction
    pub archer_msa_deduction_amt: Usd,
    /// Line 24a: Jury duty pay (see instructions)
    pub jury_duty_pay_deduction_amt: Usd,
    /// Line 24b: Deductible expenses related to income reported on line 8l from the rental of
    /// personal property engaged in for profit
    pub rntl_incm_prsnl_prop_expnss_ded_amt: Usd,
    /// Line 24c: Nontaxable amount of the value of Olympic and Paralympic medals and USOC
    /// prize money reported on line 8m
    pub olympc_prlympc_medal_usoc_ded_amt: Usd,
    /// Line 24d: Reforestation amortization and expenses
    pub rfor_amortz_expnss_ded_amt: Usd,
    /// Line 24e: Repayment of supplemental unemployment benefits under the Trade Act of
    /// 1974
    pub repayment_supp_unempl_bnft_ded_amt: Usd,
    /// Line 24f: Contributions to section 501(c)(18)(D) pension plans
    pub sect_501c18d_contri_ded_amt: Usd,
    /// Line 24g: Contributions by certain chaplains to section 403(b) plans
    pub section_403b_contri_ded_amt: Usd,
    /// Line 24h: Attorney fees and court costs for actions involving certain unlawful
    /// discrimination claims (see instructions)
    pub atty_fees_crt_costs_ded_amt: Usd,
    /// Line 24i: Attorney fees and court costs you paid in connection with an award from the
    /// IRS for information you provided that helped the IRS detect tax law violations
    pub atty_fees_crt_costs_pd_ded_amt: Usd,
    /// Line 24j: Housing deduction from Form 2555
    pub housing_deduction_amt: Usd,
    /// Line 24k: Excess deductions of section 67(e) expenses from Schedule K-1 (Form 1041)
    pub section_67e_excess_deduction_amt: Usd,
    /// Line 24z: Other adjustments. List type and amount
    pub other_adjustments_total_amt: Usd,
    /// Line 25: Total other adjustments. Add lines 24a through 24z
    pub total_other_adjustments_amt: Usd,
    /// Line 26: Add lines 11 through 23 and 25. These are your adjustments to income. Enter here and
    /// on Form 1040, 1040-SR, or 1040-NR, line 10
    pub total_adjustments_amt: Usd,
}
