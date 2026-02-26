use us_tax_brackets::TaxYear;

use crate::forms::{DynForm, Form, FormType, OutputForm};
use crate::{GideonTaxError, Usd};

// =========================================================================
// Input
// =========================================================================

/// All information needed to complete Form 965-A.
///
/// Form 965-A is a reporting form for the Tax Cuts and Jobs Act transition
/// tax (section 965). All fields are passthrough — they come directly from
/// the taxpayer's records with no computation.
#[derive(Debug, Clone)]
pub struct F965AInput {
    /// Check this box if this is an amended report
    pub amended_ind: bool,
    /// Part I, Column (d): Net 965 Tax Liability (subtract column (c) from column (b))
    pub net_section965_tax_liab_paid_amt: Usd,
    /// Part II, Column (j): Net 965 Tax Liability Remaining Unpaid (see instructions)
    pub net_section965_tax_liab_unpaid_amt: Usd,
    /// Part III, Column (g): Deferred Net 965 Tax Liability
    pub net_sect965_deferred_tax_liab_amt: Usd,
    /// Part I, Column (e): S Corporation Shareholder Total Deferred Net 965 Tax Liability
    pub tot_s_corp_defrd_net965_tax_liab_amt: Usd,
}

// =========================================================================
// Output
// =========================================================================

/// Output fields for IRS Form 965-A (2025) — Individual Report of Net 965 Tax Liability.
#[derive(Debug, Clone, Default)]
pub struct Output965A {
    // -----------------------------------------------------------------------
    // Header
    // -----------------------------------------------------------------------
    /// Check this box if this is an amended report
    pub amended_ind: bool,

    // -----------------------------------------------------------------------
    // Part I — Report of Net 965 Tax Liability and Election To Pay in Installments
    // -----------------------------------------------------------------------
    /// Part I, Column (d): Net 965 Tax Liability (subtract column (c) from column (b))
    pub net_section965_tax_liab_paid_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Record of Amount of Net 965 Tax Liability Paid by the Taxpayer
    // -----------------------------------------------------------------------
    /// Part II, Column (j): Net 965 Tax Liability Remaining Unpaid (see instructions)
    pub net_section965_tax_liab_unpaid_amt: Usd,

    // -----------------------------------------------------------------------
    // Part III — S Corporation Shareholder: Report of Calculation of Net Tax Liability
    //            Related to 965 Amounts Allocated From an S Corporation and
    //            Election To Defer Such Net 965 Tax Liability
    // -----------------------------------------------------------------------
    /// Part III, Column (g): Deferred Net 965 Tax Liability (if column (f) is "Yes," enter amount from column (e))
    pub net_sect965_deferred_tax_liab_amt: Usd,
    /// Part I, Column (e): S Corporation Shareholder Total Deferred Net 965 Tax Liability
    /// (line total from Part III, column (g), see instructions)
    pub tot_s_corp_defrd_net965_tax_liab_amt: Usd,
}

// =========================================================================
// Trait impls
// =========================================================================

impl Form for Output965A {
    fn name() -> &'static str {
        "Form 965-A"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Output
    }
}

impl OutputForm for Output965A {
    type Input = F965AInput;

    fn must_file(input: &Self::Input) -> bool {
        input.net_section965_tax_liab_paid_amt != Usd::ZERO
            || input.net_section965_tax_liab_unpaid_amt != Usd::ZERO
            || input.net_sect965_deferred_tax_liab_amt != Usd::ZERO
            || input.tot_s_corp_defrd_net965_tax_liab_amt != Usd::ZERO
    }

    fn try_new(input: Self::Input) -> Result<Self, GideonTaxError> {
        Ok(Output965A {
            amended_ind: input.amended_ind,
            net_section965_tax_liab_paid_amt: input.net_section965_tax_liab_paid_amt,
            net_section965_tax_liab_unpaid_amt: input.net_section965_tax_liab_unpaid_amt,
            net_sect965_deferred_tax_liab_amt: input.net_sect965_deferred_tax_liab_amt,
            tot_s_corp_defrd_net965_tax_liab_amt: input.tot_s_corp_defrd_net965_tax_liab_amt,
        })
    }

