//! Couriers: AWB assignment, courier list, serviceability, pickup requests and blocked pincodes.

use bon::Builder;
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    Error, Shiprocket, de,
    types::{DataResponse, MessageResponse, PhpDateTime},
};

/// Shiprocket serializes `Option<bool>` flags as `1` / `0`.
fn bool_as_int<S: Serializer>(v: &Option<bool>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        Some(b) => s.serialize_u8(*b as u8),
        None => s.serialize_none(),
    }
}

/// Body of `POST courier/assign/awb`.
///
/// ```
/// let req = shiprocket::couriers::AssignAwbRequest::builder()
///     .shipment_id(16016920)
///     .courier_id(10)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct AssignAwbRequest {
    pub shipment_id: i64,
    /// Courier to use. Shiprocket picks the default courier when `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub courier_id: Option<i64>,
    /// `reassign` to change the courier of a shipment (once per 24 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Response of `POST courier/assign/awb`.
///
/// Hyperlocal shipments get `{"success": true, "message": "..."}` instead of the AWB data.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AssignAwbResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub awb_assign_status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub response: Option<AssignAwbResponseBody>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AssignAwbResponseBody {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<AwbAssignment>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// AWB details of a shipment after assignment.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AwbAssignment {
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
    /// Weight in kg used for billing.
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub child_courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_scheduled_date: Option<String>,
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
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub shipped_by: Option<ShippedBy>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Shipper and RTO address of an AWB assignment.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShippedBy {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_company_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_address_1: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_address_2: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_city: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_state: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_country: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_postcode: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub shipper_first_mile_activated: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_phone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipper_email: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub lat: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub long: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_company_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_address_1: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_address_2: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_city: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_state: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_country: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_postcode: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_phone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_email: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Filter for `GET courier/courierListWithCounts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CourierListType {
    Active,
    Inactive,
    All,
}

/// Response of `GET courier/courierListWithCounts`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CourierList {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub total_courier_count: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub serviceable_pincodes_count: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub pickup_pincodes_count: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub total_rto_count: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub total_oda_count: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_count: Option<i64>,
    #[serde(default, deserialize_with = "de::vec")]
    pub courier_data: Vec<CourierInfo>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A courier available to the account.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CourierInfo {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub base_courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub master_company: Option<String>,
    /// Minimum chargeable weight in kg.
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub min_weight: Option<f64>,
    #[serde(default, rename = "type", deserialize_with = "de::opt_i64")]
    pub r#type: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_type: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub service_type: Option<i64>,
    /// `0` surface, `1` air.
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub mode: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_hyperlocal: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_own_key_courier: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub ownkey_courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub use_sr_postcodes: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub image: Option<CourierImage>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub realtime_tracking: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivery_boy_contact: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pod_available: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub call_before_delivery: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub activated_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub newest_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_count: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CourierImage {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub logo: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub small_logo: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub email_logo_s3_path: Option<String>,
}

/// Query of `GET courier/serviceability/`.
///
/// Either `order_id`, or `cod` and `weight`, must be set.
///
/// ```
/// let query = shiprocket::couriers::ServiceabilityQuery::builder()
///     .pickup_postcode("110030")
///     .delivery_postcode("122002")
///     .cod(false)
///     .weight(2.0)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ServiceabilityQuery {
    pub pickup_postcode: String,
    pub delivery_postcode: String,
    /// Shiprocket order id. Replaces `cod` and `weight`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// Cash on delivery (`true`) or prepaid (`false`).
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_as_int"
    )]
    pub cod: Option<bool>,
    /// Weight in kg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    /// Length in cm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    /// Breadth in cm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadth: Option<f64>,
    /// Height in cm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// Order value in INR. Required with `is_return`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_value: Option<f64>,
    /// `Surface` or `Air`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_as_int"
    )]
    pub is_return: Option<bool>,
    /// `1` to show only document couriers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub couriers_type: Option<i64>,
    /// Only hyperlocal couriers.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_as_int"
    )]
    pub only_local: Option<bool>,
    /// Only QC enabled couriers. Requires `is_return`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_as_int"
    )]
    pub qc_check: Option<bool>,
}

