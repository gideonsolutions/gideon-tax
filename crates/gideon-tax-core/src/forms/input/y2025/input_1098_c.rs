use crate::Usd;

/// Input fields for IRS Form 1098-C (2025) — Contributions of Motor Vehicles,
/// Boats, and Airplanes.
///
/// Field names are **NOT** derived from the IRS MeF (Modernized e-File) schema.
/// They are Claude-authored from the paper/PDF form and use descriptive names
/// with the same suffix conventions as the rest of the crate (`_amt`, `_ind`,
/// `_dt`, `_txt`, `_nm`).
#[derive(Debug, Clone, Default)]
pub struct Input1098C {
    // -----------------------------------------------------------------------
    // Header — Donee information
    // -----------------------------------------------------------------------
    /// Donee's name
    pub donee_nm: String,
    /// Donee's street address (including apt. no.)
    pub donee_address_txt: String,
    /// Donee's city, state, and ZIP code
    pub donee_city_state_zip_txt: String,
    /// Donee's telephone number
    pub donee_phone_txt: String,
    /// DONEE'S TIN
    pub donee_tin: String,

    // -----------------------------------------------------------------------
    // Header — Donor information
    // -----------------------------------------------------------------------
    /// DONOR'S TIN
    pub donor_tin: String,
    /// Donor's name
    pub donor_nm: String,
    /// Donor's street address (including apt. no.)
    pub donor_address_txt: String,
    /// Donor's city, state, and ZIP code
    pub donor_city_state_zip_txt: String,

    // -----------------------------------------------------------------------
    // Boxes 1–3 — Contribution and vehicle information
    // -----------------------------------------------------------------------
    /// Box 1 — Date of contribution
    pub contribution_dt: String,
    /// Box 2a — Odometer mileage (motor vehicles only)
    pub odometer_mileage_txt: String,
    /// Box 2b — Year of vehicle
    pub vehicle_year_txt: String,
    /// Box 2c — Make of vehicle
    pub vehicle_make_txt: String,
    /// Box 2d — Model of vehicle
    pub vehicle_model_txt: String,
    /// Box 3 — Vehicle or other identification number
    pub vehicle_id_num: String,

    // -----------------------------------------------------------------------
    // Boxes 4a–4c — Arm's-length sale certification
    // -----------------------------------------------------------------------
    /// Box 4a — Donee certifies that vehicle was sold in arm's length
    /// transaction to unrelated party
    pub sold_arms_length_ind: bool,
    /// Box 4b — Date of sale
    pub sale_dt: String,
    /// Box 4c — Gross proceeds from sale
    pub gross_proceeds_amt: Usd,

    // -----------------------------------------------------------------------
    // Boxes 5a–5c — Material improvements / intervening use / needy transfer
    // -----------------------------------------------------------------------
    /// Box 5a — Donee certifies that vehicle will not be transferred
    /// before completion of material improvements or significant
    /// intervening use
    pub material_improvements_or_use_ind: bool,
    /// Box 5b — Donee certifies that vehicle is to be transferred to
    /// a needy individual for significantly below fair market value in
    /// furtherance of donee's charitable purpose
    pub transferred_to_needy_ind: bool,
    /// Box 5c — Donee provides the following description of material
    /// improvements or significant intervening use and duration of use
    pub improvements_or_use_desc_txt: String,

    // -----------------------------------------------------------------------
    // Boxes 6a–6c — Goods and services in exchange
    // -----------------------------------------------------------------------
    /// Box 6a — Did the organization provide goods or services in exchange
    /// for the vehicle? (true = Yes, false = No)
    pub goods_or_services_provided_ind: bool,
    /// Box 6b — Value of goods and services provided in exchange for the
    /// vehicle
    pub goods_or_services_value_amt: Usd,
    /// Box 6c — Describe the goods and services, if any, that were provided
    pub goods_or_services_desc_txt: String,

    // -----------------------------------------------------------------------
    // Box 7
    // -----------------------------------------------------------------------
    /// Box 7 — Under Internal Revenue Code section 170(f)(12)(A)(ii), the
    /// donor may not claim a deduction of more than $500 for this vehicle
    /// if this box is checked
    pub value_500_or_less_ind: bool,
}
