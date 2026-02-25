use crate::Usd;

/// Output fields for IRS Form 5695 (2025) — Residential Energy Credits.
#[derive(Debug, Clone, Default)]
pub struct Output5695 {
    // -----------------------------------------------------------------------
    // Top-of-form
    // -----------------------------------------------------------------------
    /// Name(s) shown on return
    pub name_line1_txt: String,
    /// Your social security number
    pub ssn: String,

    // -----------------------------------------------------------------------
    // Part I — Residential Clean Energy Credit
    // -----------------------------------------------------------------------
    /// Part I address: Complete address of the home where you installed the property
    pub residential_clean_egy_home_address: String,
    /// Line 1: Qualified solar electric property costs
    pub solar_elec_prop_cost_amt: Usd,
    /// Line 2: Qualified solar water heating property costs
    pub solar_water_ht_prop_cost_amt: Usd,
    /// Line 3: Qualified small wind energy property costs
    pub small_wind_prop_cost_amt: Usd,
    /// Line 4: Qualified geothermal heat pump property costs
    pub geothrml_ht_pump_prop_cost_amt: Usd,
    /// Line 5a: Qualified battery storage technology — does capacity meet at least 3 kilowatt hours?
    pub qlfy_battery_storage_tech_ind: bool,
    /// Line 5b: If you checked the "Yes" box, enter the qualified battery technology costs
    pub qlfy_battery_storage_tech_costs_amt: Usd,
    /// Line 6a: Add lines 1 through 5b
    pub total_energy_credits_amt: Usd,
    /// Line 6b: Multiply line 6a by 30% (0.30)
    pub total_energy_credits_std_pct_cr_amt: Usd,
    /// Line 7a: Qualified fuel cell property — was it installed on or in connection with your main
    /// home located in the United States?
    pub qlfy_fuel_cell_property_in_us_ind: bool,
    /// Line 7b: Address of the main home where you installed the fuel cell property
    pub qlfy_fuel_cell_property_hm_address: String,
    /// Line 7c: If the special rule for joint occupants applies, check here
    pub joint_occupancy_ind: bool,
    /// Line 8: Qualified fuel cell property costs
    pub fuel_cell_prop_cost_amt: Usd,
    /// Line 9: Multiply line 8 by 30% (0.30)
    pub fuel_cell_prop_std_pct_cr_amt: Usd,
    /// Line 10: Kilowatt capacity of property on line 8 above. If less than 0.5 kW, enter -0-
    pub fuel_cell_prop_kw_cap_num: String,
    /// Line 10: Kilowatt capacity amount (number x $1,000)
    pub fuel_cell_prop_kw_cap_amt: Usd,
    /// Line 11: Enter the smaller of line 9 or line 10
    pub fuel_cell_prop_allwbl_cost_amt: Usd,
    /// Line 12: Credit carryforward from 2024
    pub py_cfwd_rsdntl_clean_energy_cr_amt: Usd,
    /// Line 13: Add lines 6b, 11, and 12
    pub total_of_energy_credits_amt: Usd,
    /// Line 14: Limitation based on tax liability from the Residential Clean Energy Credit Limit Worksheet
    pub adjusted_credit_limit_amt: Usd,
    /// Line 14: Adjusted credit limit code
    pub adjusted_credit_limit_cd: String,
    /// Line 15: Residential clean energy credit. Enter the smaller of line 13 or line 14.
    /// Also include this amount on Schedule 3 (Form 1040), line 5a
    pub residential_clean_energy_cr_amt: Usd,
    /// Line 16: Credit carryforward to 2026. If line 15 is less than line 13, subtract line 15
    /// from line 13
    pub cfwd_rsdntl_clean_energy_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Energy Efficient Home Improvement Credit
    // Section A — Qualified Energy Efficiency Improvements
    // -----------------------------------------------------------------------
    /// Line 17a: Are the qualified energy efficiency improvements installed in or on your main
    /// home located in the United States?
    pub qlfy_energy_prop_costs_us_home_ind: bool,
    /// Line 17b: Are you the original user of the qualified energy efficiency improvements?
    pub original_user_ind: bool,
    /// Line 17c: Are the components reasonably expected to remain in use for at least 5 years?
    pub five_year_use_expectation_ind: bool,
    /// Line 17d: Address of the main home where you made the qualifying improvements
    pub home_address: String,
    /// Line 17e: Were any of these improvements related to the construction of this main home?
    pub imprv_rltd_to_const_main_home_ind: bool,
    /// Line 18a: Insulation or air sealing material or system — cost
    pub insulation_or_sys_ht_gn_loss_cost_amt: Usd,
    /// Line 18b: Multiply line 18a by 30% (0.30). Do not enter more than $1,200
    pub insulation_or_sys_ht_std_pct_cr_amt: Usd,
    /// Line 19a: Enter the cost of the most expensive exterior door you bought
    pub most_expns_extr_door_cost_amt: Usd,
    /// Line 19b: Qualified Manufacturer Identification Number of the most expensive door
    pub most_expns_extr_door_qmid: String,
    /// Line 19c: Multiply line 19a by 30% (0.30). Do not enter more than $250
    pub most_expns_extr_doors_std_pct_cr_amt: Usd,
    /// Line 19d: Enter the Qualified Manufacturer Identification Number(s) and cost(s) of the
    /// two next most expensive door(s)
    pub next_most_expns_extr_door_cost_amt: Usd,
    /// Line 19d: Cost amount
    pub cost_amt: Usd,
    /// Line 19e: Enter the cost of all other qualifying exterior doors. If none, enter -0-
    pub other_qlfy_extr_doors_cost_amt: Usd,
    /// Line 19f: Add lines 19d and 19e
    pub total_other_qlfy_extr_doors_cost_amt: Usd,
    /// Line 19g: Multiply line 19f by 30% (0.30)
    pub other_qlfy_extr_doors_std_pct_cr_amt: Usd,
    /// Line 19h: Add lines 19c and 19g. Do not enter more than $500
    pub total_extr_doors_credit_amt: Usd,
    /// Line 20a: Enter the Qualified Manufacturer Identification Number(s) and cost(s) of the
    /// four most expensive window(s)/skylight(s)
    pub most_expns_extr_wndw_skylt_cost_amt: Usd,
    /// Line 20b: Enter the cost of all other exterior windows and skylights. If none, enter -0-
    pub oth_qlfy_extr_wndw_skylt_cost_amt: Usd,
    /// Line 20c: Add lines 20a and 20b
    pub exterior_wndw_or_skylight_cost_amt: Usd,
    /// Line 20d: Multiply line 20c by 30% (0.30). Do not enter more than $600
    pub extr_wndw_skylight_std_pct_cr_amt: Usd,