/// Response of `GET courier/serviceability/`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ServiceabilityResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub currency: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<ServiceabilityData>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub company_auto_shipment_insurance_setting: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub eligible_for_insurance: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ServiceabilityData {
    #[serde(default, deserialize_with = "de::vec")]
    pub available_courier_companies: Vec<AvailableCourier>,
    #[serde(default, deserialize_with = "de::vec")]
    pub blocked_courier_companies: Vec<BlockedCourier>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub recommended_courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shiprocket_recommended_courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub child_courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_recommendation_enabled: Option<bool>,
    #[serde(default)]
    pub recommended_by: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A courier that can serve the route, with its rate and ETA.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AvailableCourier {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    /// Total rate in INR.
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rate: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub freight_charge: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub cod_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub coverage_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub other_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rto_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charge_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub min_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub air_max_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub surface_max_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub estimated_delivery_days: Option<i64>,
    /// Estimated delivery date, e.g. `Jul 01, 2024`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub etd: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub etd_hours: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub cutoff_time: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub cod: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_surface: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_hyperlocal: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_international: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub blocked: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub odablock: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub qc_courier: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub mode: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rating: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub delivery_performance: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub pickup_performance: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rto_performance: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub tracking_performance: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub state: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub postcode: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub zone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub realtime_tracking: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pod_available: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub call_before_delivery: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivery_boy_contact: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct BlockedCourier {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub block_reason: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub postcode: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Query of `GET courier/serviceability/` for hyperlocal shipments.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct HyperlocalServiceabilityQuery {
    pub pickup_postcode: String,
    pub delivery_postcode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_as_int"
    )]
    pub cod: Option<bool>,
    /// Weight in kg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    pub lat_from: f64,
    pub long_from: f64,
    pub lat_to: f64,
    pub long_to: f64,
    /// Always `1`.
    #[builder(skip = 1)]
    pub is_new_hyperlocal: u8,
}

/// Response of a hyperlocal serviceability check.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct HyperlocalServiceabilityResponse {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub status: Option<bool>,
    #[serde(default, deserialize_with = "de::vec")]
    pub data: Vec<HyperlocalRate>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct HyperlocalRate {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    /// Rate in INR.
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rates: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Body of `POST courier/generate/pickup`.
///
/// ```
/// let req = shiprocket::couriers::GeneratePickupRequest::builder()
///     .shipment_id(vec![16090109])
///     .pickup_date(vec!["2022-06-04".into()])
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct GeneratePickupRequest {
    /// Exactly one shipment id with an AWB assigned.
    pub shipment_id: Vec<i64>,
    /// `retry` to retry a failed pickup request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Future pickup dates as `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_date: Option<Vec<String>>,
}

/// Response of `POST courier/generate/pickup`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct GeneratePickupResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub pickup_status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub response: Option<PickupResponseBody>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PickupResponseBody {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_scheduled_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_token_number: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    /// JSON encoded metadata string.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub others: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub pickup_generated_date: Option<PhpDateTime>,
    /// Human readable confirmation message.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub data: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Action of `POST blocked-pincodes/upload`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PincodeBlockAction {
    Block,
    Unblock,
}

/// Body of `POST blocked-pincodes/upload`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct UpdateBlockedPincodesRequest {
    pub postcode: BlockedPostcodes,
    pub action: PincodeBlockAction,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BlockedPostcodes {
    pub delivery_blocked: Vec<String>,
}

/// Query of `GET block-pincodes/get`.
///
/// `is_download` wins over `search`, which wins over pagination.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct BlockedPincodesQuery {
    /// Return a presigned link to a CSV of all pincodes instead of a list.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_as_int"
    )]
    pub is_download: Option<bool>,
    /// Prefix search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// 1 to 15, defaults to 15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i64>,
}

