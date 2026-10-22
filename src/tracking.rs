//! Tracking: shipment tracking by AWB, shipment id or order id.

use std::collections::HashMap;

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{Error, Shiprocket, de};

/// Body of `POST courier/track/awbs`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct TrackAwbsRequest {
    /// AWB codes, max 50 per call.
    pub awbs: Vec<String>,
}

/// Query of `GET courier/track`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct TrackOrderQuery {
    /// Order id / number of your store (channel order id).
    pub order_id: String,
    /// Needed when the same order id exists in more than one channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<i64>,
}

/// `{ "tracking_data": {...} }` wrapper returned by every tracking endpoint.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackingResponse {
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub tracking_data: Option<TrackingData>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Tracking details of one shipment.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackingData {
    /// `1` when tracking data is available, `0` otherwise (see [`TrackingData::error`]).
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub track_status: Option<i64>,
    /// Shiprocket shipment status code.
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_status: Option<i64>,
    #[serde(default, deserialize_with = "de::vec")]
    pub shipment_track: Vec<ShipmentTrack>,
    #[serde(default, deserialize_with = "de::vec")]
    pub shipment_track_activities: Vec<TrackActivity>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub track_url: Option<String>,
    /// Estimated time of delivery.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub etd: Option<String>,
    #[serde(default, deserialize_with = "de::opt_struct")]
    pub qc_response: Option<QcResponse>,
    /// Set when no tracking activity exists yet.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub error: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Summary row of `shipment_track`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ShipmentTrack {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub courier_company_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivered_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub weight: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub packages: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub current_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivered_to: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub destination: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub consignee_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub origin: Option<String>,
    #[serde(default)]
    pub courier_agent_details: serde_json::Value,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    /// Estimated delivery date.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub edd: Option<String>,
    /// Proof of delivery availability, e.g. `Available`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pod: Option<String>,
    /// Proof of delivery image url.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pod_status: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// One scan event of `shipment_track_activities`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackActivity {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub date: Option<String>,
    /// Courier specific status code, e.g. `DLVD`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub activity: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub location: Option<String>,
    /// Shiprocket status code (`"NA"` when unmapped).
    #[serde(rename = "sr-status", default, deserialize_with = "de::opt_string")]
    pub sr_status: Option<String>,
    #[serde(
        rename = "sr-status-label",
        default,
        deserialize_with = "de::opt_string"
    )]
    pub sr_status_label: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Quality check result for return shipments.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct QcResponse {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub qc_image: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub qc_failed_reason: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Tracking endpoints. Obtain with [`Shiprocket::tracking`].
pub struct Tracking<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn tracking(&self) -> Tracking<'_> {
        Tracking { client: self }
    }
}

