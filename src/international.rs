//! International shipping: KYC, bank details, orders, serviceability, AWB, manifest,
//! pickup and order tracking.
//!
//! AWB / shipment tracking by AWB, shipment id or order id uses the domestic
//! [`Tracking`](crate::tracking::Tracking) endpoints.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Shiprocket, de,
    types::{MessageResponse, Paginated, PaymentMethod, PhpDateTime},
};

/// Body of `POST international/settings/international_kyc`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct KycRequest {
    /// Seller organization type, e.g. `Sole Proprietor` or `company`.
    pub organization_type: String,
    /// Seller's IP address.
    pub ip_address: String,
    pub documents: Vec<KycDocument>,
}

/// A KYC document, e.g. `iec_code`, `ad_code`, `gstin_certificate`, `pan_card`, `aadhar_card`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct KycDocument {
    pub document_name: String,
    /// The document number, e.g. the IEC code or PAN.
    pub document_value: String,
    pub attachment: Vec<KycAttachment>,
}

/// A base64 encoded document scan, max 3 MB.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct KycAttachment {
    /// Base64 encoded file content.
    pub file: String,
    /// File extension, e.g. `pdf`.
    pub extension: String,
}

/// Body of `POST international/settings/add-bank-details`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct AddBankDetailsRequest {
    /// `saving` or `current`.
    pub bank_account_type: String,
    /// Alphabets and spaces only.
    pub beneficiary_name: String,
    pub bank_ifsc_code: String,
    /// 9 to 18 digits.
    pub bank_account_number: String,
}

/// Body of `POST international/orders/create/adhoc`, `POST international/orders/update/adhoc`
/// and `POST international/shipments/create/forward-shipment`.
///
/// ```
/// use shiprocket::{PaymentMethod, international::{InternationalOrderItem, InternationalOrderRequest}};
///
/// let req = InternationalOrderRequest::builder()
///     .order_id("172647058")
///     .order_date("2022-03-30 00:34")
///     .billing_customer_name("Elena")
///     .billing_city("Panchkula")
///     .billing_pincode("134113")
///     .billing_state("Haryana")
///     .billing_country("India")
///     .billing_email("elena@example.com")
///     .billing_phone("9762343722")
///     .shipping_is_billing(true)
///     .order_items(vec![InternationalOrderItem::builder()
///         .name("Spanner")
///         .sku("5-47606")
///         .units(1)
///         .selling_price(100.0)
///         .build()])
///     .payment_method(PaymentMethod::Prepaid)
///     .sub_total(100.0)
///     .length(10.0)
///     .breadth(10.0)
///     .height(10.0)
///     .weight(0.41)
///     .currency("USD")
///     .reason_of_export(2)
///     .terms_of_invoice("FOB")
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct InternationalOrderRequest {
    /// Your order id, max 50 characters.
    pub order_id: String,
    /// `yyyy-mm-dd`, time optional.
    pub order_date: String,
    /// Name of an existing pickup location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_location_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i64>,
    /// `Reseller: [name]` to print a 'From' name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// ISD code of the shipping phone, e.g. `+1`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isd_code: Option<String>,

    pub billing_customer_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_isd_code: Option<String>,
    /// Max 30 characters.
    pub billing_city: String,
    pub billing_pincode: String,
    pub billing_state: String,
    pub billing_country: String,
    pub billing_email: String,
    pub billing_phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_alternate_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmark: Option<String>,

    /// When `false` the `shipping_*` fields are required.
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
    pub shipping_pincode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    pub order_items: Vec<InternationalOrderItem>,
    pub payment_method: PaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_charges: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giftwrap_charges: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_charges: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<f64>,
    /// Order total after deductions. Not calculated by Shiprocket.
    pub sub_total: f64,
    /// Package length in cm, more than 0.5.
    pub length: f64,
    /// Package breadth in cm, more than 0.5.
    pub breadth: f64,
    /// Package height in cm, more than 0.5.
    pub height: f64,
    /// Package weight in kg, more than 0.
    pub weight: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gstin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// `ESSENTIALS` / `NON ESSENTIALS` (the examples also send `1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_shipping_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what3words_address: Option<String>,
    /// `1` when the shipment is a document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_document: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_challan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_insurance_opt: Option<bool>,

    /// `0` gift, `1` sample, `2` commercial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose_of_shipment: Option<i64>,
    /// INR, USD, GBP, EUR, AUD, CAD, SAR, AED or SGD.
    pub currency: String,
    /// `0` bonafide sample, `1` sample, `2` gift, `3` commercial. Required on create.
    #[serde(rename = "reasonOfExport", skip_serializing_if = "Option::is_none")]
    pub reason_of_export: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commodity: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mies: Option<bool>,
    /// `A` not applicable, `B` LUT / export under bond, `C` export against payment of IGST.
    #[serde(rename = "igstPaymentStatus", skip_serializing_if = "Option::is_none")]
    pub igst_payment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub igst_amount: Option<f64>,
    /// `FOB` or `CIF`. Required on create.
    #[serde(rename = "Terms_Of_Invoice", skip_serializing_if = "Option::is_none")]
    pub terms_of_invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ioss: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori: Option<String>,
    /// Vendor pickup address; used by the forward shipment wrapper.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_details: Option<VendorDetails>,
}

