use std::fmt::{self, Formatter};

use chrono::{DateTime, Utc};
use serde::de::{Error, SeqAccess, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{Authentication, ClientExecutor, RequestBuilder};

/// Basic information about an account.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// A globally unique GUID for the account.
    pub id: String,
    /// The age of the account in seconds.
    pub age: u64,
    /// The unique display name of the account. Note that it is possible for the name to
    /// change.
    pub name: String,
    /// The home world of the account.
    pub world: u64,
    /// A list of guilds the account has joined.
    pub guilds: Vec<String>,
    /// A list of guilds the account has leader access to.
    ///
    /// **Requires the `guilds` scope.** This is `None` if the scope is missing.
    pub guild_leader: Option<Vec<String>>,
    /// The account creation date.
    pub created: DateTime<Utc>,
    /// A list of content the account has access to.
    pub access: AccountAccess,
    /// Whether the account has unlocked the commander tag.
    pub commander: bool,
    /// The fractal level of the account.
    ///
    /// **Requires the `progression` scope.** This is `None` if the scope is missing.
    pub fractal_level: Option<u8>,
    /// The number of daily achievement points unlocked by the account.
    ///
    /// **Requires the `progression` scope.** This is `None` if the scope is missing.
    pub daily_ap: Option<u16>,
    /// The number of monthly achievement points unlocked by the account.
    ///
    /// **Requires the `progression` scope.** This is `None` if the scope is missing.
    pub monthly_ap: Option<u16>,
    /// The WvW rank of the account.
    ///
    /// **Requires the `progression` scope.** This is `None` if the scope is missing.
    pub wvw_rank: Option<u16>,
    /// The date when the account information was last changed.
    pub last_modified: DateTime<Utc>,
    /// The amount of build storage slots unlocked by the account.
    ///
    /// **Requires the `builds` scope.** This is `None` if the scope is missing.
    pub build_storage_slots: Option<u64>,
}

impl Account {
    const URI: &'static str = "/v2/account";

    /// Returns the information about the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns information about the account of the current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::Account;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let account = Account::get(&client).await?;
    /// println!("{:?}", account);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::Account;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let account = Account::get(&client)?;
    /// println!("{:?}", account);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error`]: struct@crate::Error
    /// [`blocking`]: crate::blocking
    pub fn get<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        client.send(RequestBuilder::new(Self::URI).authenticated(Authentication::Required))
    }
}

/// A list of content an [`Account`] has access to.
#[derive(Copy, Clone, Debug)]
pub struct AccountAccess(u8);

impl AccountAccess {
    const NONE: u8 = 0;
    const PLAY_FOR_FREE: u8 = 1;
    const GUILD_WARS_2: u8 = 1 << 1;
    const HEART_OF_THORNS: u8 = 1 << 2;
    const PATH_OF_FIRE: u8 = 1 << 3;
    const END_OF_DRAGONS: u8 = 1 << 4;

    #[inline]
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns `true` if the account has no access.
    ///
    /// Note that this likely shouldn't never happen.
    #[inline]
    pub fn none(&self) -> bool {
        self.0 & Self::NONE != 0
    }

    /// Returns `true` if the account has free to play access.
    #[inline]
    pub fn play_for_free(&self) -> bool {
        self.0 & Self::PLAY_FOR_FREE != 0
    }

    /// Returns `true` if the account has access to the base game of Guild Wars 2.
    #[inline]
    pub fn guild_wars_2(&self) -> bool {
        self.0 & Self::GUILD_WARS_2 != 0
    }

    /// Returns `true` if the account has access to the Heart of Thorns expansion.
    #[inline]
    pub fn heart_of_thorns(&self) -> bool {
        self.0 & Self::HEART_OF_THORNS != 0
    }

    /// Returns `true` if the account has access to the Path of Fire expansion.
    #[inline]
    pub fn path_of_fire(&self) -> bool {
        self.0 & Self::PATH_OF_FIRE != 0
    }

    /// Returns `true` if the account has access to the End of Dragons expansion.
    #[inline]
    pub fn end_of_dragons(&self) -> bool {
        self.0 & Self::END_OF_DRAGONS != 0
    }
}

impl Serialize for AccountAccess {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;

        if self.0 & Self::NONE != 0 {
            seq.serialize_element("None")?;
        }

        if self.0 & Self::PLAY_FOR_FREE != 0 {
            seq.serialize_element("PlayForFree")?;
        }

        if self.0 & Self::GUILD_WARS_2 != 0 {
            seq.serialize_element("GuildWars2")?;
        }

        if self.0 & Self::HEART_OF_THORNS != 0 {
            seq.serialize_element("HeartOfThorns")?;
        }

        if self.0 & Self::PATH_OF_FIRE != 0 {
            seq.serialize_element("PathOfFire")?;
        }

        if self.0 & Self::END_OF_DRAGONS != 0 {
            seq.serialize_element("EndOfDragons")?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AccountAccess {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AccountAccessVisitor;

        impl<'de> Visitor<'de> for AccountAccessVisitor {
            type Value = AccountAccess;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("sequence")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut state = 0;

                loop {
                    let elem = seq.next_element::<&str>()?;

                    match elem {
                        Some("None") => state |= AccountAccess::NONE,
                        Some("PlayForFree") => state |= AccountAccess::PLAY_FOR_FREE,
                        Some("GuildWars2") => state |= AccountAccess::GUILD_WARS_2,
                        Some("HeartOfThorns") => state |= AccountAccess::HEART_OF_THORNS,
                        Some("PathOfFire") => state |= AccountAccess::PATH_OF_FIRE,
                        Some("EndOfDragons") => state |= AccountAccess::END_OF_DRAGONS,
                        Some(_) => return Err(A::Error::custom("invalid account access")),
                        None => return Ok(AccountAccess(state)),
                    }
                }
            }
        }

        deserializer.deserialize_seq(AccountAccessVisitor)
    }
}
