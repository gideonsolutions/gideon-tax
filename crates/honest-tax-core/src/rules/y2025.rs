//! Tax rules for 2025.

use crate::money::Money;
use crate::traits::{PhaseOut, SeniorBonusDeduction, TaxBracket, TaxRules};
use crate::types::FilingStatus;
use rust_decimal_macros::dec;

/// Tax rules for tax year 2025.
///
/// Sources:
/// - IRS Revenue Procedure 2024-40
/// - One Big Beautiful Bill Act (signed July 4, 2025)
#[derive(Debug, Clone)]
pub struct Rules2025 {
    brackets_single: Vec<TaxBracket>,
    brackets_mfj: Vec<TaxBracket>,
    brackets_mfs: Vec<TaxBracket>,
    brackets_hoh: Vec<TaxBracket>,
    ctc_phase_out: PhaseOut,
}

impl Default for Rules2025 {
    fn default() -> Self {
        Self::new()
    }
}

impl Rules2025 {
    /// Creates a new Rules2025 instance with all 2025 tax parameters.
    pub fn new() -> Self {
        Self {
            brackets_single: Self::build_single_brackets(),
            brackets_mfj: Self::build_mfj_brackets(),
            brackets_mfs: Self::build_mfs_brackets(),
            brackets_hoh: Self::build_hoh_brackets(),
            ctc_phase_out: PhaseOut {
                single_threshold: Money::from_dollars(200_000),
                joint_threshold: Money::from_dollars(400_000),
                mfs_threshold: Money::from_dollars(200_000),
                rate: dec!(0.05),
            },
        }
    }

    fn build_single_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                rate: dec!(0.10),
                min: Money::ZERO,
                max: Some(Money::from_dollars(11_925)),
            },
            TaxBracket {
                rate: dec!(0.12),
                min: Money::from_dollars(11_925),
                max: Some(Money::from_dollars(48_475)),
            },
            TaxBracket {
                rate: dec!(0.22),
                min: Money::from_dollars(48_475),
                max: Some(Money::from_dollars(103_350)),
            },
            TaxBracket {
                rate: dec!(0.24),
                min: Money::from_dollars(103_350),
                max: Some(Money::from_dollars(197_300)),
            },
            TaxBracket {
                rate: dec!(0.32),
                min: Money::from_dollars(197_300),
                max: Some(Money::from_dollars(250_525)),
            },
            TaxBracket {
                rate: dec!(0.35),
                min: Money::from_dollars(250_525),
                max: Some(Money::from_dollars(626_350)),
            },
            TaxBracket {
                rate: dec!(0.37),
                min: Money::from_dollars(626_350),
                max: None,
            },
        ]
    }

    fn build_mfj_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                rate: dec!(0.10),
                min: Money::ZERO,
                max: Some(Money::from_dollars(23_850)),
            },
            TaxBracket {
                rate: dec!(0.12),
                min: Money::from_dollars(23_850),
                max: Some(Money::from_dollars(96_950)),
            },
            TaxBracket {
                rate: dec!(0.22),
                min: Money::from_dollars(96_950),
                max: Some(Money::from_dollars(206_700)),
            },
            TaxBracket {
                rate: dec!(0.24),
                min: Money::from_dollars(206_700),
                max: Some(Money::from_dollars(394_600)),
            },
            TaxBracket {
                rate: dec!(0.32),
                min: Money::from_dollars(394_600),
                max: Some(Money::from_dollars(501_050)),
            },
            TaxBracket {
                rate: dec!(0.35),
                min: Money::from_dollars(501_050),
                max: Some(Money::from_dollars(751_600)),
            },
            TaxBracket {
                rate: dec!(0.37),
                min: Money::from_dollars(751_600),
                max: None,
            },
        ]
    }

    fn build_mfs_brackets() -> Vec<TaxBracket> {
        // MFS brackets are half of MFJ, except 35%/37% boundary
        vec![
            TaxBracket {
                rate: dec!(0.10),
                min: Money::ZERO,
                max: Some(Money::from_dollars(11_925)),
            },
            TaxBracket {
                rate: dec!(0.12),
                min: Money::from_dollars(11_925),
                max: Some(Money::from_dollars(48_475)),
            },
            TaxBracket {
                rate: dec!(0.22),
                min: Money::from_dollars(48_475),
                max: Some(Money::from_dollars(103_350)),
            },
            TaxBracket {
                rate: dec!(0.24),
                min: Money::from_dollars(103_350),
                max: Some(Money::from_dollars(197_300)),
            },
            TaxBracket {
                rate: dec!(0.32),
                min: Money::from_dollars(197_300),
                max: Some(Money::from_dollars(250_525)),
            },
            TaxBracket {
                rate: dec!(0.35),
                min: Money::from_dollars(250_525),
                max: Some(Money::from_dollars(375_800)),
            },
            TaxBracket {
                rate: dec!(0.37),
                min: Money::from_dollars(375_800),
                max: None,
            },
        ]
    }

    fn build_hoh_brackets() -> Vec<TaxBracket> {
        vec![
            TaxBracket {
                rate: dec!(0.10),
                min: Money::ZERO,
                max: Some(Money::from_dollars(17_000)),
            },
            TaxBracket {
                rate: dec!(0.12),
                min: Money::from_dollars(17_000),
                max: Some(Money::from_dollars(64_850)),
            },
            TaxBracket {
                rate: dec!(0.22),
                min: Money::from_dollars(64_850),
                max: Some(Money::from_dollars(103_350)),
            },
            TaxBracket {
                rate: dec!(0.24),
                min: Money::from_dollars(103_350),
                max: Some(Money::from_dollars(197_300)),
            },
            TaxBracket {
                rate: dec!(0.32),
                min: Money::from_dollars(197_300),
                max: Some(Money::from_dollars(250_500)),
            },
            TaxBracket {
                rate: dec!(0.35),
                min: Money::from_dollars(250_500),
                max: Some(Money::from_dollars(626_350)),
            },
            TaxBracket {
                rate: dec!(0.37),
                min: Money::from_dollars(626_350),
                max: None,
            },
        ]
    }
}

