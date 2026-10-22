//! NDR (non-delivery report): list shipments in NDR and act on them.

use bon::Builder;
use serde::{Deserialize, Serialize};

use crate::{Error, Paginated, Shiprocket, de};

/// Query of `GET ndr/all`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct ListNdrQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    /// Start date, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// End date, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Search by AWB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// A shipment in NDR.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NdrShipment {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_id: Option<i64>,
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
    pub payment_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub status_code: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub payment_method: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub created_at: Option<String>,
    /// NDR reason reported by the courier.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub reason: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub attempts: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub ndr_raised_at: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub courier: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub awb_code: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub escalation_status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub product_name: Option<String>,
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub product_price: Option<f64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub shipment_channel_id: Option<i64>,
    #[serde(default, deserialize_with = "de::vec")]
    pub history: Vec<NdrHistory>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub delivered_date: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// One NDR attempt of a shipment.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NdrHistory {
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub ndr_id: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub ndr_reason: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub action_by: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub ndr_attempt: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub medium: Option<String>,
    #[serde(default, deserialize_with = "de::opt_i64")]
    pub ndr_push_status: Option<i64>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub comment: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub call_center_call_recording: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub call_center_recording_date: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub proof_recording: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub proof_image: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub sms_response: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub ndr_raised_at: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Action to take on a shipment in NDR.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NdrAction {
    /// Ask the courier to attempt delivery again.
    ReAttempt,
    /// Dispute the courier's attempt as fake. Needs `proof_audio`/`proof_image` and `remarks`.
    FakeAttempt,
    /// Return the shipment to origin (RTO).
    Return,
}

/// Body of `POST ndr/{awb}/action`.
#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
#[builder(on(String, into))]
pub struct NdrActionRequest {
    pub action: NdrAction,
    pub comments: String,
    /// Updated customer phone, for re-attempt and fake-attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Audio proof url, for fake-attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_audio: Option<String>,
    /// Image proof url, for fake-attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_image: Option<String>,
    /// Remarks, for fake-attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// Updated address line 1, for re-attempt and fake-attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    /// Updated address line 2, for re-attempt and fake-attempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// Preferred delivery date, `yyyy-mm-dd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred_date: Option<String>,
}

/// Response of `POST ndr/{awb}/action`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct NdrActionResponse {
    /// e.g. `Data Updated Sucessfully`.
    #[serde(default, deserialize_with = "de::opt_string")]
    pub status: Option<String>,
    #[serde(default, deserialize_with = "de::opt_string")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// NDR endpoints. Obtain with [`Shiprocket::ndr`].
pub struct Ndr<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn ndr(&self) -> Ndr<'_> {
        Ndr { client: self }
    }
}

impl Ndr<'_> {
    /// `GET ndr/all`: shipments currently in NDR.
    pub async fn list(&self, query: &ListNdrQuery) -> Result<Paginated<NdrShipment>, Error> {
        self.client
            .get_query(self.client.api_url("ndr/all"), query)
            .await
    }

    /// `GET ndr/{awb}`: NDR details of one shipment.
    pub async fn get(&self, awb: &str) -> Result<Paginated<NdrShipment>, Error> {
        self.client
            .get(self.client.api_url(&format!("ndr/{awb}")))
            .await
    }

    /// `POST ndr/{awb}/action`: re-attempt, dispute (fake-attempt) or return a shipment.
    pub async fn action(
        &self,
        awb: &str,
        req: &NdrActionRequest,
    ) -> Result<NdrActionResponse, Error> {
        self.client
            .post(self.client.api_url(&format!("ndr/{awb}/action")), req)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHIPMENTS: &str = r##"{"data":[{"id":94711332,"shipment_id":94323606,"customer_name":"John Doe","customer_email":"john@gmail.com","customer_phone":"999999998","customer_address":"#123","customer_address_2":"near view estate","customer_city":"Gurgaon","customer_state":"Haryana","customer_pincode":"122003","payment_status":"2","status":"UNDELIVERED","status_code":36,"payment_method":"prepaid","created_at":"12 Mar 2021, 04:16 PM","reason":"Customer Asked For Future Delivery","attempts":1,"ndr_raised_at":"2021-03-17 22:51:17","courier":"FedEx","awb_code":"8373927474982","escalation_status":"N/A","product_name":"Gaming Chair","product_price":"14484.01","shipment_channel_id":152865,"history":[{"id":50347646,"ndr_id":14933497,"ndr_reason":"Customer Asked For Future Delivery","action_by":3,"ndr_attempt":1,"medium":null,"ndr_push_status":0,"comment":"","call_center_call_recording":"","call_center_recording_date":"","proof_recording":null,"proof_image":null,"sms_response":"No Response","ndr_raised_at":"2021-03-17 22:51:17"}],"delivered_date":""}],"meta":{"pagination":{"total":1,"count":1,"per_page":15,"current_page":1,"total_pages":1,"links":{}}}}"##;

    #[test]
    fn ndr_shipments() {
        let r: Paginated<NdrShipment> = serde_json::from_str(SHIPMENTS).unwrap();
        let s = &r.data[0];
        assert_eq!(s.id, Some(94711332));
        assert_eq!(s.status_code, Some(36));
        assert_eq!(s.product_price, Some(14484.01));
        assert_eq!(s.awb_code.as_deref(), Some("8373927474982"));
        assert_eq!(s.delivered_date, None);
        assert_eq!(s.history[0].ndr_id, Some(14933497));
        assert_eq!(s.history[0].proof_image, None);
        assert_eq!(r.meta.unwrap().pagination.unwrap().per_page, Some(15));
    }

    #[test]
    fn action_response() {
        let r: NdrActionResponse =
            serde_json::from_str(r#"{"status":"Data Updated Sucessfully"}"#).unwrap();
        assert_eq!(r.status.as_deref(), Some("Data Updated Sucessfully"));
    }

    #[test]
    fn action_serializes_kebab_case() {
        let req = NdrActionRequest::builder()
            .action(NdrAction::FakeAttempt)
            .comments("buyer was home")
            .build();
        let v = serde_json::to_value(&req).unwrap();
        assert_eq!(v["action"], "fake-attempt");
        assert!(v.get("phone").is_none());
        assert_eq!(
            serde_json::to_value(NdrAction::ReAttempt).unwrap(),
            "re-attempt"
        );
    }
}
