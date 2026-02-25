use crate::Usd;

/// Output fields for IRS Form 4136 (2025) — Credit for Federal Tax Paid on Fuels.
#[derive(Debug, Clone, Default)]
pub struct Output4136 {
    // -----------------------------------------------------------------------
    // Part I — Information About Your Business
    // -----------------------------------------------------------------------
    /// Part I (A): Qualifying business activity with qualifying usage of qualifying fuels
    /// eligible for the fuel tax credit indicator
    pub qlfy_usage_fuels_elig_ftc_ind: bool,
    /// Part I (B): How many different business activities qualify for this credit
    pub qlfy_business_activities_cnt: u32,
    /// Part I (C): Business name (line 1)
    pub business_name_line1_txt: String,
    /// Part I (C): Business name (line 2)
    pub business_name_line2_txt: String,
    /// Part I (D): EIN (if applicable)
    pub ein: String,
    /// Part I (E): Principal Business Activity Code
    pub principal_business_activity_cd: String,
    /// Part I (F)(i): Make of equipment
    pub make_nm: String,
    /// Part I (F)(ii): Model of equipment
    pub model_nm: String,
    /// Part I (F)(iii): Type of equipment description
    pub equipment_type_desc: String,

    // -----------------------------------------------------------------------
    // Part II — Credits
    // Line 1 — Nontaxable Use of Gasoline
    // -----------------------------------------------------------------------
    /// Line 1a: Off-highway business use — gallons
    pub off_hwy_bus_use_gasoline_gals_qty: u32,
    /// Line 1b: Use on a farm for farming purposes — gallons
    pub farming_purposes_gasoline_gals_qty: u32,
    /// Line 1a-c: Nontaxable use of gasoline — credit amount (CRN 362)
    pub nontaxable_use_of_gasoline_cr_amt: Usd,
    /// Line 1a-c: Actual fuel cost from your records
    pub actual_fuel_cost_amt: Usd,
    /// Line 1d: Exported — gallons
    pub exported_nontx_use_gas_gals_qty: u32,
    /// Line 1d: Exported nontaxable use of gasoline — credit amount (CRN 411)
    pub exported_nontx_use_of_gas_cr_amt: Usd,
    /// Line 1: Nontaxable use of fuel type code
    pub nontaxable_use_of_fuel_type_cd: String,
    /// Line 1: Gallons quantity (general)
    pub gallons_qty: u32,

    // -----------------------------------------------------------------------
    // Line 2 — Nontaxable Use of Aviation Gasoline
    // -----------------------------------------------------------------------
    /// Line 2a: Use in commercial aviation (other than foreign trade) — gallons
    pub aviation_gasoline_gallons_qty: u32,
    /// Line 2a: Aviation gasoline — credit amount (CRN 354)
    pub aviation_gasoline_credit_amt: Usd,
    /// Line 2a: Actual fuel cost for aviation nontaxable gasoline
    pub aviation_nontx_gas_actl_fl_cst_amt: Usd,
    /// Line 2a-b: Aviation nontaxable gasoline — credit amount
    pub aviation_nontx_gas_cr_amt: Usd,
    /// Line 2c: Exported — gallons
    pub exp_nontx_aviation_gas_gals_qty: u32,
    /// Line 2c: Exported nontaxable aviation gasoline — credit amount (CRN 412)
    pub exp_nontx_aviation_gas_cr_amt: Usd,
    /// Line 2d: LUST tax on aviation fuels used in foreign trade — gallons
    pub lust_tx_avn_fuel_frgn_trade_gals_qty: u32,
    /// Line 2d: LUST tax on aviation fuels used in foreign trade — credit amount (CRN 433)
    pub lust_tx_avn_fuel_frgn_trade_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Line 3 — Nontaxable Use of Undyed Diesel Fuel
    // -----------------------------------------------------------------------
    /// Line 3: Nontaxable use of undyed diesel (table data)
    pub nontaxable_use_of_undyed_diesel: String,
    /// Line 3: Undyed diesel use exception indicator
    pub undyed_diesel_use_exception_ind: bool,
    /// Line 3: Sales undyed diesel exception indicator
    pub sales_undyed_diesel_exception_ind: bool,
    /// Line 3a: Nontaxable use — business use of undyed diesel — gallons
    pub bus_use_of_undyed_diesel_gals_qty: u32,
    /// Line 3a: Business use of undyed diesel — credit amount
    pub bus_use_of_undyed_diesel_credit_amt: Usd,
    /// Line 3b: Use on a farm for farming purposes — undyed diesel — gallons
    pub farm_prps_undyed_dsl_fuel_gals_qty: u32,
    /// Line 3b: Farm purposes undyed diesel fuel — credit amount (CRN 360)
    pub farm_prps_undyed_dsl_fuel_cr_amt: Usd,
    /// Line 3c: Use in trains — undyed diesel — gallons
    pub train_use_of_undyed_diesel_gals_qty: u32,
    /// Line 3c: Train use of undyed diesel — credit amount (CRN 353)
    pub train_use_of_undyed_diesel_cr_amt: Usd,
    /// Line 3e: Exported undyed diesel fuel — gallons
    pub exp_undyed_diesel_fuel_gals_qty: u32,
    /// Line 3e: Exported undyed diesel fuel — credit amount (CRN 413)
    pub exp_undyed_diesel_fuel_credit_amt: Usd,
    /// Line 3: Undyed diesel registration number
    pub undyed_diesel_registration_num: String,