impl TaxRules for Rules2025 {
    fn year(&self) -> u16 {
        2025
    }

    fn brackets(&self, status: FilingStatus) -> &[TaxBracket] {
        match status {
            FilingStatus::Single => &self.brackets_single,
            FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => {
                &self.brackets_mfj
            }
            FilingStatus::MarriedFilingSeparately => &self.brackets_mfs,
            FilingStatus::HeadOfHousehold => &self.brackets_hoh,
        }
    }

    fn standard_deduction_base(&self, status: FilingStatus) -> Money {
        // Updated per One Big Beautiful Bill Act
        match status {
            FilingStatus::Single | FilingStatus::MarriedFilingSeparately => {
                Money::from_dollars(15_750)
            }
            FilingStatus::MarriedFilingJointly | FilingStatus::QualifyingSurvivingSpouse => {
                Money::from_dollars(31_500)
            }
            FilingStatus::HeadOfHousehold => Money::from_dollars(23_625),
        }
    }

    fn standard_deduction_age_65(&self, status: FilingStatus) -> Money {
        match status {
            FilingStatus::Single | FilingStatus::HeadOfHousehold => Money::from_dollars(2_000),
            FilingStatus::MarriedFilingJointly
            | FilingStatus::MarriedFilingSeparately
            | FilingStatus::QualifyingSurvivingSpouse => Money::from_dollars(1_600),
        }
    }

    fn standard_deduction_blind(&self, status: FilingStatus) -> Money {
        // Same as age 65+ amounts
        self.standard_deduction_age_65(status)
    }

    fn senior_bonus_deduction(&self) -> Option<SeniorBonusDeduction> {
        // One Big Beautiful Bill Act (2025-2028)
        Some(SeniorBonusDeduction {
            amount_per_person: 6_000,
            phase_out: PhaseOut {
                single_threshold: Money::from_dollars(75_000),
                joint_threshold: Money::from_dollars(150_000),
                mfs_threshold: Money::from_dollars(75_000),
                rate: dec!(0.06), // 6% per dollar over threshold
            },
        })
    }

    fn child_tax_credit_max(&self) -> Money {
        // Increased by One Big Beautiful Bill Act
        Money::from_dollars(2_200)
    }

    fn additional_child_tax_credit_max(&self) -> Money {
        Money::from_dollars(1_700)
    }

    fn actc_earned_income_threshold(&self) -> Money {
        Money::from_dollars(2_500)
    }

    fn child_tax_credit_phase_out(&self) -> &PhaseOut {
        &self.ctc_phase_out
    }

    fn credit_for_other_dependents(&self) -> Money {
        Money::from_dollars(500)
    }

    fn qbi_deduction_rate(&self) -> rust_decimal::Decimal {
        dec!(0.20) // 20% deduction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_deduction_single() {
        let rules = Rules2025::new();
        assert_eq!(
            rules.standard_deduction_base(FilingStatus::Single),
            Money::from_dollars(15_750)
        );
    }

    #[test]
    fn test_standard_deduction_mfj() {
        let rules = Rules2025::new();
        assert_eq!(
            rules.standard_deduction_base(FilingStatus::MarriedFilingJointly),
            Money::from_dollars(31_500)
        );
    }

    #[test]
    fn test_tax_calculation_single() {
        let rules = Rules2025::new();

        // $50,000 taxable income (single)
        // 10% on first $11,925 = $1,192.50
        // 12% on $11,925 to $48,475 = $4,386.00
        // 22% on $48,475 to $50,000 = $335.50
        // Total = $5,914.00
        let tax = rules.calculate_tax(FilingStatus::Single, Money::from_dollars(50_000));

        // Calculate expected
        let bracket_1 = Money::from_dollars(11_925).multiply_rate(dec!(0.10));
        let bracket_2 = Money::from_dollars(48_475 - 11_925).multiply_rate(dec!(0.12));
        let bracket_3 = Money::from_dollars(50_000 - 48_475).multiply_rate(dec!(0.22));
        let expected = bracket_1 + bracket_2 + bracket_3;

        assert_eq!(tax.round_to_cents(), expected.round_to_cents());
    }

    #[test]
    fn test_child_tax_credit_max() {
        let rules = Rules2025::new();
        assert_eq!(rules.child_tax_credit_max(), Money::from_dollars(2_200));
    }

    #[test]
    fn test_senior_bonus_deduction() {
        let rules = Rules2025::new();
        let bonus = rules.senior_bonus_deduction().unwrap();
        assert_eq!(bonus.amount_per_person, 6_000);
        assert_eq!(bonus.phase_out.single_threshold, Money::from_dollars(75_000));
    }
}
