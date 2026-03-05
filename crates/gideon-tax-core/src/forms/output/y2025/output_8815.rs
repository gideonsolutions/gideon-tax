use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Line 1 row
// =========================================================================

/// Per-entry row from Form 8815, Line 1 (columns a–b).
///
/// Each row identifies one person who was enrolled at or attended an
/// eligible educational institution, along with the institution's name
/// and address.
#[derive(Debug, Clone)]
pub struct F8815Line1 {
    /// Column (a): Name of person who was enrolled at or attended an
    /// eligible educational institution
    pub eligible_person_nm: String,
    /// Column (b): Name of eligible educational institution
    pub eligible_institution_nm: String,
    /// Column (b): Address of eligible educational institution
    pub eligible_institution_address: String,
}

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
    // Line 1 — Eligible persons and institutions
    // -----------------------------------------------------------------------
    /// Line 1: Persons enrolled at or attending eligible educational
    /// institutions.  The IRS form provides space for multiple rows.
    pub line1: Vec<F8815Line1>,

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

    // -- Line 6 Worksheet components (used to compute Form 8815 line 6) --
    /// Face value of post-1989 paper series EE bonds cashed in the tax year.
    /// The purchase price (principal) is 50% of face value.
    pub paper_ee_face_value_amt: Usd,
    /// Face value of electronic series EE bonds (including post-1989 paper
    /// series EE bonds converted to electronic format) cashed in the tax year
    pub electronic_ee_face_value_amt: Usd,
    /// Face value of series I bonds cashed in the tax year
    pub series_i_face_value_amt: Usd,
    /// Interest from the cashed bonds that was reported as income in
    /// previous tax years (Line 6 Worksheet, line 7). Zero if none.
    pub prior_year_interest_reported_amt: Usd,

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
#[derive(Debug, Clone)]
pub struct Output8815 {
    // -----------------------------------------------------------------------
    // Line 1 — Eligible persons and institutions
    // -----------------------------------------------------------------------
    /// Line 1: Persons enrolled at or attending eligible educational
    /// institutions
    pub line1: Vec<F8815Line1>,

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
        let paper_ee_principal = Usd::from_cents(input.paper_ee_face_value_amt.cents() / 2);
        let total_principal =
            paper_ee_principal + input.electronic_ee_face_value_amt + input.series_i_face_value_amt;
        let interest = input.total_bond_proceeds_amt
            - total_principal
            - input.prior_year_interest_reported_amt;
        interest > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
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

        // Line 6: computed via Line 6 Worksheet
        // WS line 3: paper EE principal = face value × 0.50
        let paper_ee_principal = Usd::from_cents(input.paper_ee_face_value_amt.cents() / 2);
        // WS line 5: total principal (paper EE principal + electronic EE face + series I face)
        let total_principal =
            paper_ee_principal + input.electronic_ee_face_value_amt + input.series_i_face_value_amt;
        // WS line 6: proceeds − principal
        let gross_interest = line5 - total_principal;
        // WS line 8: subtract prior-year reported interest
        let line6 = gross_interest - input.prior_year_interest_reported_amt;

        if line6 < Usd::ZERO {
            return Err(GideonTaxError::OutOfBounds(format!(
                "computed bond interest ({line6}) is negative; \
                 total_bond_proceeds_amt ({}) minus principal ({total_principal}) \
                 minus prior_year_interest_reported_amt ({}) < 0",
                line5, input.prior_year_interest_reported_amt,
            )));
        }

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
            line1: input.line1,

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

    fn make_line1() -> F8815Line1 {
        F8815Line1 {
            eligible_person_nm: "Jane Doe".to_string(),
            eligible_institution_nm: "State University".to_string(),
            eligible_institution_address: "123 Campus Dr, Springfield, IL 62701".to_string(),
        }
    }

