//! Core types used throughout the tax calculation engine.

use serde::{Deserialize, Serialize};

/// Tax year (e.g., 2025).
pub type TaxYear = u16;

/// Supported tax years.
pub const MIN_SUPPORTED_YEAR: TaxYear = 2025;
pub const MAX_SUPPORTED_YEAR: TaxYear = 2025; // Will increase as we add support

/// Filing status for federal income tax.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilingStatus {
    Single,
    MarriedFilingJointly,
    MarriedFilingSeparately,
    HeadOfHousehold,
    QualifyingSurvivingSpouse,
}

impl FilingStatus {
    /// Returns all filing statuses.
    pub fn all() -> &'static [FilingStatus] {
        &[
            FilingStatus::Single,
            FilingStatus::MarriedFilingJointly,
            FilingStatus::MarriedFilingSeparately,
            FilingStatus::HeadOfHousehold,
            FilingStatus::QualifyingSurvivingSpouse,
        ]
    }

    /// Returns the IRS form code for this filing status.
    pub fn code(&self) -> &'static str {
        match self {
            FilingStatus::Single => "S",
            FilingStatus::MarriedFilingJointly => "MFJ",
            FilingStatus::MarriedFilingSeparately => "MFS",
            FilingStatus::HeadOfHousehold => "HOH",
            FilingStatus::QualifyingSurvivingSpouse => "QSS",
        }
    }
}

impl std::fmt::Display for FilingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilingStatus::Single => write!(f, "Single"),
            FilingStatus::MarriedFilingJointly => write!(f, "Married Filing Jointly"),
            FilingStatus::MarriedFilingSeparately => write!(f, "Married Filing Separately"),
            FilingStatus::HeadOfHousehold => write!(f, "Head of Household"),
            FilingStatus::QualifyingSurvivingSpouse => write!(f, "Qualifying Surviving Spouse"),
        }
    }
}

/// Relationship of a dependent to the taxpayer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependentRelationship {
    Son,
    Daughter,
    Stepchild,
    FosterChild,
    Brother,
    Sister,
    Stepbrother,
    Stepsister,
    HalfBrother,
    HalfSister,
    Grandchild,
    Niece,
    Nephew,
    Parent,
    Grandparent,
    AuntUncle,
    Other,
}

/// Information about a dependent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Dependent {
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Social Security Number (format: XXX-XX-XXXX).
    pub ssn: String,
    /// Relationship to taxpayer.
    pub relationship: DependentRelationship,
    /// Age at end of tax year.
    pub age: u8,
    /// Months lived with taxpayer during tax year.
    pub months_lived_with_taxpayer: u8,
    /// Whether dependent is permanently and totally disabled.
    pub is_disabled: bool,
    /// Whether dependent is a full-time student (ages 19-23).
    pub is_student: bool,
}

impl Dependent {
    /// Returns true if this dependent qualifies for the Child Tax Credit.
    ///
    /// Requirements:
    /// - Under age 17 at end of tax year
    /// - Qualifying child relationship
    /// - Lived with taxpayer for more than half the year (6+ months)
    /// - Has valid SSN
    pub fn qualifies_for_ctc(&self) -> bool {
        self.age < 17
            && self.months_lived_with_taxpayer >= 6
            && self.is_qualifying_child_relationship()
            && !self.ssn.is_empty()
    }

    /// Returns true if this dependent qualifies for Credit for Other Dependents.
    ///
    /// For dependents who don't qualify for CTC (age 17+, or qualifying relatives).
    pub fn qualifies_for_odc(&self) -> bool {
        !self.qualifies_for_ctc()
    }

    /// Returns true if relationship qualifies as a "qualifying child."
    fn is_qualifying_child_relationship(&self) -> bool {
        matches!(
            self.relationship,
            DependentRelationship::Son
                | DependentRelationship::Daughter
                | DependentRelationship::Stepchild
                | DependentRelationship::FosterChild
                | DependentRelationship::Brother
                | DependentRelationship::Sister
                | DependentRelationship::Stepbrother
                | DependentRelationship::Stepsister
                | DependentRelationship::HalfBrother
                | DependentRelationship::HalfSister
                | DependentRelationship::Grandchild
                | DependentRelationship::Niece
                | DependentRelationship::Nephew
        )
    }
}

/// Basic taxpayer information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxpayerInfo {
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Social Security Number (format: XXX-XX-XXXX).
    pub ssn: String,
    /// Date of birth (YYYY-MM-DD format).
    pub date_of_birth: String,
    /// Whether taxpayer is blind.
    pub is_blind: bool,
}

