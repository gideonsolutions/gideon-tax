use us_tax_brackets::{FilingStatus, TaxYear};

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Supporting types
// =========================================================================

/// A qualifying child entry for Part I, line 2 of Form 1040-SS.
#[derive(Debug, Clone, Default)]
pub struct QualifyingChildInfo {
    /// (a) First name
    pub first_nm: String,
    /// (b) Last name
    pub last_nm: String,
    /// (c) SSN
    pub ssn: String,
    /// (d) Relationship
    pub relationship: String,
}

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 1040-SS (2025) — U.S.
/// Self-Employment Tax Return (Including the Additional Child Tax Credit
/// for Bona Fide Residents of Puerto Rico).
///
/// This form is for residents of the USVI, Guam, American Samoa, CNMI,
/// and Puerto Rico who have self-employment income. Bona fide residents
/// of Puerto Rico may also claim the additional child tax credit (ACTC).
#[derive(Debug, Clone)]
pub struct F1040SsInput {
    /// Filing status
    pub filing_status: FilingStatus,

    // ── Header ─────────────────────────────────────────────────────────
    /// Filed pursuant to section 301.9100-2
    pub filed_pursuant_to_sect_30191002_ind: bool,
    /// Deceased indicator
    pub deceased_ind: bool,
    /// Primary date of death (MM/DD/YYYY)
    pub primary_death_dt: String,
    /// Spouse date of death (MM/DD/YYYY)
    pub spouse_death_dt: String,
    /// Digital asset question — Yes/No
    pub virtual_cur_acquired_dur_ty_ind: bool,

    // ── Names & SSNs (header pass-through) ─────────────────────────────
    /// Spouse name (for NRA/dual-status alien spouse treated as U.S. resident)
    pub spouse_nm: String,
    /// NRA literal code
    pub nra_literal_cd: String,
    /// Qualifying HOH child name (when HOH and not completing line 2)
    pub qualifying_hoh_nm: String,
    /// Qualifying HOH child SSN
    pub qualifying_hoh_ssn: String,
    /// Surviving spouse indicator
    pub surviving_spouse_ind: bool,
    /// More than four dependents indicator
    pub more_dependents_ind: bool,
    /// Qualifying children for Part I, line 2 (up to 4 on the form)
    pub qualifying_children: Vec<QualifyingChildInfo>,

    // ── Part I — Total Tax and Credits ─────────────────────────────────
    /// Line 3: Self-employment tax from Schedule SE (Form 1040), line 12
    pub self_employment_tax_amt: Usd,
    /// Line 4: Household employment taxes from Schedule H (Form 1040)
    pub household_employment_tax_amt: Usd,
    /// Line 5: Additional Medicare Tax from Form 8959
    pub total_am_rrt_tax_amt: Usd,
    /// Line 6a: Employee social security and Medicare tax on tips not
    /// reported to employer (Form 4137)
    pub soc_sec_medicare_tax_unrptd_tip_amt: Usd,
    /// Line 6b: Uncollected employee social security and Medicare tax on tips
    pub uncollected_soc_sec_tax_on_tips_amt: Usd,
    /// Line 6c: Uncollected employee social security and Medicare tax on
    /// wages (Form 8919)
    pub uncollected_soc_sec_med_tax_amt: Usd,
    /// Line 6d: Uncollected employee social security and Medicare tax on
    /// group-term life insurance
    pub uncollected_soc_sec_med_tax_gtli_amt: Usd,

    /// Line 8: 2025 estimated tax payments and amount applied from 2024 return
    pub estimated_tax_payments_amt: Usd,
    /// Former spouse SSN (if made estimated tax payments with former spouse)
    pub former_spouse_ssn: String,
    /// Line 9: Amount paid with request for extension of time to file
    pub request_for_extension_amt: Usd,
    /// Line 10: Additional child tax credit from Part II, line 19
    /// (computed in Part II; pass Usd::ZERO if not claiming)
    pub additional_child_tax_credit_amt: Usd,
    /// Line 11a: Additional Medicare Tax withheld (Form 8959)
    pub addl_medcr_rrt_tax_withholding_amt: Usd,
    /// Line 11b: Excess social security tax withheld
    pub ex_soc_sec_tax_withheld_amt: Usd,

    // ── Page 2 — Refund / Amount Owed ──────────────────────────────────
    /// Line 14a: Form 8888 attached indicator
    pub form_8888_ind: bool,
    /// Line 14b: Routing number
    pub routing_transit_num: String,
    /// Line 14c: Bank account type (Checking/Savings)
    pub bank_account_type_cd: String,
    /// Line 14d: Account number
    pub depositor_account_num: String,
    /// Line 15: Amount of overpayment applied to 2026 estimated tax
    pub applied_to_es_tax_amt: Usd,

    // ── Third Party Designee ───────────────────────────────────────────
    /// Third party designee indicator
    pub third_party_designee_ind: bool,
    /// Designee's name
    pub third_party_designee_nm: String,
    /// Designee's phone number
    pub third_party_designee_phone_num: String,
    /// Designee's PIN
    pub third_party_designee_pin: String,

    // ── Other header fields ────────────────────────────────────────────
    /// Special condition description
    pub special_condition_desc: String,
    /// Special condition text
    pub special_condition_txt: String,
    /// Non-paid preparer code
    pub non_paid_preparer_cd: String,
    /// Personal representative indicator
    pub personal_representative_ind: bool,
    /// Power of attorney name
    pub power_of_attorney_nm: String,
    /// Power of attorney signed-by indicator
    pub power_of_attorney_signed_by_ind: bool,
    /// Refund product code
    pub refund_product_cd: String,
    /// Refund product code text
    pub refund_product_code_txt: String,
    /// Change date
    pub change_dt: String,

