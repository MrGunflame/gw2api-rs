//! Exposes details about player accounts. All endpoints in this module required authentication.

use std::collections::HashMap;
use std::fmt::{self, Formatter};

use chrono::{DateTime, Utc};
use serde::de::{Error, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccountAccess(u8);

impl AccountAccess {
    const NONE: u8 = 1 << 0;
    const PLAY_FOR_FREE: u8 = 1 << 1;
    const GUILD_WARS_2: u8 = 1 << 2;
    const HEART_OF_THORNS: u8 = 1 << 3;
    const PATH_OF_FIRE: u8 = 1 << 4;
    const END_OF_DRAGONS: u8 = 1 << 5;

    const NONE_STR: &'static str = "None";
    const PLAY_FOR_FREE_STR: &'static str = "PlayForFree";
    const GUILD_WARS_2_STR: &'static str = "GuildWars2";
    const HEART_OF_THORNS_STR: &'static str = "HeartOfThorns";
    const PATH_OF_FIRE_STR: &'static str = "PathOfFire";
    const END_OF_DRAGONS_STR: &'static str = "EndOfDragons";

    #[inline]
    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    /// Returns `true` if the account has no access.
    ///
    /// Note that this probably shouldn't ever happen.
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

        // if self.none() {
        //     seq.serialize_element(Self::NONE_STR)?;
        // }

        if self.play_for_free() {
            seq.serialize_element(Self::PLAY_FOR_FREE_STR)?;
        }

        if self.guild_wars_2() {
            seq.serialize_element(Self::GUILD_WARS_2_STR)?;
        }

        if self.heart_of_thorns() {
            seq.serialize_element(Self::HEART_OF_THORNS_STR)?;
        }

        if self.path_of_fire() {
            seq.serialize_element(Self::PATH_OF_FIRE_STR)?;
        }

        if self.end_of_dragons() {
            seq.serialize_element(Self::END_OF_DRAGONS_STR)?;
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
                formatter.write_str("a sequence of access strings")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut state = 0;

                loop {
                    let elem = seq.next_element::<&str>()?;

                    match elem {
                        Some(AccountAccess::NONE_STR) => {
                            state |= AccountAccess::NONE;
                        }
                        Some(AccountAccess::PLAY_FOR_FREE_STR) => {
                            state |= AccountAccess::PLAY_FOR_FREE;
                        }
                        Some(AccountAccess::GUILD_WARS_2_STR) => {
                            state |= AccountAccess::GUILD_WARS_2;
                        }
                        Some(AccountAccess::HEART_OF_THORNS_STR) => {
                            state |= AccountAccess::HEART_OF_THORNS;
                        }
                        Some(AccountAccess::PATH_OF_FIRE_STR) => {
                            state |= AccountAccess::PATH_OF_FIRE;
                        }
                        Some(AccountAccess::END_OF_DRAGONS_STR) => {
                            state |= AccountAccess::END_OF_DRAGONS;
                        }
                        Some(_) => return Err(A::Error::custom("invalid account access")),
                        None => return Ok(AccountAccess(state)),
                    }
                }
            }
        }

        deserializer.deserialize_seq(AccountAccessVisitor)
    }
}

/// A list of achievements unlocked by the account.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountAchievements(pub Vec<AccountAchievement>);

impl AccountAchievements {
    const URI: &'static str = "/v2/account/achievements";

    /// Returns a list of achievements unlocked by the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns a list of achievements unlocked by the account of the current
    /// access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountAchievements;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let achievements = AccountAchievements::get(&client).await?;
    /// println!("{:?}", achievements);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountAchievements;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let achievements = AccountAchievements::get(&client)?;
    /// println!("{:?}", achievements);
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

/// An achievement unlocked by an account.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountAchievement {
    /// The id of the achievement.
    pub id: u64,
    pub bits: Option<Vec<u64>>,
    /// The current progress towards the achievement.
    pub current: Option<u64>,
    /// The amount if objectives required to complete the achievement.
    pub max: Option<u64>,
    /// Whether the achievement has been completed.
    pub done: bool,
    /// The number of times this achievement has been completed. Only avaliable for repeatable
    /// achievements.
    pub repeated: Option<u64>,
    /// Whether the achievement has been unlocked (not completed).
    ///
    /// Note that a `None` value also indicates that the achievment is unlocked.
    pub unlocked: Option<bool>,
}

