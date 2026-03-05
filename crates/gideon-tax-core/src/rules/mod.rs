pub mod y2025;

use us_tax_brackets::{FilingStatus, TaxYear};

use crate::Usd;
use crate::types::Filer;

/// Year-specific tax parameters consumed by [`crate::spine::compute_spine`].
///
/// Each tax year gets its own implementation that supplies the IRS-published
/// dollar amounts. The [`standard_deduction`](TaxYearRules::standard_deduction)
/// algorithm is a provided method that combines them.
pub trait TaxYearRules {
    fn year(&self) -> TaxYear;

    /// Base standard deduction for Single or MFS filers.
    fn single_mfs_typical_standard_deduction(&self) -> Usd;

    /// Base standard deduction for MFJ or QSS filers.
    fn mfj_qss_typical_standard_deduction(&self) -> Usd;

    /// Base standard deduction for Head of Household filers.
    fn hoh_typical_standard_deduction(&self) -> Usd;

    /// Per-box addition for Single or Head of Household filers.
    fn additional_deduction_unmarried(&self) -> Usd;

    /// Per-box addition for MFJ, MFS, or QSS filers.
    fn additional_deduction_married(&self) -> Usd;

    /// Amount added to a dependent's earned income before clamping.
    fn dependent_earned_income_addition(&self) -> Usd;

    /// Minimum standard deduction for a dependent filer.
    fn dependent_minimum_deduction(&self) -> Usd;

    /// Maximum wages and tips subject to social security tax (wage base).
    fn social_security_wage_base(&self) -> Usd;

    /// Social security tax rate as basis points (e.g. 620 = 6.20%).
    fn social_security_rate_bps(&self) -> u16;

    /// Medicare tax rate as basis points (e.g. 145 = 1.45%).
    fn medicare_rate_bps(&self) -> u16;

    /// Additional Medicare Tax rate as basis points (e.g. 90 = 0.9%).
    fn additional_medicare_rate_bps(&self) -> u16;

    /// Additional Medicare Tax threshold for MFJ filers.
    fn additional_medicare_threshold_mfj(&self) -> Usd;

    /// Additional Medicare Tax threshold for MFS filers.
    fn additional_medicare_threshold_mfs(&self) -> Usd;

    /// Additional Medicare Tax threshold for Single, HoH, and QSS filers.
    fn additional_medicare_threshold_single(&self) -> Usd;

    /// Additional Medicare Tax threshold for the given filing status.
    fn additional_medicare_threshold(&self, status: FilingStatus) -> Usd {
        use FilingStatus::*;
        match status {
            MarriedFilingJointly => self.additional_medicare_threshold_mfj(),
            MarriedFilingSeparately => self.additional_medicare_threshold_mfs(),
            Single | HeadOfHousehold | QualifyingSurvivingSpouse => {
                self.additional_medicare_threshold_single()
            }
        }
    }

    /// Number of days in the tax year (365, or 366 for leap years).
    fn days_in_tax_year(&self) -> u32;

    /// Maximum foreign earned income exclusion (Form 2555, line 37).
    fn f2555_max_foreign_earned_income_exclusion(&self) -> Usd;

    /// Per-day base housing amount in cents (Form 2555, line 32).
    fn f2555_housing_per_day_cents(&self) -> i64;

    /// Full-year base housing amount (Form 2555, line 32 when line 31 = 365).
    fn f2555_housing_full_year(&self) -> Usd;

    /// Base standard deduction before any age/blindness additions.
    fn typical_standard_deduction(&self, status: FilingStatus) -> Usd {
        use FilingStatus::*;
        match status {
            Single | MarriedFilingSeparately => self.single_mfs_typical_standard_deduction(),
            MarriedFilingJointly | QualifyingSurvivingSpouse => {
                self.mfj_qss_typical_standard_deduction()
            }
            HeadOfHousehold => self.hoh_typical_standard_deduction(),
        }
    }

    /// Computes the full standard deduction from year-specific constants.
    ///
    /// Returns $0 for dual-status aliens or MFS when spouse itemizes.
    /// Otherwise applies the dependent formula or the typical base, plus
    /// an additional amount per qualifying age/blindness box.
    fn standard_deduction(&self, params: &DeductionParams) -> Usd {
        use FilingStatus::*;

        // ── Zero-deduction overrides ────────────────────────────────
        if params.is_dual_status_alien {
            return Usd::ZERO;
        }
        if params.filing_status == MarriedFilingSeparately && params.spouse_itemizes {
            return Usd::ZERO;
        }

        // ── Base amount ─────────────────────────────────────────────
        let base = self.typical_standard_deduction(params.filing_status);

        // ── Additional amount per qualifying box ────────────────────
        let per_box = match params.filing_status {
            Single | HeadOfHousehold => self.additional_deduction_unmarried(),
            _ => self.additional_deduction_married(),
        };

        let boxes = match params.filing_status {
            Single | HeadOfHousehold => params.taxpayer.checked_boxes(),
            MarriedFilingJointly | MarriedFilingSeparately | QualifyingSurvivingSpouse => {
                params.taxpayer.checked_boxes() + params.spouse.map_or(0, |s| s.checked_boxes())
            }
        };

        let additional = per_box * boxes;

        // ── Dependent vs. non-dependent base ────────────────────────
        if params.is_dependent {
            let earned_plus = params.earned_income + self.dependent_earned_income_addition();
            let floor = self.dependent_minimum_deduction();
            let capped_base = earned_plus.max(floor).min(base);
            capped_base + additional
        } else {
            base + additional
        }
    }
}

/// Input to [`TaxYearRules::standard_deduction`].
///
/// When `filing_status` is [`FilingStatus::MarriedFilingSeparately`], a
/// `spouse` may only be provided if the spouse had no income, is not filing
/// a return, and cannot be claimed as a dependent on another person's return.
pub struct DeductionParams {
    pub filing_status: FilingStatus,
    pub taxpayer: Filer,
    pub spouse: Option<Filer>,
    /// `true` if the taxpayer can be claimed as a dependent on another
    /// person's return, **or** if filing jointly and the spouse can be.
    pub is_dependent: bool,
    pub is_dual_status_alien: bool,
    pub spouse_itemizes: bool,
    pub earned_income: Usd,
}
