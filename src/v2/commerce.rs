use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{Authentication, ClientExecutor, RequestBuilder};

/// The coins and items currently waiting in trading post delivery.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Delivery {
    pub coins: u64,
    pub items: Vec<DeliveryItem>,
}

/// An item in the delivery box.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeliveryItem {
    pub id: u64,
    pub count: u64,
}

impl Delivery {
    /// Returns the delivery box for the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns the delivery box for the account of the current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::Delivery;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let delivery = Delivery::get(&client).await?;
    /// println!("You have {} coins and {} items waiting for pickup.", delivery.coins, delivery.items.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::Delivery;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let delivery = Delivery::get(&client)?;
    /// println!("You have {} coins and {} items waiting for pickup.", delivery.coins, delivery.items.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn get<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = "/v2/commerce/delivery";
        client.send(crate::RequestBuilder::new(uri).authenticated(Authentication::Required))
    }
}

/// The current gems to coins exchange rate.
///
/// `Exchange` handles both the coins to gems and gems to coins exchange rates.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Exchange {
    /// The current coins to gem exchange rate.
    pub coins_per_gem: u64,
    /// The amount of coins/gems you get for the requested amount of coins/gems.
    pub quantity: u64,
}

impl Exchange {
    /// Returns the current coins to gems exchange rate for the provided amount of `coins`.
    ///
    /// The `quanity` field contains the amount of gems for the provided amount of coins.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::Exchange;
    /// #
    /// # async fn run() -> Result<()> {
    /// let client = Client::new();
    /// let exchange = Exchange::coins(&client, 100).await?;
    /// println!("You get {} gems for 100 coins.", exchange.quantity);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::Exchange;
    /// #
    /// # fn run() -> Result<()> {
    /// let client = Client::new();
    /// let exchange = Exchange::coins(&client, 100)?;
    /// println!("You get {} gems for 100 coins.", exchange.quantity);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn coins<C>(client: &C, coins: u64) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("/v2/commerce/exchange/coins?quantity={}", coins);
        client.send(RequestBuilder::new(uri))
    }

    /// Returns the current gems to coins exchange rate for the provided amount of `gems`.
    ///
    /// The `quantity` field contains the amount of coins for the provided amount of gems.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::Exchange;
    /// #
    /// # async fn run() -> Result<()> {
    /// let client = Client::new();
    /// let exchange = Exchange::gems(&client, 100).await?;
    /// println!("You get {} coins for 100 gems.", exchange.quantity);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::Exchange;
    /// #
    /// # fn run() -> Result<()> {
    /// let client = Client::new();
    /// let exchange = Exchange::gems(&client, 100)?;
    /// println!("You get {} coins for 100 gems.", exchange.quantity);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn gems<C>(client: &C, gems: u64) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("/v2/commerce/exchange/gems?quantity={}", gems);
        client.send(RequestBuilder::new(uri))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listings {
    pub id: u64,
    pub buys: Listing,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Listing {
    pub listings: u64,
    pub unit_price: u64,
    pub quantity: u64,
}

impl Listings {
    const URI: &'static str = "/v2/commerce/listings";

    pub fn get<C>(client: &C, id: u64) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("{}?id={}", Self::URI, id);
        client.send(RequestBuilder::new(uri))
    }

    /// Returns a list of all items avaliable on the trading post.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::Listings;
    /// #
    /// # async fn run() -> Result<()> {
    /// let client = Client::new();
    /// let ids = Listings::ids(&client).await?;
    /// println!("There are {} items avaliable through the trading post.", ids.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::Listings;
    /// #
    /// # fn run() -> Result<()> {
    /// let client = Client::new();
    /// let ids = Listings::ids(&client)?;
    /// println!("There are {} items avaliable through the trading post.", ids.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn ids<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Vec<u64>>,
    {
        client.send(RequestBuilder::new(Self::URI))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prices {
    pub id: u64,
    /// Whether free to play accounts are allowed to buy/sell this item.
    pub whilelisted: bool,
    pub buys: Price,
    pub sells: Price,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Price {
    pub unit_price: u64,
    pub quantity: u64,
}

impl Prices {
    const URI: &'static str = "/v2/commerce/prices";

    /// Returns a list of all items avaliable on the trading post.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::Listings;
    /// #
    /// # async fn run() -> Result<()> {
    /// let client = Client::new();
    /// let ids = Listings::ids(&client).await?;
    /// println!("There are {} items avaliable through the trading post.", ids.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Using the [`blocking`] client:
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::Listings;
    /// #
    /// # fn run() -> Result<()> {
    /// let client = Client::new();
    /// let ids = Listings::ids(&client)?;
    /// println!("There are {} items avaliable through the trading post.", ids.len());
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn ids<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Vec<u64>>,
    {
        client.send(RequestBuilder::new(Self::URI))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrentTransactions {
    pub transactions: Vec<CurrentTransaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrentTransaction {
    pub id: u64,
    pub item_id: u64,
    pub price: u64,
    pub quantity: u64,
    pub created: DateTime<Utc>,
}

impl CurrentTransactions {
    const URI: &'static str = "/v2/commerce/transactions/current";

    /// Returns all outstanding *buy* transactions for the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns the outstanding *buy* transactions for the account of the
    /// current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::CurrentTransactions;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let transactions = CurrentTransactions::buys(&client).await?;
    /// println!("{:?}", transactions);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::CurrentTransactions;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let transactions = CurrentTransactions::buys(&client)?;
    /// println!("{:?}", transactions);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn buys<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("{}/buys", Self::URI);
        client.send(RequestBuilder::new(uri).authenticated(Authentication::Required))
    }

    /// Returns all outstanding *sell* transactions for the currently authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns the outstanding *sell* transactions for the account of the
    /// current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::CurrentTransactions;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let transactions = CurrentTransactions::sells(&client).await?;
    /// println!("{:?}", transactions);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::CurrentTransactions;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let transactions = CurrentTransactions::sells(&client)?;
    /// println!("{:?}", transactions);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn sells<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("{}/sells", Self::URI);
        client.send(RequestBuilder::new(uri).authenticated(Authentication::Required))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HistoryTransactions {
    pub transations: Vec<HistoryTransaction>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HistoryTransaction {
    pub id: u64,
    pub item_id: u64,
    pub price: u64,
    pub quantity: u64,
    pub created: DateTime<Utc>,
    pub purchased: DateTime<Utc>,
}

impl HistoryTransactions {
    const URI: &'static str = "/v2/commerce/transactions/history";

    /// Returns all *buy* transactions that were fulfilled in the past 90 days for the currently
    /// authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns the fulfilled *buy* transactions for the account of the
    /// current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::HistoryTransactions;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let transactions = HistoryTransactions::buys(&client).await?;
    /// println!("{:?}", transactions);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::HistoryTransactions;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let transactions = HistoryTransactions::buys(&client)?;
    /// println!("{:?}", transactions);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn buys<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("{}/buys", Self::URI);
        client.send(RequestBuilder::new(uri).authenticated(Authentication::Required))
    }

    /// Returns all *sell* transactions that were fulfilled in the past 90 days for the currently
    /// authenticated account.
    ///
    /// # Authentication
    ///
    /// This endpoint requires authentication and returns an [`Error`] if no access token is set.
    /// When authenticated it returns the fulfilled *sell* transactions for the account of the
    /// current access token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use gw2api_rs::{Client, Result};
    /// # use gw2api_rs::v2::commerce::HistoryTransactions;
    /// #
    /// # async fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let transactions = HistoryTransactions::sells(&client).await?;
    /// println!("{:?}", transactions);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// ```no_run
    /// # use gw2api_rs::Result;
    /// # use gw2api_rs::blocking::Client;
    /// # use gw2api_rs::v2::commerce::HistoryTransactions;
    /// #
    /// # fn run() -> Result<()> {
    /// # let token = "";
    /// let client: Client = Client::builder().access_token(token).into();
    /// let transactions = HistoryTransactions::sells(&client)?;
    /// println!("{:?}", transactions);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`blocking`]: crate::blocking
    pub fn sells<C>(client: &C) -> C::Result
    where
        C: ClientExecutor<Self>,
    {
        let uri = format!("{}/sells", Self::URI);
        client.send(RequestBuilder::new(uri).authenticated(Authentication::Required))
    }
}
