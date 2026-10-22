//! Wrapper API: create an order, assign an AWB, request pickup and (for forward shipments)
//! generate label and manifest in a single call.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    Error, PaymentMethod, Shiprocket, de, returns::CreateReturnRequest, types::PhpDateTime,
};

/// Body of `POST shipments/create/forward-shipment`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ForwardShipmentRequest {
    /// Your reference for the order.
    pub order_id: String,
    /// `yyyy-mm-dd`, optionally followed by `hh:mm`.
    pub order_date: String,
    /// Name of an existing pickup location, or of the new one described in `vendor_details`.
    pub pickup_location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// `Surface` or `Air`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Defaults to `true` on Shiprocket's side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_pickup: Option<bool>,
    /// Defaults to `true` on Shiprocket's side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_label: Option<bool>,
    /// Defaults to `true` on Shiprocket's side.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_manifest: Option<bool>,
    /// Courier to assign. Shiprocket picks one when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub courier_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_no: Option<String>,
    /// Name printed as the sender on the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isd_code: Option<String>,

    pub billing_customer_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_last_name: Option<String>,
    pub billing_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_2: Option<String>,
    pub billing_city: String,
    pub billing_state: String,
    pub billing_country: String,
    pub billing_pincode: String,
    pub billing_email: String,
    pub billing_phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_alternate_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_isd_code: Option<String>,

    /// When false, the `shipping_*` fields are required.
    pub shipping_is_billing: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_customer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_pincode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_phone: Option<String>,

    pub order_items: Vec<ForwardShipmentItem>,
    pub payment_method: PaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_charges: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giftwrap_charges: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_charges: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<f64>,
    pub sub_total: f64,
    /// kg, total for the shipment.
    pub weight: f64,
    /// cm, > 0.5
    pub length: f64,
    /// cm, > 0.5
    pub breadth: f64,
    /// cm, > 0.5
    pub height: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gstin: Option<String>,
    /// Adds a new pickup location and uses it for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_details: Option<VendorDetails>,
    /// `ESSENTIALS` or `NON ESSENTIALS` (case sensitive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Delivery address longitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Delivery address latitude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what3words_address: Option<String>,
    /// Ship as a document. Sent as `0`/`1`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub is_document: Option<bool>,
}

/// An item of a [`ForwardShipmentRequest`].
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ForwardShipmentItem {
    pub name: String,
    pub sku: String,
    pub units: i64,
    /// Per unit, inclusive of GST.
    pub selling_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn: Option<String>,
    /// Tax in percent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
}

/// New pickup location to add while creating a forward shipment.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct VendorDetails {
    /// New location name, alphanumeric, max 36 characters.
    pub pickup_location: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    /// Min 10 characters.
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pin_code: String,
}

/// Body of `POST shipments/create/return-shipment`: a return order plus pickup request.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct ReturnShipmentRequest {
    #[serde(flatten)]
    pub order: CreateReturnRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_pickup: Option<bool>,
}

/// Response of both wrapper endpoints.
///
/// `status` is `1` on success. On failure Shiprocket still answers `200` with `status: 0`
/// and fills `payload.action` / `payload.error_message`; check [`ShipmentResponse::is_success`].
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShipmentResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub payload: Option<ShipmentPayload>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl ShipmentResponse {
    pub fn is_success(&self) -> bool {
        self.status == Some(1)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShipmentPayload {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub pickup_location_added: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub order_created: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub awb_generated: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub label_generated: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub pickup_generated: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub manifest_generated: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_scheduled_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_booked_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub assigned_date_time: Option<PhpDateTime>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub cod: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_return: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub label_url: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub manifest_url: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub routing_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_routing_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_token_number: Option<String>,
    /// Step that failed, when `status` is `0`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub action: Option<String>,
    /// Failure reason, when `status` is `0`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub error_message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

fn opt_bool_as_int<S: serde::Serializer>(v: &Option<bool>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        Some(b) => s.serialize_u8(*b as u8),
        None => s.serialize_none(),
    }
}

/// Wrapper endpoints. Obtain with [`Shiprocket::wrapper`].
pub struct Wrapper<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn wrapper(&self) -> Wrapper<'_> {
        Wrapper { client: self }
    }
}

