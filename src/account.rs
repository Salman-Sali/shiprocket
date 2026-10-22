//! Account: wallet balance.

use serde::Deserialize;

use crate::{Error, Shiprocket, de, types::DataResponse};

/// `data` of `GET account/details/wallet-balance`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct WalletBalance {
    /// Balance in INR. Negative when the account owes Shiprocket.
    #[serde(default, deserialize_with = "de::opt_f64")]
    pub balance_amount: Option<f64>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Account endpoints. Obtain with [`Shiprocket::account`].
pub struct Account<'a> {
    client: &'a Shiprocket,
}

impl Shiprocket {
    pub fn account(&self) -> Account<'_> {
        Account { client: self }
    }
}

impl Account<'_> {
    /// `GET account/details/wallet-balance`.
    pub async fn wallet_balance(&self) -> Result<DataResponse<WalletBalance>, Error> {
        self.client
            .get(self.client.api_url("account/details/wallet-balance"))
            .await
    }
}
