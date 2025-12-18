use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    common::{
        Addr, CustomField, Email, EmailStatus, MetaData, NtRef, PrintStatus, TxnTaxDetail, TypedRef,
    },
    Class, Customer, LineField, QBCreatable, QBDeletable, QBFullUpdatable, QBItem, QBPDFable,
    QBSendable,
};

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, Default)]
#[serde(rename_all = "PascalCase", default)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(default, build_fn(error = "QBTypeError"), setter(into, strip_option))
)]
/// `CreditMemo`
///
/// a financial transaction representing a refund or credit of payment or part of a payment for goods or services that have been sold.
///
/// API reference:
/// <https://developer.intuit.com/app/developer/qbo/docs/api/accounting/all-entities/creditmemo>
pub struct CreditMemo {
    /// The unique ID of the entity
    pub id: Option<String>,
    /// The unique sync token of the entity, used for concurrency control
    pub sync_token: Option<String>,
    /// Metadata about the entity
    pub meta_data: Option<MetaData>,
    /// Line items
    pub line: Option<LineField>,
    /// Reference to the customer
    pub customer_ref: Option<TypedRef<Customer>>,
    /// Reference to the currency
    pub currency_ref: Option<NtRef>,
    /// Global tax calculation method
    pub global_tax_calculation: Option<String>,
    /// Reference to the project
    pub project_ref: Option<NtRef>,
    /// Email address for billing
    pub bill_email: Option<Email>,
    /// Date of the transaction in YYYY-MM-DD format
    pub txn_date: Option<NaiveDate>,
    /// Custom fields for the entity
    pub custom_field: Option<Vec<CustomField>>,
    /// Reference to the class
    pub class_ref: Option<TypedRef<Class>>,
    /// Print status of the credit memo
    pub print_status: Option<PrintStatus>,
    /// Source of the transaction
    pub sales_term_ref: Option<NtRef>,
    /// Due date of the credit memo in YYYY-MM-DD format
    pub total_amt: Option<f64>,
    /// Reference to the sales terms
    pub apply_tax_after_discount: Option<bool>,
    /// Reference number for the credit memo
    pub doc_number: Option<String>,
    /// Private note for the credit memo
    pub private_note: Option<String>,
    /// Custom memo for the customer
    pub customer_memo: Option<NtRef>,
    /// Tax details for the transaction
    pub txn_tax_detail: Option<TxnTaxDetail>,
    /// Payment method reference
    pub payment_method_ref: Option<NtRef>,
    /// Exchange rate for the transaction
    pub exhange_rate: Option<f64>,
    /// Address to which the items are shipped
    pub ship_addr: Option<Addr>,
    /// Reference to the department
    pub department_ref: Option<NtRef>,
    /// Email status of the credit memo
    pub email_status: Option<EmailStatus>,
    /// Billing address
    pub bill_addr: Option<Addr>,
    /// Home currency balance for the credit memo
    pub home_balance: Option<f64>,
    /// Remaining credit amount
    pub remaining_credit: Option<f64>,
    /// Reference to recurring schedule information
    pub recur_data_ref: Option<NtRef>,
    /// Reference to tax exemption information
    pub tax_exemption_ref: Option<NtRef>,
    /// Current balance of the credit memo
    pub balance: Option<f64>,
    /// Total amount in home currency
    pub home_total_amt: Option<f64>,
}

impl QBCreatable for CreditMemo {
    fn can_create(&self) -> bool {
        self.line.can_create() && self.customer_ref.is_some()
    }
}

impl QBSendable for CreditMemo {}
impl QBDeletable for CreditMemo {}
impl QBPDFable for CreditMemo {}
impl QBFullUpdatable for CreditMemo {
    fn can_full_update(&self) -> bool {
        self.has_read()
            && self.can_create()
            && (self
                .email_status
                .as_ref()
                .is_some_and(|s| matches!(s, EmailStatus::NeedToSend) && self.bill_email.is_some()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credit_memo_deserialize() {
        let json_data = r#"{
            "TxnDate": "2014-09-02",
            "domain": "QBO",
            "PrintStatus": "NeedToPrint",
            "TotalAmt": 100.0,
            "RemainingCredit": 0,
            "Line": [
              {
                "Description": "Pest Control Services",
                "DetailType": "SalesItemLineDetail",
                "SalesItemLineDetail": {
                  "TaxCodeRef": {
                    "value": "NON"
                  },
                  "Qty": 1,
                  "UnitPrice": 100,
                  "ItemRef": {
                    "name": "Pest Control",
                    "value": "10"
                  }
                },
                "LineNum": 1,
                "Amount": 100.0,
                "Id": "1"
              },
              {
                "DetailType": "SubTotalLineDetail",
                "Amount": 100.0,
                "SubTotalLineDetail": {}
              }
            ],
            "ApplyTaxAfterDiscount": false,
            "DocNumber": "1026",
            "sparse": false,
            "CustomerMemo": {
              "value": "Updated customer memo."
            },
            "ProjectRef": {
              "value": "39298034"
            },
            "Balance": 0,
            "CustomerRef": {
              "name": "Amy's Bird Sanctuary",
              "value": "1"
            },
            "TxnTaxDetail": {
              "TotalTax": 0
            },
            "SyncToken": "3",
            "CustomField": [
              {
                "DefinitionId": "1",
                "Type": "StringType",
                "Name": "Crew #"
              }
            ],
            "ShipAddr": {
              "CountrySubDivisionCode": "CA",
              "City": "Bayshore",
              "PostalCode": "94326",
              "Id": "108",
              "Line1": "4581 Finch St."
            },
            "EmailStatus": "NotSet",
            "BillAddr": {
              "Line4": "Bayshore, CA  94326",
              "Line3": "4581 Finch St.",
              "Id": "79",
              "Line1": "Amy Lauterbach",
              "Line2": "Amy's Bird Sanctuary"
            },
            "MetaData": {
              "CreateTime": "2014-09-18T12:51:27-07:00",
              "LastUpdatedTime": "2015-07-01T09:16:28-07:00"
            },
            "BillEmail": {
              "Address": "Birds@Intuit.com"
            },
            "Id": "73"
          }"#;
        let credit_memo: CreditMemo = serde_json::from_str(json_data).unwrap();
        dbg!("Deserialized CreditMemo: {:?}", &credit_memo);
        assert_eq!(credit_memo.id.unwrap(), "73");
        assert_eq!(credit_memo.sync_token.unwrap(), "3");
    }
}
