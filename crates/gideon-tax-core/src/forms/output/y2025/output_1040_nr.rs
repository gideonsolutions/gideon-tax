use crate::Usd;

/// Output fields for IRS Form 1040-NR (2025), U.S. Nonresident Alien Income Tax Return.
///
/// Fields are ordered by line number as they appear on the form.
/// Only includes fields relevant to tax computation; excludes names, addresses,
/// SSNs, EINs, and preparer metadata.
#[derive(Debug, Clone, Default)]
pub struct Output1040Nr {
    // -----------------------------------------------------------------------
    // Filing Status
    // -----------------------------------------------------------------------
    /// Filing Status: check only one box (1 = Single, 3 = Married filing separately (MFS),
    /// 5 = Qualifying surviving spouse (QSS), 6 = Estate, 7 = Trust)
    pub individual_return_filing_status_cd: u8,

    /// If you checked the QSS box, enter the child's name if
    /// the qualifying person is a child but not your dependent
    pub qualifying_hoh_nm: String,

    /// Qualifying surviving spouse (QSS) SSN
    pub qualifying_hoh_ssn: String,

    /// Qualifying person first name
    pub qualifying_person_first_nm: String,

    /// Qualifying person last name
    pub qualifying_person_last_nm: String,

    /// Qualifying person SSN
    pub qualifying_person_ssn: String,

    /// Surviving spouse indicator
    pub surviving_spouse_ind: bool,

    /// Estate or trust indicator
    pub estate_or_trust_ind: bool,

    // -----------------------------------------------------------------------
    // Header checkboxes
    // -----------------------------------------------------------------------
    /// Filed pursuant to section 301.9100-2
    pub filed_pursuant_to_sect_3019_1002_ind: bool,

    /// Deceased indicator
    pub deceased_ind: bool,

    /// Primary date of death
    pub primary_death_dt: String,

    /// Spouse date of death
    pub spouse_death_dt: String,

    /// Spouse name
    pub spouse_nm: String,

    /// Community property state return indicator
    pub comm_prop_state_rtn_ind: bool,

    // -----------------------------------------------------------------------
    // Digital Assets
    // -----------------------------------------------------------------------
    /// At any time during 2025, did you: (a) receive (as a reward, award, or payment for property
    /// or services); or (b) sell, exchange, or otherwise dispose of a digital asset (or a financial
    /// interest in a digital asset)? (See instructions.)
    pub virtual_cur_acquired_dur_ty_ind: bool,

    // -----------------------------------------------------------------------
    // Dependents
    // -----------------------------------------------------------------------
    /// Dependents: number of children who lived with you
    pub chld_who_lived_with_you_cnt: u32,

    /// Dependents: number of other dependents listed
    pub other_dependents_listed_cnt: u32,

    /// Dependents: more dependents indicator (if more than four dependents)
    pub more_dependents_ind: bool,

    // -----------------------------------------------------------------------
    // Exemptions
    // -----------------------------------------------------------------------
    /// Total exempt primary and spouse count
    pub total_exempt_primary_and_spouse_cnt: u32,

    /// Total exemptions count
    pub total_exemptions_cnt: u32,

    /// Exempt spouse name
    pub exempt_spouse_nm: String,

    /// Exempt spouse name control text
    pub exempt_spouse_name_control_txt: String,

    // -----------------------------------------------------------------------
    // Income Effectively Connected With U.S. Trade or Business (Lines 1a-1z)
    // -----------------------------------------------------------------------
    /// Line 1a: Total amount from Form(s) W-2, box 1 (see instructions)
    pub wages_salaries_and_tips_amt: Usd,

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
    pub wages_amt: Usd,

    /// Line 1h: Other earned income (see instructions). Enter type and amount:
    pub other_earned_income_amt: Usd,

    /// Line 1k: Total income exempt by a treaty from Schedule OI (Form 1040-NR), item
    /// L, line 1(e)
    pub treaty_tax_exempt_us_income_amt: Usd,

    /// Line 1z: Add lines 1a through 1h
    pub total_eci_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 2a-3b (Interest and Dividends)
    // -----------------------------------------------------------------------
    /// Line 2a: Tax-exempt interest
    pub tax_exempt_interest_amt: Usd,

    /// Line 2b: Taxable interest
    pub taxable_interest_amt: Usd,

    /// Line 3a: Qualified dividends
    pub qualified_dividends_amt: Usd,

    /// Line 3b: Ordinary dividends
    pub ordinary_dividends_amt: Usd,

    /// Line 3c: Check if your child's dividends are included in 1 Line 3a
    pub child_div_incld_qualified_div_ind: bool,

