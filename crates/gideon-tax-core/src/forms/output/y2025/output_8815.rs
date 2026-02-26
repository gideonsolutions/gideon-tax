use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8815.
///
/// Bond proceeds, interest, educational expenses, AGI, filing status limit,
/// and phaseout range feed into the exclusion computation; the
/// corresponding dependency is declared in [`OutputForm::dependencies`].
#[derive(Debug, Clone)]
pub struct F8815Input {
    // -----------------------------------------------------------------------
    // Line 1 — Eligible persons and institutions (passthrough)
    // -----------------------------------------------------------------------
    /// Line 1(a): Name of person who was enrolled at or attended an eligible educational institution
    pub eligible_person_nm: String,
    /// Line 1(b): Name of eligible educational institution
    pub eligible_institution_nm: String,
    /// Line 1(b): Address line 1 of eligible educational institution
    pub address_line1_txt: String,
    /// Line 1(b): Address line 2 of eligible educational institution
    pub address_line2_txt: String,
    /// Line 1(b): City of eligible educational institution
    pub city_nm: String,
    /// Line 1(b): State abbreviation of eligible educational institution
    pub state_abbreviation_cd: String,
    /// Line 1(b): ZIP code of eligible educational institution
    pub zip_cd: String,
    /// Line 1(b): Country code (foreign address)
    pub country_cd: String,
    /// Line 1(b): Province or state name (foreign address)
    pub province_or_state_nm: String,
    /// Line 1(b): Foreign postal code
    pub foreign_postal_cd: String,
    /// Coverdell educational savings account code
    pub coverdell_educational_sav_acct_cd: String,
    /// Qualified tuition program code
    pub qualified_tuition_program_cd: String,

    // -----------------------------------------------------------------------
    // Lines 2-14 — Exclusion computation inputs
    // -----------------------------------------------------------------------
    /// Line 2: Total qualified higher education expenses paid in the tax year
    pub qualified_higher_ed_expenses_amt: Usd,
    /// Line 3: Total nontaxable educational benefits received for the tax year
    pub nontaxable_ed_benefits_amt: Usd,
    /// Line 5: Total proceeds (principal + interest) from all series EE and I
    /// U.S. savings bonds issued after 1989 that were cashed during the tax year
    pub total_bond_proceeds_amt: Usd,
    /// Line 6: Interest included in Line 5
    pub bond_interest_amt: Usd,
    /// Line 9: Modified adjusted gross income
    pub modified_agi_amt: Usd,
    /// Line 10: Filing status limit amount ($99,500 if single/HOH/QSS;
    /// $149,250 if married filing jointly)
    pub filing_status_limit_amt: Usd,
    /// Phaseout range amount ($15,000 if single/HOH/QSS; $30,000 if MFJ).
    /// Used as the divisor on Line 12.
    pub phaseout_range_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8815 (2025) — Exclusion of Interest From Series EE and I U.S. Savings Bonds Issued After 1989.
#[derive(Debug, Clone, Default)]
pub struct Output8815 {
    // -----------------------------------------------------------------------
    // Line 1 — Eligible persons and institutions
    // -----------------------------------------------------------------------
    /// Line 1(a): Name of person who was enrolled at or attended an eligible educational institution
    pub eligible_person_nm: String,
    /// Line 1(b): Name of eligible educational institution
    pub eligible_institution_nm: String,
    /// Line 1(b): Address line 1 of eligible educational institution
    pub address_line1_txt: String,
    /// Line 1(b): Address line 2 of eligible educational institution
    pub address_line2_txt: String,
    /// Line 1(b): City of eligible educational institution
    pub city_nm: String,
    /// Line 1(b): State abbreviation of eligible educational institution
    pub state_abbreviation_cd: String,
    /// Line 1(b): ZIP code of eligible educational institution
    pub zip_cd: String,
    /// Line 1(b): Country code (foreign address)
    pub country_cd: String,
    /// Line 1(b): Province or state name (foreign address)
    pub province_or_state_nm: String,
    /// Line 1(b): Foreign postal code
    pub foreign_postal_cd: String,
    /// Coverdell educational savings account code
    pub coverdell_educational_sav_acct_cd: String,
    /// Qualified tuition program code
    pub qualified_tuition_program_cd: String,

