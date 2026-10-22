//! Labels, manifests and invoices.

use serde::{Deserialize, Serialize};

use crate::{Error, Shiprocket, de};

/// Response of `POST manifests/generate` and `POST manifests/print`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ManifestResponse {
    /// Only set by `manifests/generate`.
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    /// PDF url. `None` when nothing could be printed.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub manifest_url: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST courier/generate/label`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LabelResponse {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub label_created: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub label_url: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub response: Option<String>,
    #[serde(default, deserialize_with = "de::vec")]
    pub not_created: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST orders/print/invoice`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InvoiceResponse {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_invoice_created: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub invoice_url: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    #[serde(default, deserialize_with = "de::vec")]
    pub not_created: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST courier/generate/label-invoice`.
///
/// `completed` only means the request ran; check `success_count` / `error_count`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct LabelInvoiceResponse {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub completed: Option<bool>,
    /// Combined PDF: label followed by invoice for each shipment, in input order.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub file_url: Option<String>,
    /// Error details when some shipments failed.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub error_file_url: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub success_count: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub error_count: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize)]
struct ShipmentIds<'a> {
    shipment_id: &'a [i64],
}

#[derive(Serialize)]
struct OrderIds<'a> {
    order_ids: &'a [i64],
}

#[derive(Serialize)]
struct Ids<'a> {
    ids: &'a [i64],
}

#[derive(Serialize)]
struct ShipmentIdsPlural<'a> {
    shipment_ids: &'a [i64],
}

/// Label, manifest and invoice endpoints. Obtain with [`Shiprocket::documents`].
pub struct Documents<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn documents(&self) -> Documents<'_> {
        Documents { client: self }
    }
}

impl Documents<'_> {
    /// `POST manifests/generate`: generate the manifest for shipments with an AWB and pickup.
    pub async fn generate_manifest(&self, shipment_ids: &[i64]) -> Result<ManifestResponse, Error> {
        self.client
            .post(
                self.client.api_url("manifests/generate"),
                &ShipmentIds {
                    shipment_id: shipment_ids,
                },
            )
            .await
    }

    /// `POST manifests/print`: url of an already generated manifest, by Shiprocket order id.
    pub async fn print_manifest(&self, order_ids: &[i64]) -> Result<ManifestResponse, Error> {
        self.client
            .post(
                self.client.api_url("manifests/print"),
                &OrderIds { order_ids },
            )
            .await
    }

    /// `POST courier/generate/label`: shipping label for shipments with an AWB.
    pub async fn generate_label(&self, shipment_ids: &[i64]) -> Result<LabelResponse, Error> {
        self.client
            .post(
                self.client.api_url("courier/generate/label"),
                &ShipmentIds {
                    shipment_id: shipment_ids,
                },
            )
            .await
    }

    /// `POST orders/print/invoice`: invoice for Shiprocket order ids.
    pub async fn generate_invoice(&self, order_ids: &[i64]) -> Result<InvoiceResponse, Error> {
        self.client
            .post(
                self.client.api_url("orders/print/invoice"),
                &Ids { ids: order_ids },
            )
            .await
    }

    /// `POST courier/generate/label-invoice`: one PDF with label + invoice per shipment.
    /// Max 200 shipments; not available to reseller accounts.
    pub async fn generate_label_invoice(
        &self,
        shipment_ids: &[i64],
    ) -> Result<LabelInvoiceResponse, Error> {
        self.client
            .post(
                self.client.api_url("courier/generate/label-invoice"),
                &ShipmentIdsPlural { shipment_ids },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_responses() {
        let r: ManifestResponse = serde_json::from_str(
            r#"{"status":1,"manifest_url":"https://s3-ap-southeast-1.amazonaws.com/kr-shipmultichannel/25149/manifest/MANIFEST-3051.pdf"}"#,
        )
        .unwrap();
        assert_eq!(r.status, Some(1));
        assert!(r.manifest_url.unwrap().ends_with("MANIFEST-3051.pdf"));

        let r: ManifestResponse = serde_json::from_str(r#"{"manifest_url":""}"#).unwrap();
        assert_eq!(r.manifest_url, None);
    }

    #[test]
    fn label_response() {
        let r: LabelResponse = serde_json::from_str(
            r#"{"label_created":1,"label_url":"https://kr-shipmultichannel.s3.ap-southeast-1.amazonaws.com/25149/labels/shipping-label-16104408-788830567028.pdf","response":"Label has been created and uploaded successfully!","not_created":[]}"#,
        )
        .unwrap();
        assert_eq!(r.label_created, Some(true));
        assert!(r.label_url.is_some());
        assert!(r.not_created.is_empty());
    }

    #[test]
    fn invoice_response() {
        let r: InvoiceResponse = serde_json::from_str(
            r#"{"is_invoice_created":true,"invoice_url":"https://s3-ap-southeast-1.amazonaws.com/kr-shipmultichannel/25149/invoices/KD101019281564656872.pdf","not_created":[]}"#,
        )
        .unwrap();
        assert_eq!(r.is_invoice_created, Some(true));
        assert!(r.invoice_url.unwrap().ends_with(".pdf"));
    }

    #[test]
    fn label_invoice_response() {
        let r: LabelInvoiceResponse = serde_json::from_str(
            r#"{"completed":true,"file_url":"https://shiprocket.s3.amazonaws.com/combined-label-invoice.pdf","error_file_url":"","success_count":2,"error_count":0}"#,
        )
        .unwrap();
        assert_eq!(r.completed, Some(true));
        assert_eq!(r.error_file_url, None);
        assert_eq!(r.success_count, Some(2));
    }

    #[test]
    fn request_bodies() {
        let v = |b| serde_json::to_value(b).unwrap();
        assert_eq!(
            v(&ShipmentIds {
                shipment_id: &[1, 2]
            }),
            serde_json::json!({"shipment_id":[1,2]})
        );
        assert_eq!(
            serde_json::to_value(OrderIds { order_ids: &[3] }).unwrap(),
            serde_json::json!({"order_ids":[3]})
        );
        assert_eq!(
            serde_json::to_value(Ids { ids: &[4] }).unwrap(),
            serde_json::json!({"ids":[4]})
        );
        assert_eq!(
            serde_json::to_value(ShipmentIdsPlural { shipment_ids: &[5] }).unwrap(),
            serde_json::json!({"shipment_ids":[5]})
        );
    }
}