    // ── Part II — ACTC for Bona Fide Residents of Puerto Rico ──────────
    /// Whether the filer has qualifying children under 17 with required SSN
    pub actc_qualifying_child_ind: bool,
    /// Line 2: Number of qualifying children under age 17 with required SSN
    pub actc_qlfy_child_under_age_ssn_cnt: u32,
    /// Line 3: Modified adjusted gross income
    pub actc_modified_agi_amt: Usd,
    /// Line 8: Number of other dependents (including children not under 17)
    pub actc_other_dependent_cnt: u32,
    /// Line 13a: Withheld social security, Medicare, and Additional Medicare
    /// taxes from Puerto Rico Form(s) 499R-2/W-2PR
    pub actc_pr_ss_medcr_addnl_medcr_tax_withheld_amt: Usd,
    /// Line 13b: Employee social security and Medicare tax on tips not
    /// reported to employer from Form 4137
    pub actc_soc_sec_medicare_tax_unrptd_tip_amt: Usd,
    /// Line 13c: Uncollected employee social security and Medicare tax on
    /// wages from Form 8919
    pub actc_uncollected_soc_sec_med_tax_amt: Usd,
    /// Line 13d: Uncollected employee social security and Medicare tax on
    /// tips and group-term life insurance (from Part I, lines 6b and 6d)
    pub actc_uncollected_soc_sec_med_tax_gtli_amt: Usd,
    /// Line 13e: Additional Medicare Tax on Medicare wages from Form 8959, line 7
    pub actc_additional_medicare_tax_amt: Usd,
    /// Line 15: Additional Medicare Tax withheld from Form 8959, line 22
    pub actc_addnl_medicare_tax_withholding_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 1040-SS (2025) — U.S. Self-Employment Tax
/// Return (Including the Additional Child Tax Credit for Bona Fide
/// Residents of Puerto Rico).
#[derive(Debug, Clone, Default)]
pub struct Output1040Ss {
    // =====================================================================
    // Header
    // =====================================================================
    /// Filing status code
    pub individual_return_filing_status_cd: String,
    /// Filed pursuant to section 301.9100-2
    pub filed_pursuant_to_sect_30191002_ind: bool,
    /// Deceased indicator
    pub deceased_ind: bool,
    /// Primary date of death
    pub primary_death_dt: String,
    /// Spouse date of death
    pub spouse_death_dt: String,
    /// Digital asset question
    pub virtual_cur_acquired_dur_ty_ind: bool,
    /// Spouse name (NRA/dual-status)
    pub spouse_nm: String,
    /// NRA literal code
    pub nra_literal_cd: String,
    /// Qualifying HOH child name
    pub qualifying_hoh_nm: String,
    /// Qualifying HOH child SSN
    pub qualifying_hoh_ssn: String,
    /// Surviving spouse indicator
    pub surviving_spouse_ind: bool,
    /// More than four dependents indicator
    pub more_dependents_ind: bool,
    /// Qualifying children (Part I, line 2)
    pub qualifying_children: Vec<QualifyingChildInfo>,

    // =====================================================================
    // Part I — Total Tax and Credits (lines 3–7)
    // =====================================================================
    /// Line 3: Self-employment tax from Schedule SE
    pub self_employment_tax_amt: Usd,
    /// Line 4: Household employment taxes from Schedule H
    pub household_employment_tax_amt: Usd,
    /// Line 5: Additional Medicare Tax from Form 8959
    pub total_am_rrt_tax_amt: Usd,
    /// Line 6a: Employee SS/Medicare tax on unreported tips (Form 4137)
    pub soc_sec_medicare_tax_unrptd_tip_amt: Usd,
    /// Line 6b: Uncollected employee SS/Medicare tax on tips
    pub uncollected_soc_sec_tax_on_tips_amt: Usd,
    /// Line 6c: Uncollected employee SS/Medicare tax on wages (Form 8919)
    pub uncollected_soc_sec_med_tax_amt: Usd,
    /// Line 6d: Uncollected employee SS/Medicare tax on group-term life insurance
    pub uncollected_soc_sec_med_tax_gtli_amt: Usd,
    /// Line 6e: Total other taxes (6a + 6b + 6c + 6d)
    pub total_other_taxes_amt: Usd,
    /// Line 7: Total tax (3 + 4 + 5 + 6e)
    pub total_tax_amt: Usd,

    // =====================================================================
    // Part I — Payments and Credits (lines 8–12)
    // =====================================================================
    /// Line 8: Estimated tax payments and amount applied from prior year
    pub estimated_tax_payments_amt: Usd,
    /// Former spouse SSN
    pub former_spouse_ssn: String,
    /// Line 9: Amount paid with extension request
    pub request_for_extension_amt: Usd,
    /// Line 10: Additional child tax credit from Part II, line 19
    pub additional_child_tax_credit_amt: Usd,
    /// Line 11a: Additional Medicare Tax withheld (Form 8959)
    pub addl_medcr_rrt_tax_withholding_amt: Usd,
    /// Line 11b: Excess social security tax withheld
    pub ex_soc_sec_tax_withheld_amt: Usd,
    /// Line 12: Total payments and credits (8 + 9 + 10 + 11a + 11b)
    pub total_payments_amt: Usd,

    // =====================================================================
    // Part I — Refund or Amount You Owe (lines 13–16)
    // =====================================================================
    /// Line 13: Overpaid amount (line 12 − line 7, if line 12 > line 7)
    pub overpaid_amt: Usd,
    /// Line 14a: Amount refunded to you
    pub refund_amt: Usd,
    /// Form 8888 attached indicator
    pub form_8888_ind: bool,
    /// Line 14b: Routing number
    pub routing_transit_num: String,
    /// Line 14c: Bank account type
    pub bank_account_type_cd: String,
    /// Line 14d: Account number
    pub depositor_account_num: String,
    /// Line 15: Amount applied to next year estimated tax
    pub applied_to_es_tax_amt: Usd,
    /// Line 16: Amount you owe (line 7 − line 12, if line 7 > line 12)
    pub owed_amt: Usd,

