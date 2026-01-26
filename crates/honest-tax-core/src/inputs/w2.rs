//! Form W-2: Wage and Tax Statement

use crate::error::{TaxResult, ValidationErrors};
use crate::money::Money;
use crate::traits::InputForm;
use crate::types::{InputFormType, TaxYear};
use serde::{Deserialize, Serialize};

/// Form W-2: Wage and Tax Statement
///
/// Represents a W-2 form received from an employer.
/// Box numbers correspond to the official IRS W-2 form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct W2 {
    /// Unique identifier for this W-2 instance.
    #[serde(default = "default_id")]
    pub id: String,

    /// Tax year this W-2 is for.
    #[serde(default = "default_tax_year")]
    pub tax_year: TaxYear,

    // ─────────────────────────────────────────────────────────────────────────
    // Employer information
    // ─────────────────────────────────────────────────────────────────────────
    /// Box b: Employer Identification Number (EIN).
    #[serde(default)]
    pub employer_ein: String,

    /// Box c: Employer's name, address, and ZIP code.
    #[serde(default)]
    pub employer_name: String,

    // ─────────────────────────────────────────────────────────────────────────
    // Employee information
    // ─────────────────────────────────────────────────────────────────────────
    /// Box a: Employee's Social Security Number.
    #[serde(default)]
    pub employee_ssn: String,

    /// Box e: Employee's first name and initial.
    #[serde(default)]
    pub employee_first_name: String,

    /// Box e: Employee's last name.
    #[serde(default)]
    pub employee_last_name: String,

    // ─────────────────────────────────────────────────────────────────────────
    // Income boxes
    // ─────────────────────────────────────────────────────────────────────────
    /// Box 1: Wages, tips, other compensation.
    #[serde(default)]
    pub box_1_wages: Money,

    /// Box 2: Federal income tax withheld.
    #[serde(default)]
    pub box_2_federal_tax_withheld: Money,

    /// Box 3: Social Security wages.
    #[serde(default)]
    pub box_3_social_security_wages: Money,

    /// Box 4: Social Security tax withheld.
    #[serde(default)]
    pub box_4_social_security_tax_withheld: Money,

    /// Box 5: Medicare wages and tips.
    #[serde(default)]
    pub box_5_medicare_wages: Money,

    /// Box 6: Medicare tax withheld.
    #[serde(default)]
    pub box_6_medicare_tax_withheld: Money,

    /// Box 7: Social Security tips.
    #[serde(default)]
    pub box_7_social_security_tips: Money,

    /// Box 8: Allocated tips.
    #[serde(default)]
    pub box_8_allocated_tips: Money,

    /// Box 10: Dependent care benefits.
    #[serde(default)]
    pub box_10_dependent_care_benefits: Money,

    /// Box 11: Nonqualified plans.
    #[serde(default)]
    pub box_11_nonqualified_plans: Money,

    /// Box 12: Codes and amounts (various compensation types).
    #[serde(default)]
    pub box_12: Vec<W2Box12>,

    /// Box 13: Checkboxes.
    #[serde(default)]
    pub box_13_statutory_employee: bool,

    #[serde(default)]
    pub box_13_retirement_plan: bool,

    #[serde(default)]
    pub box_13_third_party_sick_pay: bool,

    // ─────────────────────────────────────────────────────────────────────────
    // State/local taxes
    // ─────────────────────────────────────────────────────────────────────────
    /// Box 15-17: State tax information.
    #[serde(default)]
    pub state_info: Vec<W2StateInfo>,

    /// Box 18-20: Local tax information.
    #[serde(default)]
    pub local_info: Vec<W2LocalInfo>,
}

fn default_id() -> String {
    "w2-default".to_string()
}

fn default_tax_year() -> TaxYear {
    2025
}

impl Default for W2 {
    fn default() -> Self {
        Self {
            id: default_id(),
            tax_year: default_tax_year(),
            employer_ein: String::new(),
            employer_name: String::new(),
            employee_ssn: String::new(),
            employee_first_name: String::new(),
            employee_last_name: String::new(),
            box_1_wages: Money::ZERO,
            box_2_federal_tax_withheld: Money::ZERO,
            box_3_social_security_wages: Money::ZERO,
            box_4_social_security_tax_withheld: Money::ZERO,
            box_5_medicare_wages: Money::ZERO,
            box_6_medicare_tax_withheld: Money::ZERO,
            box_7_social_security_tips: Money::ZERO,
            box_8_allocated_tips: Money::ZERO,
            box_10_dependent_care_benefits: Money::ZERO,
            box_11_nonqualified_plans: Money::ZERO,
            box_12: Vec::new(),
            box_13_statutory_employee: false,
            box_13_retirement_plan: false,
            box_13_third_party_sick_pay: false,
            state_info: Vec::new(),
            local_info: Vec::new(),
        }
    }
}

