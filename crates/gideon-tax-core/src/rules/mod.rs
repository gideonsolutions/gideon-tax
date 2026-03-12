pub mod y2025;

use us_tax_brackets::{FilingStatus, TaxYear};

use crate::Usd;
use crate::types::Filer;

/// Year-specific tax parameters consumed by [`crate::spine::compute_spine`].
///
/// Each tax year gets its own implementation that supplies the IRS-published
/// dollar amounts as associated constants. The
/// [`standard_deduction`](TaxYearRules::standard_deduction) algorithm is a
/// provided method that combines them.
pub trait TaxYearRules {
    const YEAR: TaxYear;

    /// Base standard deduction for Single or MFS filers.
    const SINGLE_MFS_TYPICAL_STANDARD_DEDUCTION: Usd;

    /// Base standard deduction for MFJ or QSS filers.
    const MFJ_QSS_TYPICAL_STANDARD_DEDUCTION: Usd;

    /// Base standard deduction for Head of Household filers.
    const HOH_TYPICAL_STANDARD_DEDUCTION: Usd;

    /// Per-box addition for Single or Head of Household filers.
    const ADDITIONAL_DEDUCTION_UNMARRIED: Usd;

    /// Per-box addition for MFJ, MFS, or QSS filers.
    const ADDITIONAL_DEDUCTION_MARRIED: Usd;

    /// Amount added to a dependent's earned income before clamping.
    const DEPENDENT_EARNED_INCOME_ADDITION: Usd;

    /// Minimum standard deduction for a dependent filer.
    const DEPENDENT_MINIMUM_DEDUCTION: Usd;

    /// Maximum wages and tips subject to social security tax (wage base).
    const SOCIAL_SECURITY_WAGE_BASE: Usd;

    /// Social security tax rate as basis points (e.g. 620 = 6.20%).
    const SOCIAL_SECURITY_RATE_BPS: u16;

    /// Medicare tax rate as basis points (e.g. 145 = 1.45%).
    const MEDICARE_RATE_BPS: u16;

    /// Additional Medicare Tax rate as basis points (e.g. 90 = 0.9%).
    const ADDITIONAL_MEDICARE_RATE_BPS: u16;

    /// Additional Medicare Tax threshold for MFJ filers.
    const ADDITIONAL_MEDICARE_THRESHOLD_MFJ: Usd;

    /// Additional Medicare Tax threshold for MFS filers.
    const ADDITIONAL_MEDICARE_THRESHOLD_MFS: Usd;

    /// Additional Medicare Tax threshold for Single, HoH, and QSS filers.
    const ADDITIONAL_MEDICARE_THRESHOLD_SINGLE: Usd;

    /// Minimum net earnings from self-employment required to file Schedule SE.
    const SE_MIN_NET_EARNINGS: Usd;

    /// Minimum church employee wages (after 92.35% factor) to include on
    /// Schedule SE line 5b. Below this, enter -0-.
    const SE_MIN_CHURCH_WAGES: Usd;

    /// Minimum church employee income (before 92.35% factor) that triggers
    /// the Schedule SE filing requirement ($108.28 for 2025).
    const SE_MIN_CHURCH_EMPLOYEE_INCOME: Usd;

    /// Minimum combined lines 1a + 2 when CRP payments (line 1b) cause both
    /// lines 4a and 4c to fall below SE_MIN_NET_EARNINGS ($434 for 2025).
    const SE_CRP_GROSS_THRESHOLD: Usd;

    /// Maximum income for optional methods (Schedule SE, Part II, line 14).
    const SE_FARM_OPTIONAL_METHOD_MAX: Usd;

    /// Number of days in the tax year (365, or 366 for leap years).
    const DAYS_IN_TAX_YEAR: u32;

    /// FUTA gross tax rate as basis points (e.g. 600 = 6.0%).
    const FUTA_RATE_BPS: u16;

    /// FUTA maximum credit rate as basis points (e.g. 540 = 5.4%).
    const FUTA_CREDIT_RATE_BPS: u16;

