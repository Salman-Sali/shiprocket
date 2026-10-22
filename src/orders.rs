//! Orders: create, update, cancel, list, show, import and export orders.

use bon::Builder;
use serde::{Deserialize, Serialize, Serializer};

use crate::{
    Error, Shiprocket, de,
    types::{DataResponse, ImportResponse, MessageResponse, Paginated, PaymentMethod, Upload},
};

fn bool_as_int<S: Serializer>(v: &Option<bool>, s: S) -> Result<S::Ok, S::Error> {
    match v {
        Some(b) => s.serialize_u8(*b as u8),
        None => s.serialize_none(),
    }
}

fn int_as_string<S: Serializer>(v: &i64, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&v.to_string())
}

/// Body of `POST orders/create/adhoc`, `POST orders/create` and `POST orders/update/adhoc`.
///
/// When `shipping_is_billing` is `false` the `shipping_*` fields are required.
/// `sub_total` is not computed by Shiprocket; pass the correct total yourself.
///
/// ```
/// use shiprocket::{PaymentMethod, orders::{CreateOrderRequest, OrderItem}};
///
/// let req = CreateOrderRequest::builder()
///     .order_id("224-477")
///     .order_date("2019-07-24 11:11")
///     .pickup_location("Jammu")
///     .billing_customer_name("Naruto")
///     .billing_address("House 221B, Leaf Village")
///     .billing_city("New Delhi")
///     .billing_pincode("110002")
///     .billing_state("Delhi")
///     .billing_country("India")
///     .billing_email("naruto@uzumaki.com")
///     .billing_phone("9876543210")
///     .shipping_is_billing(true)
///     .order_items(vec![
///         OrderItem::builder()
///             .name("Kunai")
///             .sku("chakra123")
///             .units(10)
///             .selling_price(900.0)
///             .build(),
///     ])
///     .payment_method(PaymentMethod::Prepaid)
///     .sub_total(9000.0)
///     .length(10.0)
///     .breadth(15.0)
///     .height(20.0)
///     .weight(2.5)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct CreateOrderRequest {
    /// Your order id, max 50 characters. Must not reuse an existing or cancelled order id.
    pub order_id: String,
    /// `yyyy-mm-dd`, optionally followed by `hh:mm`.
    pub order_date: String,
    /// Nickname of an existing pickup location. Required for custom orders; the default
    /// location is used for channel orders when omitted.
    pub pickup_location: String,
    /// Required for channel specific orders. Defaults to the custom channel otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i64>,
    /// Use `Reseller: [name]` to add a "From" field to the shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Vendor name to print on the label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,

    pub billing_customer_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_last_name: Option<String>,
    pub billing_address: String,
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
    /// Delivery location longitude. Required for hyperlocal orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Delivery location latitude. Required for hyperlocal orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,

    pub order_items: Vec<OrderItem>,
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
    /// Centimetres, more than 0.5.
    pub length: f64,
    /// Centimetres, more than 0.5.
    pub breadth: f64,
    /// Centimetres, more than 0.5.
    pub height: f64,
    /// Kilograms, more than 0.
    pub weight: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_gstin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// `ESSENTIALS` or `NON ESSENTIALS` (case sensitive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// SRF only: `SR_RUSH`, `SR_STANDARD`, `SR_EXPRESS` or `SR_QUICK`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_shipping_method: Option<String>,
    /// Hyperlocal shipping method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub what3words_address: Option<String>,
    /// Insure shipments above Rs 2500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_insurance_opt: Option<bool>,
    /// Create a document order. Sent as `1`/`0`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "bool_as_int",
        default,
        deserialize_with = "de::opt_bool"
    )]
    pub is_document: Option<bool>,
    /// Comma separated tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_tag: Option<String>,
}

/// A line item of [`CreateOrderRequest`].
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct OrderItem {
    pub name: String,
    pub sku: String,
    pub units: i64,
    pub selling_price: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsn: Option<String>,
}

