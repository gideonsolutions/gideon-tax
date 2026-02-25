use crate::Usd;

/// Input fields for IRS Form 1098-E (2025) — Student Loan Interest Statement.
///
/// Field names are **NOT** derived from the IRS MeF (Modernized e-File) schema.
/// They are Claude-authored from the paper/PDF form and use descriptive names
/// with the same suffix conventions as the rest of the crate (`_amt`, `_ind`,
/// `_dt`, `_txt`, `_nm`).
#[derive(Debug, Clone, Default)]
pub struct Input1098E {
    // -----------------------------------------------------------------------
    // Header — Recipient / Lender information
    // -----------------------------------------------------------------------
    /// Recipient/lender's name
    pub recipient_nm: String,
    /// Recipient/lender's street address (including apt. no.)
    pub recipient_address_txt: String,
    /// Recipient/lender's city, state, and ZIP code
    pub recipient_city_state_zip_txt: String,
    /// Recipient/lender's telephone number
    pub recipient_phone_txt: String,
    /// RECIPIENT'S/LENDER'S TIN
    pub recipient_tin: String,

    // -----------------------------------------------------------------------
    // Header — Borrower information
    // -----------------------------------------------------------------------
    /// BORROWER'S TIN
    pub borrower_tin: String,
    /// Borrower's name
    pub borrower_nm: String,
    /// Borrower's street address (including apt. no.)
    pub borrower_address_txt: String,
    /// Borrower's city, state, and ZIP code
    pub borrower_city_state_zip_txt: String,
    /// Account number (optional)
    pub account_num: String,

    // -----------------------------------------------------------------------
    // Boxes
    // -----------------------------------------------------------------------
    /// Box 1 — Student loan interest received by lender
    pub student_loan_interest_amt: Usd,
    /// Box 2 — If checked, Box 1 does not include loan origination fees
    /// and/or capitalized interest for loans made before September 1, 2004
    pub origination_fees_not_included_ind: bool,
}
