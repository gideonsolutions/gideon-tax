use crate::Usd;

/// Output fields for IRS Form 8853 (2025) — Archer MSAs and Long-Term Care Insurance Contracts.
#[derive(Debug, Clone, Default)]
pub struct Output8853 {
    // -----------------------------------------------------------------------
    // Section A — Archer MSAs
    // -----------------------------------------------------------------------
    // -- Part I — Archer MSA Contributions and Deductions --
    // -----------------------------------------------------------------------
    /// Line 1: Total employer contributions to your Archer MSA(s) for 2025
    pub archer_msa_employer_contri_amt: Usd,
    /// Line 2: Archer MSA contributions you made for 2025
    pub archer_msa_contribution_amt: Usd,
    /// Line 3: Limitation from the Line 3 Limitation Chart and Worksheet in the instructions
    pub archer_msa_contri_limitation_amt: Usd,
    /// Line 4: Compensation from the employer maintaining the high deductible health plan
    pub hdhp_employer_compensation_amt: Usd,
    /// Line 5: Archer MSA deduction. Enter the smallest of line 2, 3, or 4
    pub archer_msa_deduction_amt: Usd,

    // -----------------------------------------------------------------------
    // -- Part II — Archer MSA Distributions --
    // -----------------------------------------------------------------------
    /// Line 6a: Total distributions you and your spouse received in 2025 from all Archer MSAs
    pub total_archer_msa_distribution_amt: Usd,
    /// Line 6b: Distributions rolled over to another Archer MSA or health savings account
    pub archer_msa_distri_roll_over_amt: Usd,
    /// Line 6c: Subtract line 6b from line 6a
    pub archer_msa_net_distribution_amt: Usd,
    /// Line 7: Unreimbursed qualified medical expenses
    pub archer_msa_unreimb_qual_med_exp_amt: Usd,
    /// Line 8: Taxable Archer MSA distributions. Subtract line 7 from line 6c. If zero or less, enter -0-
    pub taxable_archer_msa_distri_amt: Usd,
    /// Line 9a: If any distributions meet any of the Exceptions to the Additional 20% Tax, check here
    pub archer_msa_distri_meet_tax_exc_ind: bool,
    /// Line 9b: Additional 20% tax on distributions included on line 8 subject to additional tax
    pub archer_msa_addnl_distri_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Section B — Medicare Advantage MSA Distributions
    // -----------------------------------------------------------------------
    /// Line 10: Total distributions you received in 2025 from all Medicare Advantage MSAs
    pub total_medicare_msa_distri_amt: Usd,
    /// Line 11: Unreimbursed qualified medical expenses
    pub medicare_msa_unrmb_qual_med_exp_amt: Usd,
    /// Line 12: Taxable Medicare Advantage MSA distributions. Subtract line 11 from line 10
    pub taxable_medicare_msa_distri_amt: Usd,
    /// Line 13a: If any distributions meet any of the Exceptions to the Additional 50% Tax, check here
    pub medicare_msa_distri_meet_tax_exc_ind: bool,
    /// Line 13b: Additional 50% tax on distributions included on line 12 subject to additional tax
    pub medicare_msa_addnl_distri_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Section C — Long-Term Care (LTC) Insurance Contracts
    // -----------------------------------------------------------------------
    /// Line 14a: Name of insured
    pub ltc_insured_nm: String,
    /// Line 14a: Name control text for insured
    pub ltc_insured_name_control_txt: String,
    /// Line 14b: Social security number of insured
    pub ltc_insured_ssn: String,
    /// Line 15: Did anyone other than you receive payments on a per diem or other periodic basis
    pub ltc_insurance_other_payment_ind: bool,
    /// Line 16: Was the insured a terminally ill individual?
    pub ltc_insured_terminally_ill_ind: bool,
    /// Line 17: Gross LTC payments received on a per diem or other periodic basis
    pub ltc_gross_payments_received_amt: Usd,
    /// Line 18: Enter the part of the amount on line 17 that is from qualified LTC insurance contracts
    pub ltc_insurance_qualified_amt: Usd,
    /// Line 19: Accelerated death benefits received on a per diem or other periodic basis
    pub accelerated_death_benefit_rcvd_amt: Usd,
    /// Line 20: Add lines 18 and 19
    pub total_ltc_and_death_benefit_rcvd_amt: Usd,
    /// Line 21: Multiply $420 by the number of days in the LTC period
    pub ltc_days_multiply_by_per_diem_amt: Usd,
    /// Line 22: Costs incurred for qualified LTC services provided for the insured during the LTC period
    pub ltc_cost_incurred_amt: Usd,
    /// Line 23: Enter the larger of line 21 or line 22
    pub larger_calc_or_actual_ltc_costs_amt: Usd,
    /// Line 24: Reimbursements for qualified LTC services provided for the insured during the LTC period
    pub ltc_reimbursement_amt: Usd,
    /// Line 25: Per diem limitation. Subtract line 24 from line 23
    pub ltc_per_diem_limitation_amt: Usd,
    /// Line 26: Taxable payments. Subtract line 25 from line 20. If zero or less, enter -0-
    pub ltc_taxable_payments_amt: Usd,

    // -----------------------------------------------------------------------
    // Additional fields
    // -----------------------------------------------------------------------
    /// If more than one Section C is attached, check here
    pub form8853_ltc_multiple_copies_ind: bool,
    /// Social security number of MSA account holder
    pub msa_holder_ssn: String,
    /// Indicator: MSA holder is deceased
    pub msa_holder_death_ind: bool,
    /// MSA calculations explanation statement code
    pub msa_calculations_expln_stmt_cd: String,
    /// Name of policyholder (as shown on return)
    pub ltc_insurance_policy_holder_nm: String,
    /// Social security number of policyholder
    pub ltc_insurance_policy_holder_ssn: String,
}