    /// Line 3c: Check if your child's dividends are included in 2 Line 3b
    pub child_div_incld_ordinary_div_ind: bool,

    // -----------------------------------------------------------------------
    // Lines 4a-4b (IRA distributions)
    // -----------------------------------------------------------------------
    /// Line 4a: IRA distributions
    pub ira_distributions_amt: Usd,

    /// Line 4b: Taxable amount
    pub taxable_ira_amt: Usd,

    /// Line 4c: Check if (see instructions) 1 Rollover
    pub ira_distribution_rollover_ind: bool,

    /// Line 4c: Check if (see instructions) 2 QCD
    pub ira_distribution_other_ind: bool,

    /// Other IRA type code
    pub other_ira_type_cd: String,

    /// Other IRA type text
    pub other_ira_type_txt: String,

    // -----------------------------------------------------------------------
    // Lines 5a-5b (Pensions and annuities)
    // -----------------------------------------------------------------------
    /// Line 5a: Pensions and annuities
    pub pensions_annuities_amt: Usd,

    /// Line 5b: Taxable amount
    pub total_taxable_pensions_amt: Usd,

    /// Line 5c: Check if (see instructions) 1 Rollover
    pub pensions_annuities_rollover_ind: bool,

    /// Line 5c: Check if (see instructions) 2 PSO
    pub pensions_annuities_pso_ind: bool,

    /// Line 5c: Check if (see instructions) other indicator
    pub pensions_annuities_other_ind: bool,

    /// Line 5c: Check if (see instructions) other type text
    pub pensions_annuities_other_type_txt: String,

    // -----------------------------------------------------------------------
    // Lines 6-9 (Other income)
    // -----------------------------------------------------------------------
    /// Line 7a: Capital gain or (loss). Attach Schedule D if required
    pub capital_gain_loss_amt: Usd,

    /// Line 7b: Check if: Schedule D not required
    pub capital_distribution_ind: bool,

    /// Line 7b: Check if: Includes child's capital gain or (loss)
    pub child_capital_gain_or_loss_incld_ind: bool,

    /// Line 8: Additional income from Schedule 1 (Form 1040), line 10
    pub total_additional_income_amt: Usd,

    /// Line 9: Add lines 1z, 2b, 3b, 4b, 5b, 7a, and 8. This is your total effectively
    /// connected income
    pub total_us_source_gross_trans_incm_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 10-11a (Adjustments and AGI)
    // -----------------------------------------------------------------------
    /// Line 10: Adjustments to income from Schedule 1 (Form 1040), line 26. These are your
    /// total adjustments to income
    pub total_adjustments_amt: Usd,

    /// Line 11a: Subtract line 10 from line 9. This is your adjusted gross income
    pub adjusted_gross_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Page 2: Tax and Credits (Lines 11b-24)
    // -----------------------------------------------------------------------
    /// Line 12: Itemized deductions (from Schedule A (Form 1040-NR)) or, for certain residents
    /// of India, standard deduction (see instructions)
    pub total_itemized_deductions_amt: Usd,

    /// Line 12: Modified standard deduction indicator (for certain residents of India)
    pub modified_standard_deduction_ind: bool,

    /// Line 12: India standard deduction tax treaty code
    pub india_standard_ded_tax_treaty_cd: String,

    /// Line 12: India standard deduction tax treaty amount
    pub india_standard_ded_tax_treaty_amt: Usd,

    /// Line 13a: Qualified business income deduction from Form 8995 or Form 8995-A
    pub qualified_business_income_ded_amt: Usd,

    /// Line 13c: Additional deductions from Schedule 1-A, line 38
    pub total_additional_deductions_amt: Usd,

    /// Line 14: Add lines 12 through 13c
    pub total_deduction_amt: Usd,

    /// Line 15: Subtract line 14 from line 11b. If zero or less, enter -0-. This is your
    /// taxable income
    pub taxable_income_amt: Usd,

    /// Line 15: Capital construction fund code
    pub capital_construction_fund_cd: String,

    /// Line 15: Capital construction fund amount
    pub capital_construction_fund_amt: Usd,

    /// Line 16: Tax (see instructions). Check if any from Form(s): 1 8814 2 4972 3 ____
    pub tax_amt: Usd,

    /// Line 16: Check if from Form(s) 8814
    pub form_8814_ind: bool,

    /// Line 16: Form 8814 child interest and dividend tax amount
    pub child_interest_and_dividend_tax_amt: Usd,

    /// Line 16: Form 8814 amount
    pub form_8814_amt: Usd,

    /// Line 16: Check if from Form(s) 4972
    pub form_4972_ind: bool,

    /// Line 16: Check if other tax amount indicator (checkbox 3)
    pub other_tax_amt_ind: bool,

