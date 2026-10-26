//! Tracking webhooks: the payload Shiprocket POSTs to your callback URL on every new tracking
//! event, the shipment status codes it carries, and verification of the `x-api-key` token.
//!
//! Framework-agnostic: hand in the raw body bytes and the `x-api-key` header value.
//!
//! ```
//! use shiprocket::webhook::{ShipmentStatus, TrackingWebhook, verify_api_key};
//!
//! # let header: Option<&str> = Some("my-token");
//! # let body: &[u8] = br#"{"awb":"19041424751540","shipment_status_id":18}"#;
//! if !verify_api_key(header, "my-token") {
//!     // reject (still answer 200 if you want Shiprocket to stop retrying)
//! }
//! let event = TrackingWebhook::from_slice(body).unwrap();
//! assert_eq!(event.status(), Some(ShipmentStatus::InTransit));
//! ```
//!
//! Shiprocket's notes for the endpoint: method `POST`, `Content-Type: application/json`, the
//! optional security token arrives as the `x-api-key` header, the URL must not contain the words
//! `shiprocket`, `kartrocket`, `sr` or `kr`, and it should always answer `200`.
//!
//! **Uncertainty:** Shiprocket's webhook reference is only a sample body, not a schema. The
//! fields below follow that sample and the crate's tracking types; every field is optional and
//! decoded leniently, and anything else lands in `extra`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{de, tracking::TrackActivity};

/// Header that carries the webhook security token.
pub const API_KEY_HEADER: &str = "x-api-key";

/// Body of a Shiprocket tracking webhook.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackingWebhook {
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier_name: Option<String>,
    /// Label of the latest event, e.g. `IN TRANSIT`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub current_status: Option<String>,
    /// Id of the latest event. In Shiprocket's sample this differs from
    /// [`shipment_status_id`](Self::shipment_status_id) for the same label (`20` vs `18` for
    /// `IN TRANSIT`), so it seems to use another code space; it is kept as a raw number.
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub current_status_id: Option<i64>,
    /// Shiprocket shipment status label, e.g. `IN TRANSIT`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub shipment_status: Option<String>,
    /// Shiprocket shipment status code; see [`TrackingWebhook::status`].
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_status_id: Option<i64>,
    /// Time of the event as sent, e.g. `23 05 2023 11:43:52` (`DD MM YYYY HH:MM:SS`, IST).
    #[serde(default, deserialize_with = "de::opt_string")]
    pub current_timestamp: Option<String>,
    /// Your (channel) order id, e.g. `1373900_150876814`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub order_id: Option<String>,
    /// Shiprocket's order id.
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub sr_order_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_assigned_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pickup_scheduled_date: Option<String>,
    /// Estimated time of delivery.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub etd: Option<String>,
    /// Scan history, oldest first in Shiprocket's sample.
    #[serde(default, deserialize_with = "de::vec")]
    pub scans: Vec<TrackActivity>,
    /// `0` / `1` in the payload.
    #[serde(default, deserialize_with = "de::opt_bool")]
    pub is_return: Option<bool>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub channel_id: Option<i64>,
    /// e.g. `OTP Based Delivery`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pod_status: Option<String>,
    /// Proof of delivery availability, e.g. `Not Available`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub pod: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub qc_image: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub qc_failure_reason: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

impl TrackingWebhook {
    /// Decodes a webhook body.
    pub fn from_slice(body: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(body)
    }

    /// [`shipment_status_id`](Self::shipment_status_id) as a [`ShipmentStatus`].
    pub fn status(&self) -> Option<ShipmentStatus> {
        self.shipment_status_id.map(ShipmentStatus::from)
    }
}

impl TrackActivity {
    /// `sr-status` as a [`ShipmentStatus`]; `None` when missing or `"NA"`.
    pub fn sr_status_code(&self) -> Option<ShipmentStatus> {
        self.sr_status
            .as_deref()
            .and_then(|s| s.trim().parse::<i64>().ok())
            .map(ShipmentStatus::from)
    }
}