    // -----------------------------------------------------------------------
    // Part II — Energy Efficient Home Improvement Credit
    // Section B — Residential Energy Property Expenditures
    // -----------------------------------------------------------------------
    /// Line 21a: Did you incur costs for qualified energy property installed on or in connection
    /// with a home located in the United States?
    pub home_located_in_usa_ind: bool,
    /// Line 21b: Was the qualified energy property originally placed into service by you?
    pub originally_placed_in_service_ind: bool,
    /// Line 21c: Address of each home where you installed qualified energy property
    pub qualified_energy_property_address: String,
    /// Line 22a: Most expensive central air conditioner — cost
    pub most_expns_central_air_cond_cost_amt: Usd,
    /// Line 22a: Most expensive central air conditioner — QMID
    pub most_expns_central_air_cond_qmid: String,
    /// Line 22b: Enter the cost of all other central air conditioners. If none, enter -0-
    pub oth_central_air_cond_cost_amt: Usd,
    /// Line 22c: Add lines 22a and 22b
    pub central_air_cond_cost_amt: Usd,
    /// Line 22d: Multiply line 22c by 30% (0.30). Do not enter more than $600
    pub central_air_cond_cost_std_pct_cr_amt: Usd,
    /// Line 23a: Cost(s) of the two most expensive natural gas, propane, or oil water heater(s)
    pub most_expns_water_ht_cost_amt: Usd,
    /// Line 23b: Enter the cost of all other natural gas, propane, or oil water heaters.
    /// If none, enter -0-
    pub oth_nat_gas_prpn_oil_wtr_htr_cost_amt: Usd,
    /// Line 23c: Add lines 23a and 23b
    pub nat_gas_prpn_oil_wtr_htr_cost_amt: Usd,
    /// Line 23d: Multiply line 23c by 30% (0.30). Do not enter more than $600
    pub nat_gas_prpn_oil_wtr_htr_std_pct_cr_amt: Usd,
    /// Line 24a: Most expensive natural gas, propane, or oil furnace or hot water boiler — cost
    pub most_expns_frnc_hot_wtr_blr_cost_amt: Usd,
    /// Line 24a: Most expensive furnace or hot water boiler — QMID
    pub most_expns_frnc_hot_wtr_blr_qmid: String,
    /// Line 24b: Enter the cost of all other natural gas, propane, or oil furnace or hot water
    /// boilers. If none, enter -0-
    pub oth_frnc_hot_wtr_blr_cost_amt: Usd,
    /// Line 24c: Add lines 24a and 24b
    pub nat_gas_prpn_oil_hot_wtr_blr_cost_amt: Usd,
    /// Line 24d: Multiply line 24c by 30% (0.30). Do not enter more than $600
    pub nat_gas_prpn_oil_hot_wtr_blr_pct_amt: Usd,
    /// Line 25a: Did you install improvements or replacements of panelboards, subpanelboards,
    /// branch circuits, or feeders (enabling property)? Enter the enabled property type code
    pub enabled_property_type_cd: String,
    /// Line 25d: Qualified Manufacturer Identification Number of the enabling property
    pub qmid: String,
    /// Line 25c: Enter the cost of improvements or replacement of enabling property
    pub panelboard_ckt_feeder_cost_amt: Usd,
    /// Line 25e: Multiply line 25c by 30% (0.30). Do not enter more than $600
    pub panelboard_ckt_feeder_std_pct_cr_amt: Usd,
    /// Line 26a: Did you incur costs for a home energy audit?
    pub main_home_egy_audit_cost_ind: bool,
    /// Line 26b: Enter the cost of the home energy audits
    pub main_home_egy_audit_cost_amt: Usd,
    /// Line 26c: Multiply line 26b by 30% (0.30). Do not enter more than $150
    pub main_home_egy_audit_std_pct_cr_amt: Usd,
    /// Line 27: Add lines 18b, 19h, 20d, 22d, 23d, 24d, 25e, and 26c
    pub egy_effcnt_imprv_credit_subtl_amt: Usd,
    /// Line 28: Enter the smaller of line 27 or $1,200
    pub egy_effcnt_hm_imprv_cr_amt: Usd,
    /// Line 29a: Most expensive electric or natural gas heat pump — cost
    pub oth_elec_gas_ht_pump_cost_amt: Usd,
    /// Line 29b: Enter the cost of all other electric or natural gas heat pumps. If none, enter -0-
    pub oth_elec_gas_ht_pump_wtr_ht_cost_amt: Usd,
    /// Line 29c: Most expensive electric or natural gas heat pump water heater — cost
    pub ht_pump_wtr_heater_bmss_cost_amt: Usd,
    /// Line 29d/29f: Enter the cost of all other heat pump water heaters and biomass items.
    /// If none, enter -0-
    pub ht_pump_wtr_heater_bmss_std_pct_cr_amt: Usd,
    /// Line 29e: Most expensive biomass stove or boiler — cost
    pub oth_bmss_stove_blr_cost_amt: Usd,
    /// Line 29g: Add lines 29a through 29f
    pub energy_effcnt_imprv_allwbl_cost_amt: Usd,
    /// Line 30: Add lines 28 and 29h
    pub taxes_less_credits_amt: Usd,
    /// Line 31: Limitation based on tax liability from the Energy Efficient Home Improvement
    /// Credit Limit Worksheet
    pub tax_less_credits_amt: Usd,
    /// Line 32a/32b: If the special rule for joint occupants applies or if you live in a
    /// condominium or cooperative, enter the applicable code
    pub married_tpw_more_than_one_home_cd: String,
}