/// `data` of `GET block-pincodes/get`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BlockedPincodes {
    #[serde(default, deserialize_with = "de::vec")]
    pub delivery_blocked: Vec<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub total: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub per_page: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub current_page: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub last_page: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Courier endpoints. Obtain with [`Shiprocket::couriers`].
pub struct Couriers<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn couriers(&self) -> Couriers<'_> {
        Couriers { client: self }
    }
}

impl Couriers<'_> {
    /// `POST courier/assign/awb`: assign an AWB (and courier) to a shipment.
    pub async fn assign_awb(&self, req: &AssignAwbRequest) -> Result<AssignAwbResponse, Error> {
        self.client
            .post(self.client.api_url("courier/assign/awb"), req)
            .await
    }

    /// `GET courier/courierListWithCounts`: couriers of the account, optionally filtered.
    pub async fn list(&self, filter: Option<CourierListType>) -> Result<CourierList, Error> {
        let url = self.client.api_url("courier/courierListWithCounts");
        match filter {
            Some(t) => self.client.get_query(url, &[("type", t)]).await,
            None => self.client.get(url).await,
        }
    }

    /// `GET courier/serviceability/`: couriers, rates and ETAs between two pincodes.
    pub async fn serviceability(
        &self,
        query: &ServiceabilityQuery,
    ) -> Result<ServiceabilityResponse, Error> {
        self.client
            .get_query(self.client.api_url("courier/serviceability/"), query)
            .await
    }

    /// `GET courier/serviceability/` with `is_new_hyperlocal=1`: hyperlocal couriers and rates.
    pub async fn hyperlocal_serviceability(
        &self,
        query: &HyperlocalServiceabilityQuery,
    ) -> Result<HyperlocalServiceabilityResponse, Error> {
        self.client
            .get_query(self.client.api_url("courier/serviceability/"), query)
            .await
    }

    /// `POST courier/generate/pickup`: request a pickup for a shipment.
    pub async fn generate_pickup(
        &self,
        req: &GeneratePickupRequest,
    ) -> Result<GeneratePickupResponse, Error> {
        self.client
            .post(self.client.api_url("courier/generate/pickup"), req)
            .await
    }

    /// `POST blocked-pincodes/upload` (serviceability host): add or remove delivery blocked pincodes.
    pub async fn update_blocked_pincodes(
        &self,
        req: &UpdateBlockedPincodesRequest,
    ) -> Result<MessageResponse, Error> {
        self.client
            .post(
                self.client.serviceability_url("blocked-pincodes/upload"),
                req,
            )
            .await
    }

    /// Shorthand for [`update_blocked_pincodes`](Self::update_blocked_pincodes) with `block`.
    pub async fn block_pincodes<I, S>(&self, pincodes: I) -> Result<MessageResponse, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.update_blocked_pincodes(&blocked_request(pincodes, PincodeBlockAction::Block))
            .await
    }

    /// Shorthand for [`update_blocked_pincodes`](Self::update_blocked_pincodes) with `unblock`.
    pub async fn unblock_pincodes<I, S>(&self, pincodes: I) -> Result<MessageResponse, Error>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.update_blocked_pincodes(&blocked_request(pincodes, PincodeBlockAction::Unblock))
            .await
    }

    /// `GET block-pincodes/get` (serviceability host): the delivery blocked pincode list.
    pub async fn blocked_pincodes(
        &self,
        query: &BlockedPincodesQuery,
    ) -> Result<DataResponse<BlockedPincodes>, Error> {
        self.client
            .get_query(self.client.serviceability_url("block-pincodes/get"), query)
            .await
    }
}

