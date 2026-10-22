//! Channels: integrated sales channels and custom channel creation.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{Error, Shiprocket, de, types::Paginated};

/// Body of `POST channels`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct CreateChannelRequest {
    /// Display name, max 12 characters, unique within the account.
    pub name: String,
    /// Brand label on invoices and labels, max 50 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_name: Option<String>,
}

/// An integrated channel.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Channel {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_updated_at: Option<String>,
    #[serde(default)]
    pub settings: serde_json::Value,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub connection: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub orders_sync: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub inventory_sync: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub catalog_sync: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub orders_synced_on: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub inventory_synced_on: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub catalog_synced_on: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub base_channel_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub base_channel: Option<BaseChannel>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub brand_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub brand_logo: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// The platform a [`Channel`] is built on (Shopify, manual, ...).
#[derive(Debug, Clone, Default, Deserialize)]
pub struct BaseChannel {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub code: Option<String>,
    #[serde(default, rename = "type", deserialize_with = "de::opt_string")]
    pub channel_type: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub logo: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub description: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `POST channels`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateChannelResponse {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub channel_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub brand_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub base_channel_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Channel endpoints. Obtain with [`Shiprocket::channels`].
pub struct Channels<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn channels(&self) -> Channels<'_> {
        Channels { client: self }
    }
}

impl Channels<'_> {
    /// `GET channels`: all channels integrated with the account.
    pub async fn list(&self) -> Result<Paginated<Channel>, Error> {
        self.client.get(self.client.api_url("channels")).await
    }

    /// `POST channels`: create a custom (manual) channel.
    ///
    /// The account must be allow-listed for "Channel Creation by API" by Shiprocket support,
    /// otherwise this fails with `403`.
    pub async fn create(&self, req: &CreateChannelRequest) -> Result<CreateChannelResponse, Error> {
        self.client.post(self.client.api_url("channels"), req).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_channels() {
        let res: Paginated<Channel> = serde_json::from_str(
            r#"{"data":[{"id":76893,"name":"CUSTOM","status":"Active","connection_response":null,
            "channel_updated_at":"2018-05-24 11:55:28","status_code":1,
            "settings":{"dimensions":"0x0x0","weight":0,"order_status":""},"auth":[],"connection":1,
            "orders_sync":0,"inventory_sync":0,"catalog_sync":0,"orders_synced_on":"Not Available",
            "inventory_synced_on":"Not Available","base_channel_code":"CS",
            "base_channel":{"id":4,"name":"MANUAL","code":"CS","type":"Carts","logo":"custom.png",
            "settings_sample":{},"auth_sample":[],"description":"Manual channel"},
            "catalog_synced_on":"31 Jul 12:49 PM","order_status_mapper":"","payment_status_mapper":"",
            "brand_name":"","brand_logo":""}]}"#,
        )
        .unwrap();
        let c = &res.data[0];
        assert_eq!(c.id, Some(76893));
        assert_eq!(c.connection, Some(true));
        assert_eq!(c.orders_sync, Some(false));
        let base = c.base_channel.as_ref().unwrap();
        assert_eq!(base.channel_type.as_deref(), Some("Carts"));
        assert!(c.extra.contains_key("auth"));
    }

    #[test]
    fn decodes_create_response() {
        let res: CreateChannelResponse = serde_json::from_str(
            r#"{"channel_id":123456,"name":"MANUAL-25149","brand_name":"SIORA-12",
            "base_channel_code":"CUSTOM","status":1,"company_id":25149,"created_at":"2026-04-29 12:34:56"}"#,
        )
        .unwrap();
        assert_eq!(res.channel_id, Some(123456));
        assert_eq!(res.company_id, Some(25149));
    }
}
