use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8853 (2025) — Archer MSAs and
/// Long-Term Care Insurance Contracts.
///
/// Covers Section A (Archer MSA contributions/deductions and distributions),
/// Section B (Medicare Advantage MSA distributions), and Section C (LTC
/// insurance contracts).
#[derive(Debug, Clone)]
pub struct F8853Input {
    // ── Header ────────────────────────────────────────────────────────
    /// Social security number of MSA account holder
    pub msa_holder_ssn: String,
    /// Indicator: MSA holder is deceased
    pub msa_holder_death_ind: bool,
    /// MSA calculations explanation statement code
    pub msa_calculations_expln_stmt_cd: String,

    // ── Section A, Part I — Archer MSA Contributions and Deductions ───
    /// Line 1: Total employer contributions to Archer MSA(s) for 2025
    pub archer_msa_employer_contri_amt: Usd,
    /// Line 2: Archer MSA contributions you made for 2025
    pub archer_msa_contribution_amt: Usd,
    /// Line 3: Limitation from Line 3 Limitation Chart and Worksheet
    pub archer_msa_contri_limitation_amt: Usd,
    /// Line 4: Compensation from employer maintaining the HDHP
    pub hdhp_employer_compensation_amt: Usd,

    // ── Section A, Part II — Archer MSA Distributions ─────────────────
    /// Line 6a: Total distributions from all Archer MSAs
    pub total_archer_msa_distribution_amt: Usd,
    /// Line 6b: Distributions rolled over to another Archer MSA or HSA
    pub archer_msa_distri_roll_over_amt: Usd,
    /// Line 7: Unreimbursed qualified medical expenses
    pub archer_msa_unreimb_qual_med_exp_amt: Usd,
    /// Line 9a: Exception to the Additional 20% Tax indicator
    pub archer_msa_distri_meet_tax_exc_ind: bool,

    // ── Section B — Medicare Advantage MSA Distributions ──────────────
    /// Line 10: Total distributions from Medicare Advantage MSAs
    pub total_medicare_msa_distri_amt: Usd,
    /// Line 11: Unreimbursed qualified medical expenses
    pub medicare_msa_unrmb_qual_med_exp_amt: Usd,
    /// Line 13a: Exception to the Additional 50% Tax indicator
    pub medicare_msa_distri_meet_tax_exc_ind: bool,

    // ── Section C — Long-Term Care Insurance Contracts ────────────────
    /// Line 14a: Name of insured
    pub ltc_insured_nm: String,
    /// Line 14a: Name control text for insured
    pub ltc_insured_name_control_txt: String,
    /// Line 14b: Social security number of insured
    pub ltc_insured_ssn: String,
    /// Line 15: Did anyone other than you receive payments on a per diem basis
    pub ltc_insurance_other_payment_ind: bool,
    /// Line 16: Was the insured a terminally ill individual?
    pub ltc_insured_terminally_ill_ind: bool,
    /// Line 17: Gross LTC payments received on a per diem basis
    pub ltc_gross_payments_received_amt: Usd,
    /// Line 18: Amount from qualified LTC insurance contracts
    pub ltc_insurance_qualified_amt: Usd,
    /// Line 19: Accelerated death benefits received on a per diem basis
    pub accelerated_death_benefit_rcvd_amt: Usd,
    /// Line 21: $420 × number of days in LTC period (pre-computed)
    pub ltc_days_multiply_by_per_diem_amt: Usd,
    /// Line 22: Costs incurred for qualified LTC services
    pub ltc_cost_incurred_amt: Usd,
    /// Line 24: Reimbursements for qualified LTC services
    pub ltc_reimbursement_amt: Usd,
    /// If more than one Section C is attached
    pub form8853_ltc_multiple_copies_ind: bool,
    /// Name of policyholder
    pub ltc_insurance_policy_holder_nm: String,
    /// Social security number of policyholder
    pub ltc_insurance_policy_holder_ssn: String,
}

