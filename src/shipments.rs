//! Shipments: list, show and cancel shipments.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Shiprocket, de,
    types::{DataResponse, MessageResponse, Paginated, PhpDateTime},
};

/// Query of `GET shipments`.
///
/// ```
/// let query = shiprocket::shipments::ListShipmentsQuery::builder()
///     .sort("DESC")
///     .sort_by("id")
///     .page(2)
///     .build();
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ListShipmentsQuery {
    /// `ASC` or `DESC`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Field to sort by, e.g. `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Filter value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Field to filter by, e.g. `id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
}

/// A shipment as returned by `GET shipments`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShipmentSummary {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub number: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub code: Option<String>,
    #[serde(default, deserialize_with = "de::vec")]
    pub products: Vec<ShipmentProduct>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb: Option<String>,
    /// Status label, e.g. `CANCELED`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    /// Human formatted, e.g. `28th Aug 2018 07:11 PM`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub channel_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub base_channel_code: Option<String>,
    /// `cod` or `prepaid`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_method: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShipmentProduct {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A shipment as returned by `GET shipments/{id}`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Shipment {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub channel_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub invoice_no: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub invoice_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub sr_courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_assign_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_generated_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_token_number: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub method: Option<String>,
    /// Weight in kg.
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub weight: Option<f64>,
    /// `LxBxH` in cm, e.g. `10.00x15.00x20.00`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub dimensions: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub cost: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub tax: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub cod_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub total: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub shipping_address: Option<ShipmentAddress>,
    #[serde(default)]
    pub customer_details: serde_json::Value,
    /// Numeric shipment status code.
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipped_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivered_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub returned_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub label_url: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub manifest_url: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub created_at: Option<PhpDateTime>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub updated_at: Option<PhpDateTime>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShipmentAddress {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub address: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub address_2: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub state: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub country: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pincode: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub company_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize)]
struct CancelShipmentsBody<'a> {
    awbs: Vec<&'a str>,
}

/// Shipment endpoints. Obtain with [`Shiprocket::shipments`].
pub struct Shipments<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn shipments(&self) -> Shipments<'_> {
        Shipments { client: self }
    }
}

impl Shipments<'_> {
    /// `GET shipments`: paginated list of shipments.
    pub async fn list(
        &self,
        query: &ListShipmentsQuery,
    ) -> Result<Paginated<ShipmentSummary>, Error> {
        self.client
            .get_query(self.client.api_url("shipments"), query)
            .await
    }

    /// `GET shipments/{shipment_id}`: details of one shipment.
    ///
    /// Shiprocket answers an unknown id with `{}`, so `data` is `None` then.
    pub async fn get(&self, shipment_id: i64) -> Result<DataResponse<Shipment>, Error> {
        self.client
            .get(self.client.api_url(&format!("shipments/{shipment_id}")))
            .await
    }

    /// `POST orders/cancel/shipment/awbs`: cancel shipments by AWB, before pickup. Max 2000 AWBs.
    pub async fn cancel<S: AsRef<str>>(&self, awbs: &[S]) -> Result<MessageResponse, Error> {
        let awbs = awbs.iter().map(AsRef::as_ref).collect();
        self.client
            .post(
                self.client.api_url("orders/cancel/shipment/awbs"),
                &CancelShipmentsBody { awbs },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_response() {
        let r: Paginated<ShipmentSummary> = serde_json::from_str(
            r#"{"data":[{"number":"","code":"","id":3322791,"order_id":3324748,"products":[{"name":"gsdgw","sku":"123","quantity":2}],"awb":"109123535421","status":"CANCELED","created_at":"28th Aug 2018 07:11 PM","channel_id":76893,"channel_name":"CUSTOM","base_channel_code":"CS","payment_method":"cod"}],"meta":{"pagination":{"total":19631,"count":15,"per_page":2,"current_page":1,"total_pages":1309,"links":{"next":"https://apiv2.shiprocket.in/v1/external/shipments?page=2"}}}}"#,
        )
        .unwrap();
        let s = &r.data[0];
        assert_eq!(s.id, Some(3322791));
        assert_eq!(s.number, None);
        assert_eq!(s.products[0].quantity, Some(2));
        assert_eq!(s.status.as_deref(), Some("CANCELED"));
        assert_eq!(r.meta.unwrap().pagination.unwrap().total_pages, Some(1309));
    }

    #[test]
    fn show_response() {
        let r: DataResponse<Shipment> = serde_json::from_str(
            r#"{"data":{"id":16016920,"order_id":16167171,"channel_id":76893,"company_id":67216,"invoice_no":null,"courier":null,"awb":null,"method":"Standard","weight":"0.000","dimensions":"0.00x0.00x0.00","quantity":1,"cost":"0.00","tax":"0.00","cod_charges":"0.00","total":"9000.00","shipping_address":{"city":"New Delhi","state":"DELHI","address":"House 221B, Leaf Village","country":"India","pincode":"110002","address_2":"Near Hokage House","company_name":null},"customer_details":null,"status":8,"label_url":null,"created_at":{"date":"2019-07-31 12:37:41.000000","timezone_type":3,"timezone":"Asia/Kolkata"},"updated_at":{"date":"2019-07-31 15:57:11.000000","timezone_type":3,"timezone":"Asia/Kolkata"}}}"#,
        )
        .unwrap();
        let s = r.data.unwrap();
        assert_eq!(s.id, Some(16016920));
        assert_eq!(s.total, Some(9000.0));
        assert_eq!(s.weight, Some(0.0));
        assert_eq!(s.status, Some(8));
        assert_eq!(
            s.shipping_address.unwrap().pincode.as_deref(),
            Some("110002")
        );
        assert_eq!(
            s.created_at.unwrap().date.as_deref(),
            Some("2019-07-31 12:37:41.000000")
        );

        let r: DataResponse<Shipment> = serde_json::from_str("{}").unwrap();
        assert!(r.data.is_none());
    }

    #[test]
    fn cancel_response_and_body() {
        let r: MessageResponse = serde_json::from_str(
            r#"{"message":"Bulk Shipment cancellation is in progress. Please wait for some time."}"#,
        )
        .unwrap();
        assert!(r.message.unwrap().starts_with("Bulk Shipment"));
        let body = serde_json::to_value(CancelShipmentsBody {
            awbs: vec!["19041211125783"],
        })
        .unwrap();
        assert_eq!(body, serde_json::json!({"awbs":["19041211125783"]}));
    }
}
