use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 8611 (2025) — Recapture of
/// Low-Income Housing Credit.
///
/// This form computes the recapture tax when a building's qualified basis
/// decreases during the 15-year compliance period (or when a flow-through
/// entity passes through a recapture amount).
#[derive(Debug, Clone)]
pub struct F8611Input {
    // ── Header (Items A–F) ────────────────────────────────────────────
    /// Item C: Address of building (as shown on Form 8609)
    pub building_us_address: String,
    /// Item C: Building foreign address (if applicable)
    pub building_foreign_address: String,
    /// Item D: Building identification number (BIN)
    pub bin: String,
    /// Item E: Date placed in service (from Form 8609)
    pub placed_in_service_dt: String,
    /// Item F(1): Issuer's name (tax-exempt bond financing)
    pub business_name_line1_txt: String,
    /// Item F(1): Issuer's name line 2
    pub business_name_line2_txt: String,
    /// Item F(2): Date of issue (tax-exempt bond financing)
    pub issue_dt: String,
    /// Item F(3): Name of issue (tax-exempt bond financing)
    pub issue_nm: String,
    /// Item F(4): CUSIP number (tax-exempt bond financing)
    pub cusip_num: String,
    /// Item F(4): Missing CUSIP reason code
    pub missing_cusip_reason_cd: String,

    // ── Lines 1–7 ────────────────────────────────────────────────────
    /// Line 1: Total credits reported on Form 8586 in prior years for this building
    pub py_total_credits_on_form8586_amt: Usd,
    /// Line 2: Credits included on line 1 attributable to additions to qualified basis
    pub credits_included_amt: Usd,
    /// Line 4: Credit recapture percentage (in basis points, e.g. 3333 = 33.33%)
    pub credit_recapture_pct_bps: u32,
    /// Line 6: Percentage decrease in qualified basis (in basis points, e.g. 10000 = 100%)
    pub decrease_in_qualified_basis_pct_bps: u32,

    // ── Lines 8–15 ───────────────────────────────────────────────────
    /// Line 8: Recapture amount from flow-through entity (if applicable)
    pub flow_thru_entity_recapture_amt: Usd,
    /// Line 9: Unused portion of the accelerated amount from line 7
    pub accelerated_prtn_of_unsd_credit_amt: Usd,
    /// Line 11: Interest on the line 10 recapture amount
    pub interest_on_recapture_amt: Usd,
    /// Line 13: Unused credits attributable to this building reduced by
    /// the accelerated portion on line 9
    pub unused_credit_red_by_accel_prtn_amt: Usd,

    // ── Section 42(j)(5) ─────────────────────────────────────────────
    /// Section 42(j)(5) election code (empty if not applicable)
    pub section42j5_cd: String,
    /// Line 16: Interest on the line 7 recapture amount (section 42(j)(5) partnerships)
    pub section42j5_interest_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 8611 (2025) — Recapture of Low-Income Housing Credit.
#[derive(Debug, Clone, Default)]
pub struct Output8611 {
    // -----------------------------------------------------------------------
    // Header Information (Items A–F)
    // -----------------------------------------------------------------------
    /// Item C: Address of building (as shown on Form 8609)
    pub building_us_address: String,
    /// Item C: Building foreign address (if applicable)
    pub building_foreign_address: String,
    /// Item D: Building identification number (BIN)
    pub bin: String,
    /// Item E: Date placed in service (from Form 8609)
    pub placed_in_service_dt: String,
    /// Item F(1): Issuer's name (tax-exempt bond financing)
    pub business_name_line1_txt: String,
    /// Item F(1): Issuer's name line 2
    pub business_name_line2_txt: String,
    /// Item F(2): Date of issue (tax-exempt bond financing)
    pub issue_dt: String,
    /// Item F(3): Name of issue (tax-exempt bond financing)
    pub issue_nm: String,
    /// Item F(4): CUSIP number (tax-exempt bond financing)
    pub cusip_num: String,
    /// Item F(4): Missing CUSIP reason code
    pub missing_cusip_reason_cd: String,