impl TaxpayerInfo {
    /// Returns age at end of given tax year.
    ///
    /// Assumes date_of_birth is in YYYY-MM-DD format.
    pub fn age_at_year_end(&self, tax_year: TaxYear) -> Option<u8> {
        let birth_year: u16 = self.date_of_birth.get(0..4)?.parse().ok()?;
        let birth_month: u8 = self.date_of_birth.get(5..7)?.parse().ok()?;
        let birth_day: u8 = self.date_of_birth.get(8..10)?.parse().ok()?;

        // IRS considers you 65 on the day before your 65th birthday
        // For simplicity, we check if birthday is on or before Dec 31 of tax year
        let age = tax_year.saturating_sub(birth_year) as u8;

        // If birthday hasn't occurred by year end, subtract 1
        // (birth month > 12 means birthday is next year)
        if birth_month == 12 && birth_day == 31 {
            // Born on Dec 31 - birthday occurs on last day of year
        } else if birth_month > 12 {
            // Invalid month
            return None;
        }

        Some(age)
    }

    /// Returns true if taxpayer is 65 or older at end of tax year.
    ///
    /// Per IRS: "You are considered 65 on the day before your 65th birthday."
    pub fn is_65_or_older(&self, tax_year: TaxYear) -> bool {
        self.age_at_year_end(tax_year)
            .map(|age| age >= 65)
            .unwrap_or(false)
    }
}

/// Types of input forms (documents received by taxpayer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InputFormType {
    /// Form W-2: Wage and Tax Statement
    W2,
    /// Form 1099-INT: Interest Income
    F1099Int,
    /// Form 1099-DIV: Dividends and Distributions
    F1099Div,
    /// Form 1099-B: Proceeds from Broker Transactions
    F1099B,
    /// Form 1099-R: Distributions from Pensions, Annuities, etc.
    F1099R,
    /// Form 1099-G: Government Payments (unemployment, state refunds)
    F1099G,
    /// Form 1099-MISC: Miscellaneous Income
    F1099Misc,
    /// Form 1099-NEC: Nonemployee Compensation
    F1099Nec,
    /// Form 1099-K: Payment Card and Third Party Network Transactions
    F1099K,
    /// Form SSA-1099: Social Security Benefit Statement
    Ssa1099,
    /// Form 1098: Mortgage Interest Statement
    F1098,
    /// Form 1098-T: Tuition Statement
    F1098T,
    /// Form 1098-E: Student Loan Interest Statement
    F1098E,
}

impl std::fmt::Display for InputFormType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputFormType::W2 => write!(f, "W-2"),
            InputFormType::F1099Int => write!(f, "1099-INT"),
            InputFormType::F1099Div => write!(f, "1099-DIV"),
            InputFormType::F1099B => write!(f, "1099-B"),
            InputFormType::F1099R => write!(f, "1099-R"),
            InputFormType::F1099G => write!(f, "1099-G"),
            InputFormType::F1099Misc => write!(f, "1099-MISC"),
            InputFormType::F1099Nec => write!(f, "1099-NEC"),
            InputFormType::F1099K => write!(f, "1099-K"),
            InputFormType::Ssa1099 => write!(f, "SSA-1099"),
            InputFormType::F1098 => write!(f, "1098"),
            InputFormType::F1098T => write!(f, "1098-T"),
            InputFormType::F1098E => write!(f, "1098-E"),
        }
    }
}

/// Types of output forms (forms generated by the calculator).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OutputFormType {
    /// Form 1040: U.S. Individual Income Tax Return
    Form1040,
    /// Schedule 1: Additional Income and Adjustments to Income
    Schedule1,
    /// Schedule 2: Additional Taxes
    Schedule2,
    /// Schedule 3: Additional Credits and Payments
    Schedule3,
    /// Schedule A: Itemized Deductions
    ScheduleA,
    /// Schedule B: Interest and Ordinary Dividends
    ScheduleB,
    /// Schedule C: Profit or Loss From Business
    ScheduleC,
    /// Schedule D: Capital Gains and Losses
    ScheduleD,
    /// Schedule E: Supplemental Income and Loss
    ScheduleE,
    /// Schedule SE: Self-Employment Tax
    ScheduleSE,
    /// Schedule 8812: Credits for Qualifying Children
    Schedule8812,
    /// Schedule EIC: Earned Income Credit
    ScheduleEic,
}

impl std::fmt::Display for OutputFormType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormType::Form1040 => write!(f, "Form 1040"),
            OutputFormType::Schedule1 => write!(f, "Schedule 1"),
            OutputFormType::Schedule2 => write!(f, "Schedule 2"),
            OutputFormType::Schedule3 => write!(f, "Schedule 3"),
            OutputFormType::ScheduleA => write!(f, "Schedule A"),
            OutputFormType::ScheduleB => write!(f, "Schedule B"),
            OutputFormType::ScheduleC => write!(f, "Schedule C"),
            OutputFormType::ScheduleD => write!(f, "Schedule D"),
            OutputFormType::ScheduleE => write!(f, "Schedule E"),
            OutputFormType::ScheduleSE => write!(f, "Schedule SE"),
            OutputFormType::Schedule8812 => write!(f, "Schedule 8812"),
            OutputFormType::ScheduleEic => write!(f, "Schedule EIC"),
        }
    }
}