fn blocked_request<I, S>(pincodes: I, action: PincodeBlockAction) -> UpdateBlockedPincodesRequest
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    UpdateBlockedPincodesRequest {
        postcode: BlockedPostcodes {
            delivery_blocked: pincodes.into_iter().map(Into::into).collect(),
        },
        action,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_awb_response() {
        let r: AssignAwbResponse = serde_json::from_str(
            r#"{"awb_assign_status":1,"response":{"data":{"courier_company_id":142,"awb_code":"321055706540","cod":0,"order_id":281248157,"shipment_id":16090281,"awb_code_status":1,"assigned_date_time":{"date":"2022-11-25 11:17:52.878599","timezone_type":3,"timezone":"Asia/Kolkata"},"applied_weight":0.5,"company_id":25149,"courier_name":"Amazon Surface","child_courier_name":null,"pickup_scheduled_date":"2022-11-25 14:00:00","routing_code":"","invoice_no":"retail5769122647118","shipped_by":{"shipper_company_name":"manoj","shipper_postcode":"110030","shipper_first_mile_activated":0,"lat":"32.731899","long":"74.860376","rto_postcode":"560060"}}}}"#,
        )
        .unwrap();
        assert_eq!(r.awb_assign_status, Some(1));
        let d = r.response.unwrap().data.unwrap();
        assert_eq!(d.awb_code.as_deref(), Some("321055706540"));
        assert_eq!(d.cod, Some(false));
        assert_eq!(d.shipment_id, Some(16090281));
        assert_eq!(d.routing_code, None);
        assert_eq!(
            d.assigned_date_time.unwrap().timezone.as_deref(),
            Some("Asia/Kolkata")
        );
        let s = d.shipped_by.unwrap();
        assert_eq!(s.lat, Some(32.731899));
        assert_eq!(s.shipper_postcode.as_deref(), Some("110030"));

        let r: AssignAwbResponse =
            serde_json::from_str(r#"{"success":true,"message":"We are processing your request"}"#)
                .unwrap();
        assert_eq!(r.success, Some(true));
    }

    #[test]
    fn courier_list() {
        let r: CourierList = serde_json::from_str(
            r#"{"total_courier_count":72,"courier_count":12,"courier_data":[{"is_own_key_courier":0,"ownkey_courier_id":0,"id":53,"min_weight":1,"base_courier_id":53,"name":"Gati Surface 1 Kg","use_sr_postcodes":1,"type":1,"status":1,"courier_type":0,"master_company":"Gati Surface 1 Kg","service_type":1,"mode":0,"image":{"logo":"post_order/img/courier/GATI-SURFACE.png","small_logo":"post_order/img/courier/thumb/gati.jpg","email_logo_s3_path":"gati.png"},"realtime_tracking":"Real Time","activated_date":"2021-10-14","newest_date":null,"shipment_count":"","is_hyperlocal":0}]}"#,
        )
        .unwrap();
        assert_eq!(r.total_courier_count, Some(72));
        let c = &r.courier_data[0];
        assert_eq!(c.id, Some(53));
        assert_eq!(c.min_weight, Some(1.0));
        assert_eq!(c.r#type, Some(1));
        assert_eq!(c.shipment_count, None);
        assert_eq!(c.is_hyperlocal, Some(false));
        assert_eq!(
            c.image.as_ref().unwrap().email_logo_s3_path.as_deref(),
            Some("gati.png")
        );
    }

    #[test]
    fn serviceability_response() {
        let r: ServiceabilityResponse = serde_json::from_str(
            r#"{"company_auto_shipment_insurance_setting":true,"covid_zones":{"delivery_zone":null,"pickup_zone":null},"currency":"INR","data":{"available_courier_companies":[{"air_max_weight":"0.00","blocked":0,"charge_weight":0.5,"city":"Mandi","cod":1,"cod_charges":0,"courier_company_id":43,"courier_name":"Delhivery Surface","courier_type":"0","estimated_delivery_days":"4","etd":"Jul 01, 2024","etd_hours":91,"freight_charge":54,"id":459245934,"is_hyperlocal":false,"is_surface":true,"rate":54,"rating":4.9,"postcode":"175019","surface_max_weight":"4.00","zone":"z_e"}],"blocked_courier_companies":[{"block_reason":"Operational Issues","courier_company_id":33,"courier_name":"Xpressbees Air","postcode":"175019"}],"child_courier_id":null,"is_recommendation_enabled":1,"recommended_by":{"id":6,"title":"Recommendation By Shiprocket"},"recommended_courier_company_id":43,"shiprocket_recommended_courier_id":43},"dg_courier":0,"seller_address":[],"status":200}"#,
        )
        .unwrap();
        assert_eq!(r.status, Some(200));
        assert!(r.extra.contains_key("dg_courier"));
        let d = r.data.unwrap();
        assert_eq!(d.recommended_courier_company_id, Some(43));
        assert_eq!(d.is_recommendation_enabled, Some(true));
        let c = &d.available_courier_companies[0];
        assert_eq!(c.courier_company_id, Some(43));
        assert_eq!(c.rate, Some(54.0));
        assert_eq!(c.estimated_delivery_days, Some(4));
        assert_eq!(c.surface_max_weight, Some(4.0));
        assert_eq!(c.cod, Some(true));
        assert_eq!(
            d.blocked_courier_companies[0].block_reason.as_deref(),
            Some("Operational Issues")
        );
    }

    #[test]
    fn hyperlocal_serviceability() {
        let r: HyperlocalServiceabilityResponse = serde_json::from_str(
            r#"{"status":true,"data":[{"courier_name":"Shiprocket Quick","rates":"345.3"}]}"#,
        )
        .unwrap();
        assert_eq!(r.status, Some(true));
        assert_eq!(r.data[0].rates, Some(345.3));
    }

    #[test]
    fn generate_pickup_response() {
        let r: GeneratePickupResponse = serde_json::from_str(
            r#"{"pickup_status":1,"response":{"pickup_scheduled_date":"2021-12-10 12:39:54","pickup_token_number":"Reference No: 194_BIGFOOT 1966840_11122021","status":3,"others":"{\"tier_id\":5}","pickup_generated_date":{"date":"2021-12-10 12:39:54.034695","timezone_type":3,"timezone":"Asia/Kolkata"},"data":"Pickup is confirmed by Xpressbees 1kg For AWB :- 143254213727423"}}"#,
        )
        .unwrap();
        assert_eq!(r.pickup_status, Some(1));
        let b = r.response.unwrap();
        assert_eq!(b.status, Some(3));
        assert!(b.data.unwrap().starts_with("Pickup is confirmed"));
        assert!(b.pickup_generated_date.is_some());
    }

    #[test]
    fn blocked_pincodes() {
        let r: DataResponse<BlockedPincodes> = serde_json::from_str(
            r#"{"data":{"delivery_blocked":["110006","110007"],"total":42,"per_page":5,"current_page":2,"last_page":9}}"#,
        )
        .unwrap();
        let d = r.data.unwrap();
        assert_eq!(d.delivery_blocked, vec!["110006", "110007"]);
        assert_eq!(d.last_page, Some(9));

        let body =
            serde_json::to_value(blocked_request(["110001"], PincodeBlockAction::Block)).unwrap();
        assert_eq!(
            body,
            serde_json::json!({"postcode":{"delivery_blocked":["110001"]},"action":"block"})
        );
    }

    #[test]
    fn query_serialization() {
        let q = ServiceabilityQuery::builder()
            .pickup_postcode("110030")
            .delivery_postcode("122002")
            .cod(true)
            .weight(2.0)
            .mode("Air")
            .build();
        let v = serde_json::to_value(&q).unwrap();
        assert_eq!(v["cod"], 1);
        assert_eq!(v["mode"], "Air");
        assert!(v.get("order_id").is_none());

        let q = HyperlocalServiceabilityQuery::builder()
            .pickup_postcode("110030")
            .delivery_postcode("110031")
            .lat_from(28.5)
            .long_from(77.0)
            .lat_to(28.6)
            .long_to(77.1)
            .build();
        assert_eq!(serde_json::to_value(&q).unwrap()["is_new_hyperlocal"], 1);
    }
}
