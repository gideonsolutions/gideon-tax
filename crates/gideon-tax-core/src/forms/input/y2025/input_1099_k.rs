use us_tax_brackets::TaxYear;

use crate::Usd;
use crate::forms::{Form, FormType, InputForm};

/// All fields for IRS Form 1099-K (2025), Payment Card and Third Party Network Transactions.
///
/// Fields are ordered by box number as they appear on the form.
/// Field names and types follow the IRS e-file schema (`irs-form-schema`).
#[derive(Debug, Clone, Default)]
pub struct Input1099K {
    /// Corrected indicator
    pub corrected_ind: bool,
    /// Calendar year
    pub calendar_yr: String,

    // =====================================================================
    // Filer/PSE identification
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
    /// Payment settlement entity (PSE) name
    pub pse_nm: String,
    /// PSE telephone number
    pub pse_phone_num: String,

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

    // =====================================================================
    // Transaction type indicators
    // =====================================================================
    /// Payment card indicator
    pub payment_card_ind: bool,
    /// Third party network indicator
    pub third_party_network_ind: bool,
    /// PSE indicator
    pub pse_ind: bool,
    /// Electronic payment facilitator (EPF) or other third party indicator
    pub epf_or_other_third_party_ind: bool,

    // =====================================================================
    // Boxes 1a–4: Amounts and transaction count
    // =====================================================================
    /// Box 1a: Gross amount of payment card/third party network transactions
    pub gross_amt: Usd,
    /// Box 1b: Card not present transactions
    pub card_not_present_trans_amt: Usd,
    /// Box 2: Merchant category code
    pub merchant_category_cd: String,
    /// Box 3: Number of payment transactions
    pub payment_transaction_cnt: String,
    /// Box 4: Federal income tax withheld
    pub federal_income_tax_withheld_amt: Usd,

    // =====================================================================
    // Box 5a–5l: Monthly amounts
    // =====================================================================
    /// Box 5a: January
    pub january_amt: Usd,
    /// Box 5b: February
    pub february_amt: Usd,
    /// Box 5c: March
    pub march_amt: Usd,
    /// Box 5d: April
    pub april_amt: Usd,
    /// Box 5e: May
    pub may_amt: Usd,
    /// Box 5f: June
    pub june_amt: Usd,
    /// Box 5g: July
    pub july_amt: Usd,
    /// Box 5h: August
    pub august_amt: Usd,
    /// Box 5i: September
    pub september_amt: Usd,
    /// Box 5j: October
    pub october_amt: Usd,
    /// Box 5k: November
    pub november_amt: Usd,
    /// Box 5l: December
    pub december_amt: Usd,

    // =====================================================================
    // State/local (Boxes 6–8)
    // =====================================================================
    /// State abbreviation code
    pub state_abbreviation_cd: String,
    /// State identification number
    pub state_id_num: String,
    /// State tax withheld
    pub state_tax_withheld_amt: Usd,
    /// Local abbreviation code text
    pub local_abbreviation_cd_txt: String,
    /// Local tax withheld
    pub local_tax_withheld_amt: Usd,
    /// Locality name
    pub locality_nm: String,
}

impl Form for Input1099K {
    fn name() -> &'static str {
        "Form 1099-K"
    }

    fn year(&self) -> TaxYear {
        TaxYear::Y2025
    }

    fn form_type() -> FormType {
        FormType::Input
    }
}

impl InputForm for Input1099K {}