    /// Helper: basic single-filer input.
    ///
    /// Proceeds $20,000 with electronic EE face value $15,000 gives
    /// line 6 interest = $5,000.
    fn basic_input() -> F8815Input {
        F8815Input {
            line1: vec![make_line1()],
            qualified_higher_ed_expenses_amt: Usd::from_dollars(10_000),
            nontaxable_ed_benefits_amt: Usd::ZERO,
            total_bond_proceeds_amt: Usd::from_dollars(20_000),
            paper_ee_face_value_amt: Usd::ZERO,
            electronic_ee_face_value_amt: Usd::from_dollars(15_000),
            series_i_face_value_amt: Usd::ZERO,
            prior_year_interest_reported_amt: Usd::ZERO,
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
        // Set face value equal to proceeds so computed interest = 0
        input.electronic_ee_face_value_amt = Usd::from_dollars(20_000);
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
        assert_eq!(form.excl_bond_int_excess_agi_amt, Usd::from_dollars(6_500));
        assert_eq!(form.excl_bond_int_excess_agi_rt, "0.433");
        assert_eq!(form.excl_bond_int_offset_amt, Usd::from_cents(108_333));
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
        assert_eq!(form.excl_bond_int_excess_agi_amt, Usd::from_dollars(20_500));
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
        assert_eq!(form.excl_bond_int_excess_agi_amt, Usd::from_dollars(15_000));
        assert_eq!(form.excl_bond_int_excess_agi_rt, "0.500");
        assert_eq!(form.excl_bond_int_offset_amt, Usd::from_dollars(1_250));
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
        input.electronic_ee_face_value_amt = Usd::ZERO;
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(form.excl_bond_int_tentative_bond_int_amt, Usd::ZERO);
        assert_eq!(form.excludable_savings_bond_int_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn principal_exceeds_proceeds_returns_error() {
        let mut input = basic_input();
        // electronic EE face $25,000 > proceeds $20,000 => negative interest
        input.electronic_ee_face_value_amt = Usd::from_dollars(25_000);
        let err = Output8815::try_new(input).unwrap_err();
        assert!(matches!(err, GideonTaxError::OutOfBounds(_)));
    }

    #[test]
    fn prior_year_interest_exceeds_gross_returns_error() {
        let mut input = basic_input();
        // gross interest = 20,000 - 15,000 = 5,000; prior year = 6,000 => negative
        input.prior_year_interest_reported_amt = Usd::from_dollars(6_000);
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
    fn paper_ee_bonds_half_face_value() {
        // Paper EE face value $20,000 → principal = $10,000
        // Proceeds $20,000 → interest = 20,000 - 10,000 = 10,000
        let mut input = basic_input();
        input.paper_ee_face_value_amt = Usd::from_dollars(20_000);
        input.electronic_ee_face_value_amt = Usd::ZERO;
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(
            form.excl_bond_int_tot_py_bond_int_amt,
            Usd::from_dollars(10_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn mixed_bond_types() {
        // Paper EE face $10,000 → principal $5,000
        // Electronic EE face $3,000 → principal $3,000
        // Series I face $2,000 → principal $2,000
        // Total principal = $10,000
        // Proceeds $15,000 → interest = 15,000 - 10,000 = 5,000
        let mut input = basic_input();
        input.total_bond_proceeds_amt = Usd::from_dollars(15_000);
        input.paper_ee_face_value_amt = Usd::from_dollars(10_000);
        input.electronic_ee_face_value_amt = Usd::from_dollars(3_000);
        input.series_i_face_value_amt = Usd::from_dollars(2_000);
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(
            form.excl_bond_int_tot_py_bond_int_amt,
            Usd::from_dollars(5_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn prior_year_interest_reduces_line6() {
        // Gross interest = 20,000 - 15,000 = 5,000
        // Prior year = 2,000
        // Line 6 = 5,000 - 2,000 = 3,000
        let mut input = basic_input();
        input.prior_year_interest_reported_amt = Usd::from_dollars(2_000);
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(
            form.excl_bond_int_tot_py_bond_int_amt,
            Usd::from_dollars(3_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn line1_entries_preserved() {
        let mut input = basic_input();
        input.line1.push(F8815Line1 {
            eligible_person_nm: "John Doe".to_string(),
            eligible_institution_nm: "City College".to_string(),
            eligible_institution_address: "456 Academic Blvd, Chicago, IL 60601".to_string(),
        });
        let form = Output8815::try_new(input).unwrap();
        assert_eq!(form.line1.len(), 2);
        assert_eq!(form.line1[0].eligible_person_nm, "Jane Doe");
        assert_eq!(form.line1[1].eligible_person_nm, "John Doe");
        assert_eq!(
            form.line1[1].eligible_institution_address,
            "456 Academic Blvd, Chicago, IL 60601"
        );
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
