use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Schedule 1 (Form 1040).
///
/// Dollar amounts are non-negative.  Lines that the IRS directs the taxpayer
/// to enter as negative (8d foreign earned income exclusion, 8s nontaxable
/// Medicaid waiver payments, 8u incarcerated wages) are accepted here as
/// **positive** values and subtracted during computation.
#[derive(Debug, Clone)]
pub struct Schedule1Input {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// For 2025, enter the amount reported to you on Form(s) 1099-K that was
    /// included in error or for personal items sold at a loss
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
    /// Line 4: Check if any from Form(s): 4797
    pub form_4797_ind: bool,
    /// Line 4: Check if any from Form(s): 4684
    pub form_4684_ind: bool,
    /// Line 4: Other gains or (losses)
    pub other_gain_loss_amt: Usd,
    /// Line 5: Rental real estate, royalties, partnerships, S corporations,
    /// trusts, etc. Attach Schedule E
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
    /// Line 8d: Foreign earned income exclusion from Form 2555 (positive;
    /// subtracted in line 9 computation)
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
    /// Line 8l: Income from the rental of personal property if you engaged in
    /// the rental for profit but were not in the business of renting such
    /// property
    pub rental_income_personal_prop_amt: Usd,
    /// Line 8m: Olympic and Paralympic medals and USOC prize money
    pub olympic_paralympic_medal_usoc_amt: Usd,
    /// Line 8n: Section 951(a) inclusion
    pub section_951a_inclusion_amt: Usd,
    /// Line 8o: Section 951A(a) inclusion
    pub section_951_aa_inclusion_amt: Usd,
    /// Line 8p: Section 461(l) excess business loss adjustment
    pub excess_business_loss_amt: Usd,
    /// Line 8q: Taxable distributions from an ABLE account
    pub taxable_able_distributions_amt: Usd,
    /// Line 8r: Scholarship and fellowship grants not reported on Form W-2
    pub grants_or_scholarships_amt: Usd,
    /// Line 8s: Nontaxable amount of Medicaid waiver payments included on
    /// Form 1040, line 1a or 1d (positive; subtracted in line 9 computation)
    pub nontx_medicaid_waiver_pymt_amt: Usd,
    /// Line 8t: Pension or annuity from a nonqualified deferred compensation
    /// plan or a nongovernmental section 457 plan
    pub nonqlfy_deferred_compensation_amt: Usd,
    /// Line 8u: Wages earned while incarcerated (positive; subtracted in
    /// line 9 computation)
    pub certain_penal_instn_wages_amt: Usd,
    /// Line 8v: Digital assets received as ordinary income not reported
    /// elsewhere
    pub digital_assets_amt: Usd,
    /// Line 8z: Other income. List type and amount
    pub total_other_income_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Adjustments to Income
    // -----------------------------------------------------------------------
    /// Line 11: Educator expenses
    pub educator_expenses_amt: Usd,
    /// Line 12: Certain business expenses of reservists, performing artists,
    /// and fee-basis government officials. Attach Form 2106
    pub bus_expns_reservists_and_others_amt: Usd,
    /// Line 13: Health savings account deduction. Attach Form 8889
    pub health_savings_account_ded_amt: Usd,
    /// Line 14: Moving expenses for members of the Armed Forces. Attach
    /// Form 3903
    pub moving_expense_amt: Usd,
    /// Line 14: If claiming only storage fees, check here
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
    /// Line 20: If you are married filing separately and lived apart from
    /// your spouse for the entire year, check here
    pub mfs_live_apart_entire_yr_ind: bool,
    /// Line 21: Student loan interest deduction
    pub student_loan_interest_ded_amt: Usd,
    /// Line 23: Archer MSA deduction
    pub archer_msa_deduction_amt: Usd,
    /// Line 24a: Jury duty pay
    pub jury_duty_pay_deduction_amt: Usd,
    /// Line 24b: Deductible expenses related to income reported on line 8l
    /// from the rental of personal property engaged in for profit
    pub rntl_incm_prsnl_prop_expnss_ded_amt: Usd,
    /// Line 24c: Nontaxable amount of the value of Olympic and Paralympic
    /// medals and USOC prize money reported on line 8m
    pub olympc_prlympc_medal_usoc_ded_amt: Usd,
    /// Line 24d: Reforestation amortization and expenses
    pub rfor_amortz_expnss_ded_amt: Usd,
    /// Line 24e: Repayment of supplemental unemployment benefits under the
    /// Trade Act of 1974
    pub repayment_supp_unempl_bnft_ded_amt: Usd,
    /// Line 24f: Contributions to section 501(c)(18)(D) pension plans
    pub sect_501c18d_contri_ded_amt: Usd,
    /// Line 24g: Contributions by certain chaplains to section 403(b) plans
    pub section_403b_contri_ded_amt: Usd,
    /// Line 24h: Attorney fees and court costs for actions involving certain
    /// unlawful discrimination claims
    pub atty_fees_crt_costs_ded_amt: Usd,
    /// Line 24i: Attorney fees and court costs you paid in connection with an
    /// award from the IRS for information you provided that helped the IRS
    /// detect tax law violations
    pub atty_fees_crt_costs_pd_ded_amt: Usd,
    /// Line 24j: Housing deduction from Form 2555
    pub housing_deduction_amt: Usd,
    /// Line 24k: Excess deductions of section 67(e) expenses from Schedule
    /// K-1 (Form 1041)
    pub section_67e_excess_deduction_amt: Usd,
    /// Line 24z: Other adjustments. List type and amount
    pub other_adjustments_total_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Schedule 1 (Form 1040) 2025 — Additional Income and Adjustments to Income.
#[derive(Debug, Clone)]
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

// =========================================================================
// Trait impls
// =========================================================================

impl Form for OutputSchedule1 {
    fn name() -> &'static str {
        "Schedule 1"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for OutputSchedule1 {
    type Input = Schedule1Input;

    fn must_file(input: &Self::Input) -> bool {
        [
            input.form_1099k_rpt_error_or_loss_amt,
            input.state_local_income_tax_refund_amt,
            input.total_alimony_received_amt,
            input.business_income_loss_amt,
            input.other_gain_loss_amt,
            input.rental_real_estate_income_loss_amt,
            input.net_farm_profit_loss_amt,
            input.unemployment_comp_amt,
            input.repayment_amt,
            input.net_operating_loss_deduction_amt,
            input.gambling_reportable_winning_amt,
            input.debt_cancellation_amt,
            input.total_income_exclusion_amt,
            input.tot_archer_msa_medcr_ltc_amt,
            input.tot_hsa_distri_hdhp_amt,
            input.alaska_permanent_fund_div_amt,
            input.jury_duty_pay_amt,
            input.prizes_awards_amt,
            input.activity_not_for_profit_incm_amt,
            input.stock_options_amt,
            input.rental_income_personal_prop_amt,
            input.olympic_paralympic_medal_usoc_amt,
            input.section_951a_inclusion_amt,
            input.section_951_aa_inclusion_amt,
            input.excess_business_loss_amt,
            input.taxable_able_distributions_amt,
            input.grants_or_scholarships_amt,
            input.nontx_medicaid_waiver_pymt_amt,
            input.nonqlfy_deferred_compensation_amt,
            input.certain_penal_instn_wages_amt,
            input.digital_assets_amt,
            input.total_other_income_amt,
            input.educator_expenses_amt,
            input.bus_expns_reservists_and_others_amt,
            input.health_savings_account_ded_amt,
            input.moving_expense_amt,
            input.deductible_self_employment_tax_amt,
            input.self_empld_sep_simple_qlfy_plans_amt,
            input.self_empld_health_ins_ded_amt,
            input.pnlty_on_erly_wthdrw_of_savings_amt,
            input.total_alimony_paid_amt,
            input.ira_deduction_amt,
            input.student_loan_interest_ded_amt,
            input.archer_msa_deduction_amt,
            input.jury_duty_pay_deduction_amt,
            input.rntl_incm_prsnl_prop_expnss_ded_amt,
            input.olympc_prlympc_medal_usoc_ded_amt,
            input.rfor_amortz_expnss_ded_amt,
            input.repayment_supp_unempl_bnft_ded_amt,
            input.sect_501c18d_contri_ded_amt,
            input.section_403b_contri_ded_amt,
            input.atty_fees_crt_costs_ded_amt,
            input.atty_fees_crt_costs_pd_ded_amt,
            input.housing_deduction_amt,
            input.section_67e_excess_deduction_amt,
            input.other_adjustments_total_amt,
        ]
        .iter()
        .any(|&amt| amt != Usd::ZERO)
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // Line 9: Total other income.  Add lines 8a through 8z.
        // Lines 8d, 8s, 8u are entered negative on the IRS form; we accept
        // them as positive and subtract here.
        let line9 = input.net_operating_loss_deduction_amt          // 8a
            + input.gambling_reportable_winning_amt                 // 8b
            + input.debt_cancellation_amt                           // 8c
            - input.total_income_exclusion_amt                      // 8d (negative on form)
            + input.tot_archer_msa_medcr_ltc_amt                    // 8e
            + input.tot_hsa_distri_hdhp_amt                         // 8f
            + input.alaska_permanent_fund_div_amt                   // 8g
            + input.jury_duty_pay_amt                               // 8h
            + input.prizes_awards_amt                               // 8i
            + input.activity_not_for_profit_incm_amt                // 8j
            + input.stock_options_amt                               // 8k
            + input.rental_income_personal_prop_amt                 // 8l
            + input.olympic_paralympic_medal_usoc_amt               // 8m
            + input.section_951a_inclusion_amt                      // 8n
            + input.section_951_aa_inclusion_amt                    // 8o
            + input.excess_business_loss_amt                        // 8p
            + input.taxable_able_distributions_amt                  // 8q
            + input.grants_or_scholarships_amt                      // 8r
            - input.nontx_medicaid_waiver_pymt_amt                  // 8s (negative on form)
            + input.nonqlfy_deferred_compensation_amt               // 8t
            - input.certain_penal_instn_wages_amt                   // 8u (negative on form)
            + input.digital_assets_amt                              // 8v
            + input.total_other_income_amt;                         // 8z

        // Line 10: Combine lines 1 through 7 and 9.
        let line10 = input.state_local_income_tax_refund_amt        // 1
            + input.total_alimony_received_amt                      // 2a
            + input.business_income_loss_amt                        // 3
            + input.other_gain_loss_amt                             // 4
            + input.rental_real_estate_income_loss_amt              // 5
            + input.net_farm_profit_loss_amt                        // 6
            + input.unemployment_comp_amt                           // 7
            + line9;                                                // 9

        // Line 25: Total other adjustments.  Add lines 24a through 24z.
        let line25 = input.jury_duty_pay_deduction_amt              // 24a
            + input.rntl_incm_prsnl_prop_expnss_ded_amt             // 24b
            + input.olympc_prlympc_medal_usoc_ded_amt               // 24c
            + input.rfor_amortz_expnss_ded_amt                      // 24d
            + input.repayment_supp_unempl_bnft_ded_amt              // 24e
            + input.sect_501c18d_contri_ded_amt                     // 24f
            + input.section_403b_contri_ded_amt                     // 24g
            + input.atty_fees_crt_costs_ded_amt                     // 24h
            + input.atty_fees_crt_costs_pd_ded_amt                  // 24i
            + input.housing_deduction_amt                           // 24j
            + input.section_67e_excess_deduction_amt                // 24k
            + input.other_adjustments_total_amt;                    // 24z

        // Line 26: Add lines 11 through 23 and 25.
        let line26 = input.educator_expenses_amt                    // 11
            + input.bus_expns_reservists_and_others_amt              // 12
            + input.health_savings_account_ded_amt                   // 13
            + input.moving_expense_amt                               // 14
            + input.deductible_self_employment_tax_amt               // 15
            + input.self_empld_sep_simple_qlfy_plans_amt             // 16
            + input.self_empld_health_ins_ded_amt                    // 17
            + input.pnlty_on_erly_wthdrw_of_savings_amt             // 18
            + input.total_alimony_paid_amt                           // 19a
            + input.ira_deduction_amt                                // 20
            + input.student_loan_interest_ded_amt                    // 21
            + input.archer_msa_deduction_amt                         // 23
            + line25;                                                // 25

        Ok(OutputSchedule1 {
            form_1099k_rpt_error_or_loss_amt: input.form_1099k_rpt_error_or_loss_amt,
            state_local_income_tax_refund_amt: input.state_local_income_tax_refund_amt,
            total_alimony_received_amt: input.total_alimony_received_amt,
            business_income_loss_amt: input.business_income_loss_amt,
            form_4797_ind: input.form_4797_ind,
            form_4684_ind: input.form_4684_ind,
            other_gain_loss_amt: input.other_gain_loss_amt,
            rental_real_estate_income_loss_amt: input.rental_real_estate_income_loss_amt,
            net_farm_profit_loss_amt: input.net_farm_profit_loss_amt,
            unemployment_comp_amt: input.unemployment_comp_amt,
            repaid_overpayment_ind: input.repaid_overpayment_ind,
            repayment_amt: input.repayment_amt,
            net_operating_loss_deduction_amt: input.net_operating_loss_deduction_amt,
            gambling_reportable_winning_amt: input.gambling_reportable_winning_amt,
            debt_cancellation_amt: input.debt_cancellation_amt,
            total_income_exclusion_amt: input.total_income_exclusion_amt,
            tot_archer_msa_medcr_ltc_amt: input.tot_archer_msa_medcr_ltc_amt,
            tot_hsa_distri_hdhp_amt: input.tot_hsa_distri_hdhp_amt,
            alaska_permanent_fund_div_amt: input.alaska_permanent_fund_div_amt,
            jury_duty_pay_amt: input.jury_duty_pay_amt,
            prizes_awards_amt: input.prizes_awards_amt,
            activity_not_for_profit_incm_amt: input.activity_not_for_profit_incm_amt,
            stock_options_amt: input.stock_options_amt,
            rental_income_personal_prop_amt: input.rental_income_personal_prop_amt,
            olympic_paralympic_medal_usoc_amt: input.olympic_paralympic_medal_usoc_amt,
            section_951a_inclusion_amt: input.section_951a_inclusion_amt,
            section_951_aa_inclusion_amt: input.section_951_aa_inclusion_amt,
            excess_business_loss_amt: input.excess_business_loss_amt,
            taxable_able_distributions_amt: input.taxable_able_distributions_amt,
            grants_or_scholarships_amt: input.grants_or_scholarships_amt,
            nontx_medicaid_waiver_pymt_amt: input.nontx_medicaid_waiver_pymt_amt,
            nonqlfy_deferred_compensation_amt: input.nonqlfy_deferred_compensation_amt,
            certain_penal_instn_wages_amt: input.certain_penal_instn_wages_amt,
            digital_assets_amt: input.digital_assets_amt,
            total_other_income_amt: input.total_other_income_amt,
            other_income_total_amt: line9,
            total_additional_income_amt: line10,
            educator_expenses_amt: input.educator_expenses_amt,
            bus_expns_reservists_and_others_amt: input.bus_expns_reservists_and_others_amt,
            health_savings_account_ded_amt: input.health_savings_account_ded_amt,
            moving_expense_amt: input.moving_expense_amt,
            claim_storage_fees_ind: input.claim_storage_fees_ind,
            deductible_self_employment_tax_amt: input.deductible_self_employment_tax_amt,
            self_empld_sep_simple_qlfy_plans_amt: input.self_empld_sep_simple_qlfy_plans_amt,
            self_empld_health_ins_ded_amt: input.self_empld_health_ins_ded_amt,
            pnlty_on_erly_wthdrw_of_savings_amt: input.pnlty_on_erly_wthdrw_of_savings_amt,
            total_alimony_paid_amt: input.total_alimony_paid_amt,
            ira_deduction_amt: input.ira_deduction_amt,
            mfs_live_apart_entire_yr_ind: input.mfs_live_apart_entire_yr_ind,
            student_loan_interest_ded_amt: input.student_loan_interest_ded_amt,
            archer_msa_deduction_amt: input.archer_msa_deduction_amt,
            jury_duty_pay_deduction_amt: input.jury_duty_pay_deduction_amt,
            rntl_incm_prsnl_prop_expnss_ded_amt: input.rntl_incm_prsnl_prop_expnss_ded_amt,
            olympc_prlympc_medal_usoc_ded_amt: input.olympc_prlympc_medal_usoc_ded_amt,
            rfor_amortz_expnss_ded_amt: input.rfor_amortz_expnss_ded_amt,
            repayment_supp_unempl_bnft_ded_amt: input.repayment_supp_unempl_bnft_ded_amt,
            sect_501c18d_contri_ded_amt: input.sect_501c18d_contri_ded_amt,
            section_403b_contri_ded_amt: input.section_403b_contri_ded_amt,
            atty_fees_crt_costs_ded_amt: input.atty_fees_crt_costs_ded_amt,
            atty_fees_crt_costs_pd_ded_amt: input.atty_fees_crt_costs_pd_ded_amt,
            housing_deduction_amt: input.housing_deduction_amt,
            section_67e_excess_deduction_amt: input.section_67e_excess_deduction_amt,
            other_adjustments_total_amt: input.other_adjustments_total_amt,
            total_other_adjustments_amt: line25,
            total_adjustments_amt: line26,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[
            DynForm::ScheduleC,
            DynForm::F4797,
            DynForm::F4684,
            DynForm::ScheduleE,
            DynForm::ScheduleF,
            DynForm::F1099G,
            DynForm::F2555,
            DynForm::F8853,
            DynForm::F8889,
            DynForm::F2106,
            DynForm::F3903,
            DynForm::ScheduleSe,
            DynForm::F1041ScheduleK1,
        ]
    }

    fn is_valid(&self) -> bool {
        // Line 9 = sum of 8a–8z (8d, 8s, 8u subtracted)
        let line9_ok = self.other_income_total_amt
            == self.net_operating_loss_deduction_amt
                + self.gambling_reportable_winning_amt
                + self.debt_cancellation_amt
                - self.total_income_exclusion_amt
                + self.tot_archer_msa_medcr_ltc_amt
                + self.tot_hsa_distri_hdhp_amt
                + self.alaska_permanent_fund_div_amt
                + self.jury_duty_pay_amt
                + self.prizes_awards_amt
                + self.activity_not_for_profit_incm_amt
                + self.stock_options_amt
                + self.rental_income_personal_prop_amt
                + self.olympic_paralympic_medal_usoc_amt
                + self.section_951a_inclusion_amt
                + self.section_951_aa_inclusion_amt
                + self.excess_business_loss_amt
                + self.taxable_able_distributions_amt
                + self.grants_or_scholarships_amt
                - self.nontx_medicaid_waiver_pymt_amt
                + self.nonqlfy_deferred_compensation_amt
                - self.certain_penal_instn_wages_amt
                + self.digital_assets_amt
                + self.total_other_income_amt;

        // Line 10 = lines 1–7 + line 9
        let line10_ok = self.total_additional_income_amt
            == self.state_local_income_tax_refund_amt
                + self.total_alimony_received_amt
                + self.business_income_loss_amt
                + self.other_gain_loss_amt
                + self.rental_real_estate_income_loss_amt
                + self.net_farm_profit_loss_amt
                + self.unemployment_comp_amt
                + self.other_income_total_amt;

        // Line 25 = sum of 24a–24z
        let line25_ok = self.total_other_adjustments_amt
            == self.jury_duty_pay_deduction_amt
                + self.rntl_incm_prsnl_prop_expnss_ded_amt
                + self.olympc_prlympc_medal_usoc_ded_amt
                + self.rfor_amortz_expnss_ded_amt
                + self.repayment_supp_unempl_bnft_ded_amt
                + self.sect_501c18d_contri_ded_amt
                + self.section_403b_contri_ded_amt
                + self.atty_fees_crt_costs_ded_amt
                + self.atty_fees_crt_costs_pd_ded_amt
                + self.housing_deduction_amt
                + self.section_67e_excess_deduction_amt
                + self.other_adjustments_total_amt;

        // Line 26 = lines 11–23 + line 25
        let line26_ok = self.total_adjustments_amt
            == self.educator_expenses_amt
                + self.bus_expns_reservists_and_others_amt
                + self.health_savings_account_ded_amt
                + self.moving_expense_amt
                + self.deductible_self_employment_tax_amt
                + self.self_empld_sep_simple_qlfy_plans_amt
                + self.self_empld_health_ins_ded_amt
                + self.pnlty_on_erly_wthdrw_of_savings_amt
                + self.total_alimony_paid_amt
                + self.ira_deduction_amt
                + self.student_loan_interest_ded_amt
                + self.archer_msa_deduction_amt
                + self.total_other_adjustments_amt;

        line9_ok && line10_ok && line25_ok && line26_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_input() -> Schedule1Input {
        Schedule1Input {
            form_1099k_rpt_error_or_loss_amt: Usd::ZERO,
            state_local_income_tax_refund_amt: Usd::ZERO,
            total_alimony_received_amt: Usd::ZERO,
            business_income_loss_amt: Usd::ZERO,
            form_4797_ind: false,
            form_4684_ind: false,
            other_gain_loss_amt: Usd::ZERO,
            rental_real_estate_income_loss_amt: Usd::ZERO,
            net_farm_profit_loss_amt: Usd::ZERO,
            unemployment_comp_amt: Usd::ZERO,
            repaid_overpayment_ind: false,
            repayment_amt: Usd::ZERO,
            net_operating_loss_deduction_amt: Usd::ZERO,
            gambling_reportable_winning_amt: Usd::ZERO,
            debt_cancellation_amt: Usd::ZERO,
            total_income_exclusion_amt: Usd::ZERO,
            tot_archer_msa_medcr_ltc_amt: Usd::ZERO,
            tot_hsa_distri_hdhp_amt: Usd::ZERO,
            alaska_permanent_fund_div_amt: Usd::ZERO,
            jury_duty_pay_amt: Usd::ZERO,
            prizes_awards_amt: Usd::ZERO,
            activity_not_for_profit_incm_amt: Usd::ZERO,
            stock_options_amt: Usd::ZERO,
            rental_income_personal_prop_amt: Usd::ZERO,
            olympic_paralympic_medal_usoc_amt: Usd::ZERO,
            section_951a_inclusion_amt: Usd::ZERO,
            section_951_aa_inclusion_amt: Usd::ZERO,
            excess_business_loss_amt: Usd::ZERO,
            taxable_able_distributions_amt: Usd::ZERO,
            grants_or_scholarships_amt: Usd::ZERO,
            nontx_medicaid_waiver_pymt_amt: Usd::ZERO,
            nonqlfy_deferred_compensation_amt: Usd::ZERO,
            certain_penal_instn_wages_amt: Usd::ZERO,
            digital_assets_amt: Usd::ZERO,
            total_other_income_amt: Usd::ZERO,
            educator_expenses_amt: Usd::ZERO,
            bus_expns_reservists_and_others_amt: Usd::ZERO,
            health_savings_account_ded_amt: Usd::ZERO,
            moving_expense_amt: Usd::ZERO,
            claim_storage_fees_ind: false,
            deductible_self_employment_tax_amt: Usd::ZERO,
            self_empld_sep_simple_qlfy_plans_amt: Usd::ZERO,
            self_empld_health_ins_ded_amt: Usd::ZERO,
            pnlty_on_erly_wthdrw_of_savings_amt: Usd::ZERO,
            total_alimony_paid_amt: Usd::ZERO,
            ira_deduction_amt: Usd::ZERO,
            mfs_live_apart_entire_yr_ind: false,
            student_loan_interest_ded_amt: Usd::ZERO,
            archer_msa_deduction_amt: Usd::ZERO,
            jury_duty_pay_deduction_amt: Usd::ZERO,
            rntl_incm_prsnl_prop_expnss_ded_amt: Usd::ZERO,
            olympc_prlympc_medal_usoc_ded_amt: Usd::ZERO,
            rfor_amortz_expnss_ded_amt: Usd::ZERO,
            repayment_supp_unempl_bnft_ded_amt: Usd::ZERO,
            sect_501c18d_contri_ded_amt: Usd::ZERO,
            section_403b_contri_ded_amt: Usd::ZERO,
            atty_fees_crt_costs_ded_amt: Usd::ZERO,
            atty_fees_crt_costs_pd_ded_amt: Usd::ZERO,
            housing_deduction_amt: Usd::ZERO,
            section_67e_excess_deduction_amt: Usd::ZERO,
            other_adjustments_total_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_all_zeros() {
        assert!(!OutputSchedule1::must_file(&empty_input()));
    }

    #[test]
    fn must_file_additional_income() {
        let mut input = empty_input();
        input.unemployment_comp_amt = Usd::from_dollars(5_000);
        assert!(OutputSchedule1::must_file(&input));
    }

    #[test]
    fn must_file_adjustment_only() {
        let mut input = empty_input();
        input.educator_expenses_amt = Usd::from_dollars(300);
        assert!(OutputSchedule1::must_file(&input));
    }

    #[test]
    fn all_zeros() {
        let form = OutputSchedule1::try_new(empty_input()).unwrap();
        assert_eq!(form.other_income_total_amt, Usd::ZERO);
        assert_eq!(form.total_additional_income_amt, Usd::ZERO);
        assert_eq!(form.total_other_adjustments_amt, Usd::ZERO);
        assert_eq!(form.total_adjustments_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn basic_additional_income_lines_1_through_7() {
        let mut input = empty_input();
        input.state_local_income_tax_refund_amt = Usd::from_dollars(1_000);
        input.business_income_loss_amt = Usd::from_dollars(50_000);
        input.unemployment_comp_amt = Usd::from_dollars(8_000);

        let form = OutputSchedule1::try_new(input).unwrap();

        assert_eq!(form.other_income_total_amt, Usd::ZERO);
        // Line 10: 1,000 + 50,000 + 8,000 = 59,000
        assert_eq!(
            form.total_additional_income_amt,
            Usd::from_dollars(59_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn other_income_line9() {
        let mut input = empty_input();
        input.gambling_reportable_winning_amt = Usd::from_dollars(2_000);
        input.alaska_permanent_fund_div_amt = Usd::from_dollars(1_500);
        input.prizes_awards_amt = Usd::from_dollars(500);

        let form = OutputSchedule1::try_new(input).unwrap();

        // Line 9: 2,000 + 1,500 + 500 = 4,000
        assert_eq!(form.other_income_total_amt, Usd::from_dollars(4_000));
        // Line 10: 0 (lines 1–7) + 4,000 = 4,000
        assert_eq!(form.total_additional_income_amt, Usd::from_dollars(4_000));
        assert!(form.is_valid());
    }

    #[test]
    fn negative_entries_reduce_line9() {
        let mut input = empty_input();
        input.gambling_reportable_winning_amt = Usd::from_dollars(10_000);
        input.total_income_exclusion_amt = Usd::from_dollars(3_000);  // 8d
        input.nontx_medicaid_waiver_pymt_amt = Usd::from_dollars(2_000); // 8s
        input.certain_penal_instn_wages_amt = Usd::from_dollars(1_000); // 8u

        let form = OutputSchedule1::try_new(input).unwrap();

        // Line 9: 10,000 − 3,000 − 2,000 − 1,000 = 4,000
        assert_eq!(form.other_income_total_amt, Usd::from_dollars(4_000));
        assert!(form.is_valid());
    }

    #[test]
    fn negative_entries_can_make_line9_negative() {
        let mut input = empty_input();
        input.gambling_reportable_winning_amt = Usd::from_dollars(1_000);
        input.total_income_exclusion_amt = Usd::from_dollars(5_000); // 8d

        let form = OutputSchedule1::try_new(input).unwrap();

        // Line 9: 1,000 − 5,000 = −4,000
        assert_eq!(form.other_income_total_amt, Usd::from_dollars(-4_000));
        // Line 10: 0 + (−4,000) = −4,000
        assert_eq!(
            form.total_additional_income_amt,
            Usd::from_dollars(-4_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn basic_adjustments_lines_11_through_23() {
        let mut input = empty_input();
        input.educator_expenses_amt = Usd::from_dollars(300);
        input.health_savings_account_ded_amt = Usd::from_dollars(4_150);
        input.deductible_self_employment_tax_amt = Usd::from_dollars(7_065);
        input.ira_deduction_amt = Usd::from_dollars(7_000);
        input.student_loan_interest_ded_amt = Usd::from_dollars(2_500);

        let form = OutputSchedule1::try_new(input).unwrap();

        assert_eq!(form.total_other_adjustments_amt, Usd::ZERO);
        // Line 26: 300 + 4,150 + 7,065 + 7,000 + 2,500 = 21,015
        assert_eq!(form.total_adjustments_amt, Usd::from_dollars(21_015));
        assert!(form.is_valid());
    }

    #[test]
    fn other_adjustments_line25() {
        let mut input = empty_input();
        input.jury_duty_pay_deduction_amt = Usd::from_dollars(200);
        input.atty_fees_crt_costs_ded_amt = Usd::from_dollars(5_000);
        input.housing_deduction_amt = Usd::from_dollars(3_000);

        let form = OutputSchedule1::try_new(input).unwrap();

        // Line 25: 200 + 5,000 + 3,000 = 8,200
        assert_eq!(
            form.total_other_adjustments_amt,
            Usd::from_dollars(8_200)
        );
        // Line 26: 0 (lines 11–23) + 8,200 = 8,200
        assert_eq!(form.total_adjustments_amt, Usd::from_dollars(8_200));
        assert!(form.is_valid());
    }

    #[test]
    fn combined_income_and_adjustments() {
        let mut input = empty_input();
        input.business_income_loss_amt = Usd::from_dollars(80_000);
        input.gambling_reportable_winning_amt = Usd::from_dollars(5_000);
        input.deductible_self_employment_tax_amt = Usd::from_dollars(6_120);
        input.self_empld_health_ins_ded_amt = Usd::from_dollars(8_400);

        let form = OutputSchedule1::try_new(input).unwrap();

        // Line 9: 5,000
        assert_eq!(form.other_income_total_amt, Usd::from_dollars(5_000));
        // Line 10: 80,000 + 5,000 = 85,000
        assert_eq!(
            form.total_additional_income_amt,
            Usd::from_dollars(85_000)
        );
        // Line 26: 6,120 + 8,400 = 14,520
        assert_eq!(form.total_adjustments_amt, Usd::from_dollars(14_520));
        assert!(form.is_valid());
    }

    #[test]
    fn all_8_series_lines() {
        let mut input = empty_input();
        input.net_operating_loss_deduction_amt = Usd::from_dollars(100);      // 8a
        input.gambling_reportable_winning_amt = Usd::from_dollars(200);       // 8b
        input.debt_cancellation_amt = Usd::from_dollars(300);                 // 8c
        input.total_income_exclusion_amt = Usd::from_dollars(50);             // 8d (−)
        input.tot_archer_msa_medcr_ltc_amt = Usd::from_dollars(400);         // 8e
        input.tot_hsa_distri_hdhp_amt = Usd::from_dollars(500);              // 8f
        input.alaska_permanent_fund_div_amt = Usd::from_dollars(600);         // 8g
        input.jury_duty_pay_amt = Usd::from_dollars(700);                     // 8h
        input.prizes_awards_amt = Usd::from_dollars(800);                     // 8i
        input.activity_not_for_profit_incm_amt = Usd::from_dollars(900);      // 8j
        input.stock_options_amt = Usd::from_dollars(1_000);                   // 8k
        input.rental_income_personal_prop_amt = Usd::from_dollars(1_100);     // 8l
        input.olympic_paralympic_medal_usoc_amt = Usd::from_dollars(1_200);   // 8m
        input.section_951a_inclusion_amt = Usd::from_dollars(1_300);          // 8n
        input.section_951_aa_inclusion_amt = Usd::from_dollars(1_400);        // 8o
        input.excess_business_loss_amt = Usd::from_dollars(1_500);            // 8p
        input.taxable_able_distributions_amt = Usd::from_dollars(1_600);      // 8q
        input.grants_or_scholarships_amt = Usd::from_dollars(1_700);          // 8r
        input.nontx_medicaid_waiver_pymt_amt = Usd::from_dollars(30);         // 8s (−)
        input.nonqlfy_deferred_compensation_amt = Usd::from_dollars(1_800);   // 8t
        input.certain_penal_instn_wages_amt = Usd::from_dollars(20);          // 8u (−)
        input.digital_assets_amt = Usd::from_dollars(1_900);                  // 8v
        input.total_other_income_amt = Usd::from_dollars(2_000);              // 8z

        let form = OutputSchedule1::try_new(input).unwrap();

        // Positive: 100+200+300+400+500+600+700+800+900+1000+1100+1200
        //           +1300+1400+1500+1600+1700+1800+1900+2000 = 21,000
        // Negative: 50+30+20 = 100
        // Line 9: 21,000 − 100 = 20,900
        assert_eq!(form.other_income_total_amt, Usd::from_dollars(20_900));
        assert!(form.is_valid());
    }

    #[test]
    fn all_24_series_lines() {
        let mut input = empty_input();
        input.jury_duty_pay_deduction_amt = Usd::from_dollars(100);           // 24a
        input.rntl_incm_prsnl_prop_expnss_ded_amt = Usd::from_dollars(200);  // 24b
        input.olympc_prlympc_medal_usoc_ded_amt = Usd::from_dollars(300);     // 24c
        input.rfor_amortz_expnss_ded_amt = Usd::from_dollars(400);            // 24d
        input.repayment_supp_unempl_bnft_ded_amt = Usd::from_dollars(500);    // 24e
        input.sect_501c18d_contri_ded_amt = Usd::from_dollars(600);            // 24f
        input.section_403b_contri_ded_amt = Usd::from_dollars(700);            // 24g
        input.atty_fees_crt_costs_ded_amt = Usd::from_dollars(800);            // 24h
        input.atty_fees_crt_costs_pd_ded_amt = Usd::from_dollars(900);         // 24i
        input.housing_deduction_amt = Usd::from_dollars(1_000);                // 24j
        input.section_67e_excess_deduction_amt = Usd::from_dollars(1_100);     // 24k
        input.other_adjustments_total_amt = Usd::from_dollars(1_200);          // 24z

        let form = OutputSchedule1::try_new(input).unwrap();

        // Line 25: 100+200+300+400+500+600+700+800+900+1000+1100+1200 = 7,800
        assert_eq!(
            form.total_other_adjustments_amt,
            Usd::from_dollars(7_800)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn pass_through_fields_preserved() {
        let mut input = empty_input();
        input.form_1099k_rpt_error_or_loss_amt = Usd::from_dollars(500);
        input.form_4797_ind = true;
        input.form_4684_ind = true;
        input.repaid_overpayment_ind = true;
        input.repayment_amt = Usd::from_dollars(200);
        input.claim_storage_fees_ind = true;
        input.mfs_live_apart_entire_yr_ind = true;

        let form = OutputSchedule1::try_new(input).unwrap();

        assert_eq!(
            form.form_1099k_rpt_error_or_loss_amt,
            Usd::from_dollars(500)
        );
        assert!(form.form_4797_ind);
        assert!(form.form_4684_ind);
        assert!(form.repaid_overpayment_ind);
        assert_eq!(form.repayment_amt, Usd::from_dollars(200));
        assert!(form.claim_storage_fees_ind);
        assert!(form.mfs_live_apart_entire_yr_ind);
        assert!(form.is_valid());
    }
}