    /// Line 16: Other tax amount code
    pub other_tax_amt_cd: String,

    /// Line 16: Other tax amount
    pub other_tax_amt: Usd,

    /// Line 16: Schedule Q code
    pub schedule_q_cd: String,

    /// Line 17: Amount from Schedule 2 (Form 1040), line 3
    pub additional_tax_amt: Usd,

    /// Line 18: Add lines 16 and 17
    pub total_tax_before_cr_and_oth_taxes_amt: Usd,

    /// Line 19: Child tax credit or credit for other dependents from Schedule 8812 (Form 1040)
    pub ctc_odc_amt: Usd,

    /// Line 20: Amount from Schedule 3 (Form 1040), line 8
    pub total_nonrefundable_credits_amt: Usd,

    /// Line 21: Add lines 19 and 20
    pub total_credits_amt: Usd,

    /// Line 22: Subtract line 21 from line 18. If zero or less, enter -0-
    pub tax_less_credits_amt: Usd,

    /// Line 23a: Tax on income not effectively connected with a U.S. trade or business from
    /// Schedule NEC (Form 1040-NR), line 15
    pub income_not_us_business_tax_amt: Usd,

    /// Line 23b: Other taxes, including self-employment tax, from Schedule 2 (Form 1040),
    /// line 21
    pub total_other_taxes_amt: Usd,

    /// Line 23c: Transportation tax (see instructions)
    pub total_nec_trans_other_tax_amt: Usd,

    /// Line 24: Add lines 22 and 23d. This is your total tax
    pub total_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Payments and Refundable Credits (Lines 25a-33)
    // -----------------------------------------------------------------------
    /// Line 25a: Form(s) W-2
    pub form_w2_withheld_tax_amt: Usd,

    /// Line 25b: Form(s) 1099
    pub form_1099_withheld_tax_amt: Usd,

    /// Line 25c: Other forms (see instructions)
    pub tax_withheld_other_amt: Usd,

    /// Line 25d: Add lines 25a through 25c
    pub withholding_tax_amt: Usd,

    /// Line 25e: Form(s) 8805
    pub withholding_tx_from_8805_amt: Usd,

    /// Line 25f: Form(s) 8288-A
    pub withholding_tx_from_8288a_amt: Usd,

    /// Line 25g: Form(s) 1042-S
    pub withholding_tx_from_1042s_amt: Usd,

    /// Line 26: 2025 estimated tax payments and amount applied from 2024 return
    pub estimated_tax_payments_amt: Usd,

    /// Line 26: Divorced spouse SSN (for estimated tax payments)
    pub divorced_spouse_ssn: String,

    /// Line 26: Divorced literal code
    pub divorced_literal_cd: String,

    /// Line 28: Additional child tax credit (ACTC) from Schedule 8812 (Form 1040). If you
    /// do not want to claim the ACTC, check here
    pub additional_child_tax_credit_amt: Usd,

    /// Line 28: Do not want to claim the ACTC indicator
    pub do_not_claim_actc_ind: bool,

    /// Line 29: Credit for amount paid with Form 1040-C
    pub paid_with_form_1040c_amt: Usd,

    /// Line 30: Refundable adoption credit from Form 8839, line 13
    pub refundable_adoption_credit_amt: Usd,

    /// Line 31: Amount from Schedule 3 (Form 1040), line 15
    pub refundable_credits_amt: Usd,

    /// Line 32: Add lines 28, 29, 30, and 31. These are your total other payments and
    /// refundable credits
    pub total_other_payments_rfdbl_cr_amt: Usd,

    /// Line 33: Add lines 25d, 25e, 25f, 25g, 26, and 32. These are your total payments
    pub total_payments_amt: Usd,

    /// Line 33: Form 8689 code
    pub form_8689_cd: String,

    /// Line 33: Form 8689 amount
    pub form_8689_amt: Usd,

    // -----------------------------------------------------------------------
    // Refund (Lines 34-36)
    // -----------------------------------------------------------------------
    /// Line 34: If line 33 is more than line 24, subtract line 24 from line 33. This is the
    /// amount you overpaid
    pub overpaid_amt: Usd,

    /// Line 35a: Amount of line 34 you want refunded to you. If Form 8888 is attached, check here
    pub refund_amt: Usd,

    /// Line 35a: Form 8888 indicator
    pub form_8888_ind: bool,

    /// Line 35b: Routing number
    pub routing_transit_num: String,

    /// Line 35c: Account type (Checking or Savings)
    pub bank_account_type_cd: String,

    /// Line 35d: Account number
    pub depositor_account_num: String,