/// One line of [`InternationalOrderRequest::order_items`].
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct InternationalOrderItem {
    pub name: String,
    pub sku: String,
    pub units: i64,
    /// Price per unit, inclusive of GST.
    pub selling_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    /// Tax percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
}

/// Vendor address sent with the forward shipment wrapper.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct VendorDetails {
    pub email: String,
    pub phone: String,
    pub name: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pin_code: String,
    pub pickup_location: String,
}

/// Query of `GET international/courier/serviceability`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct InternationalServiceabilityQuery {
    /// Shipment weight in kg.
    pub weight: f64,
    /// Must be `0`; COD is not available internationally.
    #[builder(default)]
    pub cod: i64,
    /// Destination country ISO alpha-2 code, e.g. `US`.
    pub delivery_country: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// Pickup postcode other than the primary pickup address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_postcode: Option<String>,
}

/// Body of `POST international/courier/assign/awb`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct InternationalAssignAwbRequest {
    pub shipment_id: i64,
    /// Default courier when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub courier_id: Option<i64>,
    /// `reassign` to change the courier (once per 24 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Body of `POST international/manifests/generate`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct InternationalManifestRequest {
    pub shipment_id: Vec<i64>,
}

/// Body of `POST courier/generate/pickup`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct InternationalPickupRequest {
    /// Only one shipment id is accepted per call.
    pub shipment_id: Vec<i64>,
    /// `retry` to retry a failed pickup request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Response of `POST international/orders/create/adhoc`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalOrderCreated {
    /// Shiprocket order id; keep it for later calls.
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub onboarding_completed_now: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST international/orders/update/adhoc`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalOrderUpdated {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub partially_update: Option<bool>,
    /// Comma separated list of fields that were ignored.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub not_updated_fields: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub new_order_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub old_order_status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST international/shipments/create/forward-shipment`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalForwardShipment {
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
    #[serde(default, deserialize_with = "de::opt_string")]
    pub assigned_date_time: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub cod: Option<bool>,
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
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `GET international/courier/serviceability`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalServiceability {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<InternationalServiceabilityData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalServiceabilityData {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_recommendation_enabled: Option<bool>,
    #[serde(default)]
    pub recommended_by: serde_json::Value,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub recommended_courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shiprocket_recommended_courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::vec")]
    pub available_courier_companies: Vec<InternationalCourier>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A courier able to serve an international shipment.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalCourier {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub mode: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub min_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charge_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub realtime_tracking: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pod_available: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_international: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rating: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub blocked: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub etd: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub estimated_delivery_days: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub etd_hours: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub rate: Option<InternationalCourierRate>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub coverage_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub insurance_applicable: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Rate block of an [`InternationalCourier`].
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalCourierRate {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rate: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub zone: Option<String>,
    /// Rate per zone, e.g. `{"roc": "109.01", "default": "108.01"}`.
    #[serde(default)]
    pub zone_rates: serde_json::Value,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub first_mile_charge: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub last_mile_charge: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub total: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST international/courier/assign/awb`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalAssignAwbResponse {
    /// `1` on success, `0` on failure (see `response.data.awb_assign_error`).
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub awb_assign_status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub response: Option<InternationalAwbResponseBody>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalAwbResponseBody {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<InternationalAwbData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalAwbData {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub cod: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub awb_code_status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub assigned_date_time: Option<PhpDateTime>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub child_courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub routing_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_routing_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub invoice_no: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub transporter_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub transporter_name: Option<String>,
    /// Shipper and RTO address details.
    #[serde(default)]
    pub shipped_by: serde_json::Value,
    /// Set when the assignment failed.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_assign_error: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST international/manifests/generate`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalManifestResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub manifest_url: Option<String>,
    /// Set when the manifest was not generated.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    /// Shipment ids to check when the manifest was not generated.
    #[serde(default, deserialize_with = "de::vec")]
    pub check_ids: Vec<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST courier/generate/pickup` for an international shipment.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalPickupResponse {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_token_number: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_scheduled_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub pickup_generated: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub manifest_generated: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub manifest_url: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// An order returned by `GET international/orders/track`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalTrackedOrder {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub channel_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_order_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_email: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_phone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_address: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_address_2: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_city: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_state: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_pincode: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_country: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_location: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub total: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub tax: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub master_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_method: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_international: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub purpose_of_shipment: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub updated_at: Option<String>,
    #[serde(default, deserialize_with = "de::vec")]
    pub products: Vec<InternationalTrackedProduct>,
    #[serde(default, deserialize_with = "de::vec")]
    pub shipments: Vec<InternationalTrackedShipment>,
    /// Event names, e.g. `ORDER_CREATED`, `ORDER_DELIVERED`.
    #[serde(default, deserialize_with = "de::vec")]
    pub activities: Vec<String>,
    /// Billing details, currency, package info, ...
    #[serde(default)]
    pub others: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalTrackedProduct {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_order_product_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub product_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub price: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub product_cost: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub hsn: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct InternationalTrackedShipment {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub weight: Option<f64>,
    /// `LxBxH`, e.g. `10x10x10`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub dimensions: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipped_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_scheduled_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub return_awb: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub volumetric_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub etd: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivered_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// International endpoints. Obtain with [`Shiprocket::international`].
pub struct International<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn international(&self) -> International<'_> {
        International { client: self }
    }
}

impl International<'_> {
    /// `POST international/settings/international_kyc`: submit KYC documents.
    pub async fn kyc(&self, req: &KycRequest) -> Result<MessageResponse, Error> {
        self.client
            .post(
                self.client
                    .api_url("international/settings/international_kyc"),
                req,
            )
            .await
    }

    /// `POST international/settings/add-bank-details`.
    pub async fn add_bank_details(
        &self,
        req: &AddBankDetailsRequest,
    ) -> Result<MessageResponse, Error> {
        self.client
            .post(
                self.client
                    .api_url("international/settings/add-bank-details"),
                req,
            )
            .await
    }

    /// `POST international/orders/create/adhoc`: create an international order.
    pub async fn create_order(
        &self,
        req: &InternationalOrderRequest,
    ) -> Result<InternationalOrderCreated, Error> {
        self.client
            .post(
                self.client.api_url("international/orders/create/adhoc"),
                req,
            )
            .await
    }

    /// `POST international/orders/update/adhoc`: update an order before AWB assignment.
    pub async fn update_order(
        &self,
        req: &InternationalOrderRequest,
    ) -> Result<InternationalOrderUpdated, Error> {
        self.client
            .post(
                self.client.api_url("international/orders/update/adhoc"),
                req,
            )
            .await
    }

    /// `POST international/shipments/create/forward-shipment`: create the order, assign AWB,
    /// and generate pickup, label and manifest in one call.
    pub async fn forward_shipment(
        &self,
        req: &InternationalOrderRequest,
    ) -> Result<InternationalForwardShipment, Error> {
        self.client
            .post(
                self.client
                    .api_url("international/shipments/create/forward-shipment"),
                req,
            )
            .await
    }

    /// `GET international/courier/serviceability`: couriers available for a destination country.
    pub async fn serviceability(
        &self,
        query: &InternationalServiceabilityQuery,
    ) -> Result<InternationalServiceability, Error> {
        self.client
            .get_query(
                self.client.api_url("international/courier/serviceability"),
                query,
            )
            .await
    }

    /// `POST international/courier/assign/awb`: assign an AWB to a shipment.
    pub async fn assign_awb(
        &self,
        req: &InternationalAssignAwbRequest,
    ) -> Result<InternationalAssignAwbResponse, Error> {
        self.client
            .post(self.client.api_url("international/courier/assign/awb"), req)
            .await
    }

    /// `POST international/manifests/generate`.
    pub async fn generate_manifest(
        &self,
        req: &InternationalManifestRequest,
    ) -> Result<InternationalManifestResponse, Error> {
        self.client
            .post(self.client.api_url("international/manifests/generate"), req)
            .await
    }

    /// `POST courier/generate/pickup`: request pickup of an international shipment.
    pub async fn generate_pickup(
        &self,
        req: &InternationalPickupRequest,
    ) -> Result<InternationalPickupResponse, Error> {
        self.client
            .post(self.client.api_url("courier/generate/pickup"), req)
            .await
    }

    /// `GET international/orders/track`: international orders with their shipments and status.
    pub async fn track_orders(&self) -> Result<Paginated<InternationalTrackedOrder>, Error> {
        self.client
            .get(self.client.api_url("international/orders/track"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_kyc_and_bank_responses() {
        let res: MessageResponse = serde_json::from_str(
            r#"{"success" : true, "message" : "Document is successfully updated!"}"#,
        )
        .unwrap();
        assert_eq!(res.success, Some(true));

        let res: MessageResponse = serde_json::from_str(
            r#"{"success":true,"status_code":200,"errors":[],"message":"BankDetailsissuccessfullyupdated!"}"#,
        )
        .unwrap();
        assert_eq!(res.status_code, Some(200));
    }

    #[test]
    fn serializes_order_request_with_renamed_fields() {
        let req = InternationalOrderRequest::builder()
            .order_id("1")
            .order_date("2022-03-30")
            .billing_customer_name("Elena")
            .billing_city("Panchkula")
            .billing_pincode("134113")
            .billing_state("Haryana")
            .billing_country("India")
            .billing_email("e@example.com")
            .billing_phone("9762343722")
            .shipping_is_billing(true)
            .order_items(vec![
                InternationalOrderItem::builder()
                    .name("Spanner")
                    .sku("5-47606")
                    .units(1)
                    .selling_price(100.0)
                    .build(),
            ])
            .payment_method(PaymentMethod::Prepaid)
            .sub_total(100.0)
            .length(10.0)
            .breadth(10.0)
            .height(10.0)
            .weight(0.41)
            .currency("USD")
            .reason_of_export(2)
            .igst_payment_status("C")
            .terms_of_invoice("FOB")
            .build();
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["reasonOfExport"], 2);
        assert_eq!(v["igstPaymentStatus"], "C");
        assert_eq!(v["Terms_Of_Invoice"], "FOB");
        assert_eq!(v["payment_method"], "Prepaid");
        assert!(v.get("shipping_city").is_none());
    }

    #[test]
    fn decodes_create_and_update() {
        let res: InternationalOrderCreated = serde_json::from_str(
            r#"{"order_id":153210169,"shipment_id":152757155,"status":"NEW","status_code":1,"onboarding_completed_now":0,"awb_code":"","courier_company_id":"","courier_name":""}"#,
        )
        .unwrap();
        assert_eq!(res.order_id, Some(153210169));
        assert_eq!(res.awb_code, None);
        assert_eq!(res.courier_company_id, None);

        let res: InternationalOrderUpdated = serde_json::from_str(
            r#"{"success":true,"partially_update":true,"not_updated_fields":"isd_code ,billing_isd_code","order_id":153210169,"shipment_id":152757155,"new_order_status":"NEW","old_order_status":1,"awb_code":"","courier_company_id":"","courier_name":""}"#,
        )
        .unwrap();
        assert_eq!(res.partially_update, Some(true));
        assert_eq!(res.old_order_status, Some(1));
    }

    #[test]
    fn decodes_forward_shipment() {
        let res: InternationalForwardShipment = serde_json::from_str(
            r#"{"pickup_location_added":0,"order_created":1,"awb_generated":1,"label_generated":1,"pickup_generated":1,"manifest_generated":1,"pickup_scheduled_date":"2022-12-16 09:00:00","pickup_booked_date":null,"order_id":53861,"shipment_id":52367,"awb_code":"8329468061579","courier_company_id":140,"courier_name":"Shiprocket Premium","assigned_date_time":"2023-10-10T08:14:00.278336Z","applied_weight":1,"cod":0,"label_url":"https://example.com/label.pdf","manifest_url":"https://example.com/manifest.pdf","routing_code":"","rto_routing_code":"","pickup_token_number":102725472}"#,
        )
        .unwrap();
        assert_eq!(res.order_created, Some(true));
        assert_eq!(res.pickup_location_added, Some(false));
        assert_eq!(res.awb_code.as_deref(), Some("8329468061579"));
        assert_eq!(res.pickup_token_number.as_deref(), Some("102725472"));
    }

    #[test]
    fn decodes_serviceability() {
        let res: InternationalServiceability = serde_json::from_str(
            r#"{"status":200,"data":{"is_recommendation_enabled":1,"recommended_by":{"id":6,"title":"Recommendation By Shiprocket"},"child_courier_id":null,"recommended_courier_company_id":140,"shiprocket_recommended_courier_id":140,"available_courier_companies":[{"courier_company_id":140,"courier_name":"SRX Premium","mode":1,"min_weight":0.05,"charge_weight":0.5,"is_international":1,"rating":4.7,"blocked":0,"etd":"Feb 17, 2024 - Feb 22, 2024","estimated_delivery_days":"10 - 15","etd_hours":360,"rate":{"courier_id":140,"id":9314109,"rate":"108.01","zone_rates":{"roc":"109.01","default":"108.01"},"zone":"default"}},{"courier_company_id":326,"rate":{"rate":2330.6,"first_mile_charge":6,"last_mile_charge":"2324.6","total":2330.6}}]}}"#,
        )
        .unwrap();
        let data = res.data.unwrap();
        assert_eq!(data.recommended_courier_company_id, Some(140));
        let couriers = &data.available_courier_companies;
        assert_eq!(couriers.len(), 2);
        assert_eq!(couriers[0].rate.as_ref().unwrap().rate, Some(108.01));
        assert_eq!(
            couriers[1].rate.as_ref().unwrap().last_mile_charge,
            Some(2324.6)
        );
    }

    #[test]
    fn decodes_assign_awb() {
        let res: InternationalAssignAwbResponse = serde_json::from_str(
            r#"{"awb_assign_status":1,"response":{"data":{"courier_company_id":10,"awb_code":"1091208940593","cod":0,"order_id":181771297,"shipment_id":160169474,"awb_code_status":1,"assigned_date_time":{"date":"2022-05-10 11:18:37.397226","timezone_type":3,"timezone":"Asia/Kolkata"},"applied_weight":1,"company_id":25149,"courier_name":"Delhivery","child_courier_name":null,"routing_code":"DEL/KIS","shipped_by":{"shipper_company_name":"New RtO"}}}}"#,
        )
        .unwrap();
        let data = res.response.unwrap().data.unwrap();
        assert_eq!(res.awb_assign_status, Some(1));
        assert_eq!(data.awb_code.as_deref(), Some("1091208940593"));
        assert_eq!(
            data.assigned_date_time.unwrap().timezone.as_deref(),
            Some("Asia/Kolkata")
        );

        let res: InternationalAssignAwbResponse = serde_json::from_str(
            r#"{"awb_assign_status":0,"response":{"data":{"awb_assign_error":"Awb Assignment Failed"}}}"#,
        )
        .unwrap();
        let data = res.response.unwrap().data.unwrap();
        assert_eq!(
            data.awb_assign_error.as_deref(),
            Some("Awb Assignment Failed")
        );
    }

    #[test]
    fn decodes_manifest_and_pickup() {
        let res: InternationalManifestResponse = serde_json::from_str(
            r#"{"status":1,"manifest_url":"https://example.com/MANIFEST-3051.pdf"}"#,
        )
        .unwrap();
        assert_eq!(res.status, Some(1));

        let res: InternationalManifestResponse =
            serde_json::from_str(r#"{"message":"Manifest not generated","check_ids":[152757105]}"#)
                .unwrap();
        assert_eq!(res.check_ids, vec![152757105]);

        let res: InternationalPickupResponse = serde_json::from_str(
            r#"{"pickup_token_number":"12345","pickup_scheduled_date":"2022-05-17","pickup_generated":1,"shipment_id":152757127,"manifest_generated":1,"manifest_url":"https://example.com/m.pdf"}"#,
        )
        .unwrap();
        assert_eq!(res.pickup_generated, Some(true));
        assert_eq!(res.shipment_id, Some(152757127));
    }

    #[test]
    fn decodes_tracked_orders() {
        let res: Paginated<InternationalTrackedOrder> = serde_json::from_str(
            r#"{"data":[{"id":153210141,"channel_id":2252386,"channel_name":"CUSTOM","channel_order_id":"7022318826","customer_city":"UPTON","customer_pincode":"11973","customer_country":"United States","total":"1000.00","tax":"0.00","status":"DELIVERED","status_code":7,"master_status":"FULFILLED","payment_method":"prepaid","is_international":1,"products":[{"id":212990291,"channel_order_product_id":"212990291","name":"Tshirt","channel_sku":"sku300","quantity":1,"price":"1000.00","hsn":""}],"shipments":[{"id":152757127,"courier":"SR International","courier_id":140,"weight":"0.45","dimensions":"10x10x10","awb":"1527571275","volumetric_weight":0.2,"etd":"NA","delivered_date":"2021-12-08 05:40:00","status":7}],"activities":["ORDER_CREATED","ORDER_DELIVERED"],"others":{"currency_code":"INR"}}]}"#,
        )
        .unwrap();
        let order = &res.data[0];
        assert_eq!(order.total, Some(1000.0));
        assert_eq!(order.is_international, Some(true));
        assert_eq!(order.products[0].hsn, None);
        assert_eq!(order.shipments[0].awb.as_deref(), Some("1527571275"));
        assert_eq!(order.activities.len(), 2);
        assert_eq!(order.others["currency_code"], "INR");
    }
}