    fn dependencies() -> &'static [DynForm] {
        &[]
    }

    fn is_valid(&self) -> bool {
        true
    }
}

// =========================================================================
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn zero_input() -> F965AInput {
        F965AInput {
            amended_ind: false,
            net_section965_tax_liab_paid_amt: Usd::ZERO,
            net_section965_tax_liab_unpaid_amt: Usd::ZERO,
            net_sect965_deferred_tax_liab_amt: Usd::ZERO,
            tot_s_corp_defrd_net965_tax_liab_amt: Usd::ZERO,
        }
    }

    #[test]
    fn must_file_all_zero() {
        let input = zero_input();
        assert!(!Output965A::must_file(&input));
    }

    #[test]
    fn must_file_net_liability_paid() {
        let mut input = zero_input();
        input.net_section965_tax_liab_paid_amt = Usd::from_dollars(10_000);
        assert!(Output965A::must_file(&input));
    }

    #[test]
    fn must_file_unpaid_liability() {
        let mut input = zero_input();
        input.net_section965_tax_liab_unpaid_amt = Usd::from_dollars(5_000);
        assert!(Output965A::must_file(&input));
    }

    #[test]
    fn must_file_deferred_liability() {
        let mut input = zero_input();
        input.net_sect965_deferred_tax_liab_amt = Usd::from_dollars(3_000);
        assert!(Output965A::must_file(&input));
    }

    #[test]
    fn must_file_s_corp_deferred() {
        let mut input = zero_input();
        input.tot_s_corp_defrd_net965_tax_liab_amt = Usd::from_dollars(7_000);
        assert!(Output965A::must_file(&input));
    }

    #[test]
    fn try_new_passthrough() {
        let input = F965AInput {
            amended_ind: true,
            net_section965_tax_liab_paid_amt: Usd::from_dollars(100_000),
            net_section965_tax_liab_unpaid_amt: Usd::from_dollars(50_000),
            net_sect965_deferred_tax_liab_amt: Usd::from_dollars(25_000),
            tot_s_corp_defrd_net965_tax_liab_amt: Usd::from_dollars(12_000),
        };
        let form = Output965A::try_new(input).unwrap();
        assert!(form.amended_ind);
        assert_eq!(
            form.net_section965_tax_liab_paid_amt,
            Usd::from_dollars(100_000)
        );
        assert_eq!(
            form.net_section965_tax_liab_unpaid_amt,
            Usd::from_dollars(50_000)
        );
        assert_eq!(
            form.net_sect965_deferred_tax_liab_amt,
            Usd::from_dollars(25_000)
        );
        assert_eq!(
            form.tot_s_corp_defrd_net965_tax_liab_amt,
            Usd::from_dollars(12_000)
        );
        assert!(form.is_valid());
    }

    #[test]
    fn try_new_zero_input() {
        let form = Output965A::try_new(zero_input()).unwrap();
        assert!(!form.amended_ind);
        assert_eq!(form.net_section965_tax_liab_paid_amt, Usd::ZERO);
        assert_eq!(form.net_section965_tax_liab_unpaid_amt, Usd::ZERO);
        assert_eq!(form.net_sect965_deferred_tax_liab_amt, Usd::ZERO);
        assert_eq!(form.tot_s_corp_defrd_net965_tax_liab_amt, Usd::ZERO);
        assert!(form.is_valid());
    }

    #[test]
    fn is_valid_always_true() {
        let form = Output965A::default();
        assert!(form.is_valid());
    }

    #[test]
    fn no_dependencies() {
        assert!(Output965A::dependencies().is_empty());
    }

    #[test]
    fn form_name() {
        assert_eq!(Output965A::name(), "Form 965-A");
    }

    #[test]
    fn form_year() {
        let form = Output965A::default();
        assert_eq!(form.year(), TaxYear::Y2025);
    }

    #[test]
    fn form_type_is_output() {
        assert_eq!(<Output965A as Form>::form_type(), FormType::Output);
    }
}