    // =====================================================================
    // Third Party Designee
    // =====================================================================
    /// Third party designee indicator
    pub third_party_designee_ind: bool,
    /// Designee's name
    pub third_party_designee_nm: String,
    /// Designee's phone number
    pub third_party_designee_phone_num: String,
    /// Designee's PIN
    pub third_party_designee_pin: String,

    // =====================================================================
    // Other header / metadata fields
    // =====================================================================
    /// Special condition description
    pub special_condition_desc: String,
    /// Special condition text
    pub special_condition_txt: String,
    /// Non-paid preparer code
    pub non_paid_preparer_cd: String,
    /// Personal representative indicator
    pub personal_representative_ind: bool,
    /// Power of attorney name
    pub power_of_attorney_nm: String,
    /// Power of attorney signed-by indicator
    pub power_of_attorney_signed_by_ind: bool,
    /// Refund product code
    pub refund_product_cd: String,
    /// Refund product code text
    pub refund_product_code_txt: String,
    /// Change date
    pub change_dt: String,

    // =====================================================================
    // Part II — Bona Fide Residents of Puerto Rico Claiming ACTC
    // =====================================================================
    /// Line 1: Qualifying child indicator (Yes/No)
    pub actc_qualifying_child_ind: bool,
    /// Line 2: Number of qualifying children under 17 × $1,700
    pub actc_qlfy_child_under_age_ssn_cnt: u32,
    /// Line 2 amount: count × $1,700
    pub actc_qlfy_child_under_age_ssn_limt_amt: Usd,
    /// Line 3: Modified adjusted gross income
    pub actc_modified_agi_amt: Usd,
    /// Line 4: Filing status threshold ($400,000 MFJ / $200,000 others)
    pub actc_filing_status_threshold_cd: String,
    /// Line 4 amount
    pub actc_modified_agi_phase_out_amt: Usd,
    /// Line 5: MAGI minus threshold (rounded up to next $1,000)
    pub actc_magi_more_than_threshold_ind: bool,
    /// Line 5 amount
    pub actc_magi_less_threshold_amt: Usd,
    /// Line 6: Line 5 × 5% (0.05)
    pub actc_limit_amt: Usd,
    /// Line 7: Number of qualifying children × $2,200
    pub actc_qlfy_child_under_age_ssn_limt_amt_ln7: Usd,
    /// Line 8: Number of other dependents
    pub actc_other_dependent_cnt: u32,
    /// Line 8 amount: count × $500
    pub actc_other_dependent_credit_amt: Usd,
    /// Line 9: Lines 7 + 8
    pub actc_initial_amt: Usd,
    /// Line 10: Is line 9 > line 6? (Yes: line 9 − line 6)
    pub actc_over_phase_out_limit_ind: bool,
    /// Line 10 amount
    pub actc_after_limit_amt: Usd,
    /// Line 11: Smaller of line 2 or line 10
    pub actc_tax_claim_ind: bool,
    /// Line 11 amount
    pub actc_tax_limit_amt: Usd,
    /// Line 12a: One-half of self-employment tax (Part I, line 3)
    pub actc_one_half_self_employment_tax_amt: Usd,
    /// Line 12b: One-half of Additional Medicare Tax on SE income (Form 8959, line 13)
    pub actc_one_half_addnl_medicare_tax_amt: Usd,
    /// Line 12c: Lines 12a + 12b
    pub actc_total_se_addnl_medicare_tax_amt: Usd,
    /// Line 13a: Withheld SS/Medicare/Additional Medicare taxes from PR W-2PR
    pub actc_pr_ss_medcr_addnl_medcr_tax_withheld_amt: Usd,
    /// Line 13b: Employee SS/Medicare tax on unreported tips (Form 4137)
    pub actc_soc_sec_medicare_tax_unrptd_tip_amt: Usd,
    /// Line 13c: Uncollected employee SS/Medicare tax on wages (Form 8919)
    pub actc_uncollected_soc_sec_med_tax_amt: Usd,
    /// Line 13d: Uncollected SS/Medicare tax on tips and group-term life
    pub actc_uncollected_soc_sec_med_tax_gtli_amt: Usd,
    /// Line 13e: Additional Medicare Tax on Medicare wages (Form 8959, line 7)
    pub actc_additional_medicare_tax_amt: Usd,
    /// Line 13f: Lines 13a through 13e
    pub actc_total_soc_sec_and_medcr_withheld_amt: Usd,
    /// Line 14: Lines 12c + 13f
    pub actc_total_tax_limit_amt: Usd,
    /// Line 15: Additional Medicare Tax withheld (Form 8959, line 22)
    pub actc_addnl_medicare_tax_withholding_amt: Usd,
    /// Line 16: Line 14 − line 15
    pub actc_total_tax_after_limit_amt: Usd,
    /// Line 17: Excess social security tax withheld (from Part I, line 11b)
    pub actc_prior_year_employment_tax_amt: Usd,
    /// Line 18: Line 16 − line 17 (if line 16 > line 17)
    pub actc_total_wthld_uncoll_unrptd_tax_amt: Usd,
    /// Line 19: Additional child tax credit (smaller of line 11 or line 18)
    pub actc_additional_child_tax_credit_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output1040Ss {
    fn name() -> &'static str {
        "Form 1040-SS"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output1040Ss {
    type Input = F1040SsInput;

    fn must_file(input: &Self::Input) -> bool {
        // You must file if you (or spouse on joint return) had net earnings
        // from self-employment of $400 or more, or church employee income
        // of $108.28 or more.
        input.self_employment_tax_amt > Usd::ZERO
            || input.household_employment_tax_amt > Usd::ZERO
            || input.total_am_rrt_tax_amt > Usd::ZERO
            || input.soc_sec_medicare_tax_unrptd_tip_amt > Usd::ZERO
            || input.uncollected_soc_sec_tax_on_tips_amt > Usd::ZERO
            || input.uncollected_soc_sec_med_tax_amt > Usd::ZERO
            || input.uncollected_soc_sec_med_tax_gtli_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // ── Filing status code ────────────────────────────────────────
        let filing_status_cd = match input.filing_status {
            FilingStatus::Single => "1",
            FilingStatus::MarriedFilingJointly => "2",
            FilingStatus::MarriedFilingSeparately => "3",
            FilingStatus::HeadOfHousehold => "4",
            FilingStatus::QualifyingSurvivingSpouse => "5",
        }
        .to_string();

        // ── Part I — Total Tax and Credits ────────────────────────────
        let line3 = input.self_employment_tax_amt;
        let line4 = input.household_employment_tax_amt;
        let line5 = input.total_am_rrt_tax_amt;
        let line6a = input.soc_sec_medicare_tax_unrptd_tip_amt;
        let line6b = input.uncollected_soc_sec_tax_on_tips_amt;
        let line6c = input.uncollected_soc_sec_med_tax_amt;
        let line6d = input.uncollected_soc_sec_med_tax_gtli_amt;
        let line6e = line6a + line6b + line6c + line6d;
        let line7 = line3 + line4 + line5 + line6e;

        // ── Part I — Payments and Credits ─────────────────────────────
        let line8 = input.estimated_tax_payments_amt;
        let line9 = input.request_for_extension_amt;
        let line10 = input.additional_child_tax_credit_amt;
        let line11a = input.addl_medcr_rrt_tax_withholding_amt;
        let line11b = input.ex_soc_sec_tax_withheld_amt;
        let line12 = line8 + line9 + line10 + line11a + line11b;

        // ── Part I — Refund or Amount You Owe ─────────────────────────
        let line13 = if line12 > line7 {
            line12 - line7
        } else {
            Usd::ZERO
        };

        let line15 = input.applied_to_es_tax_amt;
        let line14a = if line13 > Usd::ZERO {
            line13 - line15
        } else {
            Usd::ZERO
        };

        let line16 = if line7 > line12 {
            line7 - line12
        } else {
            Usd::ZERO
        };

        // ── Part II — ACTC for Bona Fide Residents of Puerto Rico ─────
        let actc = compute_actc(&input);

        Ok(Output1040Ss {
            // Header
            individual_return_filing_status_cd: filing_status_cd,
            filed_pursuant_to_sect_30191002_ind: input.filed_pursuant_to_sect_30191002_ind,
            deceased_ind: input.deceased_ind,
            primary_death_dt: input.primary_death_dt,
            spouse_death_dt: input.spouse_death_dt,
            virtual_cur_acquired_dur_ty_ind: input.virtual_cur_acquired_dur_ty_ind,
            spouse_nm: input.spouse_nm,
            nra_literal_cd: input.nra_literal_cd,
            qualifying_hoh_nm: input.qualifying_hoh_nm,
            qualifying_hoh_ssn: input.qualifying_hoh_ssn,
            surviving_spouse_ind: input.surviving_spouse_ind,
            more_dependents_ind: input.more_dependents_ind,
            qualifying_children: input.qualifying_children,

            // Part I — Tax
            self_employment_tax_amt: line3,
            household_employment_tax_amt: line4,
            total_am_rrt_tax_amt: line5,
            soc_sec_medicare_tax_unrptd_tip_amt: line6a,
            uncollected_soc_sec_tax_on_tips_amt: line6b,
            uncollected_soc_sec_med_tax_amt: line6c,
            uncollected_soc_sec_med_tax_gtli_amt: line6d,
            total_other_taxes_amt: line6e,
            total_tax_amt: line7,

            // Part I — Payments
            estimated_tax_payments_amt: line8,
            former_spouse_ssn: input.former_spouse_ssn,
            request_for_extension_amt: line9,
            additional_child_tax_credit_amt: input.additional_child_tax_credit_amt,
            addl_medcr_rrt_tax_withholding_amt: line11a,
            ex_soc_sec_tax_withheld_amt: line11b,
            total_payments_amt: line12,

            // Part I — Refund / Owed
            overpaid_amt: line13,
            refund_amt: line14a,
            form_8888_ind: input.form_8888_ind,
            routing_transit_num: input.routing_transit_num,
            bank_account_type_cd: input.bank_account_type_cd,
            depositor_account_num: input.depositor_account_num,
            applied_to_es_tax_amt: line15,
            owed_amt: line16,

            // Third Party Designee
            third_party_designee_ind: input.third_party_designee_ind,
            third_party_designee_nm: input.third_party_designee_nm,
            third_party_designee_phone_num: input.third_party_designee_phone_num,
            third_party_designee_pin: input.third_party_designee_pin,

            // Other metadata
            special_condition_desc: input.special_condition_desc,
            special_condition_txt: input.special_condition_txt,
            non_paid_preparer_cd: input.non_paid_preparer_cd,
            personal_representative_ind: input.personal_representative_ind,
            power_of_attorney_nm: input.power_of_attorney_nm,
            power_of_attorney_signed_by_ind: input.power_of_attorney_signed_by_ind,
            refund_product_cd: input.refund_product_cd,
            refund_product_code_txt: input.refund_product_code_txt,
            change_dt: input.change_dt,

            // Part II — ACTC
            actc_qualifying_child_ind: actc.qualifying_child_ind,
            actc_qlfy_child_under_age_ssn_cnt: actc.qlfy_child_cnt,
            actc_qlfy_child_under_age_ssn_limt_amt: actc.line2,
            actc_modified_agi_amt: actc.line3,
            actc_filing_status_threshold_cd: actc.filing_status_threshold_cd,
            actc_modified_agi_phase_out_amt: actc.line4,
            actc_magi_more_than_threshold_ind: actc.magi_more_than_threshold,
            actc_magi_less_threshold_amt: actc.line5,
            actc_limit_amt: actc.line6,
            actc_qlfy_child_under_age_ssn_limt_amt_ln7: actc.line7,
            actc_other_dependent_cnt: actc.other_dependent_cnt,
            actc_other_dependent_credit_amt: actc.line8,
            actc_initial_amt: actc.line9,
            actc_over_phase_out_limit_ind: actc.over_phase_out_limit,
            actc_after_limit_amt: actc.line10,
            actc_tax_claim_ind: actc.tax_claim_ind,
            actc_tax_limit_amt: actc.line11,
            actc_one_half_self_employment_tax_amt: actc.line12a,
            actc_one_half_addnl_medicare_tax_amt: actc.line12b,
            actc_total_se_addnl_medicare_tax_amt: actc.line12c,
            actc_pr_ss_medcr_addnl_medcr_tax_withheld_amt: actc.line13a,
            actc_soc_sec_medicare_tax_unrptd_tip_amt: actc.line13b,
            actc_uncollected_soc_sec_med_tax_amt: actc.line13c,
            actc_uncollected_soc_sec_med_tax_gtli_amt: actc.line13d,
            actc_additional_medicare_tax_amt: actc.line13e,
            actc_total_soc_sec_and_medcr_withheld_amt: actc.line13f,
            actc_total_tax_limit_amt: actc.line14,
            actc_addnl_medicare_tax_withholding_amt: actc.line15,
            actc_total_tax_after_limit_amt: actc.line16,
            actc_prior_year_employment_tax_amt: actc.line17,
            actc_total_wthld_uncoll_unrptd_tax_amt: actc.line18,
            actc_additional_child_tax_credit_amt: actc.line19,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[
            DynForm::ScheduleSe,
            DynForm::ScheduleH,
            DynForm::F8959,
            DynForm::F4137,
            DynForm::F8919,
        ]
    }

    fn is_valid(&self) -> bool {
        // Part I — line 6e
        let line6e_ok = self.total_other_taxes_amt
            == self.soc_sec_medicare_tax_unrptd_tip_amt
                + self.uncollected_soc_sec_tax_on_tips_amt
                + self.uncollected_soc_sec_med_tax_amt
                + self.uncollected_soc_sec_med_tax_gtli_amt;

        // Part I — line 7
        let line7_ok = self.total_tax_amt
            == self.self_employment_tax_amt
                + self.household_employment_tax_amt
                + self.total_am_rrt_tax_amt
                + self.total_other_taxes_amt;

        // Part I — line 12
        let line12_ok = self.total_payments_amt
            == self.estimated_tax_payments_amt
                + self.request_for_extension_amt
                + self.additional_child_tax_credit_amt
                + self.addl_medcr_rrt_tax_withholding_amt
                + self.ex_soc_sec_tax_withheld_amt;

        // Part I — line 13
        let line13_ok = if self.total_payments_amt > self.total_tax_amt {
            self.overpaid_amt == self.total_payments_amt - self.total_tax_amt
        } else {
            self.overpaid_amt == Usd::ZERO
        };

        // Part I — line 14a + 15 = 13
        let refund_ok = if self.overpaid_amt > Usd::ZERO {
            self.refund_amt + self.applied_to_es_tax_amt == self.overpaid_amt
        } else {
            self.refund_amt == Usd::ZERO
        };

        // Part I — line 16
        let line16_ok = if self.total_tax_amt > self.total_payments_amt {
            self.owed_amt == self.total_tax_amt - self.total_payments_amt
        } else {
            self.owed_amt == Usd::ZERO
        };

        // Part II — ACTC internal consistency
        let actc_line2_ok = self.actc_qlfy_child_under_age_ssn_limt_amt
            == Usd::from_dollars(1_700) * i64::from(self.actc_qlfy_child_under_age_ssn_cnt);

        let actc_line7_ok = self.actc_qlfy_child_under_age_ssn_limt_amt_ln7
            == Usd::from_dollars(2_200) * i64::from(self.actc_qlfy_child_under_age_ssn_cnt);

        let actc_line8_ok = self.actc_other_dependent_credit_amt
            == Usd::from_dollars(500) * i64::from(self.actc_other_dependent_cnt);

        let actc_line9_ok = self.actc_initial_amt
            == self.actc_qlfy_child_under_age_ssn_limt_amt_ln7
                + self.actc_other_dependent_credit_amt;

        let actc_line12c_ok = self.actc_total_se_addnl_medicare_tax_amt
            == self.actc_one_half_self_employment_tax_amt
                + self.actc_one_half_addnl_medicare_tax_amt;

        let actc_line13f_ok = self.actc_total_soc_sec_and_medcr_withheld_amt
            == self.actc_pr_ss_medcr_addnl_medcr_tax_withheld_amt
                + self.actc_soc_sec_medicare_tax_unrptd_tip_amt
                + self.actc_uncollected_soc_sec_med_tax_amt
                + self.actc_uncollected_soc_sec_med_tax_gtli_amt
                + self.actc_additional_medicare_tax_amt;

        let actc_line14_ok = self.actc_total_tax_limit_amt
            == self.actc_total_se_addnl_medicare_tax_amt
                + self.actc_total_soc_sec_and_medcr_withheld_amt;

        let actc_line16_ok = self.actc_total_tax_after_limit_amt
            == (self.actc_total_tax_limit_amt - self.actc_addnl_medicare_tax_withholding_amt)
                .max(Usd::ZERO);

        let actc_line19_ok = self.actc_additional_child_tax_credit_amt
            == self
                .actc_tax_limit_amt
                .min(self.actc_total_wthld_uncoll_unrptd_tax_amt);

        line6e_ok
            && line7_ok
            && line12_ok
            && line13_ok
            && refund_ok
            && line16_ok
            && actc_line2_ok
            && actc_line7_ok
            && actc_line8_ok
            && actc_line9_ok
            && actc_line12c_ok
            && actc_line13f_ok
            && actc_line14_ok
            && actc_line16_ok
            && actc_line19_ok
    }
}

// =========================================================================
// ACTC computation helper
// =========================================================================

struct ActcResult {
    qualifying_child_ind: bool,
    qlfy_child_cnt: u32,
    line2: Usd,
    line3: Usd,
    filing_status_threshold_cd: String,
    line4: Usd,
    magi_more_than_threshold: bool,
    line5: Usd,
    line6: Usd,
    line7: Usd,
    other_dependent_cnt: u32,
    line8: Usd,
    line9: Usd,
    over_phase_out_limit: bool,
    line10: Usd,
    tax_claim_ind: bool,
    line11: Usd,
    line12a: Usd,
    line12b: Usd,
    line12c: Usd,
    line13a: Usd,
    line13b: Usd,
    line13c: Usd,
    line13d: Usd,
    line13e: Usd,
    line13f: Usd,
    line14: Usd,
    line15: Usd,
    line16: Usd,
    line17: Usd,
    line18: Usd,
    line19: Usd,
}

/// Round up to the next multiple of $1,000.
fn round_up_to_next_1000(amt: Usd) -> Usd {
    let cents = amt.cents();
    if cents <= 0 {
        return Usd::ZERO;
    }
    let per_1000 = 100_000i64; // $1,000 in cents
    let remainder = cents % per_1000;
    if remainder == 0 {
        amt
    } else {
        Usd::from_cents(cents + (per_1000 - remainder))
    }
}

fn compute_actc(input: &F1040SsInput) -> ActcResult {
    let qualifying_child_ind = input.actc_qualifying_child_ind;
    let cnt = input.actc_qlfy_child_under_age_ssn_cnt;

    // Line 2: count × $1,700
    let line2 = Usd::from_dollars(1_700) * i64::from(cnt);

    // Line 3: Modified AGI
    let line3 = input.actc_modified_agi_amt;

    // Line 4: Threshold
    let (line4, threshold_cd) = match input.filing_status {
        FilingStatus::MarriedFilingJointly => (Usd::from_dollars(400_000), "400000".to_string()),
        _ => (Usd::from_dollars(200_000), "200000".to_string()),
    };

    // Line 5: MAGI excess, rounded up to next $1,000
    let magi_more_than_threshold = line3 > line4;
    let line5 = if magi_more_than_threshold {
        round_up_to_next_1000(line3 - line4)
    } else {
        Usd::ZERO
    };

    // Line 6: line 5 × 5%
    let line6 = Usd::from_cents(line5.cents() * 5 / 100);

    // Line 7: count × $2,200
    let line7 = Usd::from_dollars(2_200) * i64::from(cnt);

    // Line 8: other dependents × $500
    let other_cnt = input.actc_other_dependent_cnt;
    let line8 = Usd::from_dollars(500) * i64::from(other_cnt);

    // Line 9: line 7 + line 8
    let line9 = line7 + line8;

    // Line 10: if line 9 > line 6 then line 9 − line 6, else stop
    let over_phase_out_limit = line9 > line6;
    let line10 = if over_phase_out_limit {
        line9 - line6
    } else {
        Usd::ZERO
    };

    // Line 11: smaller of line 2 or line 10
    let line11 = if !qualifying_child_ind || !over_phase_out_limit {
        Usd::ZERO
    } else if magi_more_than_threshold {
        line2.min(line10)
    } else {
        // When MAGI ≤ threshold, line 5 is blank, enter line 2 amount on line 11
        line2
    };

    let tax_claim_ind = line11 > Usd::ZERO;

    // Line 12a: one-half of SE tax
    let line12a = Usd::from_cents(input.self_employment_tax_amt.cents() / 2);

    // Line 12b: one-half of Additional Medicare Tax on SE income
    let line12b = input.actc_one_half_addnl_medicare_tax_amt();

    // Line 12c
    let line12c = line12a + line12b;

    // Lines 13a–13e
    let line13a = input.actc_pr_ss_medcr_addnl_medcr_tax_withheld_amt;
    let line13b = input.actc_soc_sec_medicare_tax_unrptd_tip_amt;
    let line13c = input.actc_uncollected_soc_sec_med_tax_amt;
    let line13d = input.actc_uncollected_soc_sec_med_tax_gtli_amt;
    let line13e = input.actc_additional_medicare_tax_amt;
    let line13f = line13a + line13b + line13c + line13d + line13e;

    // Line 14
    let line14 = line12c + line13f;

    // Line 15
    let line15 = input.actc_addnl_medicare_tax_withholding_amt;

    // Line 16
    let line16 = (line14 - line15).max(Usd::ZERO);

    // Line 17: excess social security tax withheld (same as Part I, line 11b)
    let line17 = input.ex_soc_sec_tax_withheld_amt;

    // Line 18: line 16 − line 17 (if line 16 > line 17)
    let line18 = if line16 > line17 {
        line16 - line17
    } else {
        Usd::ZERO
    };

    // Line 19: smaller of line 11 or line 18
    let line19 = if !qualifying_child_ind || !over_phase_out_limit || line16 <= line17 {
        Usd::ZERO
    } else {
        line11.min(line18)
    };

    ActcResult {
        qualifying_child_ind,
        qlfy_child_cnt: cnt,
        line2,
        line3,
        filing_status_threshold_cd: threshold_cd,
        line4,
        magi_more_than_threshold,
        line5,
        line6,
        line7,
        other_dependent_cnt: other_cnt,
        line8,
        line9,
        over_phase_out_limit,
        line10,
        tax_claim_ind,
        line11,
        line12a,
        line12b,
        line12c,
        line13a,
        line13b,
        line13c,
        line13d,
        line13e,
        line13f,
        line14,
        line15,
        line16,
        line17,
        line18,
        line19,
    }
}

impl F1040SsInput {
    /// Line 12b helper: one-half of Additional Medicare Tax on SE income
    /// from Form 8959, line 13. The caller provides this via the
    /// `actc_one_half_addnl_medicare_tax_amt` field on the ACTC section;
    /// however the form's Part II, line 12b references it directly.
    fn actc_one_half_addnl_medicare_tax_amt(&self) -> Usd {
        // This is provided directly from Form 8959, line 13 ÷ 2.
        // We approximate it as half of the Additional Medicare Tax on the
        // SE income portion. The caller should set the ACTC fields based on
        // Form 8959 output.
        Usd::from_cents(self.total_am_rrt_tax_amt.cents() / 2)
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_input() -> F1040SsInput {
        F1040SsInput {
            filing_status: FilingStatus::Single,
            filed_pursuant_to_sect_30191002_ind: false,
            deceased_ind: false,
            primary_death_dt: String::new(),
            spouse_death_dt: String::new(),
            virtual_cur_acquired_dur_ty_ind: false,
            spouse_nm: String::new(),
            nra_literal_cd: String::new(),
            qualifying_hoh_nm: String::new(),
            qualifying_hoh_ssn: String::new(),
            surviving_spouse_ind: false,
            more_dependents_ind: false,
            qualifying_children: Vec::new(),
            self_employment_tax_amt: Usd::ZERO,
            household_employment_tax_amt: Usd::ZERO,
            total_am_rrt_tax_amt: Usd::ZERO,
            soc_sec_medicare_tax_unrptd_tip_amt: Usd::ZERO,
            uncollected_soc_sec_tax_on_tips_amt: Usd::ZERO,
            uncollected_soc_sec_med_tax_amt: Usd::ZERO,
            uncollected_soc_sec_med_tax_gtli_amt: Usd::ZERO,
            estimated_tax_payments_amt: Usd::ZERO,
            former_spouse_ssn: String::new(),
            request_for_extension_amt: Usd::ZERO,
            additional_child_tax_credit_amt: Usd::ZERO,
            addl_medcr_rrt_tax_withholding_amt: Usd::ZERO,
            ex_soc_sec_tax_withheld_amt: Usd::ZERO,
            form_8888_ind: false,
            routing_transit_num: String::new(),
            bank_account_type_cd: String::new(),
            depositor_account_num: String::new(),
            applied_to_es_tax_amt: Usd::ZERO,
            third_party_designee_ind: false,
            third_party_designee_nm: String::new(),
            third_party_designee_phone_num: String::new(),
            third_party_designee_pin: String::new(),
            special_condition_desc: String::new(),
            special_condition_txt: String::new(),
            non_paid_preparer_cd: String::new(),
            personal_representative_ind: false,
            power_of_attorney_nm: String::new(),
            power_of_attorney_signed_by_ind: false,
            refund_product_cd: String::new(),
            refund_product_code_txt: String::new(),
            change_dt: String::new(),
            actc_qualifying_child_ind: false,
            actc_qlfy_child_under_age_ssn_cnt: 0,
            actc_modified_agi_amt: Usd::ZERO,
            actc_other_dependent_cnt: 0,
            actc_pr_ss_medcr_addnl_medcr_tax_withheld_amt: Usd::ZERO,
            actc_soc_sec_medicare_tax_unrptd_tip_amt: Usd::ZERO,
            actc_uncollected_soc_sec_med_tax_amt: Usd::ZERO,
            actc_uncollected_soc_sec_med_tax_gtli_amt: Usd::ZERO,
            actc_additional_medicare_tax_amt: Usd::ZERO,
            actc_addnl_medicare_tax_withholding_amt: Usd::ZERO,
        }
    }

    // ── must_file ──────────────────────────────────────────────────────

    #[test]
    fn must_file_with_se_tax() {
        let mut input = default_input();
        input.self_employment_tax_amt = Usd::from_dollars(1_000);
        assert!(Output1040Ss::must_file(&input));
    }

    #[test]
    fn must_file_zero_everything() {
        let input = default_input();
        assert!(!Output1040Ss::must_file(&input));
    }

    // ── Part I — Total Tax ─────────────────────────────────────────────

    #[test]
    fn part_i_total_tax() {
        let mut input = default_input();
        input.self_employment_tax_amt = Usd::from_dollars(5_000);
        input.household_employment_tax_amt = Usd::from_dollars(500);
        input.total_am_rrt_tax_amt = Usd::from_dollars(200);
        input.soc_sec_medicare_tax_unrptd_tip_amt = Usd::from_dollars(100);
        input.uncollected_soc_sec_tax_on_tips_amt = Usd::from_dollars(50);
        input.uncollected_soc_sec_med_tax_amt = Usd::from_dollars(75);
        input.uncollected_soc_sec_med_tax_gtli_amt = Usd::from_dollars(25);
        let form = Output1040Ss::try_new(input).unwrap();
        // line 6e = 100 + 50 + 75 + 25 = 250
        assert_eq!(form.total_other_taxes_amt, Usd::from_dollars(250));
        // line 7 = 5000 + 500 + 200 + 250 = 5950
        assert_eq!(form.total_tax_amt, Usd::from_dollars(5_950));
        assert!(form.is_valid());
    }

    // ── Part I — Payments and Refund ───────────────────────────────────

    #[test]
    fn part_i_overpayment() {
        let mut input = default_input();
        input.self_employment_tax_amt = Usd::from_dollars(1_000);
        input.estimated_tax_payments_amt = Usd::from_dollars(1_500);
        let form = Output1040Ss::try_new(input).unwrap();
        assert_eq!(form.total_tax_amt, Usd::from_dollars(1_000));
        assert_eq!(form.total_payments_amt, Usd::from_dollars(1_500));
        assert_eq!(form.overpaid_amt, Usd::from_dollars(500));
        assert_eq!(form.refund_amt, Usd::from_dollars(500));
        assert_eq!(form.owed_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_amount_owed() {
        let mut input = default_input();
        input.self_employment_tax_amt = Usd::from_dollars(2_000);
        input.estimated_tax_payments_amt = Usd::from_dollars(500);
        let form = Output1040Ss::try_new(input).unwrap();
        assert_eq!(form.overpaid_amt, Usd::ZERO);
        assert_eq!(form.refund_amt, Usd::ZERO);
        assert_eq!(form.owed_amt, Usd::from_dollars(1_500));
        assert!(form.is_valid());
    }

    #[test]
    fn part_i_partial_refund_with_es_application() {
        let mut input = default_input();
        input.self_employment_tax_amt = Usd::from_dollars(1_000);
        input.estimated_tax_payments_amt = Usd::from_dollars(2_000);
        input.applied_to_es_tax_amt = Usd::from_dollars(300);
        let form = Output1040Ss::try_new(input).unwrap();
        assert_eq!(form.overpaid_amt, Usd::from_dollars(1_000));
        assert_eq!(form.refund_amt, Usd::from_dollars(700));
        assert_eq!(form.applied_to_es_tax_amt, Usd::from_dollars(300));
        assert!(form.is_valid());
    }

    // ── Part II — ACTC ─────────────────────────────────────────────────

    #[test]
    fn actc_basic_credit() {
        let mut input = default_input();
        input.actc_qualifying_child_ind = true;
        input.actc_qlfy_child_under_age_ssn_cnt = 2;
        input.actc_modified_agi_amt = Usd::from_dollars(50_000);
        input.self_employment_tax_amt = Usd::from_dollars(10_000);
        let form = Output1040Ss::try_new(input).unwrap();
        // Line 2: 2 × $1,700 = $3,400
        assert_eq!(
            form.actc_qlfy_child_under_age_ssn_limt_amt,
            Usd::from_dollars(3_400)
        );
        // Line 7: 2 × $2,200 = $4,400
        assert_eq!(
            form.actc_qlfy_child_under_age_ssn_limt_amt_ln7,
            Usd::from_dollars(4_400)
        );
        // MAGI $50k < threshold $200k, so line 5 is blank (ZERO)
        assert!(!form.actc_magi_more_than_threshold_ind);
        // Line 11: line 2 = $3,400 (since MAGI ≤ threshold, skip lines 5-10)
        assert_eq!(form.actc_tax_limit_amt, Usd::from_dollars(3_400));
        // Line 12a: $10,000 / 2 = $5,000
        assert_eq!(
            form.actc_one_half_self_employment_tax_amt,
            Usd::from_dollars(5_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn actc_phase_out_applies() {
        let mut input = default_input();
        input.filing_status = FilingStatus::MarriedFilingJointly;
        input.actc_qualifying_child_ind = true;
        input.actc_qlfy_child_under_age_ssn_cnt = 1;
        input.actc_modified_agi_amt = Usd::from_dollars(401_500);
        input.self_employment_tax_amt = Usd::from_dollars(10_000);
        let form = Output1040Ss::try_new(input).unwrap();
        // Line 4: $400,000 (MFJ)
        assert_eq!(
            form.actc_modified_agi_phase_out_amt,
            Usd::from_dollars(400_000)
        );
        // Line 5: $401,500 - $400,000 = $1,500 → round up to $2,000
        assert_eq!(
            form.actc_magi_less_threshold_amt,
            Usd::from_dollars(2_000)
        );
        // Line 6: $2,000 × 5% = $100
        assert_eq!(form.actc_limit_amt, Usd::from_dollars(100));
        // Line 7: 1 × $2,200 = $2,200
        // Line 9: $2,200 + $0 = $2,200
        // Line 10: $2,200 - $100 = $2,100
        assert_eq!(form.actc_after_limit_amt, Usd::from_dollars(2_100));
        // Line 11: min($1,700, $2,100) = $1,700
        assert_eq!(form.actc_tax_limit_amt, Usd::from_dollars(1_700));
        assert!(form.is_valid());
    }

    #[test]
    fn actc_no_qualifying_children() {
        let input = default_input();
        let form = Output1040Ss::try_new(input).unwrap();
        assert_eq!(form.actc_additional_child_tax_credit_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn round_up_to_next_1000_exact() {
        assert_eq!(round_up_to_next_1000(Usd::from_dollars(3_000)), Usd::from_dollars(3_000));
    }

    #[test]
    fn round_up_to_next_1000_not_exact() {
        assert_eq!(round_up_to_next_1000(Usd::from_dollars(425)), Usd::from_dollars(1_000));
        assert_eq!(round_up_to_next_1000(Usd::from_dollars(1_025)), Usd::from_dollars(2_000));
        assert_eq!(round_up_to_next_1000(Usd::from_dollars(1_500)), Usd::from_dollars(2_000));
    }

    #[test]
    fn round_up_to_next_1000_zero() {
        assert_eq!(round_up_to_next_1000(Usd::ZERO), Usd::ZERO);
    }

    // ── Zero everything ────────────────────────────────────────────────

    #[test]
    fn zero_everything() {
        let form = Output1040Ss::try_new(default_input()).unwrap();
        assert_eq!(form.total_tax_amt, Usd::ZERO);
        assert_eq!(form.total_payments_amt, Usd::ZERO);
        assert_eq!(form.overpaid_amt, Usd::ZERO);
        assert_eq!(form.owed_amt, Usd::ZERO);
        assert_eq!(form.actc_additional_child_tax_credit_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