    // -----------------------------------------------------------------------
    // Line 4 — Nontaxable Use of Undyed Kerosene (Other Than Kerosene
    //           Used in Aviation)
    // -----------------------------------------------------------------------
    /// Line 4: Undyed kerosene use exception indicator
    pub undyed_kerosene_use_exception_ind: bool,
    /// Line 4: Sales undyed kerosene exception indicator
    pub sls_undyed_kerosene_exception_ind: bool,
    /// Line 4a: Nontaxable use taxed at $.244 — kerosene — credit amount (CRN 346)
    pub nontx_use_undyed_krsn_txd044_cr_amt: Usd,
    /// Line 4a: Actual fuel cost for nontaxable undyed kerosene taxed at $.044
    pub nontx_undyed_krsn044_act_fl_cst_amt: Usd,
    /// Line 4a: Actual fuel cost for nontaxable undyed kerosene taxed at $.044 (alternate)
    pub nontx_undyed_krsn044_actl_fl_cst_amt: Usd,
    /// Line 4b: Use on a farm for farming purposes — undyed kerosene — gallons
    pub farm_prps_undyed_kerosene_gals_qty: u32,
    /// Line 4b: Farm purposes undyed kerosene — credit amount
    pub farm_prps_undyed_kerosene_cr_amt: Usd,
    /// Line 4d: Exported — kerosene — gallons
    pub exported_undyed_kerosene_gals_qty: u32,
    /// Line 4d: Exported undyed kerosene — credit amount (CRN 414)
    pub exported_undyed_kerosene_cr_amt: Usd,
    /// Line 4e: Nontaxable use taxed at $.044 — credit amount (CRN 377)
    pub kerosene_tax_rate_cd: String,
    /// Line 4f: Nontaxable use taxed at $.219 — credit amount (CRN 369)
    pub nontx_use_undyed_krsn_txd219: String,
    /// Line 4f: Nontaxable use undyed kerosene taxed at $.219 — credit amount
    pub nontx_use_undyed_krsn_txd219_cr_amt: Usd,
    /// Line 4f: Actual fuel cost for nontaxable undyed kerosene taxed at $.219
    pub nontx_undyed_krsn219_actl_fl_cst_amt: Usd,
    /// Line 4: Undyed kerosene registration number
    pub undyed_kerosene_registration_num: String,