    // -----------------------------------------------------------------------
    // Lines 2-14 — Exclusion computation
    // -----------------------------------------------------------------------
    /// Line 2: Enter the total qualified higher education expenses you paid in 2025
    pub excl_bond_int_tot_qlfy_educ_expns_amt: Usd,
    /// Line 3: Enter the total of any nontaxable educational benefits received for 2025
    pub excl_bond_int_tot_non_tx_educ_bnft_amt: Usd,
    /// Line 4: Subtract line 3 from line 2. If zero or less, stop
    pub excl_bond_int_txbl_educ_benefit_amt: Usd,
    /// Line 5: Enter the total proceeds (principal and interest) from all series EE and I U.S. savings bonds issued after 1989 that you cashed during 2025
    pub excl_bond_tot_py_bond_proc_amt: Usd,
    /// Line 6: Enter the interest included on line 5
    pub excl_bond_int_tot_py_bond_int_amt: Usd,
    /// Line 7: If line 4 is equal to or more than line 5, enter "1.000". If line 4 is less than line 5, divide line 4 by line 5 (decimal)
    pub excl_bond_int_txbl_expns_bond_proc_rt: String,
    /// Line 8: Multiply line 6 by line 7
    pub excl_bond_int_tentative_bond_int_amt: Usd,
    /// Line 9: Enter your modified adjusted gross income
    pub excl_bond_int_modified_agi_amt: Usd,
    /// Line 10: Enter $99,500 if single, head of household, or qualifying surviving spouse; or $149,250 if married filing jointly
    pub excl_bond_int_filing_status_lmt_amt: Usd,
    /// Line 11: Subtract line 10 from line 9. If zero or less, skip line 12, enter -0- on line 13, and go to line 14
    pub excl_bond_int_excess_agi_amt: Usd,
    /// Line 12: Divide line 11 by $15,000 if single, head of household, or qualifying surviving spouse; or $30,000 if married filing jointly (decimal)
    pub excl_bond_int_excess_agi_rt: String,
    /// Line 13: Multiply line 8 by line 12
    pub excl_bond_int_offset_amt: Usd,
    /// Line 14: Excludable savings bond interest. Subtract line 13 from line 8. Enter the result here and on Schedule B (Form 1040), line 3
    pub excludable_savings_bond_int_amt: Usd,