// =========================================================================
// Output
// =========================================================================

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

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8853 {
    fn name() -> &'static str {
        "Form 8853"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8853 {
    type Input = F8853Input;

    fn must_file(input: &Self::Input) -> bool {
        input.archer_msa_employer_contri_amt > Usd::ZERO
            || input.archer_msa_contribution_amt > Usd::ZERO
            || input.total_archer_msa_distribution_amt > Usd::ZERO
            || input.total_medicare_msa_distri_amt > Usd::ZERO
            || input.ltc_gross_payments_received_amt > Usd::ZERO
            || input.accelerated_death_benefit_rcvd_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // ── Section A, Part I ───────────────────────────────────────
        let line1 = input.archer_msa_employer_contri_amt;
        let line2 = input.archer_msa_contribution_amt;
        let line3 = input.archer_msa_contri_limitation_amt;
        let line4 = input.hdhp_employer_compensation_amt;
        let line5 = line2.min(line3).min(line4);

        // ── Section A, Part II ──────────────────────────────────────
        let line6a = input.total_archer_msa_distribution_amt;
        let line6b = input.archer_msa_distri_roll_over_amt;
        let line6c = line6a - line6b;
        let line7 = input.archer_msa_unreimb_qual_med_exp_amt;
        let line8 = (line6c - line7).max(Usd::ZERO);
        let line9a = input.archer_msa_distri_meet_tax_exc_ind;
        let line9b = if line9a {
            Usd::ZERO
        } else {
            Usd::from_cents(line8.cents() * 20 / 100)
        };

        // ── Section B ───────────────────────────────────────────────
        let line10 = input.total_medicare_msa_distri_amt;
        let line11 = input.medicare_msa_unrmb_qual_med_exp_amt;
        let line12 = (line10 - line11).max(Usd::ZERO);
        let line13a = input.medicare_msa_distri_meet_tax_exc_ind;
        let line13b = if line13a {
            Usd::ZERO
        } else {
            Usd::from_cents(line12.cents() * 50 / 100)
        };

        // ── Section C ───────────────────────────────────────────────
        let line18 = input.ltc_insurance_qualified_amt;
        let line19 = input.accelerated_death_benefit_rcvd_amt;
        let line20 = line18 + line19;
        let line21 = input.ltc_days_multiply_by_per_diem_amt;
        let line22 = input.ltc_cost_incurred_amt;
        let line23 = line21.max(line22);
        let line24 = input.ltc_reimbursement_amt;
        let line25 = line23 - line24;

        // If terminally ill AND only accelerated death benefits, line 26 = 0
        let line26 = if input.ltc_insured_terminally_ill_ind
            && input.ltc_insurance_qualified_amt == Usd::ZERO
        {
            Usd::ZERO
        } else {
            (line20 - line25).max(Usd::ZERO)
        };

        Ok(Output8853 {
            // Section A, Part I
            archer_msa_employer_contri_amt: line1,
            archer_msa_contribution_amt: line2,
            archer_msa_contri_limitation_amt: line3,
            hdhp_employer_compensation_amt: line4,
            archer_msa_deduction_amt: line5,
            // Section A, Part II
            total_archer_msa_distribution_amt: line6a,
            archer_msa_distri_roll_over_amt: line6b,
            archer_msa_net_distribution_amt: line6c,
            archer_msa_unreimb_qual_med_exp_amt: line7,
            taxable_archer_msa_distri_amt: line8,
            archer_msa_distri_meet_tax_exc_ind: line9a,
            archer_msa_addnl_distri_tax_amt: line9b,
            // Section B
            total_medicare_msa_distri_amt: line10,
            medicare_msa_unrmb_qual_med_exp_amt: line11,
            taxable_medicare_msa_distri_amt: line12,
            medicare_msa_distri_meet_tax_exc_ind: line13a,
            medicare_msa_addnl_distri_tax_amt: line13b,
            // Section C
            ltc_insured_nm: input.ltc_insured_nm,
            ltc_insured_name_control_txt: input.ltc_insured_name_control_txt,
            ltc_insured_ssn: input.ltc_insured_ssn,
            ltc_insurance_other_payment_ind: input.ltc_insurance_other_payment_ind,
            ltc_insured_terminally_ill_ind: input.ltc_insured_terminally_ill_ind,
            ltc_gross_payments_received_amt: input.ltc_gross_payments_received_amt,
            ltc_insurance_qualified_amt: line18,
            accelerated_death_benefit_rcvd_amt: line19,
            total_ltc_and_death_benefit_rcvd_amt: line20,
            ltc_days_multiply_by_per_diem_amt: line21,
            ltc_cost_incurred_amt: line22,
            larger_calc_or_actual_ltc_costs_amt: line23,
            ltc_reimbursement_amt: line24,
            ltc_per_diem_limitation_amt: line25,
            ltc_taxable_payments_amt: line26,
            // Additional
            form8853_ltc_multiple_copies_ind: input.form8853_ltc_multiple_copies_ind,
            msa_holder_ssn: input.msa_holder_ssn,
            msa_holder_death_ind: input.msa_holder_death_ind,
            msa_calculations_expln_stmt_cd: input.msa_calculations_expln_stmt_cd,
            ltc_insurance_policy_holder_nm: input.ltc_insurance_policy_holder_nm,
            ltc_insurance_policy_holder_ssn: input.ltc_insurance_policy_holder_ssn,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::W2]
    }

    fn is_valid(&self) -> bool {
        // Section A, Part I
        let line5_ok = self.archer_msa_deduction_amt
            == self
                .archer_msa_contribution_amt
                .min(self.archer_msa_contri_limitation_amt)
                .min(self.hdhp_employer_compensation_amt);

        // Section A, Part II
        let line6c_ok = self.archer_msa_net_distribution_amt
            == self.total_archer_msa_distribution_amt - self.archer_msa_distri_roll_over_amt;
        let line8_ok = self.taxable_archer_msa_distri_amt
            == (self.archer_msa_net_distribution_amt - self.archer_msa_unreimb_qual_med_exp_amt)
                .max(Usd::ZERO);
        let line9b_ok = if self.archer_msa_distri_meet_tax_exc_ind {
            self.archer_msa_addnl_distri_tax_amt == Usd::ZERO
        } else {
            self.archer_msa_addnl_distri_tax_amt
                == Usd::from_cents(self.taxable_archer_msa_distri_amt.cents() * 20 / 100)
        };

        // Section B
        let line12_ok = self.taxable_medicare_msa_distri_amt
            == (self.total_medicare_msa_distri_amt - self.medicare_msa_unrmb_qual_med_exp_amt)
                .max(Usd::ZERO);
        let line13b_ok = if self.medicare_msa_distri_meet_tax_exc_ind {
            self.medicare_msa_addnl_distri_tax_amt == Usd::ZERO
        } else {
            self.medicare_msa_addnl_distri_tax_amt
                == Usd::from_cents(self.taxable_medicare_msa_distri_amt.cents() * 50 / 100)
        };

        // Section C
        let line20_ok = self.total_ltc_and_death_benefit_rcvd_amt
            == self.ltc_insurance_qualified_amt + self.accelerated_death_benefit_rcvd_amt;
        let line23_ok = self.larger_calc_or_actual_ltc_costs_amt
            == self
                .ltc_days_multiply_by_per_diem_amt
                .max(self.ltc_cost_incurred_amt);
        let line25_ok = self.ltc_per_diem_limitation_amt
            == self.larger_calc_or_actual_ltc_costs_amt - self.ltc_reimbursement_amt;

        line5_ok
            && line6c_ok
            && line8_ok
            && line9b_ok
            && line12_ok
            && line13b_ok
            && line20_ok
            && line23_ok
            && line25_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_input() -> F8853Input {
        F8853Input {
            msa_holder_ssn: String::new(),
            msa_holder_death_ind: false,
            msa_calculations_expln_stmt_cd: String::new(),
            archer_msa_employer_contri_amt: Usd::ZERO,
            archer_msa_contribution_amt: Usd::from_dollars(2_000),
            archer_msa_contri_limitation_amt: Usd::from_dollars(2_850),
            hdhp_employer_compensation_amt: Usd::from_dollars(50_000),
            total_archer_msa_distribution_amt: Usd::ZERO,
            archer_msa_distri_roll_over_amt: Usd::ZERO,
            archer_msa_unreimb_qual_med_exp_amt: Usd::ZERO,
            archer_msa_distri_meet_tax_exc_ind: false,
            total_medicare_msa_distri_amt: Usd::ZERO,
            medicare_msa_unrmb_qual_med_exp_amt: Usd::ZERO,
            medicare_msa_distri_meet_tax_exc_ind: false,
            ltc_insured_nm: String::new(),
            ltc_insured_name_control_txt: String::new(),
            ltc_insured_ssn: String::new(),
            ltc_insurance_other_payment_ind: false,
            ltc_insured_terminally_ill_ind: false,
            ltc_gross_payments_received_amt: Usd::ZERO,
            ltc_insurance_qualified_amt: Usd::ZERO,
            accelerated_death_benefit_rcvd_amt: Usd::ZERO,
            ltc_days_multiply_by_per_diem_amt: Usd::ZERO,
            ltc_cost_incurred_amt: Usd::ZERO,
            ltc_reimbursement_amt: Usd::ZERO,
            form8853_ltc_multiple_copies_ind: false,
            ltc_insurance_policy_holder_nm: String::new(),
            ltc_insurance_policy_holder_ssn: String::new(),
        }
    }

    #[test]
    fn must_file_with_contributions() {
        assert!(Output8853::must_file(&default_input()));
    }

    #[test]
    fn must_file_false_no_activity() {
        let mut input = default_input();
        input.archer_msa_contribution_amt = Usd::ZERO;
        assert!(!Output8853::must_file(&input));
    }

    #[test]
    fn must_file_with_ltc_payments() {
        let mut input = default_input();
        input.archer_msa_contribution_amt = Usd::ZERO;
        input.ltc_gross_payments_received_amt = Usd::from_dollars(10_000);
        assert!(Output8853::must_file(&input));
    }

    #[test]
    fn section_a_part_i_deduction_smallest_of_2_3_4() {
        let form = Output8853::try_new(default_input()).unwrap();
        // min(2,000, 2,850, 50,000) = 2,000
        assert_eq!(form.archer_msa_deduction_amt, Usd::from_dollars(2_000));
        assert!(form.is_valid());
    }

    #[test]
    fn section_a_part_i_limitation_is_smallest() {
        let mut input = default_input();
        input.archer_msa_contribution_amt = Usd::from_dollars(5_000);
        input.archer_msa_contri_limitation_amt = Usd::from_dollars(2_850);
        let form = Output8853::try_new(input).unwrap();
        // min(5,000, 2,850, 50,000) = 2,850
        assert_eq!(form.archer_msa_deduction_amt, Usd::from_dollars(2_850));
        assert!(form.is_valid());
    }

    #[test]
    fn section_a_part_i_compensation_is_smallest() {
        let mut input = default_input();
        input.archer_msa_contribution_amt = Usd::from_dollars(5_000);
        input.archer_msa_contri_limitation_amt = Usd::from_dollars(10_000);
        input.hdhp_employer_compensation_amt = Usd::from_dollars(3_000);
        let form = Output8853::try_new(input).unwrap();
        // min(5,000, 10,000, 3,000) = 3,000
        assert_eq!(form.archer_msa_deduction_amt, Usd::from_dollars(3_000));
        assert!(form.is_valid());
    }

    #[test]
    fn section_a_part_ii_no_taxable_distribution() {
        let mut input = default_input();
        input.total_archer_msa_distribution_amt = Usd::from_dollars(3_000);
        input.archer_msa_unreimb_qual_med_exp_amt = Usd::from_dollars(4_000);
        let form = Output8853::try_new(input).unwrap();
        assert_eq!(form.taxable_archer_msa_distri_amt, Usd::ZERO);
        assert_eq!(form.archer_msa_addnl_distri_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn section_a_part_ii_taxable_with_20_percent() {
        let mut input = default_input();
        input.total_archer_msa_distribution_amt = Usd::from_dollars(5_000);
        input.archer_msa_unreimb_qual_med_exp_amt = Usd::from_dollars(2_000);
        let form = Output8853::try_new(input).unwrap();
        // line 8: max(5,000 - 2,000, 0) = 3,000
        assert_eq!(
            form.taxable_archer_msa_distri_amt,
            Usd::from_dollars(3_000)
        );
        // 20% of 3,000 = 600
        assert_eq!(
            form.archer_msa_addnl_distri_tax_amt,
            Usd::from_dollars(600)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn section_a_part_ii_exception_to_tax() {
        let mut input = default_input();
        input.total_archer_msa_distribution_amt = Usd::from_dollars(5_000);
        input.archer_msa_unreimb_qual_med_exp_amt = Usd::from_dollars(2_000);
        input.archer_msa_distri_meet_tax_exc_ind = true;
        let form = Output8853::try_new(input).unwrap();
        assert_eq!(
            form.taxable_archer_msa_distri_amt,
            Usd::from_dollars(3_000)
        );
        assert_eq!(form.archer_msa_addnl_distri_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn section_b_medicare_msa_taxable_with_50_percent() {
        let mut input = default_input();
        input.archer_msa_contribution_amt = Usd::ZERO;
        input.total_medicare_msa_distri_amt = Usd::from_dollars(10_000);
        input.medicare_msa_unrmb_qual_med_exp_amt = Usd::from_dollars(4_000);
        let form = Output8853::try_new(input).unwrap();
        // line 12: max(10,000 - 4,000, 0) = 6,000
        assert_eq!(
            form.taxable_medicare_msa_distri_amt,
            Usd::from_dollars(6_000)
        );
        // 50% of 6,000 = 3,000
        assert_eq!(
            form.medicare_msa_addnl_distri_tax_amt,
            Usd::from_dollars(3_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn section_c_ltc_no_taxable_payments() {
        let mut input = default_input();
        input.ltc_insurance_qualified_amt = Usd::from_dollars(30_000);
        input.ltc_days_multiply_by_per_diem_amt = Usd::from_dollars(42_000); // 100 days * $420
        input.ltc_cost_incurred_amt = Usd::from_dollars(35_000);
        input.ltc_reimbursement_amt = Usd::ZERO;
        let form = Output8853::try_new(input).unwrap();
        // line 20: 30,000 + 0 = 30,000
        // line 23: max(42,000, 35,000) = 42,000
        // line 25: 42,000 - 0 = 42,000
        // line 26: max(30,000 - 42,000, 0) = 0
        assert_eq!(form.ltc_taxable_payments_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn section_c_ltc_taxable_payments() {
        let mut input = default_input();
        input.ltc_insurance_qualified_amt = Usd::from_dollars(50_000);
        input.ltc_days_multiply_by_per_diem_amt = Usd::from_dollars(42_000);
        input.ltc_cost_incurred_amt = Usd::from_dollars(35_000);
        input.ltc_reimbursement_amt = Usd::from_dollars(5_000);
        let form = Output8853::try_new(input).unwrap();
        // line 20: 50,000 + 0 = 50,000
        // line 23: max(42,000, 35,000) = 42,000
        // line 25: 42,000 - 5,000 = 37,000
        // line 26: max(50,000 - 37,000, 0) = 13,000
        assert_eq!(
            form.ltc_taxable_payments_amt,
            Usd::from_dollars(13_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn section_c_terminally_ill_only_death_benefits() {
        let mut input = default_input();
        input.ltc_insured_terminally_ill_ind = true;
        input.accelerated_death_benefit_rcvd_amt = Usd::from_dollars(100_000);
        // ltc_insurance_qualified_amt is ZERO → only accelerated death benefits
        let form = Output8853::try_new(input).unwrap();
        assert_eq!(form.ltc_taxable_payments_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn section_c_actual_costs_larger_than_per_diem() {
        let mut input = default_input();
        input.ltc_insurance_qualified_amt = Usd::from_dollars(60_000);
        input.ltc_days_multiply_by_per_diem_amt = Usd::from_dollars(42_000);
        input.ltc_cost_incurred_amt = Usd::from_dollars(55_000);
        input.ltc_reimbursement_amt = Usd::ZERO;
        let form = Output8853::try_new(input).unwrap();
        // line 23: max(42,000, 55,000) = 55,000
        assert_eq!(
            form.larger_calc_or_actual_ltc_costs_amt,
            Usd::from_dollars(55_000)
        );
        // line 25: 55,000 - 0 = 55,000
        // line 26: max(60,000 - 55,000, 0) = 5,000
        assert_eq!(form.ltc_taxable_payments_amt, Usd::from_dollars(5_000));
        assert!(form.is_valid());
    }

    #[test]
    fn zero_everything() {
        let mut input = default_input();
        input.archer_msa_contribution_amt = Usd::ZERO;
        input.archer_msa_contri_limitation_amt = Usd::ZERO;
        input.hdhp_employer_compensation_amt = Usd::ZERO;
        let form = Output8853::try_new(input).unwrap();
        assert_eq!(form.archer_msa_deduction_amt, Usd::ZERO);
        assert_eq!(form.taxable_archer_msa_distri_amt, Usd::ZERO);
        assert_eq!(form.taxable_medicare_msa_distri_amt, Usd::ZERO);
        assert_eq!(form.ltc_taxable_payments_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
