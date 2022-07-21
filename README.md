# gw2api-rs

An asynchronous wrapper for the offical Guild Wars 2 api using hyper (tokio).

```
use gw2api_rs::Client;
use gw2api_rs::v2::build::Build;

let client = Client::new();
let build = Build::get(&client).await.unwrap();
```

Or use the blocking client (requires the optional `blocking` feature).

```
use gw2api_rs::blocking::Client;
use gw2api_rs::v2::build::Build;

let client = Client::new();
let build = Build::get(&client).unwrap();
```

## Implemented endpoints

- [x] /v2/achievements
- [ ] /v2/achievements/daily
- [ ] /v2/achievements/daily/tomorrow
- [ ] /v2/achievements/groups
- [ ] /v2/achievements/categories

- [ ] /v2/account
- [ ] /v2/account/achievements
- [ ] /v2/account/bank
- [ ] /v2/account/dailycrafting
- [ ] /v2/account/dungeons
- [ ] /v2/account/dyes
- [ ] /v2/account/finishers
- [ ] /v2/account/gliders
- [ ] /v2/account/home
- [ ] /v2/account/home/cats
- [ ] /v2/account/home/nodes
- [ ] /v2/account/inventory
- [ ] /v2/account/legendaryarmory
- [ ] /v2/account/luck
- [ ] /v2/account/mailcarriers
- [ ] /v2/account/mapchests
- [ ] /v2/account/masteries
- [ ] /v2/account/mastery/points
- [ ] /v2/account/materials
- [ ] /v2/account/minis
- [ ] /v2/account/mounts/skins
- [ ] /v2/account/mounts/types
- [ ] /v2/account/novelties
- [ ] /v2/account/outfits
- [ ] /v2/account/pvp/heroes
- [ ] /v2/account/raids
- [ ] /v2/account/recipes
- [ ] /v2/account/skins
- [ ] /v2/account/titles
- [ ] /v2/account/wallet
- [ ] /v2/account/worldbosses

- [x] /v2/build
- [ ] /v2/characters
- [ ] /v2/pvp/stats
- [ ] /v2/pvp/games
- [ ] /v2/pvp/standings
- [x] /v2/tokeninfo
- [ ] /v2/dailycrafting
- [ ] /v2/mapchests
- [ ] /v2/worldbosses
- [ ] /v2/masteries
- [ ] /v2/mounts
- [ ] /v2/mounts/skins
- [ ] /v2/mounts/types
- [ ] /v2/outfits
- [ ] /v2/pets
- [ ] /v2/professions
- [ ] /v2/races
- [ ] /v2/specializations
- [ ] /v2/skills
- [ ] /v2/traits
- [ ] /v2/legendaryarmory
- [ ] /v2/legends
- [ ] /v2/guild/:id
- [ ] /v2/emblem

- [ ] /v2/guild/permissions
- [ ] /v2/guild/search
- [ ] /v2/guild/upgrades
- [ ] /v2/guild/:id/log
- [ ] /v2/guild/:id/members
- [ ] /v2/guild/:id/ranks
- [ ] /v2/guild/:id/stash
- [ ] /v2/guild/:id/treasury
- [ ] /v2/guild/:id/teams
- [ ] /v2/guild/:id/upgrades

- [ ] /v2/home/cats
- [ ] /v2/home/nodes
- [ ] /v2/finishers
- [ ] /v2/items
- [ ] /v2/itemstats
- [ ] /v2/materials
- [ ] /v2/pvp/amulets
- [ ] /v2/recipes
- [ ] /v2/recipes/search
- [ ] /v2/skins
- [ ] /v2/continents
- [ ] /v2/maps
- [x] /v2/build
- [x] /v2/colors
- [x] /v2/currencies
- [x] /v2/dungeons
- [x] /v2/files
- [x] /v2/quaggans
- [x] /v2/minis
- [x] /v2/novelties
- [x] /v2/raids
- [x] /v2/titles
- [x] /v2/worlds
- [ ] /v2/backstory/answers
- [ ] /v2/backstory/questions
- [ ] /v2/stories
- [ ] /v2/stories/seasons
- [ ] /v2/quests
- [ ] /v2/pvp
- [ ] /v2/pvp/ranks
- [ ] /v2/pvp/seasons
- [ ] /v2/pvp/seasons/:id/leaderboards

- [ ] /v2/commerce/delivery
- [ ] /v2/commerce/exchange
- [ ] /v2/commerce/exchange/coins
- [ ] /v2/commerce/exchange/gems
- [ ] /v2/commerce/listings
- [ ] /v2/commerce/prices
- [ ] /v2/commerce/transactions

- [x] /v2/wvw/abilities
- [x] /v2/wvw/matches
- [x] /v2/wvw/objectives
- [x] /v2/wvw/ranks
- [x] /v2/wvw/upgrades

## License

Licensed under either 
- [MIT License](https://github.com/MrGunflame/gw2api-rs/blob/master/LICENSE-MIT)
or
- [Apache License, Version 2.0](https://github.com/MrGunflame/gw2api-rs/blob/master/LICENSE-APACHE)
at your option.
