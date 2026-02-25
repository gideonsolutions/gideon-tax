use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form W-2G (2025), Certain Gambling Winnings.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct InputW2G {
    /// Corrected W-2G indicator
    pub corrected_w2g_ind: bool,
    /// Void indicator
    pub void_ind: bool,
    /// Calendar year
    pub calendar_yr: String,
    /// Standard or non-standard code
    pub standard_or_non_standard_cd: String,

    // =====================================================================
    // Payer identification
    // =====================================================================
    /// Payer's name — business name line 1
    pub payer_name_line_1_txt: String,
    /// Payer's name — business name line 2
    pub payer_name_line_2_txt: String,
    /// Payer name control text
    pub payer_name_control_txt: String,
    /// Payer's federal identification number (EIN)
    pub payer_ein: String,
    /// Payer's SSN (if individual)
    pub payer_ssn: String,
    /// Payer's US address line 1
    pub payer_us_address_line_1_txt: String,
    /// Payer's US address line 2
    pub payer_us_address_line_2_txt: String,
    /// Payer's foreign address line 1
    pub payer_foreign_address_line_1_txt: String,
    /// Payer's foreign address line 2
    pub payer_foreign_address_line_2_txt: String,
    /// Payer's foreign address country code
    pub payer_foreign_country_cd: String,
    /// Payer's telephone number
    pub payer_telephone_num: String,

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
    /// Recipient's foreign city
    pub recipient_foreign_city_nm: String,
    /// Recipient's foreign province or state
    pub recipient_foreign_province_or_state_nm: String,
    /// Recipient's foreign country code
    pub recipient_foreign_country_cd: String,
    /// Recipient's foreign postal code
    pub recipient_foreign_postal_cd: String,
    /// Recipient's first additional identification number
    pub recipient_first_additional_id_num: String,
    /// Recipient's second additional identification number
    pub recipient_second_additional_id_num: String,

    // =====================================================================
    // Boxes 1–7: Winnings and withholding
    // =====================================================================
    /// Box 1: Reportable winnings
    pub gambling_reportable_winning_amt: Usd,
    /// Box 2: Date won
    pub gambling_winning_dt: String,
    /// Box 3: Type of wager
    pub gambling_win_wager_type_desc: String,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,
    /// Box 5: Transaction
    pub gambling_winning_transaction_desc: String,
    /// Box 6: Race/event
    pub gambling_winning_event_desc: String,
    /// Box 7: Winnings from identical wagers
    pub gambling_win_from_idntcl_wagers_amt: Usd,
    /// Cashier ID
    pub gambling_win_cashier_id: String,
    /// Window number
    pub gambling_win_window_num: String,

    // =====================================================================
    // State/local (Boxes 14–17)
    // =====================================================================
    /// State abbreviation code
    pub state_abbreviation_cd: String,
    /// Payer's state identification number
    pub payer_state_id_num: String,
    /// State winnings amount
    pub state_gambling_winning_amt: Usd,
    /// State income tax withheld
    pub state_tax_withheld_amt: Usd,
    /// Local winnings amount
    pub local_gambling_winning_amt: Usd,
    /// Local income tax withheld
    pub local_income_tax_amt: Usd,
    /// Locality name
    pub locality_nm: String,
}

impl Form for InputW2G {
    fn name() -> &'static str {
        "Form W-2G"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for InputW2G {}
