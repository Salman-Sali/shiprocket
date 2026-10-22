//! Billing: account statement and weight discrepancies.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{Error, Shiprocket, de, types::Paginated};

/// Query of `GET account/details/statement`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct StatementQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    /// Start date, `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// End date, `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// A statement line.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct StatementEntry {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub transaction_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub order_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_order_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub return_awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charged_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub billed_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub action: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charge: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub debit_amount: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub credit_amount: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub balance_amount: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub balance_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub volumetric_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub entered_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub can_ship: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `GET billing/discrepancy`.
///
/// The docs only show an empty `data` list, so entries are left untyped.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DiscrepancyResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::vec")]
    pub data: Vec<serde_json::Value>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub upper_fold_text: Option<String>,
    /// Sic: the API spells it `lower_fild_text`.
    #[serde(
        default,
        alias = "lower_fold_text",
        rename = "lower_fild_text",
        deserialize_with = "de::opt_string"
    )]
    pub lower_fold_text: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Billing endpoints. Obtain with [`Shiprocket::billing`].
pub struct Billing<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn billing(&self) -> Billing<'_> {
        Billing { client: self }
    }
}

impl Billing<'_> {
    /// `GET account/details/statement`: account statement lines.
    pub async fn statement(
        &self,
        query: &StatementQuery,
    ) -> Result<Paginated<StatementEntry>, Error> {
        self.client
            .get_query(self.client.api_url("account/details/statement"), query)
            .await
    }

    /// `GET billing/discrepancy`: weight discrepancies raised against your shipments.
    pub async fn discrepancy(&self) -> Result<DiscrepancyResponse, Error> {
        self.client
            .get(self.client.api_url("billing/discrepancy"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_statement() {
        let res: Paginated<StatementEntry> = serde_json::from_str(
            r#"{"data":[{"transaction_id":"","order_id":"","channel_order_id":"","awb_code":"",
            "return_awb_code":null,"applied_weight":"","charged_weight":"","billed_weight":"","action":"",
            "charge":"","description":"Wallet Balance","debit_amount":"","credit_amount":"",
            "balance_amount":"0","balance_weight":0,"volumetric_weight":"","entered_weight":"",
            "created_at":"","can_ship":true}]}"#,
        )
        .unwrap();
        let e = &res.data[0];
        assert_eq!(e.description.as_deref(), Some("Wallet Balance"));
        assert_eq!(e.balance_amount, Some(0.0));
        assert_eq!(e.debit_amount, None);
        assert_eq!(e.can_ship, Some(true));
    }

    #[test]
    fn decodes_discrepancy() {
        let res: DiscrepancyResponse = serde_json::from_str(
            r#"{"status":200,"data":[],"upper_fold_text":"The entered weight...","lower_fild_text":"To raise a ticket..."}"#,
        )
        .unwrap();
        assert_eq!(res.status, Some(200));
        assert!(res.data.is_empty());
        assert_eq!(res.lower_fold_text.as_deref(), Some("To raise a ticket..."));
    }
}