    // -----------------------------------------------------------------------
    // Line 5 — Kerosene Used in Aviation
    // -----------------------------------------------------------------------
    /// Line 5a: Kerosene used in commercial aviation (other than foreign trade) taxed at
    /// $.244 — gallons
    pub kerosene_used_in_avn_txd244_gals_qty: u32,
    /// Line 5a: Kerosene used in aviation taxed at $.244 — credit amount (CRN 417)
    pub kerosene_used_in_avn_txd244_cr_amt: Usd,
    /// Line 5a: Actual fuel cost for kerosene used in aviation taxed at $.244
    pub nontx_krsn_avn_txd244_actl_fl_cst_amt: Usd,
    /// Line 5b: Kerosene used in commercial aviation (other than foreign trade) taxed at
    /// $.219 — gallons
    pub kerosene_used_in_avn_txd219_gals_qty: u32,
    /// Line 5b: Kerosene used in aviation taxed at $.219 — credit amount (CRN 355)
    pub kerosene_used_in_avn_txd219_cr_amt: Usd,
    /// Line 5b: Actual fuel cost for kerosene used in aviation taxed at $.219
    pub nontx_krsn_avn_txd219_actl_fl_cst_amt: Usd,
    /// Line 5c: Nontaxable use (other than use by state or local government) taxed at $.244
    /// — credit amount (CRN 346)
    pub non_tx_krsn_used_in_avn_txd244_cr_amt: Usd,
    /// Line 5c: Actual fuel cost for kerosene other nontaxable use taxed at $.244
    pub krsn_oth_nontx_txd244_actl_fl_cst_amt: Usd,
    /// Line 5d: Nontaxable use (other than use by state or local government) taxed at $.219
    /// — credit amount (CRN 369)
    pub non_tx_krsn_used_in_avn_txd219_cr_amt: Usd,
    /// Line 5d: Actual fuel cost for kerosene other nontaxable use taxed at $.219
    pub krsn_oth_nontx_txd219_actl_fl_cst_amt: Usd,
    /// Line 5e: LUST tax on aviation fuels used in foreign trade — gallons
    pub lust_tx_krsn_avn_frgn_trd_gals_qty: u32,
    /// Line 5e: LUST tax on kerosene for aviation in foreign trade — credit amount (CRN 433)
    pub lust_tx_krsn_avn_frgn_trd_cr_amt: Usd,
    /// Line 5: Kerosene for aviation registration number
    pub kerosene_for_avn_registration_num: String,