    /// FUTA credit reduction states for this tax year.
    ///
    /// States that have outstanding federal unemployment loans are subject to
    /// a credit reduction. Employers in these states receive a smaller FUTA
    /// credit, effectively increasing the FUTA tax owed.
    ///
    /// See IRS Schedule H instructions, Worksheet 2 — Household Employers in
    /// a Credit Reduction State.
    const FUTA_CREDIT_REDUCTION_STATES: &'static [FutaCreditReductionState];

    /// HSA annual contribution limit for self-only HDHP coverage.
    const HSA_SELF_ONLY_CONTRIBUTION_LIMIT: Usd;

    /// HSA annual contribution limit for family HDHP coverage.
    const HSA_FAMILY_CONTRIBUTION_LIMIT: Usd;

    /// HSA catch-up contribution for individuals age 55 or older.
    const HSA_CATCH_UP_CONTRIBUTION: Usd;

    /// Maximum foreign earned income exclusion (Form 2555, line 37).
    const F2555_MAX_FOREIGN_EARNED_INCOME_EXCLUSION: Usd;

    /// Per-day base housing amount in cents (Form 2555, line 32).
    const F2555_HOUSING_PER_DAY_CENTS: i64;

    /// Full-year base housing amount (Form 2555, line 32 when line 31 = 365).
    const F2555_HOUSING_FULL_YEAR: Usd;

    /// Additional Medicare Tax threshold for the given filing status.
    fn additional_medicare_threshold(status: FilingStatus) -> Usd {
        use FilingStatus::*;
        match status {
            MarriedFilingJointly => Self::ADDITIONAL_MEDICARE_THRESHOLD_MFJ,
            MarriedFilingSeparately => Self::ADDITIONAL_MEDICARE_THRESHOLD_MFS,
            Single | HeadOfHousehold | QualifyingSurvivingSpouse => {
                Self::ADDITIONAL_MEDICARE_THRESHOLD_SINGLE
            }
        }
    }

    /// Base standard deduction before any age/blindness additions.
    fn typical_standard_deduction(status: FilingStatus) -> Usd {
        use FilingStatus::*;
        match status {
            Single | MarriedFilingSeparately => Self::SINGLE_MFS_TYPICAL_STANDARD_DEDUCTION,
            MarriedFilingJointly | QualifyingSurvivingSpouse => {
                Self::MFJ_QSS_TYPICAL_STANDARD_DEDUCTION
            }
            HeadOfHousehold => Self::HOH_TYPICAL_STANDARD_DEDUCTION,
        }
    }

    /// Computes the full standard deduction from year-specific constants.
    ///
    /// Returns $0 for dual-status aliens or MFS when spouse itemizes.
    /// Otherwise applies the dependent formula or the typical base, plus
    /// an additional amount per qualifying age/blindness box.
    fn standard_deduction(params: &DeductionParams) -> Usd {
        use FilingStatus::*;

        // ── Zero-deduction overrides ────────────────────────────────
        if params.is_dual_status_alien {
            return Usd::ZERO;
        }
        if params.filing_status == MarriedFilingSeparately && params.spouse_itemizes {
            return Usd::ZERO;
        }

        // ── Base amount ─────────────────────────────────────────────
        let base = Self::typical_standard_deduction(params.filing_status);

        // ── Additional amount per qualifying box ────────────────────
        let per_box = match params.filing_status {
            Single | HeadOfHousehold => Self::ADDITIONAL_DEDUCTION_UNMARRIED,
            _ => Self::ADDITIONAL_DEDUCTION_MARRIED,
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
            let earned_plus = params.earned_income + Self::DEPENDENT_EARNED_INCOME_ADDITION;
            let floor = Self::DEPENDENT_MINIMUM_DEDUCTION;
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

/// A state or territory subject to FUTA credit reduction for a given tax year.
///
/// States with outstanding federal unemployment trust fund loans receive a
/// reduced FUTA credit. The reduction rate is applied to FUTA taxable wages
/// for that state, increasing the effective FUTA tax.
///
/// See IRS Schedule H instructions, Worksheet 2 — Household Employers in
/// a Credit Reduction State.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutaCreditReductionState {
    /// Two-letter postal abbreviation (e.g. "CA", "VI").
    pub state_cd: &'static str,
    /// Credit reduction rate in basis points (e.g. 120 = 1.2%).
    pub reduction_rate_bps: u16,
}
