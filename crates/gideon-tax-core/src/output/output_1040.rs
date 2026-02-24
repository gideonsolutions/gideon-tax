use crate::Usd;

/// Compute-relevant output fields for IRS Form 1040 (2025).
///
/// Fields are ordered by line number as they appear on the form.
/// Only includes lines that carry computed dollar amounts, indicators,
/// or counts; excludes names, addresses, SSNs, EINs, dates, and
/// preparer/third-party metadata.
#[derive(Debug, Clone, Default)]
pub struct Output1040 {
    // -----------------------------------------------------------------------
    // Line 1: Wages, salaries, tips, etc.
    // -----------------------------------------------------------------------
    /// Line 1a: Total amount from Form(s) W-2, box 1 (see instructions)
    pub wages_amt: Usd,

    /// Line 1b: Household employee wages not reported on Form(s) W-2
    pub household_employee_wages_amt: Usd,

    /// Line 1c: Tip income not reported on line 1a (see instructions)
    pub tip_income_amt: Usd,

    /// Line 1d: Medicaid waiver payments not reported on Form(s) W-2 (see instructions)
    pub medicaid_waiver_pymt_not_rpt_w2_amt: Usd,

    /// Line 1e: Taxable dependent care benefits from Form 2441, line 26
    pub taxable_benefits_amt: Usd,

    /// Line 1f: Employer-provided adoption benefits from Form 8839, line 31
    pub taxable_benefits_form_8839_amt: Usd,

    /// Line 1g: Wages from Form 8919, line 6
    pub total_wages_with_no_withholding_amt: Usd,

    /// Line 1h: Other earned income (see instructions). Enter type and amount
    pub other_earned_income_amt: Usd,

    /// Line 1i: Nontaxable combat pay election (see instructions)
    pub nontx_combat_pay_election_amt: Usd,

    /// Line 1z: Add lines 1a through 1h
    pub wages_salaries_and_tips_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 2–3: Interest and dividends
    // -----------------------------------------------------------------------
    /// Line 2a: Tax-exempt interest
    pub tax_exempt_interest_amt: Usd,

    /// Line 2b: Taxable interest
    pub taxable_interest_amt: Usd,

    /// Line 3a: Qualified dividends
    pub qualified_dividends_amt: Usd,

    /// Line 3b: Ordinary dividends
    pub ordinary_dividends_amt: Usd,

    /// Line 3c checkbox 1: Check if your child's dividends are included in Line 3a
    pub child_div_incld_qualified_div_ind: bool,

    /// Line 3c checkbox 2: Check if your child's dividends are included in Line 3b
    pub child_div_incld_ordinary_div_ind: bool,

    // -----------------------------------------------------------------------
    // Line 4: IRA distributions
    // -----------------------------------------------------------------------
    /// Line 4a: IRA distributions
    pub ira_distributions_amt: Usd,

    /// Line 4b: Taxable amount
    pub taxable_ira_amt: Usd,

    /// Line 4c checkbox 1: Rollover
    pub ira_distribution_rollover_ind: bool,

    /// Line 4c checkbox 2: QCD
    pub ira_distribution_qcd_ind: bool,

    /// Line 4c checkbox 3 (other, see instructions)
    pub ira_distribution_other_ind: bool,

    // -----------------------------------------------------------------------
    // Line 5: Pensions and annuities
    // -----------------------------------------------------------------------
    /// Line 5a: Pensions and annuities
    pub pensions_annuities_amt: Usd,

    /// Line 5b: Taxable amount
    pub total_taxable_pensions_amt: Usd,

    /// Line 5c checkbox 1: Rollover
    pub pensions_annuities_rollover_ind: bool,

    /// Line 5c checkbox 2: PSO
    pub pensions_annuities_pso_ind: bool,

    /// Line 5c checkbox 3 (other, see instructions)
    pub pensions_annuities_other_ind: bool,

    // -----------------------------------------------------------------------
    // Line 6: Social security benefits
    // -----------------------------------------------------------------------
    /// Line 6a: Social security benefits
    pub soc_sec_bnft_amt: Usd,

    /// Line 6b: Taxable amount
    pub taxable_soc_sec_amt: Usd,

    /// Line 6c: If you elect to use the lump-sum election method, check here (see instructions)
    pub lump_sum_election_method_ind: bool,

    /// Line 6d: If you are married filing separately and lived apart from your spouse the entire year (see inst.), check here
    pub sepd_sps_filing_sep_ret_meets_rqr_ind: bool,

    // -----------------------------------------------------------------------
    // Line 7: Capital gain or (loss)
    // -----------------------------------------------------------------------
    /// Line 7a: Capital gain or (loss). Attach Schedule D if required
    pub capital_gain_loss_amt: Usd,

    /// Line 7b checkbox 1: Schedule D not required
    pub capital_distribution_ind: bool,

    /// Line 7b checkbox 2: Includes child's capital gain or (loss)
    pub child_capital_gain_or_loss_incld_ind: bool,

    // -----------------------------------------------------------------------
    // Lines 8–11: Total income, adjustments, AGI
    // -----------------------------------------------------------------------
    /// Line 8: Additional income from Schedule 1, line 10
    pub total_additional_income_amt: Usd,

    /// Line 9: Add lines 1z, 2b, 3b, 4b, 5b, 6b, 7a, and 8. This is your total income
    pub total_income_amt: Usd,

    /// Line 10: Adjustments to income from Schedule 1, line 26
    pub total_adjustments_amt: Usd,

    /// Line 11a: Subtract line 10 from line 9. This is your adjusted gross income
    pub adjusted_gross_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Page 2 — Tax and Credits
    // -----------------------------------------------------------------------
    /// Line 12a checkbox: Someone can claim: You as a dependent
    pub primary_claim_as_dependent_ind: bool,