/// Response of `POST orders/create/adhoc` and `POST orders/create`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateOrderResponse {
    /// Shiprocket order id. Save it; later calls use it.
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

/// Response of `POST orders/update/adhoc`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct UpdateOrderResponse {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub partially_update: Option<bool>,
    /// Comma separated list of fields that were not updated.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub not_updated_fields: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub new_order_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub old_order_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Body of `PATCH orders/address/pickup`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct UpdatePickupLocationRequest {
    /// Shiprocket order ids.
    pub order_id: Vec<i64>,
    /// Nickname of an existing pickup location.
    pub pickup_location: String,
}

/// Body of `POST orders/address/update`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct UpdateDeliveryAddressRequest {
    /// Shiprocket order id.
    pub order_id: i64,
    pub shipping_customer_name: String,
    pub shipping_phone: String,
    pub shipping_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<String>,
    pub shipping_city: String,
    pub shipping_state: String,
    pub shipping_country: String,
    pub shipping_pincode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_alternate_phone: Option<String>,
}

/// Body of `POST orders/cancel`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct CancelOrdersRequest {
    /// Shiprocket order ids.
    pub ids: Vec<i64>,
}

/// Body of `PATCH orders/fulfill`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct FulfillOrdersRequest {
    pub data: Vec<FulfillItem>,
}

/// Inventory to add for an ordered product.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct FulfillItem {
    /// Shiprocket order id.
    pub order_id: i64,
    pub order_product_id: i64,
    #[serde(serialize_with = "int_as_string")]
    pub quantity: i64,
    /// Only `add` is documented.
    #[builder(default = "add".to_string())]
    pub action: String,
}

/// One entry of the `PATCH orders/fulfill` response.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct FulfillResult {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<FulfillResultData>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct FulfillResultData {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_product_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub action: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Body of `PATCH orders/mapping`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct MapProductsRequest {
    pub data: Vec<MapProductItem>,
}

/// Maps an unmapped ordered product to a master SKU.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct MapProductItem {
    /// Shiprocket order id.
    pub order_id: i64,
    pub order_product_id: i64,
    pub master_sku: String,
}

/// One entry of the `PATCH orders/mapping` response.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct MapProductResult {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub data: Option<MapProductResultData>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MapProductResultData {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_product_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub master_sku: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}
/// Response of `POST orders/export`. The CSV link is emailed to the API user.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ExportOrdersResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_background_downloading: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Query of `GET orders`. All fields are optional; `ListOrdersQuery::default()` lists everything.
///
/// `updated_from` is required with `updated_to`, the range may span at most 30 days and
/// `updated_from` may be at most 30 days in the past.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ListOrdersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    /// `ASC` or `DESC`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// `id` or `status`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// End date, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Start date, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// `status`, `payment_method`, `delivery_country` or `channel_order_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<String>,
    /// Value for `filter_by`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// AWB or channel order id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i64>,
    /// `1` to show SRF orders only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbs: Option<u8>,
    /// `1` to show both SRF and last mile orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbs_all_orders: Option<u8>,
    /// Filter by update date (start), `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_from: Option<String>,
    /// Filter by update date (end), `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_to: Option<String>,
}

