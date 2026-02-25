use crate::Usd;

/// Input fields for IRS Form 1098 (2025) — Mortgage Interest Statement.
///
/// Field names are **NOT** derived from the IRS MeF (Modernized e-File) schema.
/// They are Claude-authored from the paper/PDF form and use descriptive names
/// with the same suffix conventions as the rest of the crate (`_amt`, `_ind`,
/// `_dt`, `_txt`, `_nm`).
#[derive(Debug, Clone, Default)]
pub struct Input1098 {
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
    // Header — Payer / Borrower information
    // -----------------------------------------------------------------------
    /// PAYER'S/BORROWER'S TIN
    pub payer_tin: String,
    /// Payer's/borrower's name
    pub payer_nm: String,
    /// Payer's/borrower's street address (including apt. no.)
    pub payer_address_txt: String,
    /// Payer's/borrower's city, state, and ZIP code
    pub payer_city_state_zip_txt: String,
    /// Account number (optional)
    pub account_num: String,

    // -----------------------------------------------------------------------
    // Boxes
    // -----------------------------------------------------------------------
    /// Box 1 — Mortgage interest received from payer(s)/borrower(s)
    pub mortgage_interest_amt: Usd,
    /// Box 2 — Outstanding mortgage principal
    pub outstanding_mortgage_principal_amt: Usd,
    /// Box 3 — Mortgage origination date
    pub mortgage_origination_dt: String,
    /// Box 4 — Refund of overpaid interest
    pub refund_of_overpaid_interest_amt: Usd,
    /// Box 5 — Mortgage insurance premiums
    pub mortgage_insurance_premiums_amt: Usd,
    /// Box 6 — Points paid on purchase of principal residence
    pub points_paid_on_purchase_amt: Usd,
    /// Box 7 — If address of property securing mortgage is the same
    /// as PAYER'S/BORROWER'S address, check this box, or enter the
    /// address or description in Box 8
    pub property_address_same_as_payer_ind: bool,
    /// Box 8 — Address or description of property securing mortgage
    /// (including city, state, and ZIP code)
    pub property_address_txt: String,
    /// Box 9 — Number of mortgaged properties
    pub num_mortgaged_properties_cnt: u32,
    /// Box 10 — Other (e.g., real estate taxes, insurance, etc.)
    pub other_txt: String,
    /// Box 11 — Mortgage acquisition date
    pub mortgage_acquisition_dt: String,
}