    /// Line 12a checkbox: Someone can claim: Your spouse as a dependent
    pub spouse_claim_as_dependent_ind: bool,

    /// Line 12b: Spouse itemizes on a separate return
    pub must_itemize_ind: bool,

    /// Line 12c: You were a dual-status alien
    pub dual_status_alien_ind: bool,

    /// Line 12d: You: Were born before January 2, 1961
    pub primary_65_or_older_ind: bool,

    /// Line 12d: You: Are blind
    pub primary_blind_ind: bool,

    /// Line 12d: Spouse: Was born before January 2, 1961
    pub spouse_65_or_older_ind: bool,

    /// Line 12d: Spouse: Is blind
    pub spouse_blind_ind: bool,

    /// Line 12d: Total boxes checked count
    pub total_boxes_checked_cnt: u8,

    /// Line 12e: Standard deduction or itemized deductions (from Schedule A)
    pub total_itemized_or_standard_ded_amt: Usd,

    /// Line 13a: Qualified business income deduction from Form 8995 or Form 8995-A
    pub qualified_business_income_ded_amt: Usd,

    /// Line 13b: Additional deductions from Schedule 1-A, line 38
    pub total_additional_deductions_amt: Usd,

    /// Line 14: Add lines 12e, 13a, and 13b
    pub total_deductions_amt: Usd,

    /// Line 15: Subtract line 14 from line 11b. If zero or less, enter -0-. This is your taxable income
    pub taxable_income_amt: Usd,

    /// Line 16: Tax (see instructions). Check if any from Form(s): 1 [ ] 8814  2 [ ] 4972  3 [ ]
    pub tax_amt: Usd,

    /// Line 16 checkbox 1: Form 8814
    pub form_8814_ind: bool,

    /// Line 16 checkbox 2: Form 4972
    pub form_4972_ind: bool,

    /// Line 17: Amount from Schedule 2, line 3
    pub additional_tax_amt: Usd,

    /// Line 18: Add lines 16 and 17
    pub total_tax_before_cr_and_oth_taxes_amt: Usd,

    /// Line 19: Child tax credit or credit for other dependents from Schedule 8812
    pub ctc_odc_amt: Usd,

    /// Line 20: Amount from Schedule 3, line 8
    pub total_nonrefundable_credits_amt: Usd,

    /// Line 21: Add lines 19 and 20
    pub total_credits_amt: Usd,

    /// Line 22: Subtract line 21 from line 18. If zero or less, enter -0-
    pub tax_less_credits_amt: Usd,

    /// Line 23: Other taxes, including self-employment tax, from Schedule 2, line 21
    pub total_other_taxes_amt: Usd,

    /// Line 24: Add lines 22 and 23. This is your total tax
    pub total_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 25–33: Payments and Refundable Credits
    // -----------------------------------------------------------------------
    /// Line 25a: Federal income tax withheld from Form(s) W-2
    pub form_w2_withheld_tax_amt: Usd,

    /// Line 25b: Federal income tax withheld from Form(s) 1099
    pub form_1099_withheld_tax_amt: Usd,

    /// Line 25c: Federal income tax withheld from other forms (see instructions)
    pub tax_withheld_other_amt: Usd,

    /// Line 25d: Add lines 25a through 25c
    pub withholding_tax_amt: Usd,

    /// Line 26: 2025 estimated tax payments and amount applied from 2024 return
    pub estimated_tax_payments_amt: Usd,

    /// Line 27a: Earned income credit (EIC)
    pub earned_income_credit_amt: Usd,

    /// Line 27b: Clergy filing Schedule SE (see instructions)
    pub clergy_member_ind: bool,

    /// Line 27c: If you do not want to claim the EIC, check here
    pub do_not_claim_eic_ind: bool,

    /// Line 28: Additional child tax credit (ACTC) from Schedule 8812. If you do not want to claim the ACTC, check here
    pub additional_child_tax_credit_amt: Usd,

    /// Line 28 checkbox: If you do not want to claim the ACTC, check here
    pub do_not_claim_actc_ind: bool,

    /// Line 29: American opportunity credit from Form 8863, line 8
    pub refundable_amer_opp_credit_amt: Usd,

    /// Line 30: Refundable adoption credit from Form 8839, line 13
    pub refundable_adoption_credit_amt: Usd,

    /// Line 31: Amount from Schedule 3, line 15
    pub total_other_payments_rfdbl_cr_amt: Usd,

    /// Line 32: Add lines 27a, 28, 29, 30, and 31. These are your total other payments and refundable credits
    pub refundable_credits_amt: Usd,

    /// Line 33: Add lines 25d, 26, and 32. These are your total payments
    pub total_payments_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 34–36: Refund
    // -----------------------------------------------------------------------
    /// Line 34: If line 33 is more than line 24, subtract line 24 from line 33. This is the amount you overpaid
    pub overpaid_amt: Usd,

    /// Line 35a: Amount of line 34 you want refunded to you. If Form 8888 is attached, check here
    pub refund_amt: Usd,

    /// Line 35a checkbox: If Form 8888 is attached, check here
    pub form_8888_ind: bool,

    /// Line 36: Amount of line 34 you want applied to your 2026 estimated tax
    pub applied_to_es_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 37–38: Amount You Owe
    // -----------------------------------------------------------------------
    /// Line 37: Subtract line 33 from line 24. This is the amount you owe.
    /// For details on how to pay, go to www.irs.gov/Payments or see instructions
    pub owed_amt: Usd,

    /// Line 38: Estimated tax penalty (see instructions)
    pub es_penalty_amt: Usd,
}