/// An order as returned by `GET orders`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OrderSummary {
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
    pub pickup_location: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub total: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub tax: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sla: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipping_method: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub expedited: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    /// `prepaid` or `cod`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_method: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_international: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub purpose_of_shipment: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::vec")]
    pub products: Vec<OrderProduct>,
    #[serde(default, deserialize_with = "de::vec")]
    pub shipments: Vec<OrderShipment>,
    #[serde(default, deserialize_with = "de::vec")]
    pub activities: Vec<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub allow_return: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_incomplete: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A product line of an order (list and show responses).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OrderProduct {
    /// Order product id.
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub product_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_order_product_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub hsn: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub brand: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub color: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub size: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub returnable_quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub available: Option<i64>,
    /// Text (`CANCELED`) in list responses, numeric code in show responses.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub dimensions: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub price: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub cost: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub mrp: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub selling_price: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub tax: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub tax_percentage: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub discount: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub net_total: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Shipment of an order (list and show responses).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OrderShipment {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub channel_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub isd_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub return_awb: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_awb: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_assign_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub volumetric_weight: Option<f64>,
    /// `LxBxH` in centimetres.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub dimensions: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub length: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub breadth: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub height: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub cost: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub tax: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub cod_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub manifest_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_scheduled_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_token_number: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pod: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub etd: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipped_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivered_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_initiated_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_delivered_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub eway_bill_number: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_rto: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub invoice_link: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// `data` of `GET orders/show/{id}`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OrderDetails {
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
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_international: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_document: Option<bool>,
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
    pub pickup_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_location: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub pickup_location_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub currency: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub total: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub net_total: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub other_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub other_discounts: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub giftwrap_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub tax: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub discount: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub expedited: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub cod: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sla: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sub_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_method: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub order_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub updated_at: Option<String>,
    #[serde(default, deserialize_with = "de::vec")]
    pub products: Vec<OrderProduct>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub invoice_no: Option<String>,
    /// A single object here, unlike the list endpoint.
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub shipments: Option<OrderShipment>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub awb_data: Option<AwbData>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub order_insurance: Option<OrderInsurance>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub return_pickup_data: Option<ReturnPickupData>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub allow_return: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_return: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_incomplete: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub reseller_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub company_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub shipping_is_billing: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipping_method: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub eway_bill_number: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub order_tag: Option<String>,
    /// SRF orders only: `Ready to Pack`, `Packed`, `Added to Picklist` or `Picked up`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub fulfillment_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub etd_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub out_for_delivery_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivered_date: Option<String>,
    /// Shipping address details and other loosely structured data.
    #[serde(default)]
    pub others: serde_json::Value,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// AWB and billing weight details of an order.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AwbData {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charged_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub billed_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub routing_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub rto_routing_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub charges: Option<AwbCharges>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AwbCharges {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub zone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub cod_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight_amount: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub freight_charges: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charged_weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charged_weight_amount: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub charged_weight_amount_rto: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub applied_weight_amount_rto: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub service_type_id: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct OrderInsurance {
    /// `Yes` / `No`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub insurance_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub policy_no: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub claim_enable: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Address returns are picked up from.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ReturnPickupData {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub email: Option<String>,
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
    pub pin_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub phone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub lat: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub long: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub updated_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Order endpoints. Obtain with [`Shiprocket::orders`].
pub struct Orders<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn orders(&self) -> Orders<'_> {
        Orders { client: self }
    }
}