impl AccountAchievement {
    /// Returns `true` if this achievement is unlocked.
    #[inline]
    pub fn is_unlocked(&self) -> bool {
        self.unlocked.unwrap_or(true)
    }
}

/// A list of items stored in the account's bank/vault.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountBank(pub Vec<Option<BankItem>>);

impl AccountBank {
    const URI: &'static str = "/v2/account/bank";

    /// Returns a list of items stored in the account's bank.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns the account's bank of the current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountBank;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let bank = AccountBank::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountBank;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let bank = AccountBank::get(&client)?;
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

/// A single item stored in an [`AccountBank`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BankItem {
    /// The id of the item.
    pub id: u64,
    /// The number of items stored on this stack.
    pub count: u16,
    /// The number of charges remaining on this item. Only avaliable for some items.
    pub charges: Option<u64>,
    /// The id of the skin applied on this item. Only avaliable for some items.
    pub skin: Option<u64>,
    /// A list of ids of dyes applied on this item. Only avaliable for some items.
    pub dyes: Option<Vec<u64>>,
    /// A list of upgrades applied on this item. Only avaliable for some items.
    pub upgrades: Option<Vec<u64>>,
    pub upgrade_slot_indices: Option<Vec<u64>>,
    /// A list of infusions applied on this item. Only avaliable for some items.
    pub infusions: Option<Vec<u64>>,
    /// Whom the item is bound to. If `None` the item is not bound at all.
    pub binding: Option<ItemBinding>,
    /// The name of the character the item is bound to. Only avaliable if `binding` is
    /// `Character`.
    pub bound_to: Option<String>,
    pub stats: Option<Vec<ItemStats>>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemBinding {
    Account,
    Character,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ItemStats {
    pub id: u64,
    pub attributes: HashMap<String, f64>,
}

/// A list of items that have been crafted by the account since daily reset.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountDailyCrafting(pub Vec<String>);

impl AccountDailyCrafting {
    const URI: &'static str = "/v2/account/dailycrafting";

    /// Returns a list of items that have been crafted by the currently authenticated account since
    /// daily reset.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns a list of items crafted by the account of the current access
    /// token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountDailyCrafting;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let items = AccountDailyCrafting::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountDailyCrafting;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let items = AccountDailyCrafting::get(&client)?;
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

/// A list of dungeon paths completed since daily reset.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountDungeons(pub Vec<String>);

impl AccountDungeons {
    const URI: &'static str = "/v2/account/dungeons";

    /// Returns a list of dungeon paths completed by the currently authenticatd account since the
    /// daily reset.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns a list of dungeon paths completed by the account of the
    /// current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountDungeons;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let paths = AccountDungeons::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountDungeons;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let paths = AccountDungeons::get(&client)?;
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

/// A list of dyes unlocked by an account.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountDyes(pub Vec<u64>);

impl AccountDyes {
    const URI: &'static str = "/v2/account/dyes";

    /// Returns a list of dyes unlocked by the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns the dyes unlocked by the account of the current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountDyes;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let dyes = AccountDyes::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountDyes;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let dyes = AccountDyes::get(&client)?;
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

/// A list of finishers unlocked.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountFinishers(pub Vec<AccountFinisher>);

impl AccountFinishers {
    const URI: &'static str = "/v2/account/finishers";

    /// Returns a list of finishers unlocked by the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns a list of finishers unlocked by the account of the current
    /// access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountFinishers;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let finishers = AccountFinishers::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountFinishers;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let finishers = AccountFinishers::get(&client)?;
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

/// A single finisher unlocked by an account.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AccountFinisher {
    /// The id of the finisher.
    pub id: u64,
    /// Whether the finisher is unlocked permanently.
    #[serde(default = "AccountFinisher::serde_default_permanent")]
    pub permanent: bool,
    /// The number of uses remaining if `permanent` is `false`.
    ///
    /// This field is always `0` if `permanent` is `true`.
    #[serde(default)]
    pub quantity: u64,
}

impl AccountFinisher {
    #[inline]
    fn serde_default_permanent() -> bool {
        true
    }
}

/// A list of gliders unlocked by an account.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountGliders(pub Vec<u64>);

impl AccountGliders {
    const URI: &'static str = "/v2/account/gliders";

    /// Returns a list of gliders unlocked by the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns a list of gliders unlocked by the account of the current
    /// access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountGliders;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let gliders = AccountGliders::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountGliders;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let gliders = AccountGliders::get(&client)?;
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

/// A list of home cats unlocked by an account.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountHomeCats(pub Vec<u64>);

impl AccountHomeCats {
    const URI: &'static str = "/v2/account/home/cats";

    /// Returns a list of home cats unlocked by the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns a list of home cats unlocked by the account of the current
    /// access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountHomeCats;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let cats = AccountHomeCats::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountHomeCats;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let cats = AccountHomeCats::get(&client)?;
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

/// A list of home nodes unlocked by an account.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountHomeNodes(pub Vec<String>);

impl AccountHomeNodes {
    const URI: &'static str = "/v2/account/home/nodes";

    /// Returns a list of home nodes unlocked by the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns a list of home nodes unlocked by the account of the current
    /// access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountHomeNodes;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let nodes = AccountHomeNodes::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountHomeNodes;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let nodes = AccountHomeNodes::get(&client)?;
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

/// A list of items stored in the shared inventory slots of an account.
///
/// A `None` value indicates an empty slot.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AccountInventory(pub Vec<Option<InventoryItem>>);

impl AccountInventory {
    const URI: &'static str = "/v2/account/inventory";

    /// Returns a list of items stored in the shared inventory slots of the currently authenticated
    /// account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns a list of item stored in the shared inventory slots of the
    /// account of the current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountInventory;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let inventory = AccountInventory::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountInventory;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let inventory = AccountInventory::get(&client)?;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: u64,
    pub count: u64,
    pub charges: Option<u64>,
    pub skin: Option<u64>,
    pub upgrades: Option<Vec<u64>>,
    pub infusions: Option<Vec<u64>>,
    pub binding: ItemBinding,
}

/// The current luck value of an account.
pub struct AccountLuck(pub u64);

impl AccountLuck {
    const URI: &'static str = "/v2/account/luck";

    /// Returns the unlocked luck value of the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns unlocked luck value of the account of the current access
    /// token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::account::AccountLuck;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let luck = AccountLuck::get(&client).await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::account::AccountLuck;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let luck = AccountLuck::get(&client)?;
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

impl Serialize for AccountLuck {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        struct Map(u64);

        impl Serialize for Map {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("id", "luck")?;
                map.serialize_entry("value", &self.0)?;
                map.end()
            }
        }

        let len = if self.0 == 0 { 0 } else { 1 };

        let mut seq = serializer.serialize_seq(Some(len))?;

        if self.0 != 0 {
            seq.serialize_element(&Map(self.0))?;
        }

        seq.end()
    }
}

impl<'de> Deserialize<'de> for AccountLuck {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LuckVisitor;

        impl<'de> Visitor<'de> for LuckVisitor {
            type Value = AccountLuck;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a sequence with one or zero elements")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                struct Map(AccountLuck);

                impl<'de> Deserialize<'de> for Map {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: Deserializer<'de>,
                    {
                        struct LuckMapVisitor;

                        impl<'de> Visitor<'de> for LuckMapVisitor {
                            type Value = Map;

                            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                                write!(formatter, "a map containing account luck")
                            }

                            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                            where
                                A: MapAccess<'de>,
                            {
                                let value;

                                match map.next_key::<&str>()? {
                                    Some(key) => match key {
                                        "id" => match map.next_value()? {
                                            Some("luck") => (),
                                            _ => {
                                                return Err(A::Error::custom(
                                                    "expected a luck id value",
                                                ))
                                            }
                                        },
                                        _ => {
                                            return Err(A::Error::custom(
                                                "expected a luck id value",
                                            ))
                                        }
                                    },
                                    None => {
                                        return Err(A::Error::custom("missing fields id, value"))
                                    }
                                }

                                match map.next_key::<&str>()? {
                                    Some(key) => match key {
                                        "value" => match map.next_value()? {
                                            Some(val) => value = val,
                                            None => {
                                                return Err(A::Error::custom("expected a value"))
                                            }
                                        },
                                        _ => return Err(A::Error::custom("expected a value")),
                                    },
                                    None => return Err(A::Error::custom("missing fields value")),
                                }

                                Ok(Map(AccountLuck(value)))
                            }
                        }

                        deserializer.deserialize_map(LuckMapVisitor)
                    }
                }

                match seq.next_element::<Map>()? {
                    Some(map) => Ok(map.0),
                    None => Ok(AccountLuck(0)),
                }
            }
        }

        deserializer.deserialize_seq(LuckVisitor)
    }
}
