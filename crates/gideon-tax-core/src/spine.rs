use std::fmt;

use us_tax_brackets::{self, FilingStatus, TaxYear};

use crate::Usd;
use crate::input::{
    Core1099B, Core1099Div, Core1099G, Core1099Int, Core1099K, Core1099Misc, Core1099Nec,
    Core1099Oid, Core1099Patr, Core1099R, CoreW2, CoreW2G,
};
use crate::output::{Output1040, OutputSchedule1, OutputScheduleB};
use crate::rules::{DeductionParams, TaxYearRules};
use crate::types::Filer;

// ---------------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub struct SpineOutput {
    pub form_1040: Output1040,
    pub schedule_1: OutputSchedule1,
    pub schedule_b: OutputScheduleB,
}

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------

pub struct ReturnInput {
    pub tax_year: TaxYear,
    pub filing_status: FilingStatus,
    pub taxpayer: Filer,
    pub spouse: Option<Filer>,
    pub is_dependent: bool,
    pub is_dual_status_alien: bool,
    pub spouse_itemizes: bool,
    // Source documents
    pub w2s: Vec<CoreW2>,
    pub f1099_ints: Vec<Core1099Int>,
    pub f1099_divs: Vec<Core1099Div>,
    pub f1099_rs: Vec<Core1099R>,
    pub f1099_gs: Vec<Core1099G>,
    pub f1099_bs: Vec<Core1099B>,
    pub f1099_oids: Vec<Core1099Oid>,
    pub w2gs: Vec<CoreW2G>,
    pub f1099_necs: Vec<Core1099Nec>,
    pub f1099_miscs: Vec<Core1099Misc>,
    pub f1099_ks: Vec<Core1099K>,
    pub f1099_patrs: Vec<Core1099Patr>,
}

impl ReturnInput {
    fn deduction_params(&self) -> DeductionParams {
        DeductionParams {
            filing_status: self.filing_status,
            taxpayer: self.taxpayer,
            spouse: self.spouse,
            is_dependent: self.is_dependent,
            is_dual_status_alien: self.is_dual_status_alien,
            spouse_itemizes: self.spouse_itemizes,
            // TODO: include other earned income sources (self-employment, etc.)
            earned_income: self.w2s.iter().map(|w| w.wages_amt).sum(),
        }
    }
}

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

#[derive(Debug)]
pub enum SpineError {
    YearMismatch { input: TaxYear, rules: TaxYear },
    TaxComputeError(us_tax_brackets::TaxError),
}

impl From<us_tax_brackets::TaxError> for SpineError {
    fn from(e: us_tax_brackets::TaxError) -> Self {
        SpineError::TaxComputeError(e)
    }
}

impl fmt::Display for SpineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpineError::YearMismatch { input, rules } => {
                write!(f, "tax year mismatch: input={input}, rules={rules}")
            }
            SpineError::TaxComputeError(e) => write!(f, "tax computation error: {e}"),
        }
    }
}

impl std::error::Error for SpineError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpineError::TaxComputeError(e) => Some(e),
            _ => None,
        }
    }
}

// ---------------------------------------------------------------------------
// Spine
// ---------------------------------------------------------------------------

