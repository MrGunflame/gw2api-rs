use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Authentication, ClientExecutor, RequestBuilder};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub emblem: GuildEmblem,
    // Avaliable with leader token.
    pub level: Option<u8>,
    pub motd: Option<String>,
    pub influence: Option<u64>,
    pub aetherium: Option<u64>,
    pub favor: Option<u64>,
    pub member_count: Option<u16>,
    pub member_capacity: Option<u16>,
}

impl Guild {
    /// Returns the guild with the given `id`.
    pub fn get<C>(client: &C, id: &str) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("/v2/guild/{}", id);
        client.send(RequestBuilder::new(uri))
    }

    /// Returns a list of guild ids matching the searched `name`. If no matches are found this
    /// returns an empty [`Vec`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::guild::Guild;
    /// #
    /// # async fn run() -> Result<()> {
    /// let client = Client::new();
    /// let guilds = Guild::search(&client, "Covenant Of The Just").await?;
    /// println!("{:?}", guilds);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::guild::Guild;
    /// #
    /// # fn run() -> Result<()> {
    /// let client = Client::new();
    /// let guilds = Guild::search(&client, "Covenant Of The Just")?;
    /// println!("{:?}", guilds);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn search<C>(client: &C, name: &str) -> C::Result
    where
        C: ClientExecutor<Vec<String>>,
    {
        let uri = format!("/v2/guild/search?name={}", name);
        client.send(RequestBuilder::new(uri))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildEmblem {
    pub background: GuildEmblemSection,
    pub foreground: GuildEmblemSection,
    pub flags: Vec<GuildEmblemFlag>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildEmblemSection {
    pub id: u64,
    pub colors: Vec<u64>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GuildEmblemFlag {
    FlipBackgroundHorizontal,
    FlipBackgroundVertical,
}

/// A list of [`GuildMember`]s.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GuildMembers(pub Vec<GuildMember>);

impl GuildMembers {
    /// Returns a list of all members in the guild with the provided `guild_id`.
    ///
    /// Note that the current access token must be a guild leader of the provided `guild_id`.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// If the account of the current access token is not a guild leader of the guild, an [`Error`]
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::guild::GuildMembers;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// # let guild = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let members = GuildMembers::get(&client, guild).await?;
    /// println!("{:?}", members);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::guild::GuildMembers;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// # let guild = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let members = GuildMembers::get(&client, guild)?;
    /// println!("{:?}", members);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error`]: struct@crate::Error
    /// [`blocking`]: crate::blocking
    pub fn get<C>(client: &C, guild_id: &str) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("/v2/guild/{}/members", guild_id);
        client.send(RequestBuilder::new(uri).authenticated(Authentication::Required))
    }
}

/// A member in a guild.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildMember {
    /// The account name of the member.
    pub name: String,
    /// The rank of the member.
    pub rank: String,
    /// The date the member joined the guild.
    pub joined: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GuildRanks(Vec<GuildRank>);

impl GuildRanks {
    /// Returns a list of ranks in the guild with the provided `guild_id`.
    ///
    /// Note that the current access token must be a guild leader of the provided `guild_id`.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// If the account of the current access token is not a guild leader of the guild, an [`Error`]
    /// is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::guild::GuildRanks;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// # let guild = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let ranks = GuildRanks::get(&client, guild).await?;
    /// println!("{:?}", ranks);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::guild::GuildRanks;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// # let guild = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let ranks = GuildRanks::get(&client, guild)?;
    /// println!("{:?}", ranks);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`Error`]: struct@crate::Error
    /// [`blocking`]: crate::blocking
    pub fn get<C>(client: &C, guild_id: &str) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("/v2/guild/{}/ranks", guild_id);
        client.send(RequestBuilder::new(uri).authenticated(Authentication::Required))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildRank {
    /// The unique name of the rank.
    pub id: String,
    /// The sorting order of the rank. A lower order is a higher rank.
    pub order: u64,
    /// A list of permissions granted to this rank.
    pub permissions: Vec<String>,
    /// A url pointing to the icon of the rank.
    pub icon: String,
}