    // -----------------------------------------------------------------------
    // Lines 1–7 — Credit Recapture Computation
    // -----------------------------------------------------------------------
    /// Line 1: Total credits reported on Form 8586 in prior years for this building
    pub py_total_credits_on_form8586_amt: Usd,
    /// Line 2: Credits included on line 1 attributable to additions to qualified basis
    pub credits_included_amt: Usd,
    /// Line 3: Credits subject to recapture (subtract line 2 from line 1)
    pub credits_subject_to_recapture_amt: Usd,
    /// Line 4: Credit recapture percentage
    pub credit_recapture_percent_rt: String,
    /// Line 5: Accelerated portion of credit (multiply line 3 by line 4)
    pub accelerated_portion_of_credit_amt: Usd,
    /// Line 6: Percentage decrease in qualified basis (decimal amount)
    pub decrease_in_qualified_basis_pct_rt: String,
    /// Line 7: Amount of accelerated portion recaptured (multiply line 5 by line 6)
    pub accelerated_prtn_recaptured_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 8–15 — Recapture Tax and Carryforward
    // -----------------------------------------------------------------------
    /// Line 8: Recapture amount from flow-through entity
    pub flow_thru_entity_recapture_amt: Usd,
    /// Line 9: Unused portion of the accelerated amount from line 7
    pub accelerated_prtn_of_unsd_credit_amt: Usd,
    /// Line 10: Net recapture (subtract line 9 from line 7 or line 8, not less than zero)
    pub net_recapture_amt: Usd,
    /// Line 11: Interest on the line 10 recapture amount
    pub interest_on_recapture_amt: Usd,
    /// Line 12: Total amount subject to recapture (add lines 10 and 11)
    pub total_subject_to_recapture_amt: Usd,
    /// Line 13: Unused credits attributable to this building reduced by the accelerated portion on line 9
    pub unused_credit_red_by_accel_prtn_amt: Usd,
    /// Line 14: Recapture tax (subtract line 13 from line 12, not less than zero)
    pub recapture_tax_amt: Usd,
    /// Line 15: Carryforward of the low-income housing credit attributable to this building
    pub carryforward_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Lines 16–17 — Section 42(j)(5) Partnerships Only
    // -----------------------------------------------------------------------
    /// Line 16: Interest on the line 7 recapture amount (section 42(j)(5) partnerships)
    pub recapture_amt: Usd,
    /// Line 17: Total recapture (add lines 7 and 16) (section 42(j)(5) partnerships)
    pub total_recapture_amt: Usd,