impl Wrapper<'_> {
    /// `POST shipments/create/forward-shipment`: create order, assign AWB, request pickup,
    /// generate label and manifest.
    pub async fn forward_shipment(
        &self,
        req: &ForwardShipmentRequest,
    ) -> Result<ShipmentResponse, Error> {
        self.client
            .post(
                self.client.api_url("shipments/create/forward-shipment"),
                req,
            )
            .await
    }

    /// `POST shipments/create/return-shipment`: create return order, assign AWB, schedule
    /// reverse pickup.
    pub async fn return_shipment(
        &self,
        req: &ReturnShipmentRequest,
    ) -> Result<ShipmentResponse, Error> {
        self.client
            .post(self.client.api_url("shipments/create/return-shipment"), req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forward_success() {
        let r: ShipmentResponse = serde_json::from_str(
            r#"{"status":1,"payload":{"pickup_location_added":0,"order_created":1,"awb_generated":1,"label_generated":1,"pickup_generated":1,"manifest_generated":1,"pickup_scheduled_date":"2022-06-04 09:00:00","pickup_booked_date":null,"order_id":222521420,"shipment_id":222002884,"awb_code":"14326421307048","courier_company_id":25,"courier_name":"Xpressbees 5kg","assigned_date_time":{"date":"2022-06-03 13:52:05.051557","timezone_type":3,"timezone":"Asia/Kolkata"},"applied_weight":10,"cod":1,"label_url":"https://x/label.pdf","manifest_url":"https://x/manifest.pdf","routing_code":"N/S-01/12B/2","rto_routing_code":"","pickup_token_number":"Reference No: 194"}}"#,
        )
        .unwrap();
        assert!(r.is_success());
        let p = r.payload.unwrap();
        assert_eq!(p.pickup_location_added, Some(false));
        assert_eq!(p.awb_code.as_deref(), Some("14326421307048"));
        assert_eq!(p.courier_company_id, Some(25));
        assert_eq!(p.applied_weight, Some(10.0));
        assert_eq!(p.cod, Some(true));
        assert_eq!(
            p.assigned_date_time.unwrap().timezone.as_deref(),
            Some("Asia/Kolkata")
        );
        assert_eq!(p.rto_routing_code, None);
    }

    #[test]
    fn forward_failure() {
        let r: ShipmentResponse = serde_json::from_str(
            r#"{"status":0,"payload":{"action":"Adding pickup location","error_message":"Oops! Invalid Data."}}"#,
        )
        .unwrap();
        assert!(!r.is_success());
        let p = r.payload.unwrap();
        assert_eq!(p.action.as_deref(), Some("Adding pickup location"));
        assert_eq!(p.error_message.as_deref(), Some("Oops! Invalid Data."));
    }

    #[test]
    fn return_success() {
        let r: ShipmentResponse = serde_json::from_str(
            r#"{"status":1,"payload":{"order_created":1,"awb_generated":1,"pickup_generated":1,"pickup_scheduled_date":"2022-02-17 09:00:00","order_id":186255214,"shipment_id":185783600,"awb_code":"24112321126686","courier_company_id":125,"courier_name":"Xpressbees Reverse","applied_weight":0.5,"cod":0,"is_return":1,"routing_code":"N/S-01/13B/015","rto_routing_code":"","pickup_token_number":""}}"#,
        )
        .unwrap();
        let p = r.payload.unwrap();
        assert_eq!(p.is_return, Some(true));
        assert_eq!(p.shipment_id, Some(185783600));
    }

    #[test]
    fn return_request_flattens() {
        let order = CreateReturnRequest::builder()
            .order_id("R1")
            .order_date("2024-01-01")
            .pickup_customer_name("a")
            .pickup_address("b")
            .pickup_city("c")
            .pickup_state("d")
            .pickup_country("India")
            .pickup_pincode("110030")
            .pickup_email("a@b.c")
            .pickup_phone("9999999999")
            .shipping_customer_name("s")
            .shipping_address("x")
            .shipping_city("y")
            .shipping_state("z")
            .shipping_country("India")
            .shipping_pincode("201005")
            .shipping_phone("8888888888")
            .order_items(vec![])
            .payment_method(PaymentMethod::Prepaid)
            .sub_total(10.0)
            .length(1.0)
            .breadth(1.0)
            .height(1.0)
            .weight(0.5)
            .build();
        let req = ReturnShipmentRequest::builder()
            .order(order)
            .request_pickup(true)
            .build();
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["order_id"], "R1");
        assert_eq!(v["request_pickup"], true);
        assert!(v.get("order").is_none());
    }
}
