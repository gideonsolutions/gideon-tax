use crate::Usd;

use super::StateLocalTax;

/// Compute-relevant fields from IRS Form W-2 (2025).
///
/// Only includes fields that affect federal tax computation.
/// Excludes names, addresses, SSNs, EINs, employer metadata,
/// and W2 security/download information.
#[derive(Debug, Clone, Default)]
pub struct CoreW2 {
    /// Box 1: wages, tips, other compensation
    pub wages_amt: Usd,
    /// Box 2: federal income tax withheld
    pub withholding_amt: Usd,
    /// Box 3: social security wages
    pub social_security_wages_amt: Usd,
    /// Box 4: social security tax withheld
    pub social_security_tax_amt: Usd,
    /// Box 5: Medicare wages and tips
    pub medicare_wages_and_tips_amt: Usd,
    /// Box 6: Medicare tax withheld
    pub medicare_tax_withheld_amt: Usd,
    /// Box 7: social security tips
    pub social_security_tips_amt: Usd,
    /// Box 8: allocated tips
    pub allocated_tips_amt: Usd,
    /// Box 10: dependent care benefits
    pub dependent_care_benefits_amt: Usd,
    /// Box 11: nonqualified plans
    pub nonqualified_plans_amt: Usd,
    /// Box 13: statutory employee
    pub statutory_employee_ind: bool,
    /// Box 13: retirement plan
    pub retirement_plan_ind: bool,
    /// Box 13: third-party sick pay
    pub third_party_sick_pay_ind: bool,
    /// Boxes 15–19: state and local wages/withholding
    pub state_local_tax: Vec<StateLocalTax>,
}
