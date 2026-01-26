//! Trait for year-specific tax rules.

use crate::money::Money;
use crate::types::{FilingStatus, TaxYear};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// A single tax bracket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBracket {
    /// Tax rate as a decimal (e.g., 0.10 for 10%).
    pub rate: Decimal,
    /// Minimum taxable income for this bracket.
    pub min: Money,
    /// Maximum taxable income for this bracket (None = no upper limit).
    pub max: Option<Money>,
}

impl TaxBracket {
    /// Returns the tax owed for income within this bracket.
    pub fn tax_for_income(&self, income: Money) -> Money {
        if income.as_decimal() < self.min.as_decimal() {
            return Money::ZERO;
        }

        let taxable_in_bracket = match self.max {
            Some(max) => income.min(max) - self.min,
            None => income - self.min,
        };

        if taxable_in_bracket.is_negative() {
            Money::ZERO
        } else {
            taxable_in_bracket.multiply_rate(self.rate)
        }
    }
}

/// Phase-out configuration for credits/deductions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseOut {
    /// AGI threshold where phase-out begins (single/HoH).
    pub single_threshold: Money,
    /// AGI threshold where phase-out begins (MFJ/QSS).
    pub joint_threshold: Money,
    /// AGI threshold where phase-out begins (MFS).
    pub mfs_threshold: Money,
    /// Rate at which benefit reduces (e.g., 0.05 = $50 per $1000 over threshold).
    pub rate: Decimal,
}

impl PhaseOut {
    /// Returns the threshold for the given filing status.
    pub fn threshold_for(&self, status: FilingStatus) -> Money {
        match status {
            FilingStatus::Single | FilingStatus::HeadOfHousehold => self.single_threshold,
            FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => {
                self.joint_threshold
            }
            FilingStatus::MarriedFilingSeparately => self.mfs_threshold,
        }
    }

    /// Calculates the phase-out reduction amount.
    pub fn reduction(&self, status: FilingStatus, agi: Money) -> Money {
        let threshold = self.threshold_for(status);
        if agi <= threshold {
            return Money::ZERO;
        }

        let excess = agi - threshold;
        excess.multiply_rate(self.rate)
    }
}

/// Trait for year-specific tax rules.
///
/// Implementations provide all the numeric constants and rules
/// needed to calculate taxes for a specific year.
pub trait TaxRules: Send + Sync {
    /// Returns the tax year these rules apply to.
    fn year(&self) -> TaxYear;

    // ─────────────────────────────────────────────────────────────────────────
    // Tax brackets
    // ─────────────────────────────────────────────────────────────────────────

    /// Returns the tax brackets for the given filing status.
    fn brackets(&self, status: FilingStatus) -> &[TaxBracket];

    /// Calculates tax using the tax brackets.
    fn calculate_tax(&self, status: FilingStatus, taxable_income: Money) -> Money {
        let brackets = self.brackets(status);
        let mut total_tax = Money::ZERO;
        let mut prev_max = Money::ZERO;

        for bracket in brackets {
            if taxable_income <= prev_max {
                break;
            }

            let bracket_income = match bracket.max {
                Some(max) => taxable_income.min(max) - prev_max,
                None => taxable_income - prev_max,
            };

            if bracket_income.is_positive() {
                total_tax += bracket_income.multiply_rate(bracket.rate);
            }

            prev_max = bracket.max.unwrap_or(taxable_income);
        }

        total_tax
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Standard deduction
    // ─────────────────────────────────────────────────────────────────────────

    /// Returns the base standard deduction for the given filing status.
    fn standard_deduction_base(&self, status: FilingStatus) -> Money;

    /// Returns the additional standard deduction for age 65+.
    fn standard_deduction_age_65(&self, status: FilingStatus) -> Money;

    /// Returns the additional standard deduction for blindness.
    fn standard_deduction_blind(&self, status: FilingStatus) -> Money;

    /// Returns the senior bonus deduction (OBBBA 2025-2028).
    fn senior_bonus_deduction(&self) -> Option<SeniorBonusDeduction> {
        None
    }

    /// Calculates the total standard deduction.
    fn standard_deduction(
        &self,
        status: FilingStatus,
        taxpayer_65_or_older: bool,
        taxpayer_blind: bool,
        spouse_65_or_older: bool,
        spouse_blind: bool,
        agi: Money,
    ) -> Money {
        let mut deduction = self.standard_deduction_base(status);

        // Additional for taxpayer
        if taxpayer_65_or_older {
            deduction += self.standard_deduction_age_65(status);
        }
        if taxpayer_blind {
            deduction += self.standard_deduction_blind(status);
        }

        // Additional for spouse (MFJ only)
        if status == FilingStatus::MarriedFilingJointly {
            if spouse_65_or_older {
                deduction += self.standard_deduction_age_65(status);
            }
            if spouse_blind {
                deduction += self.standard_deduction_blind(status);
            }
        }

        // Senior bonus deduction (if applicable)
        if let Some(bonus) = self.senior_bonus_deduction() {
            let mut senior_count = 0;
            if taxpayer_65_or_older {
                senior_count += 1;
            }
            if status == FilingStatus::MarriedFilingJointly && spouse_65_or_older {
                senior_count += 1;
            }

            if senior_count > 0 {
                let threshold = bonus.phase_out.threshold_for(status);
                if agi <= threshold {
                    deduction += Money::from_dollars(bonus.amount_per_person as i64)
                        * Money::from_dollars(senior_count);
                } else {
                    // Phase out calculation
                    let reduction = bonus.phase_out.reduction(status, agi);
                    let max_bonus =
                        Money::from_dollars(bonus.amount_per_person as i64 * senior_count);
                    deduction += max_bonus.saturating_sub(reduction);
                }
            }
        }

        deduction
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Child Tax Credit
    // ─────────────────────────────────────────────────────────────────────────

    /// Maximum Child Tax Credit per qualifying child.
    fn child_tax_credit_max(&self) -> Money;

    /// Maximum refundable Additional Child Tax Credit per child.
    fn additional_child_tax_credit_max(&self) -> Money;

    /// Earned income threshold for ACTC.
    fn actc_earned_income_threshold(&self) -> Money;

    /// Phase-out configuration for Child Tax Credit.
    fn child_tax_credit_phase_out(&self) -> &PhaseOut;

    /// Credit for Other Dependents (non-CTC qualifying).
    fn credit_for_other_dependents(&self) -> Money;

    // ─────────────────────────────────────────────────────────────────────────
    // Other parameters
    // ─────────────────────────────────────────────────────────────────────────

    /// Personal exemption amount (0 for 2018+).
    fn personal_exemption(&self) -> Money {
        Money::ZERO
    }

    /// Qualified Business Income deduction rate (Section 199A).
    fn qbi_deduction_rate(&self) -> Decimal;
}

/// Senior bonus deduction configuration (OBBBA 2025-2028).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeniorBonusDeduction {
    /// Amount per eligible person (age 65+).
    pub amount_per_person: u32,
    /// Phase-out configuration.
    pub phase_out: PhaseOut,
}

// Helper for multiplication
impl std::ops::Mul<Money> for Money {
    type Output = Money;

    fn mul(self, rhs: Money) -> Money {
        Money::new(self.as_decimal() * rhs.as_decimal())
    }
}