/// Box 12 code and amount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct W2Box12 {
    /// The letter code (A-HH).
    pub code: String,
    /// The amount.
    pub amount: Money,
}

/// State tax information from W-2.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct W2StateInfo {
    /// Box 15: State abbreviation.
    pub state: String,
    /// Box 15: Employer's state ID number.
    pub employer_state_id: String,
    /// Box 16: State wages, tips, etc.
    pub state_wages: Money,
    /// Box 17: State income tax withheld.
    pub state_tax_withheld: Money,
}

/// Local tax information from W-2.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct W2LocalInfo {
    /// Box 18: Local wages, tips, etc.
    pub local_wages: Money,
    /// Box 19: Local income tax withheld.
    pub local_tax_withheld: Money,
    /// Box 20: Locality name.
    pub locality_name: String,
}

impl W2 {
    /// Validates the W-2 data and returns any errors.
    pub fn validate(&self) -> TaxResult<()> {
        let mut errors = ValidationErrors::new();

        // Validate SSN format (XXX-XX-XXXX) - only if provided
        if !self.employee_ssn.is_empty() && !Self::is_valid_ssn(&self.employee_ssn) {
            errors.add_error("employee_ssn", "Invalid SSN format (expected XXX-XX-XXXX)");
        }

        // Validate EIN format (XX-XXXXXXX) - only if provided
        if !self.employer_ein.is_empty() && !Self::is_valid_ein(&self.employer_ein) {
            errors.add_error("employer_ein", "Invalid EIN format (expected XX-XXXXXXX)");
        }

        // Validate no negative values
        if self.box_1_wages.is_negative() {
            errors.add_error("box_1_wages", "Wages cannot be negative");
        }
        if self.box_2_federal_tax_withheld.is_negative() {
            errors.add_error(
                "box_2_federal_tax_withheld",
                "Federal tax withheld cannot be negative",
            );
        }
        if self.box_3_social_security_wages.is_negative() {
            errors.add_error(
                "box_3_social_security_wages",
                "Social Security wages cannot be negative",
            );
        }

        // Validate Box 12 codes
        for (i, box12) in self.box_12.iter().enumerate() {
            if !Self::is_valid_box_12_code(&box12.code) {
                errors.add_error(
                    format!("box_12[{}].code", i),
                    format!("Invalid Box 12 code: {}", box12.code),
                );
            }
        }

        errors.into_result()
    }

    /// Returns true if the SSN format is valid (XXX-XX-XXXX).
    fn is_valid_ssn(ssn: &str) -> bool {
        if ssn.len() != 11 {
            return false;
        }
        let chars: Vec<char> = ssn.chars().collect();
        chars[0..3].iter().all(|c| c.is_ascii_digit())
            && chars[3] == '-'
            && chars[4..6].iter().all(|c| c.is_ascii_digit())
            && chars[6] == '-'
            && chars[7..11].iter().all(|c| c.is_ascii_digit())
    }

    /// Returns true if the EIN format is valid (XX-XXXXXXX).
    fn is_valid_ein(ein: &str) -> bool {
        if ein.len() != 10 {
            return false;
        }
        let chars: Vec<char> = ein.chars().collect();
        chars[0..2].iter().all(|c| c.is_ascii_digit())
            && chars[2] == '-'
            && chars[3..10].iter().all(|c| c.is_ascii_digit())
    }

    /// Returns true if the Box 12 code is valid.
    fn is_valid_box_12_code(code: &str) -> bool {
        matches!(
            code,
            "A" | "B"
                | "C"
                | "D"
                | "E"
                | "F"
                | "G"
                | "H"
                | "J"
                | "K"
                | "L"
                | "M"
                | "N"
                | "P"
                | "Q"
                | "R"
                | "S"
                | "T"
                | "V"
                | "W"
                | "Y"
                | "Z"
                | "AA"
                | "BB"
                | "DD"
                | "EE"
                | "FF"
                | "GG"
                | "HH"
        )
    }

    /// Returns the total state tax withheld across all states.
    pub fn total_state_tax_withheld(&self) -> Money {
        self.state_info.iter().map(|s| s.state_tax_withheld).sum()
    }

    /// Returns the total local tax withheld across all localities.
    pub fn total_local_tax_withheld(&self) -> Money {
        self.local_info.iter().map(|l| l.local_tax_withheld).sum()
    }

