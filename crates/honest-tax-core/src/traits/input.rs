//! Trait for input forms (documents received by taxpayer).

use crate::money::Money;
use crate::types::{InputFormType, TaxYear};

/// Trait implemented by all input forms (W-2, 1099s, etc.).
///
/// Input forms represent documents received by the taxpayer from employers,
/// financial institutions, and government agencies. Each form type provides
/// specific income and withholding information.
///
/// # Example
///
/// ```ignore
/// use honest_tax_core::traits::InputForm;
///
/// let w2 = W2 { /* ... */ };
/// let wages = w2.wages().unwrap_or(Money::ZERO);
/// let withholding = w2.federal_withholding().unwrap_or(Money::ZERO);
/// ```
pub trait InputForm: Send + Sync + std::fmt::Debug {
    /// Returns the form type identifier.
    fn form_type(&self) -> InputFormType;

    /// Returns the tax year this form is for.
    fn tax_year(&self) -> TaxYear;

    /// Returns a unique identifier for this form instance.
    ///
    /// Used to track multiple forms of the same type (e.g., multiple W-2s).
    fn form_id(&self) -> &str;

    // ─────────────────────────────────────────────────────────────────────────
    // Income extraction methods
    // ─────────────────────────────────────────────────────────────────────────

    /// Wages, salaries, tips (W-2 Box 1, some 1099s).
    fn wages(&self) -> Option<Money> {
        None
    }

    /// Taxable interest income (1099-INT Box 1).
    fn taxable_interest(&self) -> Option<Money> {
        None
    }

    /// Tax-exempt interest income (1099-INT Box 8).
    fn tax_exempt_interest(&self) -> Option<Money> {
        None
    }

    /// Ordinary dividends (1099-DIV Box 1a).
    fn ordinary_dividends(&self) -> Option<Money> {
        None
    }

    /// Qualified dividends (1099-DIV Box 1b).
    fn qualified_dividends(&self) -> Option<Money> {
        None
    }

    /// Capital gain distributions (1099-DIV Box 2a).
    fn capital_gain_distributions(&self) -> Option<Money> {
        None
    }

    /// IRA distributions - gross amount (1099-R).
    fn ira_distributions_gross(&self) -> Option<Money> {
        None
    }

    /// IRA distributions - taxable amount (1099-R).
    fn ira_distributions_taxable(&self) -> Option<Money> {
        None
    }

    /// Pension/annuity - gross amount (1099-R).
    fn pension_gross(&self) -> Option<Money> {
        None
    }

    /// Pension/annuity - taxable amount (1099-R).
    fn pension_taxable(&self) -> Option<Money> {
        None
    }

    /// Social Security benefits - gross amount (SSA-1099).
    fn social_security_gross(&self) -> Option<Money> {
        None
    }

    /// Social Security benefits - taxable amount (calculated).
    fn social_security_taxable(&self) -> Option<Money> {
        None
    }

    /// Unemployment compensation (1099-G Box 1).
    fn unemployment_compensation(&self) -> Option<Money> {
        None
    }

    /// State/local tax refund (1099-G Box 2).
    fn state_tax_refund(&self) -> Option<Money> {
        None
    }

    /// Nonemployee compensation (1099-NEC Box 1).
    fn nonemployee_compensation(&self) -> Option<Money> {
        None
    }

    /// Other income not categorized above.
    fn other_income(&self) -> Option<Money> {
        None
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Withholding extraction methods
    // ─────────────────────────────────────────────────────────────────────────

    /// Federal income tax withheld.
    fn federal_withholding(&self) -> Option<Money> {
        None
    }

    /// State income tax withheld.
    fn state_withholding(&self) -> Option<Money> {
        None
    }

    /// Local income tax withheld.
    fn local_withholding(&self) -> Option<Money> {
        None
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Social Security / Medicare (W-2)
    // ─────────────────────────────────────────────────────────────────────────

    /// Social Security wages (W-2 Box 3).
    fn social_security_wages(&self) -> Option<Money> {
        None
    }

    /// Social Security tax withheld (W-2 Box 4).
    fn social_security_tax_withheld(&self) -> Option<Money> {
        None
    }

    /// Medicare wages (W-2 Box 5).
    fn medicare_wages(&self) -> Option<Money> {
        None
    }

    /// Medicare tax withheld (W-2 Box 6).
    fn medicare_tax_withheld(&self) -> Option<Money> {
        None
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Deduction-related items
    // ─────────────────────────────────────────────────────────────────────────

    /// Mortgage interest paid (1098 Box 1).
    fn mortgage_interest(&self) -> Option<Money> {
        None
    }

    /// Mortgage points paid (1098 Box 6).
    fn mortgage_points(&self) -> Option<Money> {
        None
    }

    /// Student loan interest paid (1098-E Box 1).
    fn student_loan_interest(&self) -> Option<Money> {
        None
    }

    /// Tuition and fees paid (1098-T).
    fn tuition_paid(&self) -> Option<Money> {
        None
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Employer information
    // ─────────────────────────────────────────────────────────────────────────

    /// Employer Identification Number (EIN).
    fn employer_ein(&self) -> Option<&str> {
        None
    }

    /// Employer name.
    fn employer_name(&self) -> Option<&str> {
        None
    }

    /// Payer name (for 1099 forms).
    fn payer_name(&self) -> Option<&str> {
        None
    }

    /// Payer TIN (for 1099 forms).
    fn payer_tin(&self) -> Option<&str> {
        None
    }
}

/// Extension trait for working with collections of input forms.
pub trait InputFormCollection {
    /// Sum all wages across all input forms.
    fn total_wages(&self) -> Money;

    /// Sum all taxable interest across all input forms.
    fn total_taxable_interest(&self) -> Money;

    /// Sum all ordinary dividends across all input forms.
    fn total_ordinary_dividends(&self) -> Money;

    /// Sum all qualified dividends across all input forms.
    fn total_qualified_dividends(&self) -> Money;

    /// Sum all federal withholding across all input forms.
    fn total_federal_withholding(&self) -> Money;

    /// Sum all state withholding across all input forms.
    fn total_state_withholding(&self) -> Money;
}

impl<T: AsRef<[Box<dyn InputForm>]>> InputFormCollection for T {
    fn total_wages(&self) -> Money {
        self.as_ref()
            .iter()
            .filter_map(|f| f.wages())
            .sum()
    }

    fn total_taxable_interest(&self) -> Money {
        self.as_ref()
            .iter()
            .filter_map(|f| f.taxable_interest())
            .sum()
    }

    fn total_ordinary_dividends(&self) -> Money {
        self.as_ref()
            .iter()
            .filter_map(|f| f.ordinary_dividends())
            .sum()
    }

    fn total_qualified_dividends(&self) -> Money {
        self.as_ref()
            .iter()
            .filter_map(|f| f.qualified_dividends())
            .sum()
    }

    fn total_federal_withholding(&self) -> Money {
        self.as_ref()
            .iter()
            .filter_map(|f| f.federal_withholding())
            .sum()
    }

    fn total_state_withholding(&self) -> Money {
        self.as_ref()
            .iter()
            .filter_map(|f| f.state_withholding())
            .sum()
    }
}