impl Orders<'_> {
    /// `POST orders/create/adhoc`: create a custom order (products not stored in the catalogue).
    pub async fn create(&self, req: &CreateOrderRequest) -> Result<CreateOrderResponse, Error> {
        self.client
            .post(self.client.api_url("orders/create/adhoc"), req)
            .await
    }

    /// `POST orders/create`: create an order under a specific channel.
    ///
    /// `channel_id` is required and inventory sync must be enabled for the channel.
    pub async fn create_for_channel(
        &self,
        req: &CreateOrderRequest,
    ) -> Result<CreateOrderResponse, Error> {
        self.client
            .post(self.client.api_url("orders/create"), req)
            .await
    }

    /// `POST orders/update/adhoc`: update an order before an AWB is assigned.
    ///
    /// Only `order_items`, tax/discount and `is_document` changes are applied; other fields
    /// are reported back in `not_updated_fields`.
    pub async fn update(&self, req: &CreateOrderRequest) -> Result<UpdateOrderResponse, Error> {
        self.client
            .post(self.client.api_url("orders/update/adhoc"), req)
            .await
    }

    /// `PATCH orders/address/pickup`: move orders to another existing pickup location.
    pub async fn update_pickup_location(
        &self,
        req: &UpdatePickupLocationRequest,
    ) -> Result<MessageResponse, Error> {
        self.client
            .patch(self.client.api_url("orders/address/pickup"), req)
            .await
    }

    /// `POST orders/address/update`: update the customer's delivery address.
    pub async fn update_delivery_address(
        &self,
        req: &UpdateDeliveryAddressRequest,
    ) -> Result<MessageResponse, Error> {
        self.client
            .post(self.client.api_url("orders/address/update"), req)
            .await
    }

    /// `POST orders/cancel`: cancel one or more orders.
    pub async fn cancel(&self, req: &CancelOrdersRequest) -> Result<MessageResponse, Error> {
        self.client
            .post(self.client.api_url("orders/cancel"), req)
            .await
    }

    /// `PATCH orders/fulfill`: add inventory for out of stock ordered products.
    pub async fn fulfill(&self, req: &FulfillOrdersRequest) -> Result<Vec<FulfillResult>, Error> {
        self.client
            .patch(self.client.api_url("orders/fulfill"), req)
            .await
    }

    /// `PATCH orders/mapping`: map unmapped ordered products to master SKUs.
    pub async fn map_products(
        &self,
        req: &MapProductsRequest,
    ) -> Result<Vec<MapProductResult>, Error> {
        self.client
            .patch(self.client.api_url("orders/mapping"), req)
            .await
    }

    /// `POST orders/import`: bulk import orders from a CSV file.
    pub async fn import(&self, file: &Upload) -> Result<ImportResponse, Error> {
        self.client
            .post_file(self.client.api_url("orders/import"), "file", file, &[])
            .await
    }

    /// `GET orders`: list orders with optional filters and pagination.
    pub async fn list(&self, query: &ListOrdersQuery) -> Result<Paginated<OrderSummary>, Error> {
        self.client
            .get_query(self.client.api_url("orders"), query)
            .await
    }

    /// `GET orders/show/{id}`: details of one order by Shiprocket order id.
    pub async fn get(&self, order_id: i64) -> Result<DataResponse<OrderDetails>, Error> {
        self.client
            .get(self.client.api_url(&format!("orders/show/{order_id}")))
            .await
    }

    /// `POST orders/export`: export all orders to CSV. The download link is emailed.
    pub async fn export(&self) -> Result<ExportOrdersResponse, Error> {
        self.client
            .post(
                self.client.api_url("orders/export"),
                &serde_json::Map::<String, serde_json::Value>::new(),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_response() {
        let r: CreateOrderResponse = serde_json::from_str(
            r#"{"order_id":16161616,"shipment_id":15151515,"status":"NEW","status_code":1,
            "onboarding_completed_now":0,"awb_code":null,"courier_company_id":null,"courier_name":null}"#,
        )
        .unwrap();
        assert_eq!(r.order_id, Some(16161616));
        assert_eq!(r.shipment_id, Some(15151515));
        assert_eq!(r.status.as_deref(), Some("NEW"));
        assert_eq!(r.awb_code, None);
    }

    #[test]
    fn update_response() {
        let r: UpdateOrderResponse = serde_json::from_str(
            r#"{"success":true,"partially_update":true,"not_updated_fields":"order_date ,pickup_location",
            "order_id":79491,"shipment_id":77906,"new_order_status":"NEW","old_order_status":1,
            "awb_code":"","courier_company_id":"","courier_name":""}"#,
        )
        .unwrap();
        assert_eq!(r.success, Some(true));
        assert_eq!(r.order_id, Some(79491));
        assert_eq!(r.old_order_status.as_deref(), Some("1"));
        assert_eq!(r.courier_company_id, None);
    }

    #[test]
    fn message_responses() {
        let r: MessageResponse =
            serde_json::from_str(r#"{"message":"Pickup location Updated"}"#).unwrap();
        assert_eq!(r.message.as_deref(), Some("Pickup location Updated"));
    }

    #[test]
    fn fulfill_and_mapping_responses() {
        let r: Vec<FulfillResult> = serde_json::from_str(
            r#"[{"data":{"order_id":14124005,"order_product_id":43737767570843,"quantity":"1","action":"add"},
            "success":true,"message":"Inventory added successfully"},
            {"data":{"order_id":"","order_product_id":"","quantity":"","action":""},"success":false,
            "message":"Incorrect order_id or order status is no longer unfulfillable"}]"#,
        )
        .unwrap();
        let d = r[0].data.as_ref().unwrap();
        assert_eq!(d.order_product_id, Some(43737767570843));
        assert_eq!(d.quantity, Some(1));
        assert_eq!(r[1].success, Some(false));
        assert_eq!(r[1].data.as_ref().unwrap().order_id, None);

        let r: Vec<MapProductResult> = serde_json::from_str(
            r#"[{"data":{"order_id":14303681,"order_product_id":16487731,"master_sku":"delta123"},
            "status_code":200,"success":true,"message":"Product mapped sucessfully."}]"#,
        )
        .unwrap();
        assert_eq!(
            r[0].data.as_ref().unwrap().master_sku.as_deref(),
            Some("delta123")
        );
        assert_eq!(r[0].status_code, Some(200));
    }

    #[test]
    fn import_and_export_responses() {
        let r: ImportResponse = serde_json::from_str(r#"{"id":19739203}"#).unwrap();
        assert_eq!(r.id, Some(19739203));
        let r: ExportOrdersResponse =
            serde_json::from_str(r#"{"status":200,"is_background_downloading":1}"#).unwrap();
        assert_eq!(r.status, Some(200));
        assert_eq!(r.is_background_downloading, Some(true));
    }

    #[test]
    fn list_response() {
        let r: Paginated<OrderSummary> = serde_json::from_str(
            r#"{"data":[{"id":16178831,"channel_id":76893,"channel_name":"CUSTOM","base_channel_code":"CS",
            "channel_order_id":"224-4779888","customer_name":"Majin Bu","customer_email":"naruto@uzumaki.com",
            "customer_phone":"9988998899","pickup_location":"hell","payment_status":"","total":"9000.00",
            "tax":"0.00","sla":"2 days","shipping_method":"SR","expedited":0,"status":"CANCELED","status_code":5,
            "payment_method":"prepaid","is_international":0,"purpose_of_shipment":0,
            "channel_created_at":"24 Jul 2019, 11:11 AM","created_at":"31 Jul 2019, 03:03 PM",
            "products":[{"id":18769728,"channel_order_product_id":"18769728","name":"Kunai","channel_sku":"chakra123",
            "quantity":10,"product_id":17484610,"available":50,"status":"CANCELED","hsn":"441122"}],
            "shipments":[{"id":16028538,"isd_code":"","courier":"","weight":0,"dimensions":"0.00x0.00x0.00",
            "pickup_scheduled_date":null,"pickup_token_number":null,"awb":"","return_awb":"","volumetric_weight":0,
            "pod":null,"etd":"NA","rto_delivered_date":"0000-00-00 00:00:00","delivered_date":null,
            "etd_escalation_btn":false}],"activities":["ORDER_CREATED","ADDRESS_MODIFIED","ORDER_CANCELLED"],
            "allow_return":0,"is_incomplete":0,"errors":[],"show_escalation_btn":0,"escalation_status":"",
            "escalation_history":[]}],
            "meta":{"pagination":{"total":19631,"count":15,"per_page":15,"current_page":1,"total_pages":1309,
            "links":{"next":"https://apiv2.shiprocket.in/v1/external/orders?page=2"}}}}"#,
        )
        .unwrap();
        let o = &r.data[0];
        assert_eq!(o.id, Some(16178831));
        assert_eq!(o.total, Some(9000.0));
        assert_eq!(o.products[0].product_id, Some(17484610));
        assert_eq!(o.shipments[0].id, Some(16028538));
        assert_eq!(o.activities.len(), 3);
        let p = r.meta.unwrap().pagination.unwrap();
        assert_eq!(p.total_pages, Some(1309));
    }

    #[test]
    fn show_response() {
        let r: DataResponse<OrderDetails> = serde_json::from_str(
            r#"{"data":{"id":259492257,"channel_id":38026,"channel_name":"MANUAL1","base_channel_code":"CS",
            "is_international":0,"is_document":0,"channel_order_id":"1873081902","customer_name":"DemoHome ",
            "customer_pincode":"110088","pickup_location_id":"","currency":"INR","total":345,"net_total":"345.00",
            "status":"RETURN PENDING","sub_status":null,"status_code":21,"payment_method":"prepaid",
            "products":[{"id":365076966,"order_id":259492257,"product_id":1620533,"name":"watch",
            "sku":"Tshirt-Blue-41","price":345,"mrp":400,"quantity":1,"status":1,"product_options":[]}],
            "shipments":{"id":258878960,"order_id":259492257,"cost":"0.00","awb":null,"weight":1,
            "volumetric_weight":0.266,"dimensions":"11.000x11.000x11.000","courier_id":"","manifest_id":"",
            "status":"PENDING","is_rto":false,"length":11},
            "awb_data":{"awb":"","applied_weight":"","charges":{"zone":"","cod_charges":"","service_type_id":""}},
            "order_insurance":{"insurance_status":"No","policy_no":"N/A","claim_enable":false},
            "return_pickup_data":{"id":2143757,"name":"ashwin ashwin","pin_code":"110030","lat":null,
            "order_id":259492257,"created_at":"2022-09-21 17:28:40"},
            "errors":null,"pickup_address":[],"others":{"weight":"1","quantity":1},
            "extra_info":{"qc_check":1},"etd_date":null}}"#,
        )
        .unwrap();
        let o = r.data.unwrap();
        assert_eq!(o.id, Some(259492257));
        assert_eq!(o.net_total, Some(345.0));
        assert_eq!(o.pickup_location_id, None);
        assert_eq!(o.products[0].mrp, Some(400.0));
        assert_eq!(o.products[0].status.as_deref(), Some("1"));
        let s = o.shipments.unwrap();
        assert_eq!(s.id, Some(258878960));
        assert_eq!(s.volumetric_weight, Some(0.266));
        assert_eq!(s.awb, None);
        assert!(o.awb_data.unwrap().charges.is_some());
        assert_eq!(
            o.order_insurance.unwrap().insurance_status.as_deref(),
            Some("No")
        );
        assert_eq!(
            o.return_pickup_data.unwrap().pin_code.as_deref(),
            Some("110030")
        );
        assert!(o.extra.contains_key("extra_info"));
    }

    #[test]
    fn request_serialization() {
        let req = CreateOrderRequest::builder()
            .order_id("1")
            .order_date("2024-10-28")
            .pickup_location("Home")
            .billing_customer_name("Naruto")
            .billing_address("Leaf Village")
            .billing_city("Delhi")
            .billing_pincode("110002")
            .billing_state("Delhi")
            .billing_country("India")
            .billing_email("n@u.com")
            .billing_phone("9876543210")
            .shipping_is_billing(true)
            .order_items(vec![
                OrderItem::builder()
                    .name("Kunai")
                    .sku("k1")
                    .units(1)
                    .selling_price(900.0)
                    .build(),
            ])
            .payment_method(PaymentMethod::Cod)
            .sub_total(900.0)
            .length(10.0)
            .breadth(10.0)
            .height(10.0)
            .weight(0.5)
            .is_document(true)
            .build();
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["payment_method"], "COD");
        assert_eq!(v["is_document"], 1);
        assert!(v.get("channel_id").is_none());
        assert!(v["order_items"][0].get("hsn").is_none());

        let q = serde_json::to_value(
            FulfillItem::builder()
                .order_id(1)
                .order_product_id(2)
                .quantity(3)
                .build(),
        )
        .unwrap();
        assert_eq!(q["quantity"], "3");
        assert_eq!(q["action"], "add");
    }
}
