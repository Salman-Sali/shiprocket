//! Listings: channel catalogue products and their mapping to the master catalogue.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Shiprocket, de,
    types::{ImportResponse, MessageResponse, Paginated, Upload},
};

/// Query of `GET listings`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ListListingsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    /// `ASC` or `DESC`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Field to sort by, e.g. `id`, `sku`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Value to match against `filter_by`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// Field to filter on, e.g. `id`, `sku`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_by: Option<String>,
}

/// Body of `POST listings/link`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct LinkListingRequest {
    /// Id of the product in the master catalogue.
    pub product_id: i64,
    /// Id of the product in the channel catalogue.
    pub listing_id: i64,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

/// A channel catalogue product.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Listing {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub title: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub image: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub price: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub channel_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub base_channel_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub channel_product_id: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub inventory: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub synced_on: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub product: Option<ListingProduct>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub category_name: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Physical details of the master product behind a [`Listing`].
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListingProduct {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub dimensions: Option<ListingDimensions>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub weight: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ListingDimensions {
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub length: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub width: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub height: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of the CSV export endpoints.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct DownloadUrl {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub download_url: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Listing endpoints. Obtain with [`Shiprocket::listings`].
pub struct Listings<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn listings(&self) -> Listings<'_> {
        Listings { client: self }
    }
}

impl Listings<'_> {
    /// `GET listings`: paginated list of channel catalogue products.
    pub async fn list(&self, query: &ListListingsQuery) -> Result<Paginated<Listing>, Error> {
        self.client
            .get_query(self.client.api_url("listings"), query)
            .await
    }

    /// `POST listings/link`: map a channel product to a master catalogue product.
    pub async fn link(&self, req: &LinkListingRequest) -> Result<MessageResponse, Error> {
        self.client
            .post(self.client.api_url("listings/link"), req)
            .await
    }

    /// `POST listings/import`: import channel-to-master mappings from a CSV file (multipart field `file`).
    pub async fn import(&self, file: &Upload) -> Result<ImportResponse, Error> {
        self.client
            .post_file(self.client.api_url("listings/import"), "file", file, &[])
            .await
    }

    /// `GET listings/export/mapped`: CSV download url of mapped products.
    pub async fn export_mapped(&self) -> Result<DownloadUrl, Error> {
        self.client
            .get(self.client.api_url("listings/export/mapped"))
            .await
    }

    /// `GET listings/export/unmapped`: CSV download url of unmapped products.
    pub async fn export_unmapped(&self) -> Result<DownloadUrl, Error> {
        self.client
            .get(self.client.api_url("listings/export/unmapped"))
            .await
    }

    /// `GET listings/sample`: download url of a sample catalogue mapping CSV.
    pub async fn sample(&self) -> Result<DownloadUrl, Error> {
        self.client
            .get(self.client.api_url("listings/sample"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_listings() {
        let res: Paginated<Listing> = serde_json::from_str(
            r#"{"data":[{"id":15897064,"title":"Kunai","image":null,"price":"900.00","quantity":0,
            "sku":"chakra123","channel_sku":"chakra123","channel_id":76893,"channel_name":"CUSTOM",
            "base_channel_code":"CS","channel_product_id":"","inventory":41,"synced_on":"Never Synced",
            "product":{"dimensions":{"length":"0.000","width":"0.000","height":"0.000"},"weight":"0.000"},
            "category_name":""}],
            "meta":{"pagination":{"total":12345,"count":2,"per_page":2,"current_page":1,"total_pages":6173,
            "links":{"next":"https://apiv2.shiprocket.in/v1/external/listings?page=2"}}}}"#,
        )
        .unwrap();
        let l = &res.data[0];
        assert_eq!(l.id, Some(15897064));
        assert_eq!(l.price, Some(900.0));
        assert_eq!(l.channel_id, Some(76893));
        assert_eq!(l.inventory, Some(41));
        assert_eq!(l.image, None);
        let dims = l.product.as_ref().unwrap().dimensions.as_ref().unwrap();
        assert_eq!(dims.length, Some(0.0));
    }

    #[test]
    fn decodes_link_and_export() {
        let res: MessageResponse =
            serde_json::from_str(r#"{"message":"Linking success","status_code":200}"#).unwrap();
        assert_eq!(res.status_code, Some(200));

        let res: DownloadUrl = serde_json::from_str(
            r#"{"download_url":"https://s3-ap-southeast-1.amazonaws.com/x/export.csv"}"#,
        )
        .unwrap();
        assert!(res.download_url.unwrap().ends_with("export.csv"));
    }

    #[test]
    fn serializes_link_request() {
        let req = LinkListingRequest::builder()
            .product_id(17908342)
            .listing_id(15897064)
            .build();
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["product_id"], 17908342);
        assert!(v.get("ID").is_none());
    }
}
