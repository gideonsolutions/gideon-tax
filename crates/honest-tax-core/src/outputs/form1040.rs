//! Form 1040: U.S. Individual Income Tax Return

use crate::money::Money;
use crate::traits::{FormLine, FormValue, OutputForm};
use crate::types::{OutputFormType, TaxYear};
use serde::{Deserialize, Serialize};

/// Form 1040: U.S. Individual Income Tax Return
///
/// Represents a completed Form 1040 with all calculated values.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form1040 {
    /// Tax year this form is for.
    pub tax_year: TaxYear,

    // ─────────────────────────────────────────────────────────────────────────
    // Income (Lines 1-9)
    // ─────────────────────────────────────────────────────────────────────────
    /// Line 1a: Total amount from Form(s) W-2, box 1
    pub line_1a: Money,

    /// Line 1b: Household employee wages not reported on W-2
    pub line_1b: Money,

    /// Line 1c: Tip income not reported on line 1a
    pub line_1c: Money,

    /// Line 1d: Medicaid waiver payments not reported on W-2
    pub line_1d: Money,

    /// Line 1e: Taxable dependent care benefits (Form 2441, line 26)
    pub line_1e: Money,

    /// Line 1f: Employer-provided adoption benefits (Form 8839, line 29)
    pub line_1f: Money,

    /// Line 1g: Wages from Form 8919, line 6
    pub line_1g: Money,

    /// Line 1h: Other earned income
    pub line_1h: Money,

    /// Line 1i: Nontaxable combat pay election
    pub line_1i: Money,

    /// Line 1z: Sum of lines 1a through 1h
    pub line_1z: Money,

    /// Line 2a: Tax-exempt interest
    pub line_2a: Money,

    /// Line 2b: Taxable interest
    pub line_2b: Money,

    /// Line 3a: Qualified dividends
    pub line_3a: Money,

    /// Line 3b: Ordinary dividends
    pub line_3b: Money,

    /// Line 4a: IRA distributions (gross)
    pub line_4a: Money,

    /// Line 4b: IRA distributions (taxable)
    pub line_4b: Money,

    /// Line 5a: Pensions and annuities (gross)
    pub line_5a: Money,

    /// Line 5b: Pensions and annuities (taxable)
    pub line_5b: Money,

    /// Line 6a: Social Security benefits (gross)
    pub line_6a: Money,

    /// Line 6b: Social Security benefits (taxable)
    pub line_6b: Money,

    /// Line 7: Capital gain or (loss)
    pub line_7: Money,

    /// Line 8: Additional income from Schedule 1, line 10
    pub line_8: Money,

    /// Line 9: Total income (sum of 1z, 2b, 3b, 4b, 5b, 6b, 7, 8)
    pub line_9: Money,

    // ─────────────────────────────────────────────────────────────────────────
    // Adjustments (Line 10-11)
    // ─────────────────────────────────────────────────────────────────────────
    /// Line 10: Adjustments to income from Schedule 1, line 26
    pub line_10: Money,

    /// Line 11: Adjusted Gross Income (line 9 minus line 10)
    pub line_11: Money,

    // ─────────────────────────────────────────────────────────────────────────
    // Deductions (Lines 12-14)
    // ─────────────────────────────────────────────────────────────────────────
    /// Line 12: Standard deduction or itemized deductions
    pub line_12: Money,

    /// Line 12 checkbox: Standard deduction used
    pub line_12_standard_deduction: bool,

    /// Line 13a: Qualified business income deduction
    pub line_13a: Money,

    /// Line 13b: Total other deductions from Schedule 1-A, line 38
    pub line_13b: Money,

    /// Line 14: Total deductions (line 12 + 13a + 13b)
    pub line_14: Money,

    // ─────────────────────────────────────────────────────────────────────────
    // Taxable Income (Line 15)
    // ─────────────────────────────────────────────────────────────────────────
    /// Line 15: Taxable income (line 11 minus line 14, but not less than zero)
    pub line_15: Money,

    // ─────────────────────────────────────────────────────────────────────────
    // Tax and Credits (Lines 16-24)
    // ─────────────────────────────────────────────────────────────────────────
    /// Line 16: Tax (from Tax Table or Tax Computation Worksheet)
    pub line_16: Money,

    /// Line 17: Amount from Schedule 2, line 3
    pub line_17: Money,

    /// Line 18: Total tax before credits (line 16 + line 17)
    pub line_18: Money,

    /// Line 19: Child tax credit and credit for other dependents (Schedule 8812)
    pub line_19: Money,

    /// Line 20: Amount from Schedule 3, line 8
    pub line_20: Money,

    /// Line 21: Total nonrefundable credits (line 19 + line 20)
    pub line_21: Money,

    /// Line 22: Tax after nonrefundable credits (line 18 minus line 21, but not less than zero)
    pub line_22: Money,

    /// Line 23: Other taxes from Schedule 2, line 21
    pub line_23: Money,

    /// Line 24: Total tax (line 22 + line 23)
    pub line_24: Money,

    // ─────────────────────────────────────────────────────────────────────────
    // Payments (Lines 25-33)
    // ─────────────────────────────────────────────────────────────────────────
    /// Line 25a: Federal income tax withheld from W-2
    pub line_25a: Money,

    /// Line 25b: Federal income tax withheld from 1099
    pub line_25b: Money,

    /// Line 25c: Other federal income tax withheld
    pub line_25c: Money,

    /// Line 25d: Total withholding (sum of 25a-25c)
    pub line_25d: Money,

    /// Line 26: Estimated tax payments
    pub line_26: Money,

    /// Line 27a: Earned income credit (EIC)
    pub line_27a: Money,

    /// Line 28: Additional child tax credit (Schedule 8812)
    pub line_28: Money,

    /// Line 29: American opportunity credit (Form 8863, line 8)
    pub line_29: Money,

    /// Line 30: Reserved for future use
    pub line_30: Money,

    /// Line 31: Amount from Schedule 3, line 15
    pub line_31: Money,

    /// Line 32: Total other payments and refundable credits (sum of 27a, 28, 29, 30, 31)
    pub line_32: Money,

    /// Line 33: Total payments (line 25d + line 26 + line 32)
    pub line_33: Money,

    // ─────────────────────────────────────────────────────────────────────────
    // Refund or Amount Owed (Lines 34-38)
    // ─────────────────────────────────────────────────────────────────────────
    /// Line 34: Overpayment (if line 33 > line 24)
    pub line_34: Money,

    /// Line 35a: Amount to be refunded
    pub line_35a: Money,

    /// Line 36: Amount applied to estimated tax
    pub line_36: Money,

    /// Line 37: Amount you owe (if line 24 > line 33)
    pub line_37: Money,

    /// Line 38: Estimated tax penalty
    pub line_38: Money,
}

