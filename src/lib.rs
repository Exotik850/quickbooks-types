pub use chrono::{DateTime, NaiveDate, Utc};

use serde::{Deserialize, Serialize};

mod objects;

// update `impl std::fmt::Display for Addr` below if any fields are added

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct AttachableRef {
    pub entity_ref: NtRef,
    pub include_on_send: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachable_ref: Vec<AttachableRef>,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub file_access_uri: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub file_name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    pub meta_data: MetaData,
    #[serde(default)]
    pub size: i64,
    #[serde(default, rename = "sparse")]
    pub sparse: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub temp_download_uri: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Any {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        rename = "declaredType"
    )]
    pub declared_type: String,
    #[serde(default, rename = "globalScope")]
    pub global_scope: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default)]
    pub nil: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scope: String,
    #[serde(default, rename = "typeSubstituted")]
    pub type_substituted: bool,
    #[serde(default)]
    pub value: NtRef,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BillPayment {
    #[serde(default)]
    pub check_payment: Payment,
    #[serde(default)]
    pub credit_card_payment: Payment,
    #[serde(default)]
    pub currency_ref: NtRef,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub doc_number: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<Line>,

    pub meta_data: MetaData,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub pay_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub private_note: String,
    #[serde(default, rename = "sparse")]
    pub sparse: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,
    #[serde(default)]
    pub total_amt: f32,

    pub txn_date: NaiveDate,
    #[serde(default)]
    pub vendor_ref: NtRef,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    #[serde(default)]
    pub bank_account_ref: NtRef,
    #[serde(default, rename = "CCAccountRef")]
    pub cc_account_ref: NtRef,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub print_status: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Purchase {
    #[serde(default)]
    pub account_ref: NtRef,
    #[serde(default)]
    pub credit: bool,
    #[serde(default)]
    pub currency_ref: NtRef,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub doc_number: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "domain")]
    pub domain: String,
    #[serde(default)]
    pub entity_ref: NtRef,

    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub line: Vec<Line>,

    pub meta_data: MetaData,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub payment_type: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub private_note: String,
    #[serde(default)]
    pub purchase_ex: PurchaseEx,
    #[serde(default, rename = "sparse")]
    pub sparse: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sync_token: String,
    #[serde(default)]
    pub total_amt: f32,

    pub txn_date: NaiveDate,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PurchaseEx {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub any: Vec<Any>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryResponse {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachable: Option<Vec<Attachment>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bill_payment: Option<Vec<BillPayment>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub company_info: Option<Vec<CompanyInfo>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub item: Option<Vec<Item>>,
    #[serde(default, rename = "maxResults")]
    pub max_results: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub purchase: Opton<Vec<Purchase>>,
    #[serde(default, rename = "startPosition")]
    pub start_position: i64,
    #[serde(default, rename = "totalCount")]
    pub total_count: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Response {
    #[serde(default)]
    pub query_response: QueryResponse,
}