    /// Returns the amount for a specific Box 12 code, if present.
    pub fn box_12_amount(&self, code: &str) -> Option<Money> {
        self.box_12
            .iter()
            .find(|b| b.code == code)
            .map(|b| b.amount)
    }

    /// Returns true if this is a statutory employee.
    pub fn is_statutory_employee(&self) -> bool {
        self.box_13_statutory_employee
    }

    /// Returns true if employer has a retirement plan.
    pub fn has_retirement_plan(&self) -> bool {
        self.box_13_retirement_plan
    }
}

impl InputForm for W2 {
    fn form_type(&self) -> InputFormType {
        InputFormType::W2
    }

    fn tax_year(&self) -> TaxYear {
        self.tax_year
    }

    fn form_id(&self) -> &str {
        &self.id
    }

    fn wages(&self) -> Option<Money> {
        Some(self.box_1_wages)
    }

    fn federal_withholding(&self) -> Option<Money> {
        Some(self.box_2_federal_tax_withheld)
    }

    fn state_withholding(&self) -> Option<Money> {
        let total = self.total_state_tax_withheld();
        if total.is_zero() {
            None
        } else {
            Some(total)
        }
    }

    fn local_withholding(&self) -> Option<Money> {
        let total = self.total_local_tax_withheld();
        if total.is_zero() {
            None
        } else {
            Some(total)
        }
    }

    fn social_security_wages(&self) -> Option<Money> {
        Some(self.box_3_social_security_wages)
    }

    fn social_security_tax_withheld(&self) -> Option<Money> {
        Some(self.box_4_social_security_tax_withheld)
    }

    fn medicare_wages(&self) -> Option<Money> {
        Some(self.box_5_medicare_wages)
    }

    fn medicare_tax_withheld(&self) -> Option<Money> {
        Some(self.box_6_medicare_tax_withheld)
    }

    fn employer_ein(&self) -> Option<&str> {
        Some(&self.employer_ein)
    }

    fn employer_name(&self) -> Option<&str> {
        Some(&self.employer_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_w2() -> W2 {
        W2 {
            id: "w2-001".to_string(),
            tax_year: 2025,
            employer_ein: "12-3456789".to_string(),
            employer_name: "Acme Corp".to_string(),
            employee_ssn: "123-45-6789".to_string(),
            employee_first_name: "John".to_string(),
            employee_last_name: "Doe".to_string(),
            box_1_wages: Money::from_dollars(75000),
            box_2_federal_tax_withheld: Money::from_dollars(10000),
            box_3_social_security_wages: Money::from_dollars(75000),
            box_4_social_security_tax_withheld: Money::from_cents(465000), // 6.2%
            box_5_medicare_wages: Money::from_dollars(75000),
            box_6_medicare_tax_withheld: Money::from_cents(108750), // 1.45%
            box_7_social_security_tips: Money::ZERO,
            box_8_allocated_tips: Money::ZERO,
            box_10_dependent_care_benefits: Money::ZERO,
            box_11_nonqualified_plans: Money::ZERO,
            box_12: vec![],
            box_13_statutory_employee: false,
            box_13_retirement_plan: true,
            box_13_third_party_sick_pay: false,
            state_info: vec![W2StateInfo {
                state: "CA".to_string(),
                employer_state_id: "123456789".to_string(),
                state_wages: Money::from_dollars(75000),
                state_tax_withheld: Money::from_dollars(5000),
            }],
            local_info: vec![],
        }
    }

    #[test]
    fn test_w2_validation_valid() {
        let w2 = sample_w2();
        assert!(w2.validate().is_ok());
    }

    #[test]
    fn test_w2_validation_invalid_ssn() {
        let mut w2 = sample_w2();
        w2.employee_ssn = "invalid".to_string();
        assert!(w2.validate().is_err());
    }

    #[test]
    fn test_w2_validation_invalid_ein() {
        let mut w2 = sample_w2();
        w2.employer_ein = "invalid".to_string();
        assert!(w2.validate().is_err());
    }

    #[test]
    fn test_w2_input_form_trait() {
        let w2 = sample_w2();
        assert_eq!(w2.form_type(), InputFormType::W2);
        assert_eq!(w2.tax_year(), 2025);
        assert_eq!(w2.wages(), Some(Money::from_dollars(75000)));
        assert_eq!(w2.federal_withholding(), Some(Money::from_dollars(10000)));
    }

    #[test]
    fn test_w2_default() {
        let w2 = W2::default();
        assert_eq!(w2.tax_year, 2025);
        assert_eq!(w2.box_1_wages, Money::ZERO);
    }
}
