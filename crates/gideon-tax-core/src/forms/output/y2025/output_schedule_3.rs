use crate::Usd;

/// Output fields for IRS Schedule 3 (Form 1040) 2025 — Additional Credits and Payments.
#[derive(Debug, Clone, Default)]
pub struct OutputSchedule3 {
    // -----------------------------------------------------------------------
    // Part I — Nonrefundable Credits
    // -----------------------------------------------------------------------
    /// Line 1: Foreign tax credit. Attach Form 1116 if required
    pub foreign_tax_credit_amt: Usd,
    /// Line 2: Credit for child and dependent care expenses from Form 2441, line 11.
    /// Attach Form 2441
    pub credit_for_child_and_depd_care_amt: Usd,
    /// Line 3: Education credits from Form 8863, line 19
    pub education_credit_amt: Usd,
    /// Line 4: Retirement savings contributions credit. Attach Form 8880
    pub rtr_savings_contributions_cr_amt: Usd,
    /// Line 5a: Residential clean energy credit from Form 5695, line 15
    pub residential_clean_energy_cr_amt: Usd,
    /// Line 5b: Energy efficient home improvement credit from Form 5695, line 32
    pub egy_effcnt_hm_imprv_cr_amt: Usd,
    /// Line 6a: General business credit. Attach Form 3800
    pub current_year_allowable_credit_amt: Usd,
    /// Line 6b: Credit for prior year minimum tax. Attach Form 8801
    pub min_amt_cr_amt: Usd,
    /// Line 6c: Adoption credit. Attach Form 8839
    pub nonrefundable_adoption_credit_amt: Usd,
    /// Line 6d: Credit for the elderly or disabled. Attach Schedule R
    pub credit_for_elderly_or_disabled_amt: Usd,
    /// Line 6f: Clean vehicle credit. Attach Form 8936
    pub clean_veh_prsnl_use_part_cr_amt: Usd,
    /// Line 6g: Mortgage interest credit. Attach Form 8396
    pub mortgage_interest_credit_amt: Usd,
    /// Line 6h: District of Columbia first-time homebuyer credit. Attach Form 8859
    pub dc_hm_byr_current_year_credit_amt: Usd,
    /// Line 6i: Qualified electric vehicle credit. Attach Form 8834
    pub qlfy_elec_motor_veh_cr_amt: Usd,
    /// Line 6k: Credit to holders of tax credit bonds. Attach Form 8912
    pub current_year_credit_allowed_amt: Usd,
    /// Line 6l: Amount on Form 8978, line 14. See instructions
    pub tot_rptg_yr_tx_increase_decrease_amt: Usd,
    /// Line 6m: Credit for previously owned clean vehicles. Attach Form 8936
    pub max_prev_owned_clean_veh_cr_amt: Usd,
    /// Line 6z: Other nonrefundable credits. List type and amount
    pub tot_oth_nonrefundable_credits_amt: Usd,
    /// Line 6z: Other nonrefundable credits — text description
    pub other_nonrefundable_credits_txt: String,
    /// Line 6z: Other nonrefundable credits — amount
    pub other_nonrefundable_credits_amt: Usd,
    /// Line 7: Total other nonrefundable credits. Add lines 6a through 6z
    pub other_credits_amt: Usd,
    /// Line 8: Add lines 1 through 4, 5a, 5b, and 7. Enter here and on Form 1040, 1040-SR,
    /// or 1040-NR, line 20
    pub total_nonrefundable_credits_amt: Usd,
    /// Total personal use part of credit
    pub total_personal_use_part_of_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Other Payments and Refundable Credits
    // -----------------------------------------------------------------------
    /// Line 9: Net premium tax credit. Attach Form 8962
    pub reconciled_premium_tax_credit_amt: Usd,
    /// Line 10: Amount paid with request for extension to file (see instructions)
    pub request_for_extension_amt: Usd,
    /// Line 11: Excess social security and tier 1 RRTA tax withheld
    pub excess_soc_sec_and_tier1_rrta_tax_amt: Usd,
    /// Line 12: Credit for federal tax on fuels. Attach Form 4136
    pub total_fuel_tax_credit_amt: Usd,
    /// Line 13a: Form 2439
    pub tax_paid_by_ric_or_reit_amt: Usd,
    /// Line 13b: Section 1341 credit for repayment of amounts included in income from earlier
    /// years
    pub credit_for_repayment_amt: Usd,
    /// Line 13c: Net elective payment election amount from Form 3800, Part III, line 6, column (j)
    pub net_elective_pymt_election_amt: Usd,
    /// Line 13d: Deferred amount of net 965 tax liability (see instructions)
    pub net_section965_tax_liability_amt: Usd,
    /// Line 13z: Other refundable credits (see instructions)
    pub total_other_refundable_credits_amt: Usd,
    /// Line 13z: Other refundable credits — code
    pub other_refundable_cr_cd: String,
    /// Line 13z: Other refundable credits — text description
    pub other_refundable_cr_txt: String,
    /// Line 13z: Other refundable credits — amount
    pub other_refundable_credits_amt: Usd,
    /// Line 14: Total other payments or refundable credits. Add lines 13a through 13z
    pub total_other_payments_rfdbl_cr_amt: Usd,
    /// Line 15: Add lines 9 through 12 and 14. Enter here and on Form 1040, 1040-SR,
    /// or 1040-NR, line 31
    pub other_payments_amt: Usd,
}
