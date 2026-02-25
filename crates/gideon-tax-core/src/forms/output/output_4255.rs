use crate::Usd;

/// Output fields for IRS Form 4255 (Rev. December 2025) — Certain Credit Recapture, Excessive Payments, and Penalties.
#[derive(Debug, Clone, Default)]
pub struct Output4255 {
    // -----------------------------------------------------------------------
    // Part I — Summary (columns (a) through (t), lines 1a-3)
    // -----------------------------------------------------------------------
    /// Part I, column (a): Credit or deduction claimed in prior year(s) (as adjusted, if
    /// applicable)
    pub credit_base_py_amt: Usd,
    /// Part I, column (g): Recapture percentage
    pub recapture_pct: String,
    /// Part I, column (h): Amount of column (a) recaptured, including reduction of
    /// carryover
    pub aggregate_credit_decrease_amt: Usd,
    /// Part I, column (q): Amount that can be reduced by nonrefundable credits
    pub at_risk_credit_recapture_amt: Usd,
    /// Part I, column (r): Amount that cannot be reduced by nonrefundable credits
    pub nonqualifed_finance_rcptr_tax_amt: Usd,
    /// Part I, column (r): Net change amount for nonqualified financing
    pub nonqualified_finance_net_chg_amt: Usd,
    /// Part I, line 3: Total of all recapture, excessive payments, and penalties
    pub investment_cr_recapture_tax_amt: Usd,
    /// Tax from attached code (indicates which form/schedule the recapture flows to)
    pub tax_from_attached_cd: String,

    // -----------------------------------------------------------------------
    // Part II — Recapture Calculation
    // Section A — Properties
    // -----------------------------------------------------------------------
    /// Section A: Property description (type of property and general business credit)
    pub property_desc: String,

    // -----------------------------------------------------------------------
    // Section B — Original Credit
    // -----------------------------------------------------------------------
    /// Section B, Line 1: Original rate of credit
    pub credit_rt: String,
    /// Section B, Line 2: Credit base as of the end of the previous tax year
    pub credit_base_cy_amt: Usd,
    /// Section B, Line 5: Refigured credit
    pub refigured_credit_amt: Usd,
    /// Section B, Line 6: Credit taken for this property on Form 3800 in prior years
    pub prior_years_general_business_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Section C — Recapture From Increase in Nonqualified Nonrecourse Financing
    // -----------------------------------------------------------------------
    /// Section C, Line 8: Unused general business credits that would have been allowed if
    /// original credit had been figured with current-year credit base
    pub unused_general_bus_cr_orig_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Section D — Recapture From Disposition of Property, Cessation of Use as
    //              Qualified Credit Property, or Certain Expansions
    // -----------------------------------------------------------------------
    /// Section D, Line 10: Date property was placed in service
    pub placed_in_service_dt: String,
    /// Section D, Line 11: Date property ceased to be qualified credit property
    pub property_ceased_to_qualify_dt: String,
    /// Section D, Line 12: Number of full years between the date on line 10 and line 11
    pub property_qualified_year_cnt: u32,
    /// Section D, Line 13: Unused general business credits that would have been allowed had
    /// there been no credit from this property
    pub unused_general_bus_cr_no_cr_amt: Usd,
    /// Section D, Line 15: Recapture percentage
    pub irs_issued_registration_num: String,
}