macro_rules! shipment_statuses {
    ($($(#[$doc:meta])* $variant:ident = $id:literal,)*) => {
        /// Shiprocket shipment status code (`shipment_status_id`, `sr-status`,
        /// [`TrackingData::shipment_status`](crate::tracking::TrackingData::shipment_status)).
        ///
        /// Codes follow Shiprocket's published status list; codes not listed here (or added
        /// later) decode as [`ShipmentStatus::Unknown`]. Serializes as the number.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum ShipmentStatus {
            $($(#[$doc])* $variant,)*
            /// A code this crate does not know.
            Unknown(i64),
        }

        impl ShipmentStatus {
            /// The numeric code.
            pub fn id(self) -> i64 {
                match self {
                    $(Self::$variant => $id,)*
                    Self::Unknown(id) => id,
                }
            }
        }

        impl From<i64> for ShipmentStatus {
            fn from(id: i64) -> Self {
                match id {
                    $($id => Self::$variant,)*
                    other => Self::Unknown(other),
                }
            }
        }
    };
}

shipment_statuses! {
    ManifestGenerated = 5,
    Shipped = 6,
    Delivered = 7,
    Canceled = 8,
    RtoInitiated = 9,
    RtoDelivered = 10,
    Lost = 12,
    PickupError = 13,
    RtoAcknowledged = 14,
    PickupRescheduled = 15,
    CancellationRequested = 16,
    OutForDelivery = 17,
    InTransit = 18,
    OutForPickup = 19,
    PickupException = 20,
    Undelivered = 21,
    Delayed = 22,
    PartialDelivered = 23,
    Destroyed = 24,
    Damaged = 25,
    Fulfilled = 26,
    ReachedAtDestinationHub = 38,
    Misrouted = 39,
    RtoNdr = 40,
    RtoOutForDelivery = 41,
    PickedUp = 42,
    SelfFulfilled = 43,
    DisposedOff = 44,
    CancelledBeforeDispatched = 45,
    RtoInTransit = 46,
    QcFailed = 47,
    ReachedWarehouse = 48,
    CustomCleared = 49,
    InFlight = 50,
    HandoverToCourier = 51,
    ShipmentBooked = 52,
    InTransitOverseas = 54,
    ConnectionAligned = 55,
    ReachedOverseasWarehouse = 56,
    CustomClearedOverseas = 57,
    BoxPacking = 59,
    FcAllocated = 60,
    PicklistGenerated = 61,
    ReadyToPack = 62,
    Packed = 63,
    FcManifestGenerated = 67,
    ProcessedAtWarehouse = 68,
    HandoverException = 71,
    PackedException = 72,
    RtoLock = 75,
    Untraceable = 76,
    IssueRelatedToTheRecipient = 77,
    ReachedBackAtSellerCity = 78,
}

impl Serialize for ShipmentStatus {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(self.id())
    }
}

/// Accepts a number or a numeric string.
impl<'de> Deserialize<'de> for ShipmentStatus {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        de::opt_i64(d)?
            .map(ShipmentStatus::from)
            .ok_or_else(|| serde::de::Error::custom("expected a numeric shipment status"))
    }
}

/// Checks the `x-api-key` header value against the security token configured in Shiprocket,
/// in constant time (for a given token length).
///
/// Returns `false` when the header is missing or `expected_token` is empty (fail closed: if
/// you did not configure a token, do not call this).
pub fn verify_api_key(header_value: Option<&str>, expected_token: &str) -> bool {
    match header_value {
        Some(value) if !expected_token.is_empty() => {
            constant_time_eq(value.as_bytes(), expected_token.as_bytes())
        }
        _ => false,
    }
}

/// Compares every byte up to the longer length, so the time does not depend on where the
/// inputs differ.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let len = a.len().max(b.len());
    let mut diff = a.len() ^ b.len();
    for i in 0..len {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        diff |= usize::from(std::hint::black_box(x ^ y));
    }
    std::hint::black_box(diff) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "awb":"19041424751540",
        "courier_name":"Delhivery Surface",
        "current_status":"IN TRANSIT",
        "current_status_id":20,
        "shipment_status":"IN TRANSIT",
        "shipment_status_id":18,
        "current_timestamp":"23 05 2023 11:43:52",
        "order_id":"1373900_150876814",
        "sr_order_id":348456385,
        "awb_assigned_date":"2023-05-19 11:59:16",
        "pickup_scheduled_date":"2023-05-19 11:59:17",
        "etd":"2023-05-23 15:40:19",
        "scans":[
            {"date":"2023-05-19 11:59:16","status":"X-UCI","activity":"Manifested - Manifest uploaded","location":"Chomu_SamodRd_D (Rajasthan)","sr-status":"5","sr-status-label":"MANIFEST GENERATED"},
            {"date":"2023-05-19 17:19:14","status":"X-DBL1F","activity":"In Transit - Added to Bag","location":"Chomu_SamodRd_D (Rajasthan)","sr-status":"NA","sr-status-label":"NA"},
            {"date":"2023-05-20 10:27:56","status":"X-DLL2F","activity":"In Transit - Bag Added To Trip","location":"Chomu_SamodRd_D (Rajasthan)","sr-status":"18","sr-status-label":"IN TRANSIT"}
        ],
        "is_return":0,
        "channel_id":3422553,
        "pod_status":"OTP Based Delivery",
        "pod":"Not Available",
        "qc_image":"",
        "qc_failure_reason":"",
        "new_field":"kept"
    }"#;

    #[test]
    fn decodes_sample_payload() {
        let w = TrackingWebhook::from_slice(SAMPLE.as_bytes()).unwrap();
        assert_eq!(w.awb.as_deref(), Some("19041424751540"));
        assert_eq!(w.courier_name.as_deref(), Some("Delhivery Surface"));
        assert_eq!(w.current_status.as_deref(), Some("IN TRANSIT"));
        assert_eq!(w.current_status_id, Some(20));
        assert_eq!(w.shipment_status_id, Some(18));
        assert_eq!(w.status(), Some(ShipmentStatus::InTransit));
        assert_eq!(w.current_timestamp.as_deref(), Some("23 05 2023 11:43:52"));
        assert_eq!(w.order_id.as_deref(), Some("1373900_150876814"));
        assert_eq!(w.sr_order_id, Some(348456385));
        assert_eq!(w.etd.as_deref(), Some("2023-05-23 15:40:19"));
        assert_eq!(w.is_return, Some(false));
        assert_eq!(w.channel_id, Some(3422553));
        assert_eq!(w.qc_image, None);
        assert_eq!(w.extra["new_field"], "kept");

        assert_eq!(w.scans.len(), 3);
        assert_eq!(w.scans[0].status.as_deref(), Some("X-UCI"));
        assert_eq!(
            w.scans[0].sr_status_code(),
            Some(ShipmentStatus::ManifestGenerated)
        );
        assert_eq!(w.scans[1].sr_status.as_deref(), Some("NA"));
        assert_eq!(w.scans[1].sr_status_code(), None);
        assert_eq!(w.scans[2].sr_status_label.as_deref(), Some("IN TRANSIT"));
    }

    #[test]
    fn decodes_lenient_and_minimal_payloads() {
        let w = TrackingWebhook::from_slice(
            br#"{"awb":19041424751540,"shipment_status_id":"7","order_id":42,"is_return":"1","scans":null}"#,
        )
        .unwrap();
        assert_eq!(w.awb.as_deref(), Some("19041424751540"));
        assert_eq!(w.status(), Some(ShipmentStatus::Delivered));
        assert_eq!(w.order_id.as_deref(), Some("42"));
        assert_eq!(w.is_return, Some(true));
        assert!(w.scans.is_empty());

        let w = TrackingWebhook::from_slice(b"{}").unwrap();
        assert_eq!(w.status(), None);
        assert!(TrackingWebhook::from_slice(b"not json").is_err());
    }

    #[test]
    fn status_codes_round_trip() {
        assert_eq!(ShipmentStatus::from(42), ShipmentStatus::PickedUp);
        assert_eq!(ShipmentStatus::from(999), ShipmentStatus::Unknown(999));
        assert_eq!(ShipmentStatus::Unknown(999).id(), 999);
        for id in 0..100 {
            assert_eq!(ShipmentStatus::from(id).id(), id);
        }
        assert_eq!(
            serde_json::to_value(ShipmentStatus::Delivered).unwrap(),
            serde_json::json!(7)
        );
        let s: Vec<ShipmentStatus> = serde_json::from_str(r#"[8, "17", 1234]"#).unwrap();
        assert_eq!(
            s,
            vec![
                ShipmentStatus::Canceled,
                ShipmentStatus::OutForDelivery,
                ShipmentStatus::Unknown(1234)
            ]
        );
        assert!(serde_json::from_str::<ShipmentStatus>("\"NA\"").is_err());
    }

    #[test]
    fn verifies_api_key() {
        assert!(verify_api_key(Some("s3cret-token"), "s3cret-token"));
        assert!(!verify_api_key(Some("s3cret-tokeN"), "s3cret-token"));
        assert!(!verify_api_key(Some("s3cret-token "), "s3cret-token"));
        assert!(!verify_api_key(Some("s3cret"), "s3cret-token"));
        assert!(!verify_api_key(Some(""), "s3cret-token"));
        assert!(!verify_api_key(None, "s3cret-token"));
        // No token configured never verifies.
        assert!(!verify_api_key(Some(""), ""));
        assert!(!verify_api_key(None, ""));
    }
}
