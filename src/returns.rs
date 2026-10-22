//! Return & exchange orders: create, edit and list return orders, create exchange orders,
//! and the return flavours of courier serviceability and AWB assignment.

use bon::Builder;
use serde::{Deserialize, Serialize, Serializer};

use crate::{Error, Paginated, PaymentMethod, Shiprocket, de};

/// Body of `POST orders/create/return`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct CreateReturnRequest {
    /// Your reference for the return order, max 50 characters.
    pub order_id: String,
    /// `yyyy-mm-dd`, optionally followed by `hh:mm`.
    pub order_date: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,

    /// Customer the package is picked up from.
    pub pickup_customer_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_last_name: Option<String>,
    pub pickup_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_address_2: Option<String>,
    pub pickup_city: String,
    pub pickup_state: String,
    pub pickup_country: String,
    pub pickup_pincode: String,
    pub pickup_email: String,
    pub pickup_phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_isd_code: Option<String>,

    /// Seller the package is shipped back to.
    pub shipping_customer_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_last_name: Option<String>,
    pub shipping_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<String>,
    pub shipping_city: String,
    pub shipping_state: String,
    pub shipping_country: String,
    pub shipping_pincode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_email: Option<String>,
    pub shipping_phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_isd_code: Option<String>,

    pub order_items: Vec<ReturnOrderItem>,
    /// Shiprocket documents this as always `Prepaid` for returns.
    pub payment_method: PaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<f64>,
    pub sub_total: f64,
    /// cm
    pub length: f64,
    /// cm
    pub breadth: f64,
    /// cm
    pub height: f64,
    /// kg
    pub weight: f64,
}

/// An item of a return order, with optional quality check (QC) details.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ReturnOrderItem {
    pub name: String,
    pub sku: String,
    pub units: i64,
    /// Per unit, inclusive of GST.
    pub selling_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn: Option<String>,
    /// Free text or one of the documented reasons, e.g. `Bought by Mistake`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_reason: Option<String>,
    /// Perform QC at pickup. Only one SKU per order can be QC'd.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_serial_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_ean_barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_size: Option<String>,
    /// Required when `qc_enable` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_product_name: Option<String>,
    /// png/jpg url. Required when `qc_enable` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_product_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_product_imei: Option<String>,
    /// Check the brand tag matches. Sent as `0`/`1`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub qc_brand_tag: Option<bool>,
    /// Check for signs of usage. Sent as `0`/`1`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub qc_used_check: Option<bool>,
    /// Check the seal tag is intact. Sent as `0`/`1`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub qc_sealtag_check: Option<bool>,
    /// Check for damage. Sent as `yes`/`no`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_yes_no"
    )]
    pub qc_check_damaged_product: Option<bool>,
}

/// Response of `POST orders/create/return`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateReturnResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub company_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Body of `POST orders/create/exchange`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct CreateExchangeRequest {
    /// Your reference for the new forward (exchange) order.
    pub exchange_order_id: String,
    /// Your reference for the return order.
    pub return_order_id: String,
    pub seller_pickup_location_id: String,
    pub seller_shipping_location_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_order_id: Option<String>,
    /// `yyyy-mm-dd`
    pub order_date: String,
    pub payment_method: PaymentMethod,
    /// Documented as one of the return reasons, e.g. `29`.
    pub return_reason: String,

    pub buyer_shipping_first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_shipping_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_shipping_email: Option<String>,
    pub buyer_shipping_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_shipping_address_2: Option<String>,
    pub buyer_shipping_city: String,
    pub buyer_shipping_state: String,
    pub buyer_shipping_country: String,
    pub buyer_shipping_pincode: String,
    pub buyer_shipping_phone: String,

    pub buyer_pickup_first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_pickup_last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_pickup_email: Option<String>,
    pub buyer_pickup_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_pickup_address_2: Option<String>,
    pub buyer_pickup_city: String,
    pub buyer_pickup_state: String,
    pub buyer_pickup_country: String,
    pub buyer_pickup_pincode: String,
    pub buyer_pickup_phone: String,

    pub order_items: Vec<ExchangeOrderItem>,
    pub sub_total: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_charges: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giftwrap_charges: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_charges: Option<f64>,

    /// cm
    pub return_length: f64,
    /// cm
    pub return_breadth: f64,
    /// cm
    pub return_height: f64,
    /// kg
    pub return_weight: f64,
    /// cm
    pub exchange_length: f64,
    /// cm
    pub exchange_breadth: f64,
    /// cm
    pub exchange_height: f64,
    /// kg
    pub exchange_weight: f64,
    /// Enable QC for the return leg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_check: Option<bool>,
}

