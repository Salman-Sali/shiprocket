//! File imports: check the result of bulk CSV imports (orders, products, listings).

use serde::Deserialize;

use crate::{Error, Shiprocket, de, types::DataResponse};

/// `data` of `GET errors/{import_id}/check`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ImportResult {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// File import endpoints. Obtain with [`Shiprocket::imports`].
pub struct Imports<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn imports(&self) -> Imports<'_> {
        Imports { client: self }
    }
}

impl Imports<'_> {
    /// `GET errors/{import_id}/check`: result and errors of a file import.
    pub async fn check(&self, import_id: i64) -> Result<DataResponse<ImportResult>, Error> {
        self.client
            .get(self.client.api_url(&format!("errors/{import_id}/check")))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_import_result() {
        let res: DataResponse<ImportResult> = serde_json::from_str(
            r#"{"data":{"status":"3","message":"Error in reading file data!"}}"#,
        )
        .unwrap();
        let data = res.data.unwrap();
        assert_eq!(data.status, Some(3));
        assert_eq!(data.message.as_deref(), Some("Error in reading file data!"));
    }

    #[test]
    fn decodes_import_response() {
        let res: crate::types::ImportResponse =
            serde_json::from_str(r#"{"id": 20290943}"#).unwrap();
        assert_eq!(res.id, Some(20290943));
    }
}
