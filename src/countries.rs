//! Countries: country codes, zones and postcode locality lookup.

use serde::{Deserialize, Serialize};

use crate::{Error, Shiprocket, de};

/// Response of `GET countries`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CountryList {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::vec")]
    pub data: Vec<Country>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Country {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub iso_code_2: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub iso_code_3: Option<String>,
    /// E.g. `+93`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub isd_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub address_format: Option<String>,
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub postcode_required: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `GET countries/show/{country_id}`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ZoneList {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(default, deserialize_with = "de::vec")]
    pub data: Vec<Zone>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A zone (state/region) within a country.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Zone {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub country_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub state_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status: Option<i64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Response of `GET open/postcode/details`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PostcodeDetailsResponse {
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub success: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub postcode_details: Option<PostcodeDetails>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PostcodeDetails {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub postcode: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub city: Option<String>,
    #[serde(default, deserialize_with = "de::vec")]
    pub locality: Vec<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub state: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub state_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub longitude: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub latitude: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Serialize)]
struct PostcodeQuery<'a> {
    postcode: &'a str,
}

/// Country endpoints. Obtain with [`Shiprocket::countries`].
pub struct Countries<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn countries(&self) -> Countries<'_> {
        Countries { client: self }
    }
}

impl Countries<'_> {
    /// `GET countries`: all countries with their ids and ISO/ISD codes.
    pub async fn list(&self) -> Result<CountryList, Error> {
        self.client.get(self.client.api_url("countries")).await
    }

    /// `GET countries/show/{country_id}`: zones of a country.
    pub async fn zones(&self, country_id: i64) -> Result<ZoneList, Error> {
        self.client
            .get(self.client.api_url(&format!("countries/show/{country_id}")))
            .await
    }

    /// `GET open/postcode/details?postcode=`: city, state and localities of a pincode.
    pub async fn postcode_details(&self, postcode: &str) -> Result<PostcodeDetailsResponse, Error> {
        self.client
            .get_query(
                self.client.api_url("open/postcode/details"),
                &PostcodeQuery { postcode },
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_countries() {
        let res: CountryList = serde_json::from_str(
            r#"{"status":200,"data":[{"id":1,"name":"Afghanistan","iso_code_2":"AF","iso_code_3":"AFG",
            "isd_code":"+93","address_format":"","postcode_required":1,"status":1}]}"#,
        )
        .unwrap();
        assert_eq!(res.status, Some(200));
        let c = &res.data[0];
        assert_eq!(c.iso_code_3.as_deref(), Some("AFG"));
        assert_eq!(c.postcode_required, Some(true));
        assert_eq!(c.address_format, None);
    }

    #[test]
    fn decodes_zones() {
        let res: ZoneList = serde_json::from_str(
            r#"{"status":200,"data":[{"id":117,"country_id":4,"code":"E","state_code":null,"name":"Eastern","status":1}]}"#,
        )
        .unwrap();
        let z = &res.data[0];
        assert_eq!(z.country_id, Some(4));
        assert_eq!(z.name.as_deref(), Some("Eastern"));
        assert_eq!(z.state_code, None);
    }

    #[test]
    fn decodes_postcode_details() {
        let res: PostcodeDetailsResponse = serde_json::from_str(
            r#"{"success":true,"postcode_details":{"postcode":"110077","city":"South West Delhi",
            "locality":["Bagdola","Barthal"],"state":"Delhi","state_code":"DL","longitude":"77.399","latitude":"28.2636"}}"#,
        )
        .unwrap();
        let d = res.postcode_details.unwrap();
        assert_eq!(d.state_code.as_deref(), Some("DL"));
        assert_eq!(d.locality.len(), 2);
        assert_eq!(d.latitude, Some(28.2636));
    }
}