impl Form1040 {
    /// Creates a new empty Form 1040 for the given tax year.
    pub fn new(tax_year: TaxYear) -> Self {
        Self {
            tax_year,
            line_1a: Money::ZERO,
            line_1b: Money::ZERO,
            line_1c: Money::ZERO,
            line_1d: Money::ZERO,
            line_1e: Money::ZERO,
            line_1f: Money::ZERO,
            line_1g: Money::ZERO,
            line_1h: Money::ZERO,
            line_1i: Money::ZERO,
            line_1z: Money::ZERO,
            line_2a: Money::ZERO,
            line_2b: Money::ZERO,
            line_3a: Money::ZERO,
            line_3b: Money::ZERO,
            line_4a: Money::ZERO,
            line_4b: Money::ZERO,
            line_5a: Money::ZERO,
            line_5b: Money::ZERO,
            line_6a: Money::ZERO,
            line_6b: Money::ZERO,
            line_7: Money::ZERO,
            line_8: Money::ZERO,
            line_9: Money::ZERO,
            line_10: Money::ZERO,
            line_11: Money::ZERO,
            line_12: Money::ZERO,
            line_12_standard_deduction: true,
            line_13a: Money::ZERO,
            line_13b: Money::ZERO,
            line_14: Money::ZERO,
            line_15: Money::ZERO,
            line_16: Money::ZERO,
            line_17: Money::ZERO,
            line_18: Money::ZERO,
            line_19: Money::ZERO,
            line_20: Money::ZERO,
            line_21: Money::ZERO,
            line_22: Money::ZERO,
            line_23: Money::ZERO,
            line_24: Money::ZERO,
            line_25a: Money::ZERO,
            line_25b: Money::ZERO,
            line_25c: Money::ZERO,
            line_25d: Money::ZERO,
            line_26: Money::ZERO,
            line_27a: Money::ZERO,
            line_28: Money::ZERO,
            line_29: Money::ZERO,
            line_30: Money::ZERO,
            line_31: Money::ZERO,
            line_32: Money::ZERO,
            line_33: Money::ZERO,
            line_34: Money::ZERO,
            line_35a: Money::ZERO,
            line_36: Money::ZERO,
            line_37: Money::ZERO,
            line_38: Money::ZERO,
        }
    }

    /// Calculates line 1z (total wages).
    pub fn calculate_line_1z(&mut self) {
        self.line_1z = self.line_1a
            + self.line_1b
            + self.line_1c
            + self.line_1d
            + self.line_1e
            + self.line_1f
            + self.line_1g
            + self.line_1h;
    }

    /// Calculates line 9 (total income).
    pub fn calculate_line_9(&mut self) {
        self.line_9 = self.line_1z
            + self.line_2b
            + self.line_3b
            + self.line_4b
            + self.line_5b
            + self.line_6b
            + self.line_7
            + self.line_8;
    }

