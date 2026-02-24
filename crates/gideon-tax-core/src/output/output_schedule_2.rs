use crate::Usd;

/// Output fields for IRS Schedule 2 (Form 1040) 2025 — Additional Taxes.
#[derive(Debug, Clone, Default)]
pub struct OutputSchedule2 {
    // -----------------------------------------------------------------------
    // Part I — Tax
    // -----------------------------------------------------------------------
    /// Line 1a: Excess advance premium tax credit repayment. Attach Form 8962
    pub premium_tax_credit_tax_liab_amt: Usd,
    /// Line 1b: Repayment of new clean vehicle credit(s) transferred to a registered dealer
    /// from Schedule A (Form 8936), Part II. Attach Form 8936 and Schedule A (Form 8936)
    pub cr_trnsfr_dlr_sale_amt: Usd,
    /// Line 1c: Repayment of previously owned clean vehicle credit(s) transferred to a
    /// registered dealer from Schedule A (Form 8936), Part IV. Attach Form 8936 and
    /// Schedule A (Form 8936)
    pub prev_own_cr_trnsfr_dlr_sale_amt: Usd,
    /// Line 1d: Recapture of net EPE from Form 4255, line 2a, column (l)
    pub rcptr_prtn_net_epe_cr_amt: Usd,
    /// Line 1e: Excessive payments (EPs) on gross EPE from Form 4255.
    /// Check applicable box and enter amount. See instructions.
    pub excessive_pymt_from_4255_ex_pymt_100_cr_amt: Usd,
    /// Line 1e checkbox (i): Line 1a
    pub excessive_pymt_from_4255_applicable_checkbox_i_ind: bool,
    /// Line 1e checkbox (ii): Line 1c
    pub excessive_pymt_from_4255_applicable_checkbox_ii_ind: bool,
    /// Line 1e checkbox (iii): Line 1d
    pub excessive_pymt_from_4255_applicable_checkbox_iii_ind: bool,
    /// Line 1e checkbox (iv): Line 2a
    pub excessive_pymt_from_4255_applicable_checkbox_iv_ind: bool,
    /// Line 1f: 20% EP from Form 4255. Check applicable box and enter amount. See instructions.
    pub increase_chapter1_tax_from_4255_tot_ex20_prvl_wg_aprntcshp_pnlty_amt: Usd,
    /// Line 1f checkbox (i): Line 1a
    pub increase_chapter1_tax_from_4255_applicable_checkbox_i_ind: bool,
    /// Line 1f checkbox (ii): Line 1c
    pub increase_chapter1_tax_from_4255_applicable_checkbox_ii_ind: bool,
    /// Line 1f checkbox (iii): Line 1d
    pub increase_chapter1_tax_from_4255_applicable_checkbox_iii_ind: bool,
    /// Line 1f checkbox (iv): Line 2a
    pub increase_chapter1_tax_from_4255_applicable_checkbox_iv_ind: bool,
    /// Line 1y: Other additions to tax (see instructions)
    pub total_other_tax_additions_amt: Usd,
    /// Line 1z: Add lines 1a through 1y
    pub total_tax_additions_amt: Usd,
    /// Line 2: Alternative minimum tax. Attach Form 6251
    pub alternative_minimum_tax_amt: Usd,
    /// Line 3: Add lines 1z and 2. Enter here and on Form 1040, 1040-SR, or 1040-NR, line 17
    pub additional_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Other Taxes
    // -----------------------------------------------------------------------
    /// Line 4: Self-employment tax. Attach Schedule SE
    pub self_employment_tax_amt: Usd,
    /// Line 4 checkbox 1: Form 4361
    pub exempt_form_4361_ind: bool,
    /// Line 4 checkbox 2: Form 4029
    pub exempt_form_4029_ind: bool,
    /// Line 4 checkbox 3: Exempt SE tax literal
    pub exempt_se_tax_literal_ind: bool,
    /// Line 4 checkbox 3: Exempt SE tax literal code
    pub exempt_se_tax_literal_cd: String,
    /// Line 5: Social security and Medicare tax on unreported tip income. Attach Form 4137
    pub soc_sec_medicare_tax_unrptd_tip_amt: Usd,
    /// Line 6: Uncollected social security and Medicare tax on wages. Attach Form 8919
    pub uncollected_soc_sec_med_tax_amt: Usd,
    /// Line 7: Total additional social security and Medicare tax. Add lines 5 and 6
    pub unrprtd_soc_sec_and_medcr_tax_amt: Usd,
    /// Line 8: Additional tax on IRAs or other tax-favored accounts. Attach Form 5329 if required
    pub tax_on_iras_amt: Usd,
    /// Line 8 checkbox: If not required, check here
    pub form_5329_not_required_ind: bool,
    /// Line 9: Household employment taxes. Attach Schedule H
    pub household_employment_tax_amt: Usd,
    /// Line 11: Additional Medicare Tax. Attach Form 8959
    pub additional_medicare_rrt_tax_amt: Usd,
    /// Line 12: Net investment income tax. Attach Form 8960
    pub indiv_net_invst_income_tax_amt: Usd,
    /// Line 13: Uncollected social security and Medicare or RRTA tax on tips or group-term life
    /// insurance from Form W-2, box 12
    pub uncoll_ss_medcr_rrta_grp_ins_tx_amt: Usd,
    /// Line 14: Interest on tax due on installment income from the sale of certain residential
    /// lots and timeshares
    pub int_tax_due_instal_sale_incm_amt: Usd,
    /// Line 15: Interest on the deferred tax on gain from certain installment sales with a sales
    /// price over $150,000
    pub int_defrd_tax_gain_instal_sales_amt: Usd,
    /// Line 16: Recapture of low-income housing credit. Attach Form 8611
    pub recapture_tax_amt: Usd,
    /// Line 17a: Recapture of other credits. List type, form number, and amount
    pub total_recapture_other_credits_amt: Usd,
    /// Line 17a: Recapture of other credits — credit code
    pub recapture_other_credits_cd: String,
    /// Line 17a: Recapture of other credits — credit text
    pub recapture_other_credits_txt: String,
    /// Line 17a: Recapture of other credits — credit amount
    pub recapture_other_credits_amt: Usd,
    /// Line 17b: Recapture of federal mortgage subsidy. If you sold your home, see instructions
    pub mortg_sbsdy_recapture_tax_amt: Usd,
    /// Line 17c: Additional tax on HSA distributions. Attach Form 8889
    pub hsa_distri_addnl_percent_tax_amt: Usd,
    /// Line 17d: Additional tax on an HSA because you didn't remain an eligible individual.
    /// Attach Form 8889
    pub hdhp_coverage_addnl_tax_amt: Usd,
    /// Line 17e: Additional tax on Archer MSA distributions. Attach Form 8853
    pub archer_msa_addnl_distri_tax_amt: Usd,
    /// Line 17f: Additional tax on Medicare Advantage MSA distributions. Attach Form 8853
    pub medicare_msa_addnl_distri_tax_amt: Usd,
    /// Line 17g: Recapture of a charitable contribution deduction related to a fractional interest
    /// in tangible personal property
    pub recapture_chrtbl_contri_ded_amt: Usd,
    /// Line 17h: Income you received from a nonqualified deferred compensation plan that fails
    /// to meet the requirements of section 409A
    pub incm_nonqlfy_defrd_comp_plan_amt: Usd,
    /// Line 17i: Compensation you received from a nonqualified deferred compensation plan
    /// described in section 457A
    pub comp_nonqlfy_defrd_comp_plan_amt: Usd,
    /// Line 17j: Section 72(m)(5) excess benefits tax
    pub sect72m5_excess_benefits_tax_amt: Usd,
    /// Line 17k: Golden parachute payments
    pub excess_parachute_payment_amt: Usd,
    /// Line 17l: Tax on accumulation distribution of trusts
    pub partial_tax_on_accum_distri_amt: Usd,
    /// Line 17m: Excise tax on insider stock compensation from an expatriated corporation
    pub insider_stock_comp_excise_tax_amt: Usd,
    /// Line 17n: Look-back interest under section 167(g) or 460(b) from Form 8697 or 8866
    pub look_back_int_sect167g_or460b_amt: Usd,
    /// Line 17o: Tax on non-effectively connected income for any part of the year you were a
    /// nonresident alien from Form 1040-NR
    pub income_not_us_business_tax_amt: Usd,
    /// Line 17p: Any interest from Form 8621, line 16f, relating to distributions from, and
    /// dispositions of, stock of a section 1291 fund
    pub interest_on_each_net_incr_in_tax_amt: Usd,
    /// Line 17q: Any interest from Form 8621, line 24
    pub accrued_interest_due_this_ret_amt: Usd,
    /// Line 17z: Any other taxes. List type and amount
    pub total_any_other_taxes_amt: Usd,
    /// Line 18: Total additional taxes. Add lines 17a through 17z
    pub total_other_additional_taxes_amt: Usd,
    /// Line 19: Recapture of net EPE from Form 4255, line 1d, column (l)
    pub frm3468_iv_rcptr_prtn_net_epe_cr_amt: Usd,
    /// Line 20: Section 965 net tax liability installment from Form 965-A
    pub section965_tax_installment_amt: Usd,
    /// Line 21: Add lines 4, 7 through 16, 18, and 19. These are your total other taxes. Enter
    /// here and on Form 1040 or 1040-SR, line 23; or Form 1040-NR, line 23b
    pub total_other_taxes_amt: Usd,
}