/// An item of an exchange order: the returned product and its replacement.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ExchangeOrderItem {
    pub name: String,
    pub sku: String,
    pub units: i64,
    pub selling_price: f64,
    pub hsn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_item_id: Option<String>,
    pub exchange_item_name: String,
    pub exchange_item_sku: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_enable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_product_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_product_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessories: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub qc_used_check: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub qc_sealtag_check: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub qc_brand_box: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_yes_no"
    )]
    pub qc_check_damaged_product: Option<bool>,
}

/// Response of `POST orders/create/exchange`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateExchangeResponse {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<ExchangeOrders>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// The two orders created by an exchange.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ExchangeOrders {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub forward_orders: Option<ExchangeOrderSummary>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub return_orders: Option<ExchangeOrderSummary>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ExchangeOrderSummary {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_order_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// What an [`UpdateReturnRequest`] changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateReturnAction {
    /// Edit weight and dimensions.
    ProductDetails,
    /// Change the return (warehouse) address.
    WarehouseAddress,
}

/// Body of `POST orders/edit`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct UpdateReturnRequest {
    /// The return order id.
    pub order_id: String,
    pub action: Vec<UpdateReturnAction>,
    /// cm, > 0.5. Required for [`UpdateReturnAction::ProductDetails`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    /// cm, > 0.5. Required for [`UpdateReturnAction::ProductDetails`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadth: Option<f64>,
    /// cm, > 0.5. Required for [`UpdateReturnAction::ProductDetails`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// kg, > 0. Required for [`UpdateReturnAction::ProductDetails`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    /// Pickup location id. Required for [`UpdateReturnAction::WarehouseAddress`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_warehouse_id: Option<i64>,
}

/// Response of `POST orders/edit`: one result per requested action.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateReturnResponse {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub product_details: Option<UpdateReturnResult>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub return_warehouse_address: Option<UpdateReturnResult>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateReturnResult {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub msg: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Query of `GET orders/processing/return`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ListReturnsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    /// Start date, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// End date, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

/// A return order as listed by `GET orders/processing/return`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReturnOrder {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub channel_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub base_channel_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_order_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_email: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_phone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub customer_pincode: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_location: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub total: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sla: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipping_method: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_method: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_international: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::vec")]
    pub products: Vec<ReturnOrderProduct>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivery_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub cod: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::vec")]
    pub shipments: Vec<ReturnOrderShipment>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReturnOrderProduct {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_order_product_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub product_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub hsn: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReturnOrderShipment {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub isd_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sr_courier_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub length: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub breadth: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub height: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub volumetric_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Query of `GET courier/serviceability/` for a return shipment (`is_return=1` is added for you).
///
/// Pass either `order_id`, or `cod` + `weight`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ReturnServiceabilityQuery {
    /// Customer (pickup) pincode.
    pub pickup_postcode: String,
    /// Seller (delivery) pincode.
    pub delivery_postcode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub cod: Option<bool>,
    /// kg
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breadth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declared_value: Option<f64>,
    /// `Surface` or `Air`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Only list couriers that support QC.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "opt_bool_as_int"
    )]
    pub qc_check: Option<bool>,
}

