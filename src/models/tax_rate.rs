#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
pub struct TaxRate {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Name of the tax rate
    pub name: Option<String>,
    /// Rate percentage of the tax rate
    pub rate_value: Option<f64>,
    /// Indicates if the tax rate is active
    pub active: Option<bool>,
    /// Description of the tax rate
    pub description: Option<String>,
    /// Reference to the tax agency associated with the tax rate
    pub agency_ref: Option<NtRef>,
    /// Metadata about the entity
    pub meta_data: Option<MetaData>,
    /// TaxRate DisplayType enum which acts as display config.
    pub display_type: Option<String>,
    /// Reference to the tax return line associated with the tax rate
    pub tax_return_line_ref: Option<NtRef>,
    /// Effective tax rate details
    pub effective_tax_rate: Option<EffectiveTaxRate>,
    /// Special tax type information
    pub special_tax_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EffectiveTaxRate {
    pub rate_value: f64,
    pub end_date: NaiveDateTime,
    pub effective_date: NaiveDateTime,
}
