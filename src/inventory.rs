//! Inventory: stock levels per product.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Shiprocket, de,
    types::{DataResponse, Paginated},
};

/// Query of `GET inventory`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ListInventoryQuery {
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
}

/// How [`UpdateInventoryRequest::quantity`] is applied to the current stock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InventoryAction {
    Add,
    Replace,
    Remove,
}

/// Body of `PUT inventory/{product_id}/update`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct UpdateInventoryRequest {
    pub quantity: i64,
    pub action: InventoryAction,
}

/// Stock of a single product.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InventoryItem {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub category_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_combo: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, rename = "type", deserialize_with = "de::opt_string")]
    pub product_type: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub color: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub brand: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub total_quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub available_quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub blocked_quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub updated_on: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// `data` of `PUT inventory/{product_id}/update`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct InventoryLevels {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub available_quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub blocked_quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub total_quantity: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Inventory endpoints. Obtain with [`Shiprocket::inventory`].
pub struct Inventory<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn inventory(&self) -> Inventory<'_> {
        Inventory { client: self }
    }
}

impl Inventory<'_> {
    /// `GET inventory`: paginated stock levels.
    pub async fn list(
        &self,
        query: &ListInventoryQuery,
    ) -> Result<Paginated<InventoryItem>, Error> {
        self.client
            .get_query(self.client.api_url("inventory"), query)
            .await
    }

    /// `PUT inventory/{product_id}/update`: add, replace or remove stock of a product.
    pub async fn update(
        &self,
        product_id: i64,
        req: &UpdateInventoryRequest,
    ) -> Result<DataResponse<InventoryLevels>, Error> {
        self.client
            .put(
                self.client
                    .api_url(&format!("inventory/{product_id}/update")),
                req,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_inventory() {
        let res: Paginated<InventoryItem> = serde_json::from_str(
            r#"{"data":[{"id":3448631,"sku":"123","category_name":"Default Category","is_combo":0,
            "name":"gsdgw","type":"Single","color":"","brand":"","total_quantity":6,
            "available_quantity":6,"blocked_quantity":0,"updated_on":"15 Oct 2018 04:14 PM"}],
            "meta":{"pagination":{"total":12080,"count":2,"per_page":2,"current_page":1,"total_pages":6040,
            "links":{"next":"https://apiv2.shiprocket.in/v1/external/inventory?page=2"}}}}"#,
        )
        .unwrap();
        let i = &res.data[0];
        assert_eq!(i.id, Some(3448631));
        assert_eq!(i.sku.as_deref(), Some("123"));
        assert_eq!(i.available_quantity, Some(6));
        assert_eq!(i.product_type.as_deref(), Some("Single"));
    }

    #[test]
    fn decodes_update() {
        let res: DataResponse<InventoryLevels> = serde_json::from_str(
            r#"{"data":{"available_quantity":51,"blocked_quantity":0,"total_quantity":51}}"#,
        )
        .unwrap();
        assert_eq!(res.data.unwrap().total_quantity, Some(51));
    }

    #[test]
    fn serializes_update() {
        let req = UpdateInventoryRequest::builder()
            .quantity(2)
            .action(InventoryAction::Add)
            .build();
        assert_eq!(
            serde_json::to_string(&req).unwrap(),
            r#"{"quantity":2,"action":"add"}"#
        );
    }
}
