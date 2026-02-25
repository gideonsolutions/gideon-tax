use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form 1099-MISC (2025), Miscellaneous Information.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Input1099Misc {
    /// Corrected indicator
    pub corrected_ind: bool,
    /// Calendar year
    pub calendar_yr: String,

    // =====================================================================
    // Payer identification
    // =====================================================================
    /// Payer's name — business name line 1
    pub payer_name_line_1_txt: String,
    /// Payer's name — business name line 2
    pub payer_name_line_2_txt: String,
    /// Payer name control text
    pub payer_name_control_txt: String,
    /// Payer's EIN
    pub payer_ein: String,
    /// Payer's US address
    pub payer_us_address: String,
    /// Payer's foreign address
    pub payer_foreign_address: String,
    /// Payer's telephone number
    pub phone_num: String,
    /// Foreign phone number
    pub foreign_phone_num: String,

    // =====================================================================
    // Recipient identification
    // =====================================================================
    /// Recipient's SSN
    pub recipient_ssn: String,
    /// Recipient's EIN
    pub recipient_ein: String,
    /// Recipient's name
    pub recipient_nm: String,
    /// Recipient's US address line 1
    pub recipient_us_address_line_1_txt: String,
    /// Recipient's US address line 2
    pub recipient_us_address_line_2_txt: String,
    /// Recipient's foreign address line 1
    pub recipient_foreign_address_line_1_txt: String,
    /// Recipient's foreign address line 2
    pub recipient_foreign_address_line_2_txt: String,
    /// Recipient's foreign country code
    pub recipient_foreign_country_cd: String,
    /// Recipient's account number
    pub recipient_account_num: String,
    /// FATCA filing requirement indicator
    pub fatca_filing_requirement_ind: bool,

    // =====================================================================
    // Boxes 1–14: Miscellaneous income
    // =====================================================================
    /// Box 1: Rents
    pub rent_amt: Usd,
    /// Box 2: Royalties
    pub royalty_amt: Usd,
    /// Box 3: Other income
    pub other_income_amt: Usd,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: Fishing boat proceeds
    pub fishing_boat_proceeds_amt: Usd,
    /// Box 6: Medical and health care payments
    pub medical_health_care_payments_amt: Usd,
    /// Box 7: Direct sales above threshold indicator (Payer made direct sales totaling $5,000 or more)
    pub direct_sale_above_threshold_ind: bool,
    /// Box 8: Substitute payments in lieu of dividends or interest
    pub substitute_payments_amt: Usd,
    /// Box 9: Crop insurance proceeds
    pub crop_insurance_proceeds_amt: Usd,
    /// Box 10: Gross proceeds paid to an attorney
    pub attorney_gross_proceeds_paid_amt: Usd,
    /// Box 11: Fish purchased for resale
    pub fish_purchased_for_resale_amt: Usd,
    /// Box 12: Section 409A deferrals
    pub section_409a_deferrals_amt: Usd,
    /// Box 14: Nonqualified deferred compensation
    pub nonqlfy_deferred_compensation_amt: Usd,

    // =====================================================================
    // State/local (Boxes 15–18)
    // =====================================================================
    /// State abbreviation code
    pub state_abbreviation_cd: String,
    /// State identification number
    pub state_id_num: String,
    /// State income amount
    pub state_income_amt: Usd,
    /// State tax withheld
    pub state_tax_withheld_amt: Usd,
    /// Local income amount
    pub local_income_amt: Usd,
    /// Local tax withheld
    pub local_tax_withheld_amt: Usd,
    /// Locality name
    pub locality_nm: String,
}

impl Form for Input1099Misc {
    fn name() -> &'static str {
        "Form 1099-MISC"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1099Misc {}
