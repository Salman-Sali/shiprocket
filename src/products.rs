//! Products: master catalogue products, QC products and bulk import.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{
    Error, Shiprocket, de,
    types::{DataResponse, ImportResponse, MessageResponse, Paginated, Upload},
};

/// Query of `GET products`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ListProductsQuery {
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

/// Quality check details the pickup agent verifies on return pickups.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct QcDetails {
    /// Mandatory for QC products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ean_barcode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_damaged_product: Option<bool>,
}

/// Body of `POST products`.
///
/// ```
/// let req = shiprocket::products::AddProductRequest::builder()
///     .name("Batman Toy")
///     .sku("bat123")
///     .category_code("default")
///     .r#type("Single")
///     .qty(5)
///     .build();
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct AddProductRequest {
    /// Unique stock keeping unit.
    pub sku: String,
    pub name: String,
    /// `default` when you have no category.
    pub category_code: String,
    /// `Single` or `Multiple`.
    pub r#type: String,
    pub qty: i64,
    #[serde(rename = "HSN", skip_serializing_if = "Option::is_none")]
    pub hsn: Option<String>,
    /// Tax percentage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// Quantity at which the low stock notification fires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_stock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Weight in kg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    /// Length in cm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<f64>,
    /// Width in cm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// Height in cm.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imei_serialnumber: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mrp: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Required to create a QC product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qc_details: Option<QcDetails>,
}

/// Body of `POST products/qc-product-update/{product_id}`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ConvertToQcProductRequest {
    pub sku: String,
    pub product_image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_box: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_imei: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_damaged_product: Option<bool>,
}

/// A master catalogue product.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Product {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sku: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub hsn: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub description: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub category_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub category_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub category_tax_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub image: Option<String>,
    /// Free text such as `0.5 kg` or `0.000`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub weight: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub size: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub cost_price: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub mrp: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub tax_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub low_stock: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub ean: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub upc: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub isbn: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub updated_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub quantity: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub color: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub brand: Option<String>,
    /// Free text such as `10 x 10 x 10 cm`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub dimensions: Option<String>,
    /// `ACTIVE` / `INACTIVE`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    /// `Single` / `Multiple`.
    #[serde(default, rename = "type", deserialize_with = "de::opt_string")]
    pub product_type: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_combo: Option<bool>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Product endpoints. Obtain with [`Shiprocket::products`].
pub struct Products<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn products(&self) -> Products<'_> {
        Products { client: self }
    }
}

impl Products<'_> {
    /// `GET products`: paginated list of products.
    pub async fn list(&self, query: &ListProductsQuery) -> Result<Paginated<Product>, Error> {
        self.client
            .get_query(self.client.api_url("products"), query)
            .await
    }

    /// `GET products/show/{product_id}`: a single product.
    pub async fn get(&self, product_id: i64) -> Result<DataResponse<Product>, Error> {
        self.client
            .get(self.client.api_url(&format!("products/show/{product_id}")))
            .await
    }

    /// `POST products`: add a product to the master catalogue. Responds `201` with an empty body.
    pub async fn add(&self, req: &AddProductRequest) -> Result<MessageResponse, Error> {
        self.client.post(self.client.api_url("products"), req).await
    }

    /// `POST products/qc-product-update/{product_id}`: convert an existing product to a QC product.
    pub async fn convert_to_qc(
        &self,
        product_id: i64,
        req: &ConvertToQcProductRequest,
    ) -> Result<MessageResponse, Error> {
        self.client
            .post(
                self.client
                    .api_url(&format!("products/qc-product-update/{product_id}")),
                req,
            )
            .await
    }

    /// `POST products/import`: bulk import products from a CSV file (multipart field `file`).
    pub async fn import(&self, file: &Upload) -> Result<ImportResponse, Error> {
        self.client
            .post_file(self.client.api_url("products/import"), "file", file, &[])
            .await
    }

    /// `GET products/sample`: sample CSV for [`Products::import`], returned as raw CSV text.
    pub async fn sample_csv(&self) -> Result<String, Error> {
        self.client
            .get_text(self.client.api_url("products/sample"))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_product_list() {
        let res: Paginated<Product> = serde_json::from_str(
            r#"{"data":[{"id":17484610,"sku":"chakra123","hsn":"441122","name":"Kunai","weight":"0 kg",
            "cost_price":"0.00","mrp":"0.00","low_stock":0,"quantity":41,"dimensions":"10 x 10 x 10 cm",
            "status":"INACTIVE","type":"Single"}],
            "meta":{"pagination":{"total":12080,"count":15,"per_page":15,"current_page":1,"total_pages":806,
            "links":{"next":"https://apiv2.shiprocket.in/v1/external/products?page=2"}}}}"#,
        )
        .unwrap();
        let p = &res.data[0];
        assert_eq!(p.id, Some(17484610));
        assert_eq!(p.sku.as_deref(), Some("chakra123"));
        assert_eq!(p.quantity, Some(41));
        assert_eq!(p.mrp, Some(0.0));
        assert_eq!(p.product_type.as_deref(), Some("Single"));
        let pagination = res.meta.unwrap().pagination.unwrap();
        assert_eq!(pagination.total_pages, Some(806));
    }

    #[test]
    fn decodes_product() {
        let res: DataResponse<Product> = serde_json::from_str(
            r#"{"data":{"id":17484610,"sku":"chakra123","name":"Kunai","category_code":"","weight":"0.000",
            "quantity":41,"dimensions":"0.00 x 0.00 x 0.00","status":"INACTIVE","is_combo":0}}"#,
        )
        .unwrap();
        let p = res.data.unwrap();
        assert_eq!(p.name.as_deref(), Some("Kunai"));
        assert_eq!(p.category_code, None);
        assert_eq!(p.is_combo, Some(false));
    }

    #[test]
    fn decodes_qc_update() {
        let res: MessageResponse = serde_json::from_str(
            r#"{"status":200,"message":"Product details updated successfully!"}"#,
        )
        .unwrap();
        assert_eq!(
            res.message.as_deref(),
            Some("Product details updated successfully!")
        );
    }

    #[test]
    fn serializes_add_request() {
        let req = AddProductRequest::builder()
            .name("Batman451")
            .sku("b118771212")
            .category_code("default")
            .r#type("Single")
            .qty(10)
            .hsn("4412")
            .qc_details(QcDetails::builder().brand("redlabel").size("L").build())
            .build();
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["HSN"], "4412");
        assert_eq!(v["type"], "Single");
        assert_eq!(v["qc_details"]["brand"], "redlabel");
        assert!(v.get("brand").is_none());
    }
}