    // -----------------------------------------------------------------------
    // Line 6 — Sales by Registered Ultimate Vendors of Undyed Diesel Fuel
    // -----------------------------------------------------------------------
    /// Line 6a: Use by a state or local government — undyed diesel — gallons
    pub dsl_fuel_sold_st_local_govt_gals_qty: u32,
    /// Line 6a: Diesel fuel sold to state or local government — credit amount (CRN 360)
    pub dsl_fuel_sold_st_local_govt_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Line 7 — Sales by Registered Ultimate Vendors of Undyed Kerosene
    //           (Other Than Kerosene for Use in Aviation)
    // -----------------------------------------------------------------------
    /// Line 7a: Use by a state or local government — kerosene — gallons
    pub krsn_fuel_sold_st_local_govt_gals_qty: u32,
    /// Line 7a: Kerosene fuel sold to state or local government — credit amount (CRN 346)
    pub krsn_fuel_sold_st_local_govt_cr_amt: Usd,
    /// Line 7b: Sales from a blocked pump — kerosene — gallons
    pub sls_undyed_krsn_block_pump_gals_qty: u32,
    /// Line 7b: Sales undyed kerosene from blocked pump — credit amount
    pub sls_undyed_krsn_block_pump_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Line 8 — Sales by Registered Ultimate Vendors of Kerosene for Use
    //           in Aviation
    // -----------------------------------------------------------------------
    /// Line 8a: Use in commercial aviation (other than foreign trade) taxed at $.219 —
    /// gallons
    pub sls_krsn_used_in_avn_txd219_gals_qty: u32,
    /// Line 8a: Sales kerosene used in aviation taxed at $.219 — credit amount (CRN 355)
    pub sls_krsn_used_in_avn_txd219_cr_amt: Usd,
    /// Line 8b: Use in commercial aviation (other than foreign trade) taxed at $.244 —
    /// gallons
    pub sls_krsn_used_in_avn_txd244_gals_qty: u32,
    /// Line 8b: Sales kerosene used in aviation taxed at $.244 — credit amount (CRN 417)
    pub sls_krsn_used_in_avn_txd244_cr_amt: Usd,
    /// Line 8c: Nonexempt use in noncommercial aviation — gallons
    pub sls_krsn_nnxmpt_use_in_avn_gals_qty: u32,
    /// Line 8c: Sales kerosene nonexempt use in aviation — credit amount (CRN 418)
    pub sls_krsn_nnxmpt_use_in_avn_cr_amt: Usd,
    /// Line 8d: Other nontaxable uses taxed at $.244 — credit amount (CRN 346)
    pub sls_krsn_oth_nontx_txd244_cr_amt: Usd,
    /// Line 8e: Other nontaxable uses taxed at $.219 — credit amount (CRN 369)
    pub sls_krsn_oth_nontx_txd219_cr_amt: Usd,
    /// Line 8f: LUST tax on sales kerosene for aviation in foreign trade — gallons
    pub lust_tx_sls_krsn_avn_frgn_trd_gals_qty: u32,
    /// Line 8f: LUST tax on sales kerosene for aviation in foreign trade — credit amount
    /// (CRN 433)
    pub lust_tx_sls_krsn_avn_frgn_trd_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Line 9-10 — Reserved for future use
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Line 11 — Nontaxable Use of Alternative Fuel
    // -----------------------------------------------------------------------
    /// Line 11a: Liquefied petroleum gas (LPG) — nontaxable use data
    pub bus_nontx_liquified_petroleum_gas: String,
    /// Line 11a: Nontaxable liquefied petroleum gas — credit amount (CRN 419)
    pub nontx_liquefied_ptrlm_gas_cr_amt: Usd,
    /// Line 11a: Actual fuel cost for nontaxable liquefied petroleum gas
    pub nontx_liqfd_ptrlm_actl_fl_cst_amt: Usd,
    /// Line 11b: "P Series" fuels — nontaxable use data
    pub bus_nontx_p_series_fuels: String,
    /// Line 11b: Nontaxable P Series fuels — credit amount (CRN 420)
    pub nontx_p_series_fuels_credit_amt: Usd,
    /// Line 11b: Actual fuel cost for nontaxable P Series fuels
    pub nontx_p_series_fuels_actl_fl_cst_amt: Usd,
    /// Line 11c: Compressed natural gas (CNG) — nontaxable use data
    pub bus_nontx_compressed_natural_gas: String,
    /// Line 11c: Nontaxable compressed natural gas — credit amount (CRN 421)
    pub nontx_compressed_natural_gas_cr_amt: Usd,
    /// Line 11c: Actual fuel cost for nontaxable compressed natural gas
    pub nontx_cmprsd_nat_gas_actl_fl_cst_amt: Usd,
    /// Line 11d: Liquefied hydrogen — nontaxable use data
    pub bus_nontx_liquified_hydrogen: String,
    /// Line 11d: Nontaxable liquefied hydrogen — credit amount (CRN 422)
    pub nontx_liquefied_hydrogen_cr_amt: Usd,
    /// Line 11d: Actual fuel cost for nontaxable liquefied hydrogen
    pub nontx_liqfd_hydrogen_actl_fl_cst_amt: Usd,
    /// Line 11e: Fischer-Tropsch process liquid fuel from coal (including peat) — nontaxable
    /// use data
    pub bus_nontx_liqfd_fuel_der_from_coal: String,
    /// Line 11e: Nontaxable liquefied fuel derived from coal — credit amount (CRN 423)
    pub nontx_liqfd_fuel_der_from_coal_cr_amt: Usd,
    /// Line 11e: Actual fuel cost for nontaxable liquefied fuel from coal
    pub nontx_liqfd_fuel_coal_actl_fl_cst_amt: Usd,
    /// Line 11f: Liquid fuel derived from biomass — nontaxable use data
    pub bus_nontx_liq_fuel_der_from_biomass: String,
    /// Line 11f: Nontaxable liquid fuel derived from biomass — credit amount (CRN 424)
    pub nontx_liq_fuel_der_biomass_cr_amt: Usd,
    /// Line 11f: Actual fuel cost for nontaxable liquid fuel from biomass
    pub nontx_liq_fuel_bmss_actl_fl_cst_amt: Usd,
    /// Line 11g: Liquefied natural gas (LNG) — nontaxable use data
    pub bus_nontx_liquefied_natural_gas: String,
    /// Line 11g: Nontaxable liquefied natural gas — credit amount (CRN 425)
    pub nontx_liquefied_natural_gas_cr_amt: Usd,
    /// Line 11g: Actual fuel cost for nontaxable liquefied natural gas
    pub nontx_liqfd_nat_gas_actl_fl_cst_amt: Usd,
    /// Line 11h: Liquefied gas derived from biomass — nontaxable use data
    pub bus_nontx_liquefied_gas_der_biomass: String,
    /// Line 11h: Nontaxable liquefied gas derived from biomass — credit amount (CRN 435)
    pub nontx_liquefied_gas_biomass_cr_amt: Usd,
    /// Line 11h: Actual fuel cost for nontaxable liquefied gas from biomass
    pub nontx_liqfd_gas_bmss_actl_fl_cst_amt: Usd,

    // -----------------------------------------------------------------------
    // Line 12 — Reserved for future use
    // -----------------------------------------------------------------------