    // -----------------------------------------------------------------------
    // Internal — not printed on the form but needed for is_valid
    // -----------------------------------------------------------------------
    /// Phaseout range amount used in the computation ($15,000 or $30,000)
    pub phaseout_range_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8815 {
    fn name() -> &'static str {
        "Form 8815"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output8815 {
    type Input = F8815Input;

    fn must_file(input: &Self::Input) -> bool {
        input.bond_interest_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        if input.total_bond_proceeds_amt == Usd::ZERO && input.bond_interest_amt > Usd::ZERO {
            return Err(GideonTaxError::OutOfBounds(
                "bond_interest_amt is positive but total_bond_proceeds_amt is zero".to_string(),
            ));
        }
        if input.bond_interest_amt > input.total_bond_proceeds_amt {
            return Err(GideonTaxError::OutOfBounds(format!(
                "bond_interest_amt ({}) exceeds total_bond_proceeds_amt ({})",
                input.bond_interest_amt, input.total_bond_proceeds_amt,
            )));
        }
        if input.phaseout_range_amt == Usd::ZERO {
            return Err(GideonTaxError::OutOfBounds(
                "phaseout_range_amt must be positive".to_string(),
            ));
        }

        // Line 2: qualified higher education expenses (passthrough)
        let line2 = input.qualified_higher_ed_expenses_amt;

        // Line 3: nontaxable educational benefits (passthrough)
        let line3 = input.nontaxable_ed_benefits_amt;

        // Line 4: max(line2 - line3, 0)
        let line4 = (line2 - line3).max(Usd::ZERO);

        // Line 5: total bond proceeds (passthrough)
        let line5 = input.total_bond_proceeds_amt;

        // Line 6: interest included in line 5 (passthrough)
        let line6 = input.bond_interest_amt;

        // Line 7: ratio (stored as string)
        // If line4 >= line5, ratio is "1.000"; else ratio = line4 / line5
        let line7_str = if line5 == Usd::ZERO {
            "0.000".to_string()
        } else if line4 >= line5 {
            "1.000".to_string()
        } else {
            // Compute decimal to 3 places: (line4.cents() * 1000) / line5.cents()
            let thousandths = line4.cents() * 1000 / line5.cents();
            format!("0.{:03}", thousandths)
        };

        // Line 8: tentative exclusion
        // When line4 >= line5: line8 = line6
        // When line4 < line5: line8 = line6.cents() * line4.cents() / line5.cents()
        let line8 = if line5 == Usd::ZERO {
            Usd::ZERO
        } else if line4 >= line5 {
            line6
        } else {
            Usd::from_cents(line6.cents() * line4.cents() / line5.cents())
        };

        // Line 9: modified AGI (passthrough)
        let line9 = input.modified_agi_amt;

        // Line 10: filing status limit (passthrough)
        let line10 = input.filing_status_limit_amt;

        // Line 11: max(line9 - line10, 0)
        let line11 = (line9 - line10).max(Usd::ZERO);

        let phaseout_range = input.phaseout_range_amt;

        // Line 12: ratio of excess AGI to phaseout range (stored as string)
        let line12_str = if line11 == Usd::ZERO {
            "0.000".to_string()
        } else if line11 >= phaseout_range {
            "1.000".to_string()
        } else {
            let thousandths = line11.cents() * 1000 / phaseout_range.cents();
            format!("0.{:03}", thousandths)
        };

        // Line 13: offset amount
        // line8 * line12 ratio, computed as line8.cents() * line11.cents() / phaseout_range.cents()
        // Capped so line13 does not exceed line8
        let line13 = if line11 == Usd::ZERO {
            Usd::ZERO
        } else if line11 >= phaseout_range {
            line8
        } else {
            Usd::from_cents(line8.cents() * line11.cents() / phaseout_range.cents()).min(line8)
        };

        // Line 14: excludable interest = line8 - line13
        let line14 = line8 - line13;

        Ok(Output8815 {
            // Line 1 passthrough
            eligible_person_nm: input.eligible_person_nm,
            eligible_institution_nm: input.eligible_institution_nm,
            address_line1_txt: input.address_line1_txt,
            address_line2_txt: input.address_line2_txt,
            city_nm: input.city_nm,
            state_abbreviation_cd: input.state_abbreviation_cd,
            zip_cd: input.zip_cd,
            country_cd: input.country_cd,
            province_or_state_nm: input.province_or_state_nm,
            foreign_postal_cd: input.foreign_postal_cd,
            coverdell_educational_sav_acct_cd: input.coverdell_educational_sav_acct_cd,
            qualified_tuition_program_cd: input.qualified_tuition_program_cd,

            // Lines 2-14
            excl_bond_int_tot_qlfy_educ_expns_amt: line2,
            excl_bond_int_tot_non_tx_educ_bnft_amt: line3,
            excl_bond_int_txbl_educ_benefit_amt: line4,
            excl_bond_tot_py_bond_proc_amt: line5,
            excl_bond_int_tot_py_bond_int_amt: line6,
            excl_bond_int_txbl_expns_bond_proc_rt: line7_str,
            excl_bond_int_tentative_bond_int_amt: line8,
            excl_bond_int_modified_agi_amt: line9,
            excl_bond_int_filing_status_lmt_amt: line10,
            excl_bond_int_excess_agi_amt: line11,
            excl_bond_int_excess_agi_rt: line12_str,
            excl_bond_int_offset_amt: line13,
            excludable_savings_bond_int_amt: line14,

            // Internal
            phaseout_range_amt: phaseout_range,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[DynForm::ScheduleB]
    }

    fn is_valid(&self) -> bool {
        let line2 = self.excl_bond_int_tot_qlfy_educ_expns_amt;
        let line3 = self.excl_bond_int_tot_non_tx_educ_bnft_amt;
        let line4 = self.excl_bond_int_txbl_educ_benefit_amt;
        let line5 = self.excl_bond_tot_py_bond_proc_amt;
        let line6 = self.excl_bond_int_tot_py_bond_int_amt;
        let line8 = self.excl_bond_int_tentative_bond_int_amt;
        let line9 = self.excl_bond_int_modified_agi_amt;
        let line10 = self.excl_bond_int_filing_status_lmt_amt;
        let line11 = self.excl_bond_int_excess_agi_amt;
        let line13 = self.excl_bond_int_offset_amt;
        let line14 = self.excludable_savings_bond_int_amt;
        let phaseout_range = self.phaseout_range_amt;

        // Line 4 = max(line2 - line3, 0)
        let line4_ok = line4 == (line2 - line3).max(Usd::ZERO);

        // Line 8
        let line8_ok = if line5 == Usd::ZERO {
            line8 == Usd::ZERO
        } else if line4 >= line5 {
            line8 == line6
        } else {
            line8 == Usd::from_cents(line6.cents() * line4.cents() / line5.cents())
        };

        // Line 11 = max(line9 - line10, 0)
        let line11_ok = line11 == (line9 - line10).max(Usd::ZERO);

        // Line 13
        let line13_ok = if line11 == Usd::ZERO {
            line13 == Usd::ZERO
        } else if line11 >= phaseout_range {
            line13 == line8
        } else {
            line13
                == Usd::from_cents(line8.cents() * line11.cents() / phaseout_range.cents())
                    .min(line8)
        };

        // Line 14 = line8 - line13
        let line14_ok = line14 == line8 - line13;

        line4_ok && line8_ok && line11_ok && line13_ok && line14_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: basic single-filer input with customizable amounts.
    fn basic_input() -> F8815Input {
        F8815Input {
            eligible_person_nm: "Jane Doe".to_string(),
            eligible_institution_nm: "State University".to_string(),
            address_line1_txt: "123 Campus Dr".to_string(),
            address_line2_txt: String::new(),
            city_nm: "Springfield".to_string(),
            state_abbreviation_cd: "IL".to_string(),
            zip_cd: "62701".to_string(),
            country_cd: String::new(),
            province_or_state_nm: String::new(),
            foreign_postal_cd: String::new(),
            coverdell_educational_sav_acct_cd: String::new(),
            qualified_tuition_program_cd: String::new(),
            qualified_higher_ed_expenses_amt: Usd::from_dollars(10_000),
            nontaxable_ed_benefits_amt: Usd::ZERO,
            total_bond_proceeds_amt: Usd::from_dollars(20_000),
            bond_interest_amt: Usd::from_dollars(5_000),
            modified_agi_amt: Usd::from_dollars(80_000),
            filing_status_limit_amt: Usd::from_dollars(99_500),
            phaseout_range_amt: Usd::from_dollars(15_000),
        }
    }

    #[test]
    fn must_file_with_interest() {
        let input = basic_input();
        assert!(Output8815::must_file(&input));
    }

    #[test]
    fn must_file_no_interest() {
        let mut input = basic_input();
        input.bond_interest_amt = Usd::ZERO;
        assert!(!Output8815::must_file(&input));
    }

    #[test]
    fn basic_exclusion_below_phaseout() {
        // AGI $80,000 < filing status limit $99,500 => no phaseout
        // Line 4: 10,000 - 0 = 10,000
        // Line 5: 20,000 => line4 < line5
        // Line 7: 10,000 / 20,000 = 0.500
        // Line 8: 5,000 * 10,000 / 20,000 = 2,500
        // Line 11: 80,000 - 99,500 = 0
        // Line 13: 0
        // Line 14: 2,500
        let form = Output8815::try_new(basic_input()).unwrap();
        assert_eq!(
            form.excl_bond_int_txbl_educ_benefit_amt,
            Usd::from_dollars(10_000)
        );
        assert_eq!(form.excl_bond_int_txbl_expns_bond_proc_rt, "0.500");
        assert_eq!(
            form.excl_bond_int_tentative_bond_int_amt,
            Usd::from_dollars(2_500)
        );
        assert_eq!(form.excl_bond_int_excess_agi_amt, Usd::ZERO);
        assert_eq!(form.excl_bond_int_excess_agi_rt, "0.000");
        assert_eq!(form.excl_bond_int_offset_amt, Usd::ZERO);
        assert_eq!(
            form.excludable_savings_bond_int_amt,
            Usd::from_dollars(2_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn expenses_exceed_proceeds_ratio_one() {
        // Line 4 >= Line 5 => ratio "1.000", line8 = line6
        let mut input = basic_input();
        input.qualified_higher_ed_expenses_amt = Usd::from_dollars(25_000);
        // line4 = 25,000 - 0 = 25,000 >= line5 = 20,000
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(form.excl_bond_int_txbl_expns_bond_proc_rt, "1.000");
        assert_eq!(
            form.excl_bond_int_tentative_bond_int_amt,
            Usd::from_dollars(5_000)
        );
        assert_eq!(
            form.excludable_savings_bond_int_amt,
            Usd::from_dollars(5_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn nontaxable_benefits_reduce_expenses() {
        let mut input = basic_input();
        input.nontaxable_ed_benefits_amt = Usd::from_dollars(4_000);
        // line4 = 10,000 - 4,000 = 6,000
        // line7 = 6,000 / 20,000 = 0.300
        // line8 = 5,000 * 6,000 / 20,000 = 1,500
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(
            form.excl_bond_int_txbl_educ_benefit_amt,
            Usd::from_dollars(6_000)
        );
        assert_eq!(form.excl_bond_int_txbl_expns_bond_proc_rt, "0.300");
        assert_eq!(
            form.excl_bond_int_tentative_bond_int_amt,
            Usd::from_dollars(1_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn nontaxable_benefits_exceed_expenses_zero_exclusion() {
        let mut input = basic_input();
        input.nontaxable_ed_benefits_amt = Usd::from_dollars(12_000);
        // line4 = max(10,000 - 12,000, 0) = 0
        // line7 = "0.000" (line5 > 0, line4 < line5)
        // line8 = 5,000 * 0 / 20,000 = 0
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(form.excl_bond_int_txbl_educ_benefit_amt, Usd::ZERO);
        assert_eq!(form.excl_bond_int_txbl_expns_bond_proc_rt, "0.000");
        assert_eq!(form.excl_bond_int_tentative_bond_int_amt, Usd::ZERO);
        assert_eq!(form.excludable_savings_bond_int_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn partial_phaseout() {
        // AGI within the phaseout range
        let mut input = basic_input();
        input.modified_agi_amt = Usd::from_dollars(106_000);
        // line4 = 10,000
        // line7: 10,000 / 20,000 = 0.500
        // line8 = 5,000 * 10,000 / 20,000 = 2,500
        // line11 = 106,000 - 99,500 = 6,500
        // line12 = 6,500 / 15,000 = 0.433
        // line13 = 2,500 * 6,500 / 15,000 = 1,083.33 => from_cents(250000 * 650000 / 1500000) = from_cents(108333)
        // line14 = 2,500 - 1,083.33 = 1,416.67
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(
            form.excl_bond_int_excess_agi_amt,
            Usd::from_dollars(6_500)
        );
        assert_eq!(form.excl_bond_int_excess_agi_rt, "0.433");
        assert_eq!(
            form.excl_bond_int_offset_amt,
            Usd::from_cents(108_333)
        );
        assert_eq!(
            form.excludable_savings_bond_int_amt,
            Usd::from_cents(250_000 - 108_333)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn full_phaseout_agi_exceeds_range() {
        // AGI exceeds filing status limit + phaseout range => full phaseout
        let mut input = basic_input();
        input.modified_agi_amt = Usd::from_dollars(120_000);
        // line11 = 120,000 - 99,500 = 20,500 >= phaseout_range 15,000
        // line12 = "1.000"
        // line13 = line8 (full offset)
        // line14 = 0
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(
            form.excl_bond_int_excess_agi_amt,
            Usd::from_dollars(20_500)
        );
        assert_eq!(form.excl_bond_int_excess_agi_rt, "1.000");
        assert_eq!(
            form.excl_bond_int_offset_amt,
            form.excl_bond_int_tentative_bond_int_amt
        );
        assert_eq!(form.excludable_savings_bond_int_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn married_filing_jointly_phaseout() {
        // MFJ: limit $149,250, phaseout $30,000
        let mut input = basic_input();
        input.filing_status_limit_amt = Usd::from_dollars(149_250);
        input.phaseout_range_amt = Usd::from_dollars(30_000);
        input.modified_agi_amt = Usd::from_dollars(164_250);
        // line11 = 164,250 - 149,250 = 15,000
        // line12 = 15,000 / 30,000 = 0.500
        // line8 = 2,500
        // line13 = 2,500 * 15,000 / 30,000 = 1,250
        // line14 = 2,500 - 1,250 = 1,250
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(
            form.excl_bond_int_excess_agi_amt,
            Usd::from_dollars(15_000)
        );
        assert_eq!(form.excl_bond_int_excess_agi_rt, "0.500");
        assert_eq!(
            form.excl_bond_int_offset_amt,
            Usd::from_dollars(1_250)
        );
        assert_eq!(
            form.excludable_savings_bond_int_amt,
            Usd::from_dollars(1_250)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn zero_bond_proceeds_zero_interest() {
        let mut input = basic_input();
        input.total_bond_proceeds_amt = Usd::ZERO;
        input.bond_interest_amt = Usd::ZERO;
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(form.excl_bond_int_tentative_bond_int_amt, Usd::ZERO);
        assert_eq!(form.excludable_savings_bond_int_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn interest_exceeds_proceeds_returns_error() {
        let mut input = basic_input();
        input.bond_interest_amt = Usd::from_dollars(25_000);
        input.total_bond_proceeds_amt = Usd::from_dollars(20_000);
        let err = Output8815::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn zero_phaseout_range_returns_error() {
        let mut input = basic_input();
        input.phaseout_range_amt = Usd::ZERO;
        let err = Output8815::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn passthrough_fields_preserved() {
        let input = basic_input();
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(form.eligible_person_nm, "Jane Doe");
        assert_eq!(form.eligible_institution_nm, "State University");
        assert_eq!(form.city_nm, "Springfield");
        assert_eq!(form.state_abbreviation_cd, "IL");
        assert!(form.is_valid());
    }

    #[test]
    fn agi_exactly_at_limit_no_phaseout() {
        let mut input = basic_input();
        input.modified_agi_amt = Usd::from_dollars(99_500);
        // line11 = 99,500 - 99,500 = 0 => no phaseout
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(form.excl_bond_int_excess_agi_amt, Usd::ZERO);
        assert_eq!(form.excl_bond_int_offset_amt, Usd::ZERO);
        assert_eq!(
            form.excludable_savings_bond_int_amt,
            Usd::from_dollars(2_500)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn agi_exactly_at_phaseout_top_full_phaseout() {
        let mut input = basic_input();
        input.modified_agi_amt = Usd::from_dollars(114_500);
        // line11 = 114,500 - 99,500 = 15,000 = phaseout_range => full phaseout
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(form.excl_bond_int_excess_agi_rt, "1.000");
        assert_eq!(
            form.excl_bond_int_offset_amt,
            form.excl_bond_int_tentative_bond_int_amt
        );
        assert_eq!(form.excludable_savings_bond_int_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