    /// Line 36: Amount of line 34 you want applied to your 2026 estimated tax
    pub applied_to_es_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Amount You Owe (Lines 37-38)
    // -----------------------------------------------------------------------
    /// Line 37: Subtract line 33 from line 24. This is the amount you owe.
    /// For details on how to pay, go to www.irs.gov/Payments or see instructions
    pub owed_amt: Usd,

    /// Line 38: Estimated tax penalty (see instructions)
    pub es_penalty_amt: Usd,

    // -----------------------------------------------------------------------
    // Third Party Designee
    // -----------------------------------------------------------------------
    /// Third Party Designee: Do you want to allow another person to discuss this return with
    /// the IRS? See instructions.
    pub third_party_designee_ind: bool,

    /// Third Party Designee: Designee's name
    pub third_party_designee_nm: String,

    /// Third Party Designee: Phone no.
    pub third_party_designee_phone_num: String,

    /// Third Party Designee: Foreign phone number
    pub third_party_designee_frgn_phone_num: String,

    /// Third Party Designee: Personal identification number (PIN)
    pub third_party_designee_pin: String,

    // -----------------------------------------------------------------------
    // Sign Here / Administrative
    // -----------------------------------------------------------------------
    /// Your occupation
    pub primary_occupation_txt: String,

    /// Personal representative indicator
    pub personal_representative_ind: bool,

    /// Power of attorney name
    pub power_of_attorney_nm: String,

    /// Power of attorney signed by indicator
    pub power_of_attorney_signed_by_ind: bool,

    /// Protective return indicator
    pub protective_return_ind: bool,

    /// Combat zone indicator
    pub combat_zone_ind: bool,

    /// Combat zone code
    pub combat_zone_cd: String,

    /// Combat zone date
    pub combat_zone_dt: String,

    /// Combat zone text
    pub combat_zone_txt: String,

    /// Expatriation code
    pub expatriation_cd: String,

    /// NRA literal code
    pub nra_literal_cd: String,

    /// Non-paid preparer code
    pub non_paid_preparer_cd: String,

    /// Canada income tax treaty article XXV code
    pub canada_income_tax_treaty_art_xxv_cd: String,

    /// Excluded section 933 Puerto Rico income amount
    pub excld_sect_933_puerto_rico_incm_amt: Usd,

    /// Excluded section 933 Puerto Rico income code
    pub excld_sect_933_puerto_rico_incm_cd: String,

    /// Special condition description
    pub special_condition_desc: String,

    /// Special condition text
    pub special_condition_txt: String,

    /// Special processing code text
    pub special_processing_code_txt: String,

    /// Special processing literal code
    pub special_processing_literal_cd: String,

    /// Refund product code
    pub refund_product_cd: String,

    /// Refund product code text
    pub refund_product_code_txt: String,

    /// Interest penalty amount
    pub interest_penalty_amt: Usd,

    /// Interest penalty description
    pub interest_penalty_desc: String,

    /// Wages not shown on W-2: literal code
    pub wages_literal_cd: String,

    /// Wages not shown on W-2: other wages not shown text
    pub other_wages_not_shown_txt: String,

    /// Wages not shown on W-2: amount
    pub wages_not_shown_amt: Usd,

    /// Total wages with no withholding amount
    pub total_wages_with_no_withholding_amt: Usd,

    /// Line 15: India standard deduction tax treaty amount (on TaxableIncomeAmt)
    pub taxable_income_india_standard_ded_tax_treaty_amt: Usd,

    // -----------------------------------------------------------------------
    // Separate Mailing Address (Line 35e / page 1 header)
    // -----------------------------------------------------------------------
    /// Line 35e: If you want your refund check mailed to an address outside the United States
    /// not shown on page 1, enter it here. US address line 1
    pub separate_mailing_us_address_line_1_txt: String,

    /// Separate mailing US address line 2
    pub separate_mailing_us_address_line_2_txt: String,

    /// Separate mailing US city name
    pub separate_mailing_us_city_nm: String,

    /// Separate mailing US state abbreviation code
    pub separate_mailing_us_state_abbreviation_cd: String,

    /// Separate mailing US ZIP code
    pub separate_mailing_us_zip_cd: String,

    /// Separate mailing foreign address line 1
    pub separate_mailing_foreign_address_line_1_txt: String,

    /// Separate mailing foreign address line 2
    pub separate_mailing_foreign_address_line_2_txt: String,

    /// Separate mailing foreign city name
    pub separate_mailing_foreign_city_nm: String,

    /// Separate mailing foreign province or state name
    pub separate_mailing_foreign_province_or_state_nm: String,

    /// Separate mailing foreign country code
    pub separate_mailing_foreign_country_cd: String,

    /// Separate mailing foreign postal code
    pub separate_mailing_foreign_postal_cd: String,
}