impl Tracking<'_> {
    /// `GET courier/track/awb/{awb_code}`: track a shipment by AWB.
    pub async fn by_awb(&self, awb_code: &str) -> Result<TrackingResponse, Error> {
        self.client
            .get(
                self.client
                    .api_url(&format!("courier/track/awb/{awb_code}")),
            )
            .await
    }

    /// `POST courier/track/awbs`: track up to 50 AWBs at once. Keyed by AWB code;
    /// unknown AWBs are simply missing from the map.
    pub async fn by_awbs(
        &self,
        req: &TrackAwbsRequest,
    ) -> Result<HashMap<String, TrackingResponse>, Error> {
        self.client
            .post(self.client.api_url("courier/track/awbs"), req)
            .await
    }

    /// `GET courier/track/shipment/{shipment_id}`: track by Shiprocket shipment id.
    pub async fn by_shipment(&self, shipment_id: i64) -> Result<TrackingResponse, Error> {
        self.client
            .get(
                self.client
                    .api_url(&format!("courier/track/shipment/{shipment_id}")),
            )
            .await
    }

    /// `GET courier/track?order_id=..&channel_id=..`: track by store order id.
    /// Returns one entry per shipment of the order.
    pub async fn by_order(&self, query: &TrackOrderQuery) -> Result<Vec<TrackingResponse>, Error> {
        self.client
            .get_query(self.client.api_url("courier/track"), query)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_awb_tracking() {
        let res: TrackingResponse = serde_json::from_str(
            r#"{"tracking_data":{"track_status":1,"shipment_status":7,"shipment_track":[{"id":236612717,"awb_code":"141123221084922","courier_company_id":51,"shipment_id":236612717,"order_id":237157589,"pickup_date":"2022-07-18 20:28:00","delivered_date":"2022-07-19 11:37:00","weight":"0.30","packages":1,"current_status":"Delivered","delivered_to":"Chittoor","destination":"Chittoor","consignee_name":"","origin":"Banglore","courier_agent_details":null,"courier_name":"Xpressbees Surface","edd":null,"pod":"Available","pod_status":"https://example.com/pod.png"}],"shipment_track_activities":[{"date":"2022-07-19 11:37:00","status":"DLVD","activity":"Delivered","location":"MADANPALLI","sr-status":"7","sr-status-label":"DELIVERED"}],"track_url":"https://shiprocket.co//tracking/141123221084922","etd":"2022-07-20 19:28:00","qc_response":{"qc_image":"","qc_failed_reason":""}}}"#,
        )
        .unwrap();
        let data = res.tracking_data.unwrap();
        assert_eq!(data.shipment_status, Some(7));
        let track = &data.shipment_track[0];
        assert_eq!(track.awb_code.as_deref(), Some("141123221084922"));
        assert_eq!(track.weight, Some(0.30));
        assert_eq!(track.consignee_name, None);
        let act = &data.shipment_track_activities[0];
        assert_eq!(act.sr_status.as_deref(), Some("7"));
        assert_eq!(act.sr_status_label.as_deref(), Some("DELIVERED"));
        assert!(data.qc_response.is_some());
    }

    #[test]
    fn decodes_multiple_awbs() {
        let res: HashMap<String, TrackingResponse> = serde_json::from_str(
            r#"{"788829354408":{"tracking_data":{"track_status":1,"shipment_status":1,"shipment_track":[{"id":8067757,"awb_code":"788829354408","courier_company_id":2,"shipment_id":null,"order_id":16240551,"weight":"2.5","packages":1,"current_status":"AWB Assigned"}],"shipment_track_activities":[{"date":"2019-08-01 02:05:05","activity":"Shipment information sent to FedEx - OC","location":"NA"}],"track_url":"https://app.shiprocket.in/tracking/awb/788829354408"}}}"#,
        )
        .unwrap();
        let data = res["788829354408"].tracking_data.as_ref().unwrap();
        assert_eq!(data.shipment_track[0].shipment_id, None);
        assert_eq!(data.shipment_track[0].order_id, Some(16240551));

        let empty: HashMap<String, TrackingResponse> = serde_json::from_str("{}").unwrap();
        assert!(empty.is_empty());
    }

    #[test]
    fn decodes_no_activity_error() {
        let res: TrackingResponse = serde_json::from_str(
            r#"{"tracking_data":{"track_status":0,"error":"Aahh! There is no activities found in our DB."}}"#,
        )
        .unwrap();
        let data = res.tracking_data.unwrap();
        assert_eq!(data.track_status, Some(0));
        assert!(data.error.is_some());
        assert!(data.shipment_track.is_empty());
    }

    #[test]
    fn decodes_order_tracking() {
        let res: Vec<TrackingResponse> = serde_json::from_str(
            r#"[{"tracking_data":{"track_status":1,"shipment_status":42,"shipment_track":[{"id":185584215,"awb_code":"1091188857722","edd":"2021-12-27 23:23:18"}],"shipment_track_activities":[{"date":"2021-12-23 14:23:18","status":"X-PPOM","sr-status":"42"}],"etd":"2021-12-28 10:19:35"}}]"#,
        )
        .unwrap();
        let data = res[0].tracking_data.as_ref().unwrap();
        assert_eq!(data.shipment_status, Some(42));
        assert_eq!(
            data.shipment_track[0].edd.as_deref(),
            Some("2021-12-27 23:23:18")
        );
    }

    #[test]
    fn serializes_order_query() {
        let q = TrackOrderQuery::builder().order_id("NO-123").build();
        assert_eq!(
            serde_json::to_value(&q).unwrap(),
            serde_json::json!({"order_id":"NO-123"})
        );
    }
}