    // -----------------------------------------------------------------------
    // Additional / Computed
    // -----------------------------------------------------------------------
    /// Section 42(j)(5) election code
    pub section42j5_cd: String,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output8611 {
    fn name() -> &'static str {
        "Form 8611"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

/// Format basis points as a decimal percentage string (e.g. 3333 → "33.33").
fn bps_to_pct_string(bps: u32) -> String {
    let whole = bps / 100;
    let frac = bps % 100;
    if frac == 0 {
        format!("{whole}")
    } else {
        format!("{whole}.{frac:02}")
    }
}

impl OutputForm for Output8611 {
    type Input = F8611Input;

    fn must_file(input: &Self::Input) -> bool {
        input.py_total_credits_on_form8586_amt > Usd::ZERO
            || input.flow_thru_entity_recapture_amt > Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        // ── Lines 1–7 ────────────────────────────────────────────────
        let line1 = input.py_total_credits_on_form8586_amt;
        let line2 = input.credits_included_amt;
        let line3 = line1 - line2;
        let line4_bps = input.credit_recapture_pct_bps;
        let line5 = Usd::from_cents(line3.cents() * line4_bps as i64 / 10_000);
        let line6_bps = input.decrease_in_qualified_basis_pct_bps;
        let line7 = Usd::from_cents(line5.cents() * line6_bps as i64 / 10_000);

        // ── Lines 8–15 ───────────────────────────────────────────────
        let line8 = input.flow_thru_entity_recapture_amt;
        let line9 = input.accelerated_prtn_of_unsd_credit_amt;

        // If line 8 > 0, use line 8; otherwise use line 7
        let recapture_base = if line8 > Usd::ZERO { line8 } else { line7 };
        let line10 = (recapture_base - line9).max(Usd::ZERO);
        let line11 = input.interest_on_recapture_amt;
        let line12 = line10 + line11;
        let line13 = input.unused_credit_red_by_accel_prtn_amt;
        let line14 = (line12 - line13).max(Usd::ZERO);
        let line15 = (line13 - line12).max(Usd::ZERO);

        // ── Lines 16–17 (Section 42(j)(5)) ───────────────────────────
        let line16 = input.section42j5_interest_amt;
        let line17 = line7 + line16;

        Ok(Output8611 {
            // Header
            building_us_address: input.building_us_address,
            building_foreign_address: input.building_foreign_address,
            bin: input.bin,
            placed_in_service_dt: input.placed_in_service_dt,
            business_name_line1_txt: input.business_name_line1_txt,
            business_name_line2_txt: input.business_name_line2_txt,
            issue_dt: input.issue_dt,
            issue_nm: input.issue_nm,
            cusip_num: input.cusip_num,
            missing_cusip_reason_cd: input.missing_cusip_reason_cd,
            // Lines 1–7
            py_total_credits_on_form8586_amt: line1,
            credits_included_amt: line2,
            credits_subject_to_recapture_amt: line3,
            credit_recapture_percent_rt: bps_to_pct_string(line4_bps),
            accelerated_portion_of_credit_amt: line5,
            decrease_in_qualified_basis_pct_rt: bps_to_pct_string(line6_bps),
            accelerated_prtn_recaptured_amt: line7,
            // Lines 8–15
            flow_thru_entity_recapture_amt: line8,
            accelerated_prtn_of_unsd_credit_amt: line9,
            net_recapture_amt: line10,
            interest_on_recapture_amt: line11,
            total_subject_to_recapture_amt: line12,
            unused_credit_red_by_accel_prtn_amt: line13,
            recapture_tax_amt: line14,
            carryforward_credit_amt: line15,
            // Lines 16–17
            recapture_amt: line16,
            total_recapture_amt: line17,
            // Additional
            section42j5_cd: input.section42j5_cd,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[]
    }

    fn is_valid(&self) -> bool {
        // Line 3
        let line3_ok = self.credits_subject_to_recapture_amt
            == self.py_total_credits_on_form8586_amt - self.credits_included_amt;

        // Line 10
        let recapture_base = if self.flow_thru_entity_recapture_amt > Usd::ZERO {
            self.flow_thru_entity_recapture_amt
        } else {
            self.accelerated_prtn_recaptured_amt
        };
        let line10_ok = self.net_recapture_amt
            == (recapture_base - self.accelerated_prtn_of_unsd_credit_amt).max(Usd::ZERO);

        // Line 12
        let line12_ok = self.total_subject_to_recapture_amt
            == self.net_recapture_amt + self.interest_on_recapture_amt;

        // Line 14
        let line14_ok = self.recapture_tax_amt
            == (self.total_subject_to_recapture_amt - self.unused_credit_red_by_accel_prtn_amt)
                .max(Usd::ZERO);

        // Line 15
        let line15_ok = self.carryforward_credit_amt
            == (self.unused_credit_red_by_accel_prtn_amt - self.total_subject_to_recapture_amt)
                .max(Usd::ZERO);

        // Line 17
        let line17_ok = self.total_recapture_amt
            == self.accelerated_prtn_recaptured_amt + self.recapture_amt;

        line3_ok && line10_ok && line12_ok && line14_ok && line15_ok && line17_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_input() -> F8611Input {
        F8611Input {
            building_us_address: String::new(),
            building_foreign_address: String::new(),
            bin: String::new(),
            placed_in_service_dt: String::new(),
            business_name_line1_txt: String::new(),
            business_name_line2_txt: String::new(),
            issue_dt: String::new(),
            issue_nm: String::new(),
            cusip_num: String::new(),
            missing_cusip_reason_cd: String::new(),
            py_total_credits_on_form8586_amt: Usd::ZERO,
            credits_included_amt: Usd::ZERO,
            credit_recapture_pct_bps: 0,
            decrease_in_qualified_basis_pct_bps: 0,
            flow_thru_entity_recapture_amt: Usd::ZERO,
            accelerated_prtn_of_unsd_credit_amt: Usd::ZERO,
            interest_on_recapture_amt: Usd::ZERO,
            unused_credit_red_by_accel_prtn_amt: Usd::ZERO,
            section42j5_cd: String::new(),
            section42j5_interest_amt: Usd::ZERO,
        }
    }

    // ── must_file ────────────────────────────────────────────────────

    #[test]
    fn must_file_false_no_activity() {
        assert!(!Output8611::must_file(&default_input()));
    }

    #[test]
    fn must_file_with_prior_credits() {
        let mut input = default_input();
        input.py_total_credits_on_form8586_amt = Usd::from_dollars(10_000);
        assert!(Output8611::must_file(&input));
    }

    #[test]
    fn must_file_with_flow_through() {
        let mut input = default_input();
        input.flow_thru_entity_recapture_amt = Usd::from_dollars(5_000);
        assert!(Output8611::must_file(&input));
    }

    // ── Lines 1–7 — Credit Recapture Computation ─────────────────────

    #[test]
    fn basic_recapture_computation() {
        let mut input = default_input();
        input.py_total_credits_on_form8586_amt = Usd::from_dollars(100_000);
        input.credits_included_amt = Usd::from_dollars(20_000);
        // 33.33% recapture rate
        input.credit_recapture_pct_bps = 3333;
        // 100% decrease in qualified basis
        input.decrease_in_qualified_basis_pct_bps = 10_000;
        let form = Output8611::try_new(input).unwrap();
        // line 3: 100,000 - 20,000 = 80,000
        assert_eq!(
            form.credits_subject_to_recapture_amt,
            Usd::from_dollars(80_000)
        );
        // line 5: 80,000 * 33.33% = 26,664
        assert_eq!(
            form.accelerated_portion_of_credit_amt,
            Usd::from_dollars(26_664)
        );
        // line 7: 26,664 * 100% = 26,664
        assert_eq!(
            form.accelerated_prtn_recaptured_amt,
            Usd::from_dollars(26_664)
        );
        assert_eq!(form.credit_recapture_percent_rt, "33.33");
        assert_eq!(form.decrease_in_qualified_basis_pct_rt, "100");
        assert!(form.is_valid());
    }

    #[test]
    fn partial_decrease_in_qualified_basis() {
        let mut input = default_input();
        input.py_total_credits_on_form8586_amt = Usd::from_dollars(50_000);
        input.credit_recapture_pct_bps = 3333;
        // 50% decrease in qualified basis
        input.decrease_in_qualified_basis_pct_bps = 5_000;
        let form = Output8611::try_new(input).unwrap();
        // line 3: 50,000
        // line 5: 50,000 * 33.33% = 16,665
        assert_eq!(
            form.accelerated_portion_of_credit_amt,
            Usd::from_dollars(16_665)
        );
        // line 7: 16,665 * 50% = 8,332 (truncation)
        assert_eq!(
            form.accelerated_prtn_recaptured_amt,
            Usd::from_cents(833_250)
        );
        assert!(form.is_valid());
    }

    // ── Lines 8–15 — Recapture Tax ───────────────────────────────────

    #[test]
    fn recapture_tax_exceeds_unused_credits() {
        let mut input = default_input();
        input.py_total_credits_on_form8586_amt = Usd::from_dollars(100_000);
        input.credit_recapture_pct_bps = 3333;
        input.decrease_in_qualified_basis_pct_bps = 10_000;
        input.interest_on_recapture_amt = Usd::from_dollars(1_000);
        input.unused_credit_red_by_accel_prtn_amt = Usd::from_dollars(5_000);
        let form = Output8611::try_new(input).unwrap();
        // line 7: 33,330; line 9: 0; line 10: 33,330
        // line 12: 33,330 + 1,000 = 34,330
        // line 14: 34,330 - 5,000 = 29,330
        assert_eq!(form.recapture_tax_amt, Usd::from_dollars(29_330));
        assert_eq!(form.carryforward_credit_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn unused_credits_exceed_recapture_gives_carryforward() {
        let mut input = default_input();
        input.py_total_credits_on_form8586_amt = Usd::from_dollars(10_000);
        input.credit_recapture_pct_bps = 3333;
        input.decrease_in_qualified_basis_pct_bps = 10_000;
        input.unused_credit_red_by_accel_prtn_amt = Usd::from_dollars(5_000);
        let form = Output8611::try_new(input).unwrap();
        // line 7: 3,333; line 10: 3,333; line 12: 3,333
        // line 14: max(3,333 - 5,000, 0) = 0
        // line 15: max(5,000 - 3,333, 0) = 1,667
        assert_eq!(form.recapture_tax_amt, Usd::ZERO);
        assert_eq!(
            form.carryforward_credit_amt,
            Usd::from_dollars(1_667)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn unused_accelerated_portion_reduces_net_recapture() {
        let mut input = default_input();
        input.py_total_credits_on_form8586_amt = Usd::from_dollars(100_000);
        input.credit_recapture_pct_bps = 3333;
        input.decrease_in_qualified_basis_pct_bps = 10_000;
        input.accelerated_prtn_of_unsd_credit_amt = Usd::from_dollars(10_000);
        let form = Output8611::try_new(input).unwrap();
        // line 7: 33,330; line 9: 10,000; line 10: max(33,330 - 10,000, 0) = 23,330
        assert_eq!(form.net_recapture_amt, Usd::from_dollars(23_330));
        assert!(form.is_valid());
    }

    // ── Flow-through entity ──────────────────────────────────────────

    #[test]
    fn flow_through_entity_recapture() {
        let mut input = default_input();
        input.flow_thru_entity_recapture_amt = Usd::from_dollars(15_000);
        input.interest_on_recapture_amt = Usd::from_dollars(500);
        let form = Output8611::try_new(input).unwrap();
        // line 10: 15,000 (using line 8 since > 0)
        assert_eq!(form.net_recapture_amt, Usd::from_dollars(15_000));
        // line 12: 15,000 + 500 = 15,500
        assert_eq!(
            form.total_subject_to_recapture_amt,
            Usd::from_dollars(15_500)
        );
        assert_eq!(form.recapture_tax_amt, Usd::from_dollars(15_500));
        assert!(form.is_valid());
    }

    // ── Section 42(j)(5) ─────────────────────────────────────────────

    #[test]
    fn section_42j5_total_recapture() {
        let mut input = default_input();
        input.py_total_credits_on_form8586_amt = Usd::from_dollars(100_000);
        input.credit_recapture_pct_bps = 3333;
        input.decrease_in_qualified_basis_pct_bps = 10_000;
        input.section42j5_cd = "1".to_string();
        input.section42j5_interest_amt = Usd::from_dollars(2_000);
        let form = Output8611::try_new(input).unwrap();
        // line 7: 33,330; line 16: 2,000; line 17: 33,330 + 2,000 = 35,330
        assert_eq!(form.total_recapture_amt, Usd::from_dollars(35_330));
        assert!(form.is_valid());
    }

    // ── Percentage formatting ────────────────────────────────────────

    #[test]
    fn pct_string_whole_number() {
        assert_eq!(bps_to_pct_string(10_000), "100");
        assert_eq!(bps_to_pct_string(5_000), "50");
    }

    #[test]
    fn pct_string_with_decimals() {
        assert_eq!(bps_to_pct_string(3333), "33.33");
        assert_eq!(bps_to_pct_string(6667), "66.67");
    }

    // ── Zero everything ──────────────────────────────────────────────

    #[test]
    fn zero_everything() {
        let form = Output8611::try_new(default_input()).unwrap();
        assert_eq!(form.credits_subject_to_recapture_amt, Usd::ZERO);
        assert_eq!(form.accelerated_portion_of_credit_amt, Usd::ZERO);
        assert_eq!(form.accelerated_prtn_recaptured_amt, Usd::ZERO);
        assert_eq!(form.net_recapture_amt, Usd::ZERO);
        assert_eq!(form.recapture_tax_amt, Usd::ZERO);
        assert_eq!(form.carryforward_credit_amt, Usd::ZERO);
        assert_eq!(form.total_recapture_amt, Usd::ZERO);
        assert!(form.is_valid());
    }
}