    /// Calculates line 11 (AGI).
    pub fn calculate_line_11(&mut self) {
        self.line_11 = self.line_9.saturating_sub(self.line_10);
    }

    /// Calculates line 14 (total deductions).
    pub fn calculate_line_14(&mut self) {
        self.line_14 = self.line_12 + self.line_13a + self.line_13b;
    }

    /// Calculates line 15 (taxable income).
    pub fn calculate_line_15(&mut self) {
        self.line_15 = self.line_11.saturating_sub(self.line_14);
    }

    /// Calculates line 18 (total tax before credits).
    pub fn calculate_line_18(&mut self) {
        self.line_18 = self.line_16 + self.line_17;
    }

    /// Calculates line 21 (total nonrefundable credits).
    pub fn calculate_line_21(&mut self) {
        self.line_21 = self.line_19 + self.line_20;
    }

    /// Calculates line 22 (tax after nonrefundable credits).
    pub fn calculate_line_22(&mut self) {
        self.line_22 = self.line_18.saturating_sub(self.line_21);
    }

    /// Calculates line 24 (total tax).
    pub fn calculate_line_24(&mut self) {
        self.line_24 = self.line_22 + self.line_23;
    }

    /// Calculates line 25d (total withholding).
    pub fn calculate_line_25d(&mut self) {
        self.line_25d = self.line_25a + self.line_25b + self.line_25c;
    }

    /// Calculates line 32 (total refundable credits).
    pub fn calculate_line_32(&mut self) {
        self.line_32 =
            self.line_27a + self.line_28 + self.line_29 + self.line_30 + self.line_31;
    }

    /// Calculates line 33 (total payments).
    pub fn calculate_line_33(&mut self) {
        self.line_33 = self.line_25d + self.line_26 + self.line_32;
    }

    /// Calculates lines 34-37 (refund or amount owed).
    pub fn calculate_refund_or_owed(&mut self) {
        if self.line_33 > self.line_24 {
            self.line_34 = self.line_33 - self.line_24;
            self.line_35a = self.line_34 - self.line_36;
            self.line_37 = Money::ZERO;
        } else {
            self.line_34 = Money::ZERO;
            self.line_35a = Money::ZERO;
            self.line_37 = self.line_24 - self.line_33 + self.line_38;
        }
    }

    /// Returns true if the taxpayer is getting a refund.
    pub fn is_refund(&self) -> bool {
        self.line_34.is_positive()
    }

    /// Returns the refund amount (positive) or amount owed (negative).
    pub fn net_result(&self) -> Money {
        if self.is_refund() {
            self.line_35a
        } else {
            Money::ZERO - self.line_37
        }
    }
}

impl OutputForm for Form1040 {
    fn form_type(&self) -> OutputFormType {
        OutputFormType::Form1040
    }

    fn tax_year(&self) -> TaxYear {
        self.tax_year
    }