/// Response of the return serviceability check.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReturnServiceabilityResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub currency: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<ReturnServiceabilityData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReturnServiceabilityData {
    #[serde(default, deserialize_with = "de::vec")]
    pub available_courier_companies: Vec<ReturnCourierOption>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub recommended_courier_company_id: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A courier able to pick up the return.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReturnCourierOption {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rate: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub freight_charge: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub estimated_delivery_days: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub etd: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charge_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub qc_courier: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_surface: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub rating: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Body of `POST courier/assign/awb` for a return shipment (`is_return=1` is added for you).
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct AssignReturnAwbRequest {
    pub shipment_id: i64,
    /// Courier to use. Shiprocket picks the default when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub courier_id: Option<i64>,
    /// `reassign` to change the courier (once every 24 hours).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Response of the return AWB assignment.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AssignReturnAwbResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub awb_assign_status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub response: Option<AssignReturnAwbEnvelope>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AssignReturnAwbEnvelope {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<ReturnAwbData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReturnAwbData {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_scheduled_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub routing_code: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize)]
struct WithIsReturn<'a, T> {
    #[serde(flatten)]
    inner: &'a T,
    is_return: u8,
}

fn opt_bool_as_int<S: Serializer>(v: &Option<bool>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        Some(b) => s.serialize_u8(*b as u8),
        None => s.serialize_none(),
    }
}

fn opt_bool_as_yes_no<S: Serializer>(v: &Option<bool>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        Some(true) => s.serialize_str("yes"),
        Some(false) => s.serialize_str("no"),
        None => s.serialize_none(),
    }
}

/// Return & exchange endpoints. Obtain with [`Shiprocket::returns`].
pub struct Returns<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn returns(&self) -> Returns<'_> {
        Returns { client: self }
    }
}

