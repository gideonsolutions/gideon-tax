use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::rules::TaxYearRules;
use crate::rules::y2025::Rules2025;
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Schedule SE (Form 1040).
///
/// Farm/non-farm profit from Schedule C/F, W-2 church wages, SST wages,
/// unreported tips from Form 4137, wages from Form 8919, and optional
/// method inputs feed into the computation.
#[derive(Debug, Clone)]
pub struct ScheduleSeInput {
    /// Name of person with self-employment income
    pub person_nm: String,
    /// Social security number of person with self-employment income
    pub ssn: String,
    /// Line A: Filed Form 4361 but had $400+ of other net SE earnings
    pub exempt_form_4361_ind: bool,
    /// Line 1a: Net farm profit or (loss) from Schedule F, line 34, and
    /// farm partnerships, Schedule K-1 (Form 1065), box 14, code A
    pub net_farm_profit_loss_amt: Usd,
    /// Line 1b: Conservation Reserve Program payments (positive amount;
    /// subtracted when computing line 3)
    pub conservation_reserve_prog_pymt_amt: Usd,
    /// Line 2: Net profit or (loss) from Schedule C, line 31; and
    /// Schedule K-1 (Form 1065), box 14, code A (other than farming)
    pub net_non_farm_profit_loss_amt: Usd,
    /// Line 5a: Church employee income from Form W-2
    pub w2_wages_from_churches_amt: Usd,
    /// Line 8a: Total social security wages and tips from Form(s) W-2
    /// (boxes 3 and 7) and railroad retirement (tier 1) compensation
    pub sst_wages_rrt_comp_amt: Usd,
    /// Line 8b: Unreported tips from Form 4137, line 10
    pub unreported_tips_amt: Usd,
    /// Line 8c: Wages subject to social security tax from Form 8919, line 10
    pub wages_subject_to_sst_amt: Usd,
    /// Gross farm income (for Part II farm optional method, line 15)
    pub gross_farm_income: Usd,
    /// Gross nonfarm income (for Part II nonfarm optional method, line 17)
    pub gross_nonfarm_income: Usd,
    /// Whether the taxpayer elects the farm optional method
    pub use_farm_optional_method: bool,
    /// Whether the taxpayer elects the nonfarm optional method
    pub use_nonfarm_optional_method: bool,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Schedule SE (Form 1040) — Self-Employment Tax (2025).
#[derive(Debug, Clone, Default)]
pub struct OutputScheduleSe {
    // -----------------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------------
    /// Name of person with self-employment income (as shown on Form 1040,
    /// 1040-SR, 1040-SS, or 1040-NR)
    pub person_nm: String,
    /// Social security number of person with self-employment income
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — Self-Employment Tax
    // -----------------------------------------------------------------------
    /// Line A: If you are a minister, member of a religious order, or Christian
    /// Science practitioner and you filed Form 4361, but you had $400 or more
    /// of other net earnings from self-employment, check here and continue with
    /// Part I
    pub exempt_form_4361_ind: bool,
    /// Line 1a: Net farm profit or (loss) from Schedule F, line 34, and farm
    /// partnerships, Schedule K-1 (Form 1065), box 14, code A
    pub net_farm_profit_loss_amt: Usd,
    /// Line 1b: If you received social security retirement or disability
    /// benefits, enter the amount of Conservation Reserve Program payments
    /// included on Schedule F, line 4b, or listed on Schedule K-1 (Form 1065),
    /// box 20, code AQ
    pub conservation_reserve_prog_pymt_amt: Usd,
    /// Line 2: Net profit or (loss) from Schedule C, line 31; and Schedule K-1
    /// (Form 1065), box 14, code A (other than farming). See instructions for
    /// other income to report or if you are a minister or member of a religious
    /// order
    pub net_non_farm_profit_loss_amt: Usd,
    /// Line 3: Combine lines 1a, 1b, and 2
    pub se_total_net_earnings_or_loss_amt: Usd,
    /// Line 4a: If line 3 is more than zero, multiply line 3 by 92.35%
    /// (0.9235). Otherwise, enter amount from line 3
    pub minimum_profit_for_se_tax_amt: Usd,
    /// Line 4b: If you elect one or both of the optional methods, enter the
    /// total of lines 15 and 17 here
    pub optional_method_amt: Usd,
    /// Line 4c: Combine lines 4a and 4b. If less than $400, stop; you don't
    /// owe self-employment tax. Exception: If less than $400 and you had church
    /// employee income, enter -0- and continue
    pub combined_se_amt: Usd,
    /// Line 5a: Enter your church employee income from Form W-2. See
    /// instructions for definition of church employee income
    pub w2_wages_from_churches_amt: Option<Usd>,
    /// Line 5b: Multiply line 5a by 92.35% (0.9235). If less than $100,
    /// enter -0-
    pub min_allowable_church_wages_amt: Option<Usd>,
    /// Line 6: Add lines 4c and 5b
    pub combined_se_and_church_wages_amt: Option<Usd>,
    /// Line 7: Maximum amount of combined wages and self-employment earnings
    /// subject to social security tax or the 6.2% portion of the 7.65%
    /// railroad retirement (tier 1) tax for 2025
    pub se_base_amt: Option<Usd>,
    /// Line 8a: Total social security wages and tips (total of boxes 3 and 7
    /// on Form(s) W-2) and railroad retirement (tier 1) compensation. If
    /// $176,100 or more, skip lines 8b through 10, and go to line 11
    pub sst_wages_rrt_comp_amt: Option<Usd>,
    /// Line 8b: Unreported tips subject to social security tax from Form 4137,
    /// line 10
    pub unreported_tips_amt: Option<Usd>,
    /// Line 8c: Wages subject to social security tax from Form 8919, line 10
    pub wages_subject_to_sst_amt: Option<Usd>,
    /// Line 8d: Add lines 8a, 8b, and 8c
    pub total_wages_and_unreported_tips_amt: Option<Usd>,
    /// Line 9: Subtract line 8d from line 7. If zero or less, enter -0- here
    /// and on line 10 and go to line 11
    pub tax_base_amt: Option<Usd>,
    /// Line 10: Multiply the smaller of line 6 or line 9 by 12.4% (0.124)
    pub allowable_se_amt: Option<Usd>,
    /// Line 11: Multiply line 6 by 2.9% (0.029)
    pub medicare_tax_amt: Option<Usd>,
    /// Line 12: Self-employment tax. Add lines 10 and 11. Enter here and on
    /// Schedule 2 (Form 1040), line 4, or Form 1040-SS, Part I, line 3
    pub self_employment_tax_amt: Usd,
    /// Line 13: Deduction for one-half of self-employment tax. Multiply
    /// line 12 by 50% (0.50). Enter here and on Schedule 1 (Form 1040),
    /// line 15
    pub deductible_self_employment_tax_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Optional Methods To Figure Net Earnings
    // -----------------------------------------------------------------------
    /// Line 15: Enter the smaller of: two-thirds (2/3) of gross farm income
    /// (not less than zero) or $7,240. Also, include this amount on line 4b
    /// above
    pub se_tax_farm_optional_method_amt: Option<Usd>,
    /// Line 16: Subtract line 15 from line 14
    pub se_tax_non_farm_optional_base_amt: Option<Usd>,
    /// Line 17: Enter the smaller of: two-thirds (2/3) of gross nonfarm income
    /// (not less than zero) or the amount on line 16. Also, include this amount
    /// on line 4b above
    pub se_tax_non_farm_optional_method_amt: Option<Usd>,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for OutputScheduleSe {
    fn name() -> &'static str {
        "Schedule SE"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for OutputScheduleSe {
    type Input = ScheduleSeInput;

    fn must_file(input: &Self::Input) -> bool {
        let ss_bps = Rules2025::SOCIAL_SECURITY_RATE_BPS as i64;
        let med_bps = Rules2025::MEDICARE_RATE_BPS as i64;
        let net_factor_bps: i64 = 10_000 - ss_bps - med_bps;
        let opt_max = Rules2025::SE_FARM_OPTIONAL_METHOD_MAX;

        // Line 3: combine farm profit, CRP payments (subtracted), and non-farm
        let line3 = input.net_farm_profit_loss_amt - input.conservation_reserve_prog_pymt_amt
            + input.net_non_farm_profit_loss_amt;

        // Line 4a
        let line4a = if line3 > Usd::ZERO {
            Usd::from_cents(line3.cents() * net_factor_bps / 10_000)
        } else {
            line3
        };

        // Line 4b: optional method amounts
        let line15 = if input.use_farm_optional_method {
            let two_thirds =
                Usd::from_cents(input.gross_farm_income.max(Usd::ZERO).cents() * 2 / 3);
            two_thirds.min(opt_max)
        } else {
            Usd::ZERO
        };
        let line17 = if input.use_nonfarm_optional_method {
            let two_thirds =
                Usd::from_cents(input.gross_nonfarm_income.max(Usd::ZERO).cents() * 2 / 3);
            two_thirds.min(opt_max - line15)
        } else {
            Usd::ZERO
        };
        let line4b = line15 + line17;

        // Line 4c
        let line4c = line4a + line4b;

        let se_min = Rules2025::SE_MIN_NET_EARNINGS;

        // Standard filing threshold
        if line4c >= se_min {
            return true;
        }

        // Church employee income threshold
        if input.w2_wages_from_churches_amt >= Rules2025::SE_MIN_CHURCH_EMPLOYEE_INCOME {
            return true;
        }

        // CRP special rule: if line 1b exists and both 4a and 4c < $400,
        // must file if (line 1a + line 2) >= $434
        if input.conservation_reserve_prog_pymt_amt > Usd::ZERO
            && line4a < se_min
            && line4c < se_min
        {
            let gross = input.net_farm_profit_loss_amt + input.net_non_farm_profit_loss_amt;
            return gross >= Rules2025::SE_CRP_GROSS_THRESHOLD;
        }

        false
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        let wage_base = Rules2025::SOCIAL_SECURITY_WAGE_BASE;
        let ss_rate_bps = Rules2025::SOCIAL_SECURITY_RATE_BPS as i64;
        let med_rate_bps = Rules2025::MEDICARE_RATE_BPS as i64;
        let se_min = Rules2025::SE_MIN_NET_EARNINGS;
        let church_min = Rules2025::SE_MIN_CHURCH_WAGES;
        let opt_max = Rules2025::SE_FARM_OPTIONAL_METHOD_MAX;

        // Net earnings factor: 100% - (SS rate + Medicare rate) = 92.35%
        let net_factor_bps: i64 = 10_000 - ss_rate_bps - med_rate_bps;
        // SE rates are double the employee rates
        let se_ss_bps: i64 = 2 * ss_rate_bps; // 12.4%
        let se_med_bps: i64 = 2 * med_rate_bps; // 2.9%

        // ── Part II (compute first, feeds into line 4b) ──────────────
        // Line 14 = opt_max ($7,240)
        let (out_line15, out_line16, out_line17, line4b);

        let farm_opt = if input.use_farm_optional_method {
            let two_thirds =
                Usd::from_cents(input.gross_farm_income.max(Usd::ZERO).cents() * 2 / 3);
            Some(two_thirds.min(opt_max))
        } else {
            None
        };

        let nonfarm_opt = if input.use_nonfarm_optional_method {
            let line15_val = farm_opt.unwrap_or(Usd::ZERO);
            let base = opt_max - line15_val;
            let two_thirds =
                Usd::from_cents(input.gross_nonfarm_income.max(Usd::ZERO).cents() * 2 / 3);
            Some((base, two_thirds.min(base)))
        } else {
            None
        };

        out_line15 = farm_opt;
        (out_line16, out_line17) = nonfarm_opt.map_or((None, None), |(b, a)| (Some(b), Some(a)));
        line4b = farm_opt.unwrap_or(Usd::ZERO) + out_line17.unwrap_or(Usd::ZERO);

        // ── Part I ───────────────────────────────────────────────────
        let line1a = input.net_farm_profit_loss_amt;
        let line1b = input.conservation_reserve_prog_pymt_amt;
        let line2 = input.net_non_farm_profit_loss_amt;

        // Line 3: Combine 1a, 1b (subtracted), and 2
        let line3 = line1a - line1b + line2;

        // Line 4a: if line 3 > 0, multiply by 92.35%; otherwise pass through
        let line4a = if line3 > Usd::ZERO {
            Usd::from_cents(line3.cents() * net_factor_bps / 10_000)
        } else {
            line3
        };

        // Line 4c: combine 4a and 4b
        let has_church_income = input.w2_wages_from_churches_amt > Usd::ZERO;
        let line4c = line4a + line4b;

        // If line 4c < $400 and no church income → stop; lines 5–11 = None,
        // lines 12–13 = $0
        let (out5a, out5b, out6, out7, out8a, out8b, out8c, out8d, out9, out10, out11);
        let (line12, line13);

        if line4c < se_min && !has_church_income {
            out5a = None;
            out5b = None;
            out6 = None;
            out7 = None;
            out8a = None;
            out8b = None;
            out8c = None;
            out8d = None;
            out9 = None;
            out10 = None;
            out11 = None;
            line12 = Usd::ZERO;
            line13 = Usd::ZERO;
        } else {
            // When line 4c < $400 with church income, enter -0- for line 4c
            // in the line 6 computation and continue
            let line4c_for_line6 = if line4c < se_min { Usd::ZERO } else { line4c };

            // Line 5a: church employee income
            let line5a = input.w2_wages_from_churches_amt;

            // Line 5b: line 5a * 92.35%, if < $100 enter -0-
            let raw_5b = Usd::from_cents(line5a.cents() * net_factor_bps / 10_000);
            let line5b = if raw_5b < church_min {
                Usd::ZERO
            } else {
                raw_5b
            };

            // Line 6: add 4c and 5b
            let line6 = line4c_for_line6 + line5b;

            // Line 7: social security wage base
            let line7 = wage_base;

            // Lines 8a-8d
            let line8a = input.sst_wages_rrt_comp_amt;
            let line8b = input.unreported_tips_amt;
            let line8c = input.wages_subject_to_sst_amt;
            let line8d = line8a + line8b + line8c;

            // Line 9: line 7 - line 8d, floor at 0
            let line9 = (line7 - line8d).max(Usd::ZERO);

            // Line 10: multiply smaller of line 6 or line 9 by 12.4%
            let line10 = if line9 == Usd::ZERO {
                Usd::ZERO
            } else {
                Usd::from_cents(line6.min(line9).cents() * se_ss_bps / 10_000)
            };

            // Line 11: line 6 * 2.9%
            let line11 = Usd::from_cents(line6.cents() * se_med_bps / 10_000);

            out5a = Some(line5a);
            out5b = Some(line5b);
            out6 = Some(line6);
            out7 = Some(line7);
            out8a = Some(line8a);
            out8b = Some(line8b);
            out8c = Some(line8c);
            out8d = Some(line8d);
            out9 = Some(line9);
            out10 = Some(line10);
            out11 = Some(line11);

            // Line 12: line 10 + line 11
            line12 = line10 + line11;

            // Line 13: line 12 * 50%
            line13 = Usd::from_cents(line12.cents() / 2);
        }

        Ok(OutputScheduleSe {
            // Header
            person_nm: input.person_nm,
            ssn: input.ssn,

            // Part I
            exempt_form_4361_ind: input.exempt_form_4361_ind,
            net_farm_profit_loss_amt: line1a,
            conservation_reserve_prog_pymt_amt: line1b,
            net_non_farm_profit_loss_amt: line2,
            se_total_net_earnings_or_loss_amt: line3,
            minimum_profit_for_se_tax_amt: line4a,
            optional_method_amt: line4b,
            combined_se_amt: line4c,
            w2_wages_from_churches_amt: out5a,
            min_allowable_church_wages_amt: out5b,
            combined_se_and_church_wages_amt: out6,
            se_base_amt: out7,
            sst_wages_rrt_comp_amt: out8a,
            unreported_tips_amt: out8b,
            wages_subject_to_sst_amt: out8c,
            total_wages_and_unreported_tips_amt: out8d,
            tax_base_amt: out9,
            allowable_se_amt: out10,
            medicare_tax_amt: out11,
            self_employment_tax_amt: line12,
            deductible_self_employment_tax_amt: line13,

            // Part II
            se_tax_farm_optional_method_amt: out_line15,
            se_tax_non_farm_optional_base_amt: out_line16,
            se_tax_non_farm_optional_method_amt: out_line17,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[
            DynForm::ScheduleC,
            DynForm::ScheduleF,
            DynForm::W2,
            DynForm::F4137,
            DynForm::F8919,
            DynForm::F1065ScheduleK1,
        ]
    }

    fn is_valid(&self) -> bool {
        let ss_rate_bps = Rules2025::SOCIAL_SECURITY_RATE_BPS as i64;
        let med_rate_bps = Rules2025::MEDICARE_RATE_BPS as i64;
        let net_factor_bps: i64 = 10_000 - ss_rate_bps - med_rate_bps;
        let se_ss_bps: i64 = 2 * ss_rate_bps;
        let se_med_bps: i64 = 2 * med_rate_bps;
        let se_min = Rules2025::SE_MIN_NET_EARNINGS;
        let church_min = Rules2025::SE_MIN_CHURCH_WAGES;
        let opt_max = Rules2025::SE_FARM_OPTIONAL_METHOD_MAX;

        // Line 3 = 1a - 1b + 2
        let line3 = self.net_farm_profit_loss_amt - self.conservation_reserve_prog_pymt_amt
            + self.net_non_farm_profit_loss_amt;
        let line3_ok = self.se_total_net_earnings_or_loss_amt == line3;

        // Line 4a
        let expected_4a = if line3 > Usd::ZERO {
            Usd::from_cents(line3.cents() * net_factor_bps / 10_000)
        } else {
            line3
        };
        let line4a_ok = self.minimum_profit_for_se_tax_amt == expected_4a;

        // Line 4c = 4a + 4b (always the real value)
        let expected_4c = self.minimum_profit_for_se_tax_amt + self.optional_method_amt;
        let line4c_ok = self.combined_se_amt == expected_4c;

        // Lines 5–11: None when stopped (4c < $400, no church income)
        let lines5_11_ok = match (
            self.w2_wages_from_churches_amt,
            self.min_allowable_church_wages_amt,
            self.combined_se_and_church_wages_amt,
            self.se_base_amt,
            self.sst_wages_rrt_comp_amt,
            self.unreported_tips_amt,
            self.wages_subject_to_sst_amt,
            self.total_wages_and_unreported_tips_amt,
            self.tax_base_amt,
            self.allowable_se_amt,
            self.medicare_tax_amt,
        ) {
            (
                Some(l5a),
                Some(l5b),
                Some(l6),
                Some(l7),
                Some(l8a),
                Some(l8b),
                Some(l8c),
                Some(l8d),
                Some(l9),
                Some(l10),
                Some(l11),
            ) => {
                let raw_5b = Usd::from_cents(l5a.cents() * net_factor_bps / 10_000);
                let exp_5b = if raw_5b < church_min {
                    Usd::ZERO
                } else {
                    raw_5b
                };
                let l5b_ok = l5b == exp_5b;

                let has_church = l5a > Usd::ZERO;
                let l4c_for_l6 = if expected_4c < se_min && has_church {
                    Usd::ZERO
                } else {
                    expected_4c
                };
                let l6_ok = l6 == l4c_for_l6 + l5b;
                let l7_ok = l7 == Rules2025::SOCIAL_SECURITY_WAGE_BASE;
                let l8d_ok = l8d == l8a + l8b + l8c;
                let l9_ok = l9 == (l7 - l8d).max(Usd::ZERO);
                let exp_10 = if l9 == Usd::ZERO {
                    Usd::ZERO
                } else {
                    Usd::from_cents(l6.min(l9).cents() * se_ss_bps / 10_000)
                };
                let l10_ok = l10 == exp_10;
                let l11_ok = l11 == Usd::from_cents(l6.cents() * se_med_bps / 10_000);

                let l12_ok = self.self_employment_tax_amt == l10 + l11;
                let l13_ok = self.deductible_self_employment_tax_amt
                    == Usd::from_cents(self.self_employment_tax_amt.cents() / 2);

                l5b_ok && l6_ok && l7_ok && l8d_ok && l9_ok && l10_ok && l11_ok && l12_ok && l13_ok
            }
            (None, None, None, None, None, None, None, None, None, None, None) => {
                // Stopped: lines 12–13 must be zero
                self.self_employment_tax_amt == Usd::ZERO
                    && self.deductible_self_employment_tax_amt == Usd::ZERO
            }
            _ => false, // Inconsistent: some None, some Some
        };

        // Part II: farm and nonfarm optional methods are independent
        let l15 = self.se_tax_farm_optional_method_amt.unwrap_or(Usd::ZERO);
        let l17 = self
            .se_tax_non_farm_optional_method_amt
            .unwrap_or(Usd::ZERO);
        let line4b_ok = self.optional_method_amt == l15 + l17;

        let line16_ok = match self.se_tax_non_farm_optional_base_amt {
            Some(l16) => l16 == opt_max - l15,
            None => self.se_tax_non_farm_optional_method_amt.is_none(),
        };

        // line 16/17 must both be Some or both None
        let nonfarm_consistent = self.se_tax_non_farm_optional_base_amt.is_some()
            == self.se_tax_non_farm_optional_method_amt.is_some();

        let part2_ok = line4b_ok && line16_ok && nonfarm_consistent;

        line3_ok && line4a_ok && line4c_ok && lines5_11_ok && part2_ok
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn default_input() -> ScheduleSeInput {
        ScheduleSeInput {
            person_nm: "Jane Doe".to_string(),
            ssn: "123-45-6789".to_string(),
            exempt_form_4361_ind: false,
            net_farm_profit_loss_amt: Usd::ZERO,
            conservation_reserve_prog_pymt_amt: Usd::ZERO,
            net_non_farm_profit_loss_amt: Usd::ZERO,
            w2_wages_from_churches_amt: Usd::ZERO,
            sst_wages_rrt_comp_amt: Usd::ZERO,
            unreported_tips_amt: Usd::ZERO,
            wages_subject_to_sst_amt: Usd::ZERO,
            gross_farm_income: Usd::ZERO,
            gross_nonfarm_income: Usd::ZERO,
            use_farm_optional_method: false,
            use_nonfarm_optional_method: false,
        }
    }

    // ── must_file ─────────────────────────────────────────────────────

    #[test]
    fn must_file_nonfarm_above_threshold() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(50_000);
        assert!(OutputScheduleSe::must_file(&input));
    }

    #[test]
    fn must_file_below_threshold() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(300);
        // 300 * 0.9235 = $277.05 < $400
        assert!(!OutputScheduleSe::must_file(&input));
    }

    #[test]
    fn must_file_church_income_only() {
        let mut input = default_input();
        input.w2_wages_from_churches_amt = Usd::from_dollars(500);
        assert!(OutputScheduleSe::must_file(&input));
    }

    #[test]
    fn must_file_church_income_below_threshold() {
        let mut input = default_input();
        // $108.27 < $108.28 threshold
        input.w2_wages_from_churches_amt = Usd::from_cents(10_827);
        assert!(!OutputScheduleSe::must_file(&input));
    }

    #[test]
    fn must_file_church_income_at_threshold() {
        let mut input = default_input();
        input.w2_wages_from_churches_amt = Usd::from_cents(10_828);
        assert!(OutputScheduleSe::must_file(&input));
    }

    #[test]
    fn must_file_crp_gross_above_434() {
        let mut input = default_input();
        // line 1a = 500, line 1b = 200, line 2 = 0
        // line 3 = 500 - 200 = 300; line 4a = 300 * 0.9235 = 277.05 < 400
        // line 4c = 277.05 < 400
        // But (1a + 2) = 500 >= 434 → must file
        input.net_farm_profit_loss_amt = Usd::from_dollars(500);
        input.conservation_reserve_prog_pymt_amt = Usd::from_dollars(200);
        assert!(OutputScheduleSe::must_file(&input));
    }

    #[test]
    fn must_file_crp_gross_below_434() {
        let mut input = default_input();
        // line 1a = 400, line 1b = 200, line 2 = 0
        // line 3 = 200; line 4a = 184.70 < 400; line 4c = 184.70 < 400
        // (1a + 2) = 400 < 434 → don't file
        input.net_farm_profit_loss_amt = Usd::from_dollars(400);
        input.conservation_reserve_prog_pymt_amt = Usd::from_dollars(200);
        assert!(!OutputScheduleSe::must_file(&input));
    }

    #[test]
    fn must_file_loss() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_cents(-10_000);
        assert!(!OutputScheduleSe::must_file(&input));
    }

    // ── Line 4c below $400 → no SE tax ─────────────────────────────────

    #[test]
    fn line4c_below_400_no_se_tax() {
        let mut input = default_input();
        // 300 * 0.9235 = $277.05 → line 4c < $400 → stopped
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(300);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 4c shows the real value
        assert_eq!(
            form.combined_se_amt,
            Usd::from_cents(30_000 * 9235 / 10_000)
        );
        // Lines 5–11 are None
        assert_eq!(form.w2_wages_from_churches_amt, None);
        assert_eq!(form.combined_se_and_church_wages_amt, None);
        assert_eq!(form.se_base_amt, None);
        // Lines 12–13 are zero
        assert_eq!(form.self_employment_tax_amt, Usd::ZERO);
        assert_eq!(form.deductible_self_employment_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    // ── Basic nonfarm self-employment ─────────────────────────────────

    #[test]
    fn basic_nonfarm_50k() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(50_000);
        let form = OutputScheduleSe::try_new(input).unwrap();

        // Line 3 = 50,000
        assert_eq!(
            form.se_total_net_earnings_or_loss_amt,
            Usd::from_dollars(50_000)
        );
        // Line 4a = 50,000 * 0.9235 = 46,175
        assert_eq!(
            form.minimum_profit_for_se_tax_amt,
            Usd::from_dollars(46_175)
        );
        // Line 4c = 46,175
        assert_eq!(form.combined_se_amt, Usd::from_dollars(46_175));
        // Line 6 = 46,175
        assert_eq!(
            form.combined_se_and_church_wages_amt,
            Some(Usd::from_dollars(46_175))
        );
        // Line 7 = 176,100
        assert_eq!(form.se_base_amt, Some(Usd::from_dollars(176_100)));
        // Line 8d = 0
        assert_eq!(form.total_wages_and_unreported_tips_amt, Some(Usd::ZERO));
        // Line 9 = 176,100
        assert_eq!(form.tax_base_amt, Some(Usd::from_dollars(176_100)));
        // Line 10 = min(46,175, 176,100) * 0.124 = 46,175 * 0.124 = 5,725.70
        assert_eq!(form.allowable_se_amt, Some(Usd::from_cents(572_570)));
        // Line 11 = 46,175 * 0.029 = 1,339.07 (truncated: 46175 * 290 / 10000 = 1339.075)
        let line11 = Usd::from_cents(4_617_500 * 290 / 10_000);
        // Line 12 = line 10 + line 11
        assert_eq!(
            form.self_employment_tax_amt,
            form.allowable_se_amt.unwrap() + line11
        );
        // Line 13 = line 12 * 50%
        assert_eq!(
            form.deductible_self_employment_tax_amt,
            Usd::from_cents(form.self_employment_tax_amt.cents() / 2)
        );
        assert!(form.is_valid());
    }

    // ── Farm income ──────────────────────────────────────────────────

    #[test]
    fn farm_income() {
        let mut input = default_input();
        input.net_farm_profit_loss_amt = Usd::from_dollars(30_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        assert_eq!(
            form.se_total_net_earnings_or_loss_amt,
            Usd::from_dollars(30_000)
        );
        assert_eq!(
            form.minimum_profit_for_se_tax_amt,
            Usd::from_cents(3_000_000 * 9235 / 10_000)
        );
        assert!(form.is_valid());
    }

    // ── CRP payments reduce farm income ──────────────────────────────

    #[test]
    fn crp_payments_reduce_income() {
        let mut input = default_input();
        input.net_farm_profit_loss_amt = Usd::from_dollars(20_000);
        input.conservation_reserve_prog_pymt_amt = Usd::from_dollars(5_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 3 = 20,000 - 5,000 = 15,000
        assert_eq!(
            form.se_total_net_earnings_or_loss_amt,
            Usd::from_dollars(15_000)
        );
        assert!(form.is_valid());
    }

    // ── Loss passes through without 92.35% factor ────────────────────

    #[test]
    fn loss_passes_through() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_cents(-1_000_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 4a = line 3 (no 92.35% factor on losses)
        assert_eq!(
            form.minimum_profit_for_se_tax_amt,
            Usd::from_cents(-1_000_000)
        );
        assert!(form.is_valid());
    }

    // ── SST wages reduce tax base ────────────────────────────────────

    #[test]
    fn sst_wages_reduce_tax_base() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(50_000);
        input.sst_wages_rrt_comp_amt = Usd::from_dollars(150_000);
        let form = OutputScheduleSe::try_new(input).unwrap();

        let line6 = form.combined_se_and_church_wages_amt.unwrap();
        // Line 8d = 150,000
        assert_eq!(
            form.total_wages_and_unreported_tips_amt,
            Some(Usd::from_dollars(150_000))
        );
        // Line 9 = 176,100 - 150,000 = 26,100
        assert_eq!(form.tax_base_amt, Some(Usd::from_dollars(26_100)));
        // Line 10 = min(line6, 26,100) * 12.4%
        let expected_10 =
            Usd::from_cents(line6.min(form.tax_base_amt.unwrap()).cents() * 1240 / 10_000);
        assert_eq!(form.allowable_se_amt, Some(expected_10));
        assert!(form.is_valid());
    }

    // ── SST wages exceed wage base → line 10 = 0 ─────────────────────

    #[test]
    fn sst_wages_exceed_wage_base() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(50_000);
        input.sst_wages_rrt_comp_amt = Usd::from_dollars(180_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 9 = max(176,100 - 180,000, 0) = 0
        assert_eq!(form.tax_base_amt, Some(Usd::ZERO));
        // Line 10 = 0 (no SS portion)
        assert_eq!(form.allowable_se_amt, Some(Usd::ZERO));
        // Line 12 = line 11 only (Medicare portion)
        let line11 =
            Usd::from_cents(form.combined_se_and_church_wages_amt.unwrap().cents() * 290 / 10_000);
        assert_eq!(form.self_employment_tax_amt, line11);
        assert!(form.is_valid());
    }

    // ── Church employee income ───────────────────────────────────────

    #[test]
    fn church_income_with_low_se() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(200);
        input.w2_wages_from_churches_amt = Usd::from_dollars(10_000);
        let form = OutputScheduleSe::try_new(input).unwrap();

        // Line 4a = 200 * 0.9235 = 184.70
        let line4a = Usd::from_cents(20_000 * 9235 / 10_000);
        assert_eq!(form.minimum_profit_for_se_tax_amt, line4a);
        // Line 4c: raw < $400, shows real value
        assert_eq!(form.combined_se_amt, line4a);
        // Line 5b = 10,000 * 0.9235 = 9,235
        assert_eq!(
            form.min_allowable_church_wages_amt,
            Some(Usd::from_dollars(9_235))
        );
        // Line 6 = -0- (4c entered as -0-) + 9,235 = 9,235
        assert_eq!(
            form.combined_se_and_church_wages_amt,
            Some(Usd::from_dollars(9_235))
        );
        assert!(form.is_valid());
    }

    // ── Church wages below $100 threshold → line 5b = 0 ──────────────

    #[test]
    fn church_wages_below_minimum() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(50_000);
        input.w2_wages_from_churches_amt = Usd::from_dollars(50);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // 50 * 0.9235 = 46.17 < $100 → -0-
        assert_eq!(form.min_allowable_church_wages_amt, Some(Usd::ZERO));
        assert!(form.is_valid());
    }

    // ── Farm optional method ─────────────────────────────────────────

    #[test]
    fn farm_optional_method() {
        let mut input = default_input();
        input.use_farm_optional_method = true;
        input.gross_farm_income = Usd::from_dollars(9_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 15 = min(2/3 * 9,000, 7,240) = min(6,000, 7,240) = 6,000
        assert_eq!(
            form.se_tax_farm_optional_method_amt,
            Some(Usd::from_dollars(6_000))
        );
        // Lines 16-17 not applicable (nonfarm not elected)
        assert_eq!(form.se_tax_non_farm_optional_base_amt, None);
        assert_eq!(form.se_tax_non_farm_optional_method_amt, None);
        // Line 4b = 6,000
        assert_eq!(form.optional_method_amt, Usd::from_dollars(6_000));
        assert!(form.is_valid());
    }

    #[test]
    fn farm_optional_method_capped() {
        let mut input = default_input();
        input.use_farm_optional_method = true;
        input.gross_farm_income = Usd::from_dollars(20_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 15 = min(2/3 * 20,000, 7,240) = min(13,333, 7,240) = 7,240
        assert_eq!(
            form.se_tax_farm_optional_method_amt,
            Some(Usd::from_dollars(7_240))
        );
        assert!(form.is_valid());
    }

    // ── Nonfarm optional method ──────────────────────────────────────

    #[test]
    fn nonfarm_optional_method() {
        let mut input = default_input();
        input.use_nonfarm_optional_method = true;
        input.gross_nonfarm_income = Usd::from_dollars(6_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 15 not applicable (farm not elected)
        assert_eq!(form.se_tax_farm_optional_method_amt, None);
        // Line 16 = 7,240 - 0 = 7,240
        assert_eq!(
            form.se_tax_non_farm_optional_base_amt,
            Some(Usd::from_dollars(7_240))
        );
        // Line 17 = min(2/3 * 6,000, 7,240) = min(4,000, 7,240) = 4,000
        assert_eq!(
            form.se_tax_non_farm_optional_method_amt,
            Some(Usd::from_dollars(4_000))
        );
        assert_eq!(form.optional_method_amt, Usd::from_dollars(4_000));
        assert!(form.is_valid());
    }

    // ── Both optional methods ────────────────────────────────────────

    #[test]
    fn both_optional_methods() {
        let mut input = default_input();
        input.use_farm_optional_method = true;
        input.gross_farm_income = Usd::from_dollars(9_000);
        input.use_nonfarm_optional_method = true;
        input.gross_nonfarm_income = Usd::from_dollars(6_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 15 = min(6,000, 7,240) = 6,000
        // Line 16 = 7,240 - 6,000 = 1,240
        // Line 17 = min(4,000, 1,240) = 1,240
        assert_eq!(
            form.se_tax_farm_optional_method_amt,
            Some(Usd::from_dollars(6_000))
        );
        assert_eq!(
            form.se_tax_non_farm_optional_base_amt,
            Some(Usd::from_dollars(1_240))
        );
        assert_eq!(
            form.se_tax_non_farm_optional_method_amt,
            Some(Usd::from_dollars(1_240))
        );
        assert_eq!(form.optional_method_amt, Usd::from_dollars(7_240));
        assert!(form.is_valid());
    }

    // ── Unreported tips and Form 8919 wages ──────────────────────────

    #[test]
    fn unreported_tips_and_8919_wages() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(80_000);
        input.sst_wages_rrt_comp_amt = Usd::from_dollars(100_000);
        input.unreported_tips_amt = Usd::from_dollars(5_000);
        input.wages_subject_to_sst_amt = Usd::from_dollars(10_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        // Line 8d = 100,000 + 5,000 + 10,000 = 115,000
        assert_eq!(
            form.total_wages_and_unreported_tips_amt,
            Some(Usd::from_dollars(115_000))
        );
        // Line 9 = 176,100 - 115,000 = 61,100
        assert_eq!(form.tax_base_amt, Some(Usd::from_dollars(61_100)));
        assert!(form.is_valid());
    }

    // ── Zero everything ──────────────────────────────────────────────

    #[test]
    fn zero_everything() {
        let form = OutputScheduleSe::try_new(default_input()).unwrap();
        assert_eq!(form.self_employment_tax_amt, Usd::ZERO);
        assert_eq!(form.deductible_self_employment_tax_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    // ── Deductible half ──────────────────────────────────────────────

    #[test]
    fn deductible_half_of_se_tax() {
        let mut input = default_input();
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(100_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        assert_eq!(
            form.deductible_self_employment_tax_amt,
            Usd::from_cents(form.self_employment_tax_amt.cents() / 2)
        );
        assert!(form.is_valid());
    }

    // ── Combined farm and nonfarm ────────────────────────────────────

    #[test]
    fn combined_farm_and_nonfarm() {
        let mut input = default_input();
        input.net_farm_profit_loss_amt = Usd::from_dollars(20_000);
        input.net_non_farm_profit_loss_amt = Usd::from_dollars(30_000);
        let form = OutputScheduleSe::try_new(input).unwrap();
        assert_eq!(
            form.se_total_net_earnings_or_loss_amt,
            Usd::from_dollars(50_000)
        );
        assert!(form.is_valid());
    }
}