    fn line(&self, line_id: &str) -> Option<FormValue> {
        match line_id {
            "1a" => Some(FormValue::Currency(self.line_1a)),
            "1b" => Some(FormValue::Currency(self.line_1b)),
            "1c" => Some(FormValue::Currency(self.line_1c)),
            "1d" => Some(FormValue::Currency(self.line_1d)),
            "1e" => Some(FormValue::Currency(self.line_1e)),
            "1f" => Some(FormValue::Currency(self.line_1f)),
            "1g" => Some(FormValue::Currency(self.line_1g)),
            "1h" => Some(FormValue::Currency(self.line_1h)),
            "1i" => Some(FormValue::Currency(self.line_1i)),
            "1z" => Some(FormValue::Currency(self.line_1z)),
            "2a" => Some(FormValue::Currency(self.line_2a)),
            "2b" => Some(FormValue::Currency(self.line_2b)),
            "3a" => Some(FormValue::Currency(self.line_3a)),
            "3b" => Some(FormValue::Currency(self.line_3b)),
            "4a" => Some(FormValue::Currency(self.line_4a)),
            "4b" => Some(FormValue::Currency(self.line_4b)),
            "5a" => Some(FormValue::Currency(self.line_5a)),
            "5b" => Some(FormValue::Currency(self.line_5b)),
            "6a" => Some(FormValue::Currency(self.line_6a)),
            "6b" => Some(FormValue::Currency(self.line_6b)),
            "7" => Some(FormValue::Currency(self.line_7)),
            "8" => Some(FormValue::Currency(self.line_8)),
            "9" => Some(FormValue::Currency(self.line_9)),
            "10" => Some(FormValue::Currency(self.line_10)),
            "11" => Some(FormValue::Currency(self.line_11)),
            "12" => Some(FormValue::Currency(self.line_12)),
            "13a" => Some(FormValue::Currency(self.line_13a)),
            "13b" => Some(FormValue::Currency(self.line_13b)),
            "14" => Some(FormValue::Currency(self.line_14)),
            "15" => Some(FormValue::Currency(self.line_15)),
            "16" => Some(FormValue::Currency(self.line_16)),
            "17" => Some(FormValue::Currency(self.line_17)),
            "18" => Some(FormValue::Currency(self.line_18)),
            "19" => Some(FormValue::Currency(self.line_19)),
            "20" => Some(FormValue::Currency(self.line_20)),
            "21" => Some(FormValue::Currency(self.line_21)),
            "22" => Some(FormValue::Currency(self.line_22)),
            "23" => Some(FormValue::Currency(self.line_23)),
            "24" => Some(FormValue::Currency(self.line_24)),
            "25a" => Some(FormValue::Currency(self.line_25a)),
            "25b" => Some(FormValue::Currency(self.line_25b)),
            "25c" => Some(FormValue::Currency(self.line_25c)),
            "25d" => Some(FormValue::Currency(self.line_25d)),
            "26" => Some(FormValue::Currency(self.line_26)),
            "27a" => Some(FormValue::Currency(self.line_27a)),
            "28" => Some(FormValue::Currency(self.line_28)),
            "29" => Some(FormValue::Currency(self.line_29)),
            "30" => Some(FormValue::Currency(self.line_30)),
            "31" => Some(FormValue::Currency(self.line_31)),
            "32" => Some(FormValue::Currency(self.line_32)),
            "33" => Some(FormValue::Currency(self.line_33)),
            "34" => Some(FormValue::Currency(self.line_34)),
            "35a" => Some(FormValue::Currency(self.line_35a)),
            "36" => Some(FormValue::Currency(self.line_36)),
            "37" => Some(FormValue::Currency(self.line_37)),
            "38" => Some(FormValue::Currency(self.line_38)),
            _ => None,
        }
    }

    fn lines(&self) -> Vec<FormLine> {
        vec![
            FormLine {
                line_id: "1a".to_string(),
                label: "Wages from W-2".to_string(),
                value: FormValue::Currency(self.line_1a),
            },
            FormLine {
                line_id: "1z".to_string(),
                label: "Total wages".to_string(),
                value: FormValue::Currency(self.line_1z),
            },
            FormLine {
                line_id: "2b".to_string(),
                label: "Taxable interest".to_string(),
                value: FormValue::Currency(self.line_2b),
            },
            FormLine {
                line_id: "3b".to_string(),
                label: "Ordinary dividends".to_string(),
                value: FormValue::Currency(self.line_3b),
            },
            FormLine {
                line_id: "9".to_string(),
                label: "Total income".to_string(),
                value: FormValue::Currency(self.line_9),
            },
            FormLine {
                line_id: "10".to_string(),
                label: "Adjustments to income".to_string(),
                value: FormValue::Currency(self.line_10),
            },
            FormLine {
                line_id: "11".to_string(),
                label: "Adjusted gross income".to_string(),
                value: FormValue::Currency(self.line_11),
            },
            FormLine {
                line_id: "12".to_string(),
                label: "Standard/itemized deduction".to_string(),
                value: FormValue::Currency(self.line_12),
            },
            FormLine {
                line_id: "14".to_string(),
                label: "Total deductions".to_string(),
                value: FormValue::Currency(self.line_14),
            },
            FormLine {
                line_id: "15".to_string(),
                label: "Taxable income".to_string(),
                value: FormValue::Currency(self.line_15),
            },
            FormLine {
                line_id: "16".to_string(),
                label: "Tax".to_string(),
                value: FormValue::Currency(self.line_16),
            },
            FormLine {
                line_id: "19".to_string(),
                label: "Child tax credit".to_string(),
                value: FormValue::Currency(self.line_19),
            },
            FormLine {
                line_id: "24".to_string(),
                label: "Total tax".to_string(),
                value: FormValue::Currency(self.line_24),
            },
            FormLine {
                line_id: "25d".to_string(),
                label: "Total withholding".to_string(),
                value: FormValue::Currency(self.line_25d),
            },
            FormLine {
                line_id: "33".to_string(),
                label: "Total payments".to_string(),
                value: FormValue::Currency(self.line_33),
            },
            FormLine {
                line_id: "34".to_string(),
                label: "Overpayment".to_string(),
                value: FormValue::Currency(self.line_34),
            },
            FormLine {
                line_id: "35a".to_string(),
                label: "Refund".to_string(),
                value: FormValue::Currency(self.line_35a),
            },
            FormLine {
                line_id: "37".to_string(),
                label: "Amount owed".to_string(),
                value: FormValue::Currency(self.line_37),
            },
        ]
    }
}
