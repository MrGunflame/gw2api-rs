use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Authentication, ClientExecutor, RequestBuilder};

/// Details about an api token.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    /// The unique id of the token.
    pub id: String,
    /// The name of the token defined by the user.
    pub name: String,
    /// All permissions granted to the token.
    pub permissions: Vec<TokenPermission>,
    /// The type of the token
    #[serde(rename = "type")]
    pub kind: TokenKind,
    /// Expiration time of the token. Only avaliable on subtokens.
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    /// Creation time of the token. Only avaliable on subtokens.
    pub issued_at: Option<DateTime<Utc>>,
    /// A list of urls the subtoken is restricted to. (optional)
    pub urls: Option<Vec<String>>,
}

/// A token permission.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TokenPermission {
    Account,
    Builds,
    Characters,
    Guilds,
    Inventories,
    Progression,
    Pvp,
    TradingPost,
    Unlocks,
    Wallet,
}

/// Type of an api token.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TokenKind {
    ApiKey,
    Subtoken,
}

impl TokenInfo {
    pub fn get<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        client.send(RequestBuilder::new("/v2/tokeninfo").authenticated(Authentication::Required))
    }
}