impl Returns<'_> {
    /// `POST orders/create/return`: create a return order.
    pub async fn create(&self, req: &CreateReturnRequest) -> Result<CreateReturnResponse, Error> {
        self.client
            .post(self.client.api_url("orders/create/return"), req)
            .await
    }

    /// `POST orders/create/exchange`: create an exchange (a return plus a forward order).
    pub async fn create_exchange(
        &self,
        req: &CreateExchangeRequest,
    ) -> Result<CreateExchangeResponse, Error> {
        self.client
            .post(self.client.api_url("orders/create/exchange"), req)
            .await
    }

    /// `POST orders/edit`: update dimensions/weight or the warehouse of a return order.
    pub async fn update(&self, req: &UpdateReturnRequest) -> Result<UpdateReturnResponse, Error> {
        self.client
            .post(self.client.api_url("orders/edit"), req)
            .await
    }

    /// `GET orders/processing/return`: list return orders.
    pub async fn list(&self, query: &ListReturnsQuery) -> Result<Paginated<ReturnOrder>, Error> {
        self.client
            .get_query(self.client.api_url("orders/processing/return"), query)
            .await
    }

    /// `GET courier/serviceability/?is_return=1`: couriers that can pick up a return.
    pub async fn serviceability(
        &self,
        query: &ReturnServiceabilityQuery,
    ) -> Result<ReturnServiceabilityResponse, Error> {
        let query = WithIsReturn {
            inner: query,
            is_return: 1,
        };
        self.client
            .get_query(self.client.api_url("courier/serviceability/"), &query)
            .await
    }

    /// `POST courier/assign/awb` with `is_return=1`: assign an AWB to a return shipment.
    pub async fn assign_awb(
        &self,
        req: &AssignReturnAwbRequest,
    ) -> Result<AssignReturnAwbResponse, Error> {
        let body = WithIsReturn {
            inner: req,
            is_return: 1,
        };
        self.client
            .post(self.client.api_url("courier/assign/awb"), &body)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_return_response() {
        let r: CreateReturnResponse = serde_json::from_str(
            r#"{"order_id":170872392,"shipment_id":170411259,"status":"RETURN PENDING","status_code":21,"company_name":"shiprocket"}"#,
        )
        .unwrap();
        assert_eq!(r.order_id, Some(170872392));
        assert_eq!(r.status_code, Some(21));
        assert_eq!(r.status.as_deref(), Some("RETURN PENDING"));
    }

    #[test]
    fn create_exchange_response() {
        let r: CreateExchangeResponse = serde_json::from_str(
            r#"{"success":true,"data":{"forward_orders":{"order_id":76175,"channel_order_id":"EX_TEST_101","shipment_id":659559333,"status":"NEW","status_code":1,"awb_code":"","courier_company_id":"","courier_name":""},"return_orders":{"order_id":76176,"channel_order_id":"R_TEST_101","shipment_id":659559334,"status":"RETURN PENDING","status_code":21,"awb_code":"","courier_company_id":"","courier_name":""}}}"#,
        )
        .unwrap();
        let data = r.data.unwrap();
        let fwd = data.forward_orders.unwrap();
        assert_eq!(fwd.order_id, Some(76175));
        assert_eq!(fwd.courier_company_id, None);
        assert_eq!(fwd.awb_code, None);
        assert_eq!(data.return_orders.unwrap().status_code, Some(21));
    }

    #[test]
    fn update_return_response() {
        let r: UpdateReturnResponse = serde_json::from_str(
            r#"{"product_details":{"success":true,"msg":"Product Details is updated successfully"},"return_warehouse_address":{"success":true,"msg":"Shipping Address is updated successfully"}}"#,
        )
        .unwrap();
        assert_eq!(r.product_details.unwrap().success, Some(true));
        assert!(r.return_warehouse_address.unwrap().msg.is_some());
    }

    #[test]
    fn list_returns_response() {
        let r: Paginated<ReturnOrder> = serde_json::from_str(
            r#"{"data":[{"id":16525924,"channel_id":76893,"channel_name":"CUSTOM","channel_order_id":"997119978","customer_name":"Jax Doe","total":"10.00","status":"RETURN PENDING","status_code":21,"payment_method":"prepaid","is_international":0,"products":[{"id":19192381,"name":"Tennis Ball","channel_sku":"ball123","channel_order_product_id":"19192381","quantity":1,"product_id":17949825,"sku":"ball123","status":"UNDEFINED","hsn":"4412"}],"cod":0,"shipment_id":16370752,"shipments":[{"isd_code":"+91","courier":"","sr_courier_id":"","weight":"1","length":"10","breadth":"15","height":"20","volumetric_weight":0.6,"awb":""}]}],"meta":{"pagination":{"total":16,"count":15,"per_page":1,"current_page":1,"total_pages":2,"links":{"next":"x"}}}}"#,
        )
        .unwrap();
        let o = &r.data[0];
        assert_eq!(o.id, Some(16525924));
        assert_eq!(o.total, Some(10.0));
        assert_eq!(o.cod, Some(false));
        assert_eq!(o.products[0].quantity, Some(1));
        assert_eq!(o.shipments[0].weight, Some(1.0));
        assert_eq!(r.meta.unwrap().pagination.unwrap().total_pages, Some(2));
    }

    #[test]
    fn is_return_is_added() {
        let req = AssignReturnAwbRequest::builder()
            .shipment_id(16016920)
            .build();
        let v = serde_json::to_value(WithIsReturn {
            inner: &req,
            is_return: 1,
        })
        .unwrap();
        assert_eq!(v["is_return"], 1);
        assert_eq!(v["shipment_id"], 16016920);
        assert!(v.get("courier_id").is_none());
    }

    #[test]
    fn qc_flags_serialize() {
        let item = ReturnOrderItem::builder()
            .name("shoes")
            .sku("WSH234")
            .units(1)
            .selling_price(100.0)
            .qc_used_check(true)
            .qc_check_damaged_product(false)
            .build();
        let v = serde_json::to_value(&item).unwrap();
        assert_eq!(v["qc_used_check"], 1);
        assert_eq!(v["qc_check_damaged_product"], "no");
        assert!(v.get("qc_sealtag_check").is_none());
    }
}