/// Computes the core Form 1040 workflow and returns a [`SpineOutput`].
///
/// Follows the standard 1040 flow:
/// Income → Adjustments → AGI → Deductions → Taxable Income → Regular Tax →
/// Additional Tax → Credits (nonrefundable / refundable) → Payments →
/// Refund or Amount Owed.
///
/// Returns [`SpineError::YearMismatch`] if `input.tax_year` differs from
/// `rules.year()`, or [`SpineError::TaxComputeError`] if the underlying
/// bracket lookup fails.
pub fn compute_spine(
    rules: &dyn TaxYearRules,
    input: &ReturnInput,
) -> Result<SpineOutput, SpineError> {
    if input.tax_year != rules.year() {
        return Err(SpineError::YearMismatch {
            input: input.tax_year,
            rules: rules.year(),
        });
    }

    let mut f1040 = Output1040::default();
    let mut sch1 = OutputSchedule1::default();
    let mut schb = OutputScheduleB::default();

    // =====================================================================
    // Schedule B — Part I: Interest
    // =====================================================================
    // Line 2: subtotal of all interest
    schb.interest_subtotal_amt = input
        .f1099_ints
        .iter()
        .map(|i| i.interest_income_amt)
        .sum::<Usd>()
        + input
            .f1099_oids
            .iter()
            .map(|o| o.original_issue_discount_amt + o.other_periodic_interest_amt)
            .sum::<Usd>();
    // Line 4: taxable interest (no excludable savings bond interest yet)
    schb.calculated_total_taxable_int_amt = schb.interest_subtotal_amt;

    // =====================================================================
    // Schedule B — Part II: Ordinary Dividends
    // =====================================================================
    // Line 5
    schb.ordinary_dividend_subtotal_amt = input
        .f1099_divs
        .iter()
        .map(|d| d.total_ordinary_dividends_amt)
        .sum();
    // Line 6
    schb.total_ordinary_dividends_amt = schb.ordinary_dividend_subtotal_amt;

    // =====================================================================
    // 1040 Line 1 — Wages
    // =====================================================================
    // Line 1a
    f1040.wages_amt = input.w2s.iter().map(|w| w.wages_amt).sum();
    // Lines 1b-1h are zero for now
    // Line 1z
    f1040.wages_salaries_and_tips_amt = f1040.wages_amt;

    // =====================================================================
    // 1040 Lines 2-3 — Interest & Dividends
    // =====================================================================
    // Line 2a: tax-exempt interest
    f1040.tax_exempt_interest_amt = input
        .f1099_ints
        .iter()
        .map(|i| i.tax_exempt_interest_amt)
        .sum::<Usd>()
        + input
            .f1099_divs
            .iter()
            .map(|d| d.exempt_interest_dividends_amt)
            .sum::<Usd>();
    // Line 2b
    f1040.taxable_interest_amt = schb.calculated_total_taxable_int_amt;
    // Line 3a
    f1040.qualified_dividends_amt = input
        .f1099_divs
        .iter()
        .map(|d| d.qualified_dividends_amt)
        .sum();
    // Line 3b
    f1040.ordinary_dividends_amt = schb.total_ordinary_dividends_amt;

    // =====================================================================
    // 1040 Lines 4-5 — IRA/Pension distributions
    // =====================================================================
    for r in &input.f1099_rs {
        let taxable = if r.txbl_amount_not_determined_ind {
            r.gross_distribution_amt
        } else {
            r.taxable_amt
        };
        if r.ira_sep_simple_ind {
            f1040.ira_distributions_amt = f1040.ira_distributions_amt + r.gross_distribution_amt;
            f1040.taxable_ira_amt = f1040.taxable_ira_amt + taxable;
        } else {
            f1040.pensions_annuities_amt = f1040.pensions_annuities_amt + r.gross_distribution_amt;
            f1040.total_taxable_pensions_amt = f1040.total_taxable_pensions_amt + taxable;
        }
    }

    // =====================================================================
    // 1040 Line 7 — Capital gains
    // =====================================================================
    let cap_gain_distributions: Usd = input
        .f1099_divs
        .iter()
        .map(|d| d.total_capital_distributions_amt)
        .sum();
    let brokerage_net: Usd = input
        .f1099_bs
        .iter()
        .map(|b| b.proceeds_amt - b.cost_or_other_basis_amt + b.nondeductible_wash_sale_loss_amt)
        .sum();
    f1040.capital_gain_loss_amt = cap_gain_distributions + brokerage_net;
    // Schedule D not required only when there are cap gain distributions but no 1099-Bs
    f1040.capital_distribution_ind =
        cap_gain_distributions != Usd::ZERO && input.f1099_bs.is_empty();
    // TODO: full Schedule D computation

    // =====================================================================
    // Schedule 1 — Additional Income (Part I)
    // =====================================================================
    // Line 1
    sch1.state_local_income_tax_refund_amt = input
        .f1099_gs
        .iter()
        .map(|g| g.state_lcl_refund_credit_offset_amt)
        .sum();
    // Line 7
    sch1.unemployment_comp_amt = input.f1099_gs.iter().map(|g| g.unemployment_comp_amt).sum();
    // Line 8b
    sch1.gambling_reportable_winning_amt = input
        .w2gs
        .iter()
        .map(|g| g.gambling_reportable_winning_amt)
        .sum();
    // Line 9: total of lines 8a through 8z (only 8b populated for now)
    sch1.other_income_total_amt = sch1.gambling_reportable_winning_amt;
    // Line 10: sum of lines 1-7 + line 9
    sch1.total_additional_income_amt = sch1.state_local_income_tax_refund_amt
        + sch1.unemployment_comp_amt
        + sch1.other_income_total_amt;

    // =====================================================================
    // 1040 Lines 8-11 — Total income, adjustments, AGI
    // =====================================================================
    // Line 8
    f1040.total_additional_income_amt = sch1.total_additional_income_amt;
    // Line 9
    f1040.total_income_amt = f1040.wages_salaries_and_tips_amt
        + f1040.taxable_interest_amt
        + f1040.ordinary_dividends_amt
        + f1040.taxable_ira_amt
        + f1040.total_taxable_pensions_amt
        + f1040.taxable_soc_sec_amt // zero for now
        + f1040.capital_gain_loss_amt
        + f1040.total_additional_income_amt;

    // Schedule 1 Part II — Adjustments
    // Line 18
    sch1.pnlty_on_erly_wthdrw_of_savings_amt = input
        .f1099_ints
        .iter()
        .map(|i| i.early_withdrawal_penalty_amt)
        .sum::<Usd>()
        + input
            .f1099_oids
            .iter()
            .map(|o| o.early_withdrawal_penalty_amt)
            .sum::<Usd>();
    // Line 26: sum of lines 11-23 + line 25 (only line 18 populated for now)
    sch1.total_adjustments_amt = sch1.pnlty_on_erly_wthdrw_of_savings_amt;

    // Line 10
    f1040.total_adjustments_amt = sch1.total_adjustments_amt;
    // Line 11a
    f1040.adjusted_gross_income_amt = f1040.total_income_amt - f1040.total_adjustments_amt;

    // =====================================================================
    // 1040 Lines 12-15 — Deductions and taxable income
    // =====================================================================
    // Line 12a-d indicators
    f1040.primary_claim_as_dependent_ind = input.is_dependent;
    f1040.must_itemize_ind = input.spouse_itemizes;
    f1040.dual_status_alien_ind = input.is_dual_status_alien;
    f1040.primary_65_or_older_ind = input.taxpayer.is_65_or_older;
    f1040.primary_blind_ind = input.taxpayer.is_blind;
    if let Some(sp) = &input.spouse {
        f1040.spouse_65_or_older_ind = sp.is_65_or_older;
        f1040.spouse_blind_ind = sp.is_blind;
    }
    f1040.total_boxes_checked_cnt =
        input.taxpayer.checked_boxes() as u8 + input.spouse.map_or(0, |s| s.checked_boxes() as u8);

    // Line 12e: standard deduction (TODO: choose between standard and itemized)
    f1040.total_itemized_or_standard_ded_amt = rules.standard_deduction(&input.deduction_params());
    // Line 14
    f1040.total_deductions_amt = f1040.total_itemized_or_standard_ded_amt
        + f1040.qualified_business_income_ded_amt
        + f1040.total_additional_deductions_amt;
    // Line 15
    f1040.taxable_income_amt =
        (f1040.adjusted_gross_income_amt - f1040.total_deductions_amt).max(Usd::ZERO);

    // =====================================================================
    // 1040 Lines 16-24 — Tax computation
    // =====================================================================
    // Line 16
    let taxable_whole_dollars: i64 = f1040.taxable_income_amt.irs_round().cents() / 100;
    let regular_tax_whole_dollars =
        us_tax_brackets::compute_tax(input.tax_year, input.filing_status, taxable_whole_dollars)?;
    f1040.tax_amt = Usd::from_dollars(regular_tax_whole_dollars);
    // Line 17: TODO: Schedule 2
    // Line 18
    f1040.total_tax_before_cr_and_oth_taxes_amt = f1040.tax_amt + f1040.additional_tax_amt;
    // Lines 19-21: TODO: credits
    f1040.total_credits_amt = f1040.ctc_odc_amt + f1040.total_nonrefundable_credits_amt;
    // Line 22
    f1040.tax_less_credits_amt =
        (f1040.total_tax_before_cr_and_oth_taxes_amt - f1040.total_credits_amt).max(Usd::ZERO);
    // Line 23: TODO: Schedule 2 other taxes
    // Line 24
    f1040.total_tax_amt = f1040.tax_less_credits_amt + f1040.total_other_taxes_amt;

    // =====================================================================
    // 1040 Lines 25-33 — Payments
    // =====================================================================
    // Line 25a
    f1040.form_w2_withheld_tax_amt = input.w2s.iter().map(|w| w.withholding_amt).sum();
    // Line 25b: withholding from all 1099s + W-2G
    f1040.form_1099_withheld_tax_amt = input
        .f1099_ints
        .iter()
        .map(|i| i.federal_income_tax_withheld_amt)
        .sum::<Usd>()
        + input
            .f1099_divs
            .iter()
            .map(|d| d.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .f1099_rs
            .iter()
            .map(|r| r.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .f1099_gs
            .iter()
            .map(|g| g.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .f1099_bs
            .iter()
            .map(|b| b.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .f1099_oids
            .iter()
            .map(|o| o.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .w2gs
            .iter()
            .map(|g| g.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .f1099_necs
            .iter()
            .map(|n| n.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .f1099_miscs
            .iter()
            .map(|m| m.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .f1099_ks
            .iter()
            .map(|k| k.federal_income_tax_withheld_amt)
            .sum::<Usd>()
        + input
            .f1099_patrs
            .iter()
            .map(|p| p.federal_income_tax_withheld_amt)
            .sum::<Usd>();
    // Line 25d
    f1040.withholding_tax_amt = f1040.form_w2_withheld_tax_amt
        + f1040.form_1099_withheld_tax_amt
        + f1040.tax_withheld_other_amt;
    // Line 32: TODO: refundable credits
    // Line 33
    f1040.total_payments_amt =
        f1040.withholding_tax_amt + f1040.estimated_tax_payments_amt + f1040.refundable_credits_amt;

    // =====================================================================
    // 1040 Lines 34-38 — Refund or amount owed
    // =====================================================================
    // Line 34
    f1040.overpaid_amt = (f1040.total_payments_amt - f1040.total_tax_amt).max(Usd::ZERO);
    // Line 35a
    f1040.refund_amt = f1040.overpaid_amt;
    // Line 37
    f1040.owed_amt = (f1040.total_tax_amt - f1040.total_payments_amt).max(Usd::ZERO);

    Ok(SpineOutput {
        form_1040: f1040,
        schedule_1: sch1,
        schedule_b: schb,
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::y2025::Rules2025;

    fn base_input() -> ReturnInput {
        ReturnInput {
            tax_year: TaxYear::Y2025,
            filing_status: FilingStatus::Single,
            taxpayer: Filer::default(),
            spouse: None,
            is_dependent: false,
            is_dual_status_alien: false,
            spouse_itemizes: false,
            w2s: Vec::new(),
            f1099_ints: Vec::new(),
            f1099_divs: Vec::new(),
            f1099_rs: Vec::new(),
            f1099_gs: Vec::new(),
            f1099_bs: Vec::new(),
            f1099_oids: Vec::new(),
            w2gs: Vec::new(),
            f1099_necs: Vec::new(),
            f1099_miscs: Vec::new(),
            f1099_ks: Vec::new(),
            f1099_patrs: Vec::new(),
        }
    }

    fn input(wages: i64, withholding: i64) -> ReturnInput {
        let mut inp = base_input();
        inp.w2s.push(CoreW2 {
            wages_amt: Usd::from_dollars(wages),
            withholding_amt: Usd::from_dollars(withholding),
            ..CoreW2::default()
        });
        inp
    }

    #[test]
    fn year_mismatch() {
        let mut inp = base_input();
        inp.tax_year = TaxYear::Y2024;
        inp.w2s.push(CoreW2 {
            wages_amt: Usd::from_dollars(50_000),
            ..CoreW2::default()
        });
        let err = compute_spine(&Rules2025, &inp).unwrap_err();
        assert!(matches!(
            err,
            SpineError::YearMismatch {
                input: TaxYear::Y2024,
                rules: TaxYear::Y2025,
            }
        ));
    }

    #[test]
    fn wages_below_deduction_full_refund() {
        let out = compute_spine(&Rules2025, &input(10_000, 2_000)).unwrap();
        assert_eq!(out.form_1040.taxable_income_amt, Usd::ZERO);
        assert_eq!(out.form_1040.tax_amt, Usd::ZERO);
        assert_eq!(out.form_1040.total_tax_amt, Usd::ZERO);
        assert_eq!(out.form_1040.refund_amt, Usd::from_dollars(2_000));
        assert_eq!(out.form_1040.owed_amt, Usd::ZERO);
    }

    #[test]
    fn wages_above_deduction_no_withholding_owes() {
        let out = compute_spine(&Rules2025, &input(50_000, 0)).unwrap();
        assert!(out.form_1040.taxable_income_amt > Usd::ZERO);
        assert!(out.form_1040.tax_amt > Usd::ZERO);
        assert_eq!(out.form_1040.refund_amt, Usd::ZERO);
        assert!(out.form_1040.owed_amt > Usd::ZERO);
        assert_eq!(out.form_1040.owed_amt, out.form_1040.total_tax_amt);
    }

    #[test]
    fn withholding_exceeds_tax_refund() {
        let out = compute_spine(&Rules2025, &input(50_000, 10_000)).unwrap();
        let tax = out.form_1040.total_tax_amt;
        assert!(tax > Usd::ZERO);
        assert!(Usd::from_dollars(10_000) > tax);
        assert!(out.form_1040.refund_amt > Usd::ZERO);
        assert_eq!(out.form_1040.owed_amt, Usd::ZERO);
        assert_eq!(out.form_1040.refund_amt, Usd::from_dollars(10_000) - tax,);
    }

    #[test]
    fn zero_wages_zero_withholding() {
        let out = compute_spine(&Rules2025, &input(0, 0)).unwrap();
        assert_eq!(out.form_1040.total_income_amt, Usd::ZERO);
        assert_eq!(out.form_1040.taxable_income_amt, Usd::ZERO);
        assert_eq!(out.form_1040.total_tax_amt, Usd::ZERO);
        assert_eq!(out.form_1040.refund_amt, Usd::ZERO);
        assert_eq!(out.form_1040.owed_amt, Usd::ZERO);
    }

    // -----------------------------------------------------------------------
    // New tests
    // -----------------------------------------------------------------------

    #[test]
    fn interest_income() {
        let mut inp = base_input();
        inp.f1099_ints.push(Core1099Int {
            interest_income_amt: Usd::from_dollars(500),
            ..Core1099Int::default()
        });
        inp.f1099_ints.push(Core1099Int {
            interest_income_amt: Usd::from_dollars(300),
            ..Core1099Int::default()
        });
        let out = compute_spine(&Rules2025, &inp).unwrap();
        assert_eq!(out.schedule_b.interest_subtotal_amt, Usd::from_dollars(800));
        assert_eq!(
            out.schedule_b.calculated_total_taxable_int_amt,
            Usd::from_dollars(800)
        );
        assert_eq!(out.form_1040.taxable_interest_amt, Usd::from_dollars(800));
    }

    #[test]
    fn dividend_income() {
        let mut inp = base_input();
        inp.f1099_divs.push(Core1099Div {
            total_ordinary_dividends_amt: Usd::from_dollars(1_000),
            qualified_dividends_amt: Usd::from_dollars(600),
            ..Core1099Div::default()
        });
        let out = compute_spine(&Rules2025, &inp).unwrap();
        assert_eq!(
            out.schedule_b.ordinary_dividend_subtotal_amt,
            Usd::from_dollars(1_000)
        );
        assert_eq!(
            out.schedule_b.total_ordinary_dividends_amt,
            Usd::from_dollars(1_000)
        );
        assert_eq!(
            out.form_1040.qualified_dividends_amt,
            Usd::from_dollars(600)
        );
        assert_eq!(
            out.form_1040.ordinary_dividends_amt,
            Usd::from_dollars(1_000)
        );
    }

    #[test]
    fn ira_and_pension_distributions() {
        let mut inp = base_input();
        // IRA distribution
        inp.f1099_rs.push(Core1099R {
            gross_distribution_amt: Usd::from_dollars(10_000),
            taxable_amt: Usd::from_dollars(8_000),
            ira_sep_simple_ind: true,
            ..Core1099R::default()
        });
        // Non-IRA pension
        inp.f1099_rs.push(Core1099R {
            gross_distribution_amt: Usd::from_dollars(20_000),
            taxable_amt: Usd::from_dollars(15_000),
            ira_sep_simple_ind: false,
            ..Core1099R::default()
        });
        let out = compute_spine(&Rules2025, &inp).unwrap();
        assert_eq!(
            out.form_1040.ira_distributions_amt,
            Usd::from_dollars(10_000)
        );
        assert_eq!(out.form_1040.taxable_ira_amt, Usd::from_dollars(8_000));
        assert_eq!(
            out.form_1040.pensions_annuities_amt,
            Usd::from_dollars(20_000)
        );
        assert_eq!(
            out.form_1040.total_taxable_pensions_amt,
            Usd::from_dollars(15_000)
        );
    }

    #[test]
    fn schedule_1_unemployment_and_refund() {
        let mut inp = base_input();
        inp.f1099_gs.push(Core1099G {
            unemployment_comp_amt: Usd::from_dollars(5_000),
            state_lcl_refund_credit_offset_amt: Usd::from_dollars(1_200),
            ..Core1099G::default()
        });
        let out = compute_spine(&Rules2025, &inp).unwrap();
        assert_eq!(
            out.schedule_1.state_local_income_tax_refund_amt,
            Usd::from_dollars(1_200)
        );
        assert_eq!(
            out.schedule_1.unemployment_comp_amt,
            Usd::from_dollars(5_000)
        );
        assert_eq!(
            out.schedule_1.total_additional_income_amt,
            Usd::from_dollars(6_200)
        );
        assert_eq!(
            out.form_1040.total_additional_income_amt,
            Usd::from_dollars(6_200)
        );
    }

    #[test]
    fn early_withdrawal_penalty() {
        let mut inp = base_input();
        inp.f1099_ints.push(Core1099Int {
            interest_income_amt: Usd::from_dollars(1_000),
            early_withdrawal_penalty_amt: Usd::from_dollars(150),
            ..Core1099Int::default()
        });
        let out = compute_spine(&Rules2025, &inp).unwrap();
        assert_eq!(
            out.schedule_1.pnlty_on_erly_wthdrw_of_savings_amt,
            Usd::from_dollars(150)
        );
        assert_eq!(out.schedule_1.total_adjustments_amt, Usd::from_dollars(150));
        assert_eq!(out.form_1040.total_adjustments_amt, Usd::from_dollars(150));
    }

    #[test]
    fn form_1099_withholding_aggregation() {
        let mut inp = base_input();
        inp.f1099_ints.push(Core1099Int {
            interest_income_amt: Usd::from_dollars(100),
            federal_income_tax_withheld_amt: Usd::from_dollars(10),
            ..Core1099Int::default()
        });
        inp.f1099_divs.push(Core1099Div {
            total_ordinary_dividends_amt: Usd::from_dollars(200),
            federal_income_tax_withheld_amt: Usd::from_dollars(20),
            ..Core1099Div::default()
        });
        inp.f1099_rs.push(Core1099R {
            gross_distribution_amt: Usd::from_dollars(5_000),
            taxable_amt: Usd::from_dollars(5_000),
            federal_income_tax_withheld_amt: Usd::from_dollars(500),
            ..Core1099R::default()
        });
        let out = compute_spine(&Rules2025, &inp).unwrap();
        assert_eq!(
            out.form_1040.form_1099_withheld_tax_amt,
            Usd::from_dollars(530)
        );
    }

    #[test]
    fn multiple_w2s() {
        let mut inp = base_input();
        inp.w2s.push(CoreW2 {
            wages_amt: Usd::from_dollars(30_000),
            withholding_amt: Usd::from_dollars(3_000),
            ..CoreW2::default()
        });
        inp.w2s.push(CoreW2 {
            wages_amt: Usd::from_dollars(20_000),
            withholding_amt: Usd::from_dollars(2_000),
            ..CoreW2::default()
        });
        let out = compute_spine(&Rules2025, &inp).unwrap();
        assert_eq!(out.form_1040.wages_amt, Usd::from_dollars(50_000));
        assert_eq!(
            out.form_1040.form_w2_withheld_tax_amt,
            Usd::from_dollars(5_000)
        );
        assert_eq!(out.form_1040.withholding_tax_amt, Usd::from_dollars(5_000));
    }
}
