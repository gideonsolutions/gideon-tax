use crate::Usd;

/// Output fields for IRS Form 1040 (2025), U.S. Individual Income Tax Return.
///
/// Fields are ordered by section and line number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Output1040 {
    // =====================================================================
    // Header — Filed Pursuant, Combat Zone, Deceased
    // =====================================================================
    /// Filed pursuant to section 301.9100-2 indicator
    pub filed_pursuant_to_sect_3019_1002_ind: bool,

    /// Filed pursuant to section 301.9100-2 code
    pub filed_pursuant_to_sect_3019_1002_cd: String,

    /// Combat zone indicator
    pub combat_zone_ind: bool,

    /// Combat zone code
    pub combat_zone_cd: String,

    /// Combat zone date
    pub combat_zone_dt: String,

    /// Combat zone literal code
    pub combat_zone_literal_cd: String,

    /// Deceased indicator
    pub deceased_ind: bool,

    /// Primary date of death (MM/DD/YYYY)
    pub primary_death_dt: String,

    /// Spouse date of death (MM/DD/YYYY)
    pub spouse_death_dt: String,

    // =====================================================================
    // Header — Names & Identifying Numbers
    // =====================================================================
    /// Your first name and middle initial
    pub primary_first_nm: String,

    /// Your last name
    pub primary_last_nm: String,

    /// Your social security number
    pub primary_ssn: String,

    /// If joint return, spouse's first name and middle initial
    pub spouse_first_nm: String,

    /// Spouse's last name
    pub spouse_last_nm: String,

    /// Spouse's social security number
    pub spouse_ssn: String,

    /// Spouse name (schema field `SpouseNm`, e.g. for NRA spouse treated as resident)
    pub spouse_nm: String,

    /// Spouse occupation text
    pub spouse_occupation_txt: String,

    // =====================================================================
    // Header — Home Address
    // =====================================================================
    /// Home address (number and street). If you have a P.O. box, see instructions
    pub address_line_1_txt: String,

    /// Apt. no.
    pub address_line_2_txt: String,

    /// City, town, or post office
    pub city_nm: String,

    /// State
    pub state_abbreviation_cd: String,

    /// ZIP code
    pub zip_cd: String,

    /// Foreign country name
    pub foreign_country_nm: String,

    /// Foreign province/state/county
    pub foreign_province_or_state_nm: String,

    /// Foreign postal code
    pub foreign_postal_cd: String,

    // =====================================================================
    // Filing Status
    // =====================================================================
    /// Filing Status code (1 = Single, 2 = MFJ, 3 = MFS, 4 = HOH, 5 = QSS)
    pub individual_return_filing_status_cd: u8,

    /// If treating a nonresident alien or dual-status alien spouse as a U.S. resident:
    /// NRA spouse treated as resident indicator
    pub nra_spouse_treated_as_resident_ind: bool,

    /// NRA literal code
    pub nra_literal_cd: String,

    /// If you checked the HOH or QSS box, enter the child's name if the qualifying
    /// person is a child but not your dependent
    pub qualifying_hoh_nm: String,

    /// Qualifying HOH SSN
    pub qualifying_hoh_ssn: String,

    /// Qualifying person first name (for QSS)
    pub qualifying_person_first_nm: String,

    /// Qualifying person last name (for QSS)
    pub qualifying_person_last_nm: String,

    /// Qualifying person SSN (for QSS)
    pub qualifying_person_ssn: String,

    /// Surviving spouse indicator
    pub surviving_spouse_ind: bool,

    /// Community property state return indicator
    pub comm_prop_state_rtn_ind: bool,

    /// Check if your filing status is MFS or HOH and you lived apart from your spouse
    /// for the last 6 months of 2025
    pub sepd_sps_filing_sep_ret_meets_rqr_ind: bool,

    /// Presidential Election Campaign Fund — You
    pub pecf_primary_ind: bool,

    /// Presidential Election Campaign Fund — Spouse
    pub pecf_spouse_ind: bool,

    // =====================================================================
    // Digital Assets
    // =====================================================================
    /// At any time during 2025, did you: (a) receive (as a reward, award, or payment
    /// for property or services); or (b) sell, exchange, or otherwise dispose of a
    /// digital asset (or a financial interest in a digital asset)? (See instructions.)
    pub virtual_cur_acquired_dur_ty_ind: bool,

    // =====================================================================
    // Dependents & Exemptions
    // =====================================================================
    /// Dependents: number of children who lived with you
    pub chld_who_lived_with_you_cnt: u32,

    /// Dependents: number of other dependents listed
    pub other_dependents_listed_cnt: u32,

    /// Dependents: more dependents indicator (if more than four dependents)
    pub more_dependents_ind: bool,

    /// Exempt spouse name
    pub exempt_spouse_nm: String,

    /// Exempt spouse name control text
    pub exempt_spouse_name_control_txt: String,

    /// Total exempt primary and spouse count
    pub total_exempt_primary_and_spouse_cnt: u32,

    /// Total exemptions count
    pub total_exemptions_cnt: u32,

    // =====================================================================
    // Line 1: Wages, salaries, tips, etc.
    // =====================================================================
    /// Line 1a: Total amount from Form(s) W-2, box 1 (see instructions)
    pub wages_amt: Usd,

    /// Line 1a: Fringe benefits code (attribute on WagesAmt)
    pub fringe_benefits_cd: String,

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

    /// Line 1h: Wages not shown literal only code (attribute on OtherEarnedIncomeAmt)
    pub wages_not_shown_lit_only_cd: String,

    /// Line 1i: Nontaxable combat pay election (see instructions)
    pub nontx_combat_pay_election_amt: Usd,

    /// Line 1z: Add lines 1a through 1h
    pub wages_salaries_and_tips_amt: Usd,

    // =====================================================================
    // Lines 2–3: Interest and dividends
    // =====================================================================
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

    /// Line 3a: Ordinary dividends from Form 8814 amount
    pub ordinary_f8814_amt: Usd,

    /// Line 3a: Ordinary dividends from Form 8814 code
    pub ordinary_f8814_cd: String,

    /// Line 3a: Qualified dividends from Form 8814 amount
    pub qualified_f8814_amt: Usd,

    /// Line 3a: Qualified dividends from Form 8814 code
    pub qualified_f8814_cd: String,

    // =====================================================================
    // Line 4: IRA distributions
    // =====================================================================
    /// Line 4a: IRA distributions
    pub ira_distributions_amt: Usd,

    /// Line 4a: IRA distributions literal code
    pub ira_distributions_literal_cd: String,

    /// Line 4b: Taxable amount
    pub taxable_ira_amt: Usd,

    /// Line 4c checkbox 1: Rollover
    pub ira_distribution_rollover_ind: bool,

    /// Line 4c checkbox 2: QCD
    pub ira_distribution_qcd_ind: bool,

    /// Line 4c checkbox 3 (other, see instructions)
    pub ira_distribution_other_ind: bool,

    /// Other IRA type code
    pub other_ira_type_cd: String,

    /// Other IRA type text
    pub other_ira_type_txt: String,

    // =====================================================================
    // Line 5: Pensions and annuities
    // =====================================================================
    /// Line 5a: Pensions and annuities
    pub pensions_annuities_amt: Usd,

    /// Line 5b: Taxable amount
    pub total_taxable_pensions_amt: Usd,

    /// Line 5b: Pensions and annuities literal code (attribute on TotalTaxablePensionsAmt)
    pub pensions_annuities_literal_cd: String,

    /// Line 5b: Taxable foreign pensions total amount
    pub taxable_foreign_pensions_total_amt: Usd,

    /// Line 5b: Foreign employer pension code
    pub foreign_employer_pension_cd: String,

    /// Line 5c checkbox 1: Rollover
    pub pensions_annuities_rollover_ind: bool,

    /// Line 5c checkbox 2: PSO
    pub pensions_annuities_pso_ind: bool,

    /// Line 5c checkbox 3 (other, see instructions)
    pub pensions_annuities_other_ind: bool,

    /// Line 5c: Pensions/annuities other type code
    pub pensions_annuities_other_type_cd: String,

    /// Line 5c: Pensions/annuities other type text
    pub pensions_annuities_other_type_txt: String,

    // =====================================================================
    // Line 6: Social security benefits
    // =====================================================================
    /// Line 6a: Social security benefits
    pub soc_sec_bnft_amt: Usd,

    /// Line 6a: Social security benefits code (attribute on SocSecBnftAmt)
    pub soc_sec_bnft_cd: String,

    /// Line 6b: Taxable amount
    pub taxable_soc_sec_amt: Usd,

    /// Line 6c: If you elect to use the lump-sum election method, check here (see instructions)
    pub lump_sum_election_method_ind: bool,

    /// Line 6d: If you are married filing separately and lived apart from your spouse
    /// the entire year (see inst.), check here
    pub mfs_live_apart_entire_yr_ind: bool,

    // =====================================================================
    // Line 7: Capital gain or (loss)
    // =====================================================================
    /// Line 7a: Capital gain or (loss). Attach Schedule D if required
    pub capital_gain_loss_amt: Usd,

    /// Line 7b checkbox 1: Schedule D not required
    pub capital_distribution_ind: bool,

    /// Line 7b checkbox 2: Includes child's capital gain or (loss)
    pub child_capital_gain_or_loss_incld_ind: bool,

    // =====================================================================
    // Lines 8–11: Total income, adjustments, AGI
    // =====================================================================
    /// Line 8: Additional income from Schedule 1, line 10
    pub total_additional_income_amt: Usd,

    /// Line 9: Add lines 1z, 2b, 3b, 4b, 5b, 6b, 7a, and 8. This is your total income
    pub total_income_amt: Usd,

    /// Line 10: Adjustments to income from Schedule 1, line 26
    pub total_adjustments_amt: Usd,

    /// Line 11a: Subtract line 10 from line 9. This is your adjusted gross income
    pub adjusted_gross_income_amt: Usd,

    // =====================================================================
    // Page 2 — Tax and Credits (Lines 12–24)
    // =====================================================================
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

    /// Line 12e: Modified standard deduction indicator (attribute on TotalItemizedOrStandardDedAmt)
    pub modified_standard_deduction_ind: bool,

    /// Line 13a: Qualified business income deduction from Form 8995 or Form 8995-A
    pub qualified_business_income_ded_amt: Usd,

    /// Line 13b: Additional deductions from Schedule 1-A, line 38
    pub total_additional_deductions_amt: Usd,

    /// Line 14: Add lines 12e, 13a, and 13b
    pub total_deductions_amt: Usd,

    /// Line 15: Subtract line 14 from line 11b. If zero or less, enter -0-. This is your taxable income
    pub taxable_income_amt: Usd,

    /// Line 15: Capital construction fund amount (attribute on TaxableIncomeAmt)
    pub capital_construction_fund_amt: Usd,

    /// Line 15: Capital construction fund code (attribute on TaxableIncomeAmt)
    pub capital_construction_fund_cd: String,

    /// Line 16: Tax (see instructions). Check if any from Form(s): 1 [ ] 8814  2 [ ] 4972  3 [ ]
    pub tax_amt: Usd,

    /// Line 16 checkbox 1: Form 8814
    pub form_8814_ind: bool,

    /// Line 16: Form 8814 amount
    pub form_8814_amt: Usd,

    /// Line 16: Form 8814 code
    pub form_8814_cd: String,

    /// Line 16: Child interest and dividend tax amount (attribute on Form8814Ind)
    pub child_interest_and_dividend_tax_amt: Usd,

    /// Line 16 checkbox 2: Form 4972
    pub form_4972_ind: bool,

    /// Line 16 checkbox 3: Other tax amount indicator
    pub other_tax_amt_ind: bool,

    /// Line 16: Other tax amount
    pub other_tax_amt: Usd,

    /// Line 16: Other tax amount code
    pub other_tax_amt_cd: String,

    /// Line 16: Other tax text
    pub other_tax_txt: String,

    /// Line 16: Schedule Q code
    pub schedule_q_cd: String,

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

    // =====================================================================
    // Lines 25–33: Payments and Refundable Credits
    // =====================================================================
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

    /// Line 26: Divorced literal code (attribute on EstimatedTaxPaymentsAmt)
    pub divorced_literal_cd: String,

    /// Line 26: Divorced spouse SSN (attribute on EstimatedTaxPaymentsAmt)
    pub divorced_spouse_ssn: String,

    /// Line 27a: Earned income credit (EIC)
    pub earned_income_credit_amt: Usd,

    /// Line 27a: EIC eligibility literal code
    pub eic_eligibility_literal_cd: String,

    /// Line 27b: Clergy filing Schedule SE (see instructions)
    pub clergy_member_ind: bool,

    /// Line 27c: If you do not want to claim the EIC, check here
    pub do_not_claim_eic_ind: bool,

    /// Line 28: Additional child tax credit (ACTC) from Schedule 8812. If you do not want
    /// to claim the ACTC, check here
    pub additional_child_tax_credit_amt: Usd,

    /// Line 28 checkbox: If you do not want to claim the ACTC, check here
    pub do_not_claim_actc_ind: bool,

    /// Line 29: American opportunity credit from Form 8863, line 8
    pub refundable_amer_opp_credit_amt: Usd,

    /// Line 30: Refundable adoption credit from Form 8839, line 13
    pub refundable_adoption_credit_amt: Usd,

    /// Line 31: Amount from Schedule 3, line 15
    pub total_other_payments_rfdbl_cr_amt: Usd,

    /// Line 32: Add lines 27a, 28, 29, 30, and 31. These are your total other payments
    /// and refundable credits
    pub refundable_credits_amt: Usd,

    /// Line 33: Add lines 25d, 26, and 32. These are your total payments
    pub total_payments_amt: Usd,

    /// Line 33: Form 8689 amount (attribute on TotalPaymentsAmt)
    pub form_8689_amt: Usd,

    /// Line 33: Form 8689 code (attribute on TotalPaymentsAmt)
    pub form_8689_cd: String,

    // =====================================================================
    // Lines 34–36: Refund
    // =====================================================================
    /// Line 34: If line 33 is more than line 24, subtract line 24 from line 33.
    /// This is the amount you overpaid
    pub overpaid_amt: Usd,

    /// Line 35a: Amount of line 34 you want refunded to you. If Form 8888 is attached, check here
    pub refund_amt: Usd,

    /// Line 35a checkbox: If Form 8888 is attached, check here
    pub form_8888_ind: bool,

    /// Line 35b: Routing number
    pub routing_transit_num: String,

    /// Line 35c: Account type (Checking or Savings)
    pub bank_account_type_cd: String,

    /// Line 35d: Account number
    pub depositor_account_num: String,

    /// Line 36: Amount of line 34 you want applied to your 2026 estimated tax
    pub applied_to_es_tax_amt: Usd,

    // =====================================================================
    // Lines 37–38: Amount You Owe
    // =====================================================================
    /// Line 37: Subtract line 33 from line 24. This is the amount you owe.
    /// For details on how to pay, go to www.irs.gov/Payments or see instructions
    pub owed_amt: Usd,

    /// Line 38: Estimated tax penalty (see instructions)
    pub es_penalty_amt: Usd,

    // =====================================================================
    // Third Party Designee
    // =====================================================================
    /// Third Party Designee: Do you want to allow another person to discuss this return
    /// with the IRS? See instructions.
    pub third_party_designee_ind: bool,

    /// Third Party Designee: Designee's name
    pub third_party_designee_nm: String,

    /// Third Party Designee: Phone no.
    pub third_party_designee_phone_num: String,

    /// Third Party Designee: Personal identification number (PIN)
    pub third_party_designee_pin: String,

    // =====================================================================
    // Sign Here / Administrative
    // =====================================================================
    /// Your occupation
    pub primary_occupation_txt: String,

    /// Personal representative indicator
    pub personal_representative_ind: bool,

    /// Power of attorney name
    pub power_of_attorney_nm: String,

    /// Power of attorney signed by indicator
    pub power_of_attorney_signed_by_ind: bool,

    /// Non-paid preparer code
    pub non_paid_preparer_cd: String,

    // =====================================================================
    // Additional Schema Fields (codes, literals, and auxiliary)
    // =====================================================================
    /// Excluded section 933 Puerto Rico income amount
    pub excld_sect_933_puerto_rico_incm_amt: Usd,

    /// Excluded section 933 Puerto Rico income code
    pub excld_sect_933_puerto_rico_incm_cd: String,

    /// Form 8854 deferred tax: expatriation code
    pub expatriation_cd: String,

    /// Form 8854 deferred tax: total tax deferred amount
    pub total_tax_deferred_amt: Usd,

    /// Prior year earned income amount (for additional child tax credit)
    pub prior_year_earned_income_amt: Usd,

    /// Prior year earned income code (for additional child tax credit)
    pub prior_year_earned_income_cd: String,

    /// Refund product code
    pub refund_product_cd: String,

    /// Refund product code text
    pub refund_product_code_txt: String,

    /// Special condition description
    pub special_condition_desc: String,

    /// Special condition text
    pub special_condition_txt: String,

    /// Special processing code text
    pub special_processing_code_txt: String,

    /// Special processing literal code
    pub special_processing_literal_cd: String,
}
