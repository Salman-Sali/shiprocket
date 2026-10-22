//! Pickup addresses: list and add pickup locations.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{Error, Shiprocket, de, types::DataResponse};

/// Body of `POST settings/company/addpickup`.
///
/// ```
/// let req = shiprocket::pickup::AddPickupLocationRequest::builder()
///     .pickup_location("Home")
///     .name("Deadpool")
///     .email("deadpool@chimichanga.com")
///     .phone("9777777779")
///     .address("Mutant Facility, Sector 3")
///     .city("Pune")
///     .state("Maharashtra")
///     .country("India")
///     .pin_code("110022")
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct AddPickupLocationRequest {
    /// Nickname of the location, max 36 characters. Used as `pickup_location` on orders.
    pub pickup_location: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    /// Max 80 characters.
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub pin_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long: Option<f64>,
    /// `vendor` when the address belongs to a different vendor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gstin: Option<String>,
}

/// `data` of `GET settings/company/pickup`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PickupLocations {
    #[serde(default, deserialize_with = "de::vec")]
    pub shipping_address: Vec<PickupLocation>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub allow_more: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_blackbox_seller: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub company_name: Option<String>,
    #[serde(default)]
    pub recent_addresses: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A pickup location as returned by the list and add endpoints.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PickupLocation {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    /// Location nickname. Named `pickup_code` in the add response.
    #[serde(default, alias = "pickup_code", deserialize_with = "de::opt_string")]
    pub pickup_location: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub address_type: Option<String>,
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
    pub email: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub phone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub alternate_phone: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub gstin: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub vendor_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub phone_verified: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub lat: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub long: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub rto_address_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_primary_location: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST settings/company/addpickup`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct AddPickupLocationResponse {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub address: Option<PickupLocation>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub pickup_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub company_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub full_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Pickup address endpoints. Obtain with [`Shiprocket::pickup`].
pub struct Pickup<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn pickup(&self) -> Pickup<'_> {
        Pickup { client: self }
    }
}

impl Pickup<'_> {
    /// `GET settings/company/pickup`: all pickup locations of the account.
    pub async fn list(&self) -> Result<DataResponse<PickupLocations>, Error> {
        self.client
            .get(self.client.api_url("settings/company/pickup"))
            .await
    }

    /// `POST settings/company/addpickup`: add a new pickup location.
    pub async fn add(
        &self,
        req: &AddPickupLocationRequest,
    ) -> Result<AddPickupLocationResponse, Error> {
        self.client
            .post(self.client.api_url("settings/company/addpickup"), req)
            .await
    }
}