    // -----------------------------------------------------------------------
    // Line 13 — Registered Credit Card Issuers
    // -----------------------------------------------------------------------
    /// Line 13: Credit card issuer registration number
    pub credit_card_issr_registration_num: String,
    /// Line 13a: Diesel fuel sold for the exclusive use of a state or local government —
    /// gallons
    pub sls_undyed_dsl_st_lcl_govt_gals_qty: u32,
    /// Line 13a: Sales undyed diesel to state or local government — credit amount (CRN 360)
    pub sls_undyed_dsl_use_st_lcl_govt_cr_amt: Usd,
    /// Line 13a: Sales undyed diesel use in bus — gallons
    pub sls_undyed_diesel_use_bus_gals_qty: u32,
    /// Line 13a: Sales undyed diesel use in bus — credit amount
    pub sls_undyed_diesel_use_bus_cr_amt: Usd,
    /// Line 13b: Kerosene sold for the exclusive use of a state or local government —
    /// gallons
    pub sls_undyed_krsn_st_lcl_govt_gals_qty: u32,
    /// Line 13b: Sales undyed kerosene use in bus — gallons
    pub sls_undyed_krsn_use_bus_gals_qty: u32,
    /// Line 13b: Sales undyed kerosene use in bus — credit amount
    pub sls_undyed_krsn_use_bus_cr_amt: Usd,
    /// Line 13c: Kerosene for use in aviation sold for the exclusive use of a state or local
    /// government — gallons
    pub krsn_avn_sold_st_local_govt_gals_qty: u32,
    /// Line 13c: Kerosene for aviation sold to state or local government — credit amount
    pub krsn_avn_sold_st_local_govt_cr_amt: Usd,
    /// Nontaxable use of fuels credit card users statement
    pub nontaxable_use_fuels_credit_card_users_statement: String,

    // -----------------------------------------------------------------------
    // Line 14 — Nontaxable Use of a Diesel-Water Fuel Emulsion
    // -----------------------------------------------------------------------
    /// Line 14a: Nontaxable use — diesel-water fuel emulsion — credit amount (CRN 309)
    pub nontx_use_diesel_wtr_emulsion_cr_amt: Usd,
    /// Line 14a: Actual fuel cost for nontaxable diesel-water emulsion
    pub nontx_dsl_wtr_emlsn_actl_fl_cst_amt: Usd,
    /// Line 14b: Exported nontaxable use diesel-water emulsion — gallons
    pub exp_nontx_use_dsl_wtr_emulsion_qty: u32,
    /// Line 14b: Exported nontaxable use diesel-water emulsion — credit amount (CRN 306)
    pub exp_nontx_use_dsl_wtr_emulsion_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Line 15 — Diesel-Water Fuel Emulsion Blending
    // -----------------------------------------------------------------------
    /// Line 15: Diesel-water fuel emulsion blending registration number
    pub diesel_wtr_blndg_registration_num: String,
    /// Line 15a: Blender credit — diesel-water emulsion — gallons
    pub blndr_cr_use_dsl_wtr_emulsion_qty: u32,
    /// Line 15a: Blender credit — diesel-water emulsion — credit amount (CRN 310)
    pub blndr_cr_use_dsl_wtr_emulsion_cr_amt: Usd,
    /// Line 15: Diesel-water fuel emulsion blending statement
    pub diesel_water_fuel_emulsion_blending_statement: String,

    // -----------------------------------------------------------------------
    // Line 16 — Exported Dyed Fuels and Exported Gasoline Blendstocks
    // -----------------------------------------------------------------------
    /// Line 16a: Exported dyed diesel fuel and exported gasoline blendstocks — gallons
    pub exported_dyed_diesel_fuel_gals_qty: u32,
    /// Line 16a: Exported dyed diesel fuel — credit amount (CRN 415)
    pub exported_dyed_diesel_fuel_cr_amt: Usd,
    /// Line 16b: Exported dyed kerosene — gallons
    pub exported_dyed_kerosene_gallons_qty: u32,
    /// Line 16b: Exported dyed kerosene — credit amount (CRN 416)
    pub exported_dyed_kerosene_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Line 17 — Total
    // -----------------------------------------------------------------------
    /// Line 17: Total income tax credit claimed. Add lines 1 through 16, column (e)
    pub total_fuel_tax_credit_amt: Usd,

    // -----------------------------------------------------------------------
    // Additional breakdown fields
    // -----------------------------------------------------------------------
    /// Line 4d/3e: Exported undyed kerosene — credit amount
    pub exp_undyed_diesel_fuel_cr_amt: Usd,
    /// Line 3a/4a: Bus use of undyed kerosene — gallons
    pub bus_use_of_undyed_kerosene_gals_qty: u32,
    /// Line 3a/4a: Bus use of undyed kerosene — credit amount
    pub bus_use_of_undyed_kerosene_cr_amt: Usd,
}
