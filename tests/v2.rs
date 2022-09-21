mod support;

mod account {
    use gw2api_rs::v2::account::{
        Account, AccountAchievements, AccountBank, AccountDailyCrafting, AccountDungeons,
        AccountDyes, AccountFinishers, AccountGliders, AccountHomeCats, AccountHomeNodes,
        AccountInventory, AccountLegendaryArmory, AccountLuck, AccountMailCarriers,
        AccountMapChests, AccountMasteries, AccountMasteryPoints, AccountMaterials, AccountMinis,
        AccountMountSkins, AccountMountTypes, AccountNovelties, AccountOutfits, AccountProgression,
        AccountPvPHeroes, AccountRaids, AccountRecipes, AccountSkins, AccountTitles, AccountWallet,
        AccountWorldBosses,
    };

    use crate::support::CLIENT;

    #[test]
    fn test_account() {
        Account::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_achievements() {
        AccountAchievements::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_bank() {
        AccountBank::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_daily_crafting() {
        AccountDailyCrafting::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_dungeons() {
        AccountDungeons::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_dyes() {
        AccountDyes::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_finishers() {
        AccountFinishers::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_gliders() {
        AccountGliders::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_home_cats() {
        AccountHomeCats::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_home_nodes() {
        AccountHomeNodes::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_inventory() {
        AccountInventory::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_luck() {
        AccountLuck::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_legendary_armory() {
        AccountLegendaryArmory::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_mail_carriers() {
        AccountMailCarriers::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_map_chests() {
        AccountMapChests::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_masteries() {
        AccountMasteries::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_mastery_points() {
        AccountMasteryPoints::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_materials() {
        AccountMaterials::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_minis() {
        AccountMinis::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_mount_skins() {
        AccountMountSkins::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_mount_types() {
        AccountMountTypes::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_novelties() {
        AccountNovelties::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_outfits() {
        AccountOutfits::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_progression() {
        AccountProgression::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_pvp_heroes() {
        AccountPvPHeroes::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_raids() {
        AccountRaids::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_recipes() {
        AccountRecipes::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_skins() {
        AccountSkins::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_titles() {
        AccountTitles::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_wallet() {
        AccountWallet::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_account_world_bosses() {
        AccountWorldBosses::get(&*CLIENT).unwrap();
    }
}

mod achievements {
    use gw2api_rs::v2::achievements::Achievement;

    use crate::support::CLIENT;

    #[test]
    fn test_achievements() {
        Achievement::ids(&*CLIENT).unwrap();
        Achievement::get(&*CLIENT, 4219).unwrap();
        Achievement::get(&*CLIENT, 6395).unwrap();
    }
}

mod build {
    use gw2api_rs::v2::build::Build;

    use crate::support::CLIENT;

    #[test]
    fn test_build() {
        Build::get(&*CLIENT).unwrap();
    }
}

mod colors {
    use gw2api_rs::v2::colors::Color;

    use crate::support::CLIENT;

    #[test]
    fn test_colors() {
        Color::ids(&*CLIENT).unwrap();
        Color::get_all(&*CLIENT).unwrap();
    }
}

mod commerce {
    use gw2api_rs::v2::commerce::{
        CurrentTransactions, Delivery, Exchange, HistoryTransactions, Listings, Prices,
    };

    use crate::support::CLIENT;

    #[test]
    fn test_delivery() {
        Delivery::get(&*CLIENT).unwrap();
    }

    #[test]
    fn test_exchange() {
        Exchange::coins(&*CLIENT, 10000).unwrap();
        Exchange::gems(&*CLIENT, 10000).unwrap();
    }

    #[test]
    fn test_listings() {
        Listings::ids(&*CLIENT).unwrap();
    }

    #[test]
    fn test_prices() {
        Prices::ids(&*CLIENT).unwrap();
    }

    #[test]
    fn test_current_transactions() {
        CurrentTransactions::buys(&*CLIENT).unwrap();
        CurrentTransactions::sells(&*CLIENT).unwrap();
    }

    #[test]
    fn test_history_transactions() {
        HistoryTransactions::buys(&*CLIENT).unwrap();
        HistoryTransactions::sells(&*CLIENT).unwrap();
    }
}

mod currencies {
    use gw2api_rs::v2::currencies::Currency;

    use crate::support::CLIENT;

    #[test]
    fn test_currencies() {
        Currency::ids(&*CLIENT).unwrap();
        Currency::get_all(&*CLIENT).unwrap();
    }
}

mod dungeons {
    use gw2api_rs::v2::dungeons::Dungeon;

    use crate::support::CLIENT;

    #[test]
    fn test_dungeons() {
        Dungeon::ids(&*CLIENT).unwrap();
        Dungeon::get_all(&*CLIENT).unwrap();
    }
}

mod files {
    use gw2api_rs::v2::files::File;

    use crate::support::CLIENT;

    #[test]
    fn test_files() {
        File::ids(&*CLIENT).unwrap();
        File::get_all(&*CLIENT).unwrap();
    }
}

mod guild {
    use gw2api_rs::v2::guild::{GuildMembers, GuildRanks};

    use crate::support::CLIENT;

    const GUILD_ID: &str = "14762DCE-C2A4-E711-80D5-441EA14F1E44";

    #[test]
    fn test_guild_members() {
        GuildMembers::get(&*CLIENT, GUILD_ID).unwrap();
    }

    #[test]
    fn test_guild_ranks() {
        GuildRanks::get(&*CLIENT, GUILD_ID).unwrap();
    }
}

mod minis {
    use gw2api_rs::v2::minis::Mini;

    use crate::support::CLIENT;

    #[test]
    fn test_minis() {
        Mini::ids(&*CLIENT).unwrap();
        Mini::get_all(&*CLIENT).unwrap();
    }
}

mod novelties {
    use gw2api_rs::v2::novelties::Novelty;

    use crate::support::CLIENT;

    #[test]
    fn test_novelties() {
        Novelty::ids(&*CLIENT).unwrap();
        Novelty::get_all(&*CLIENT).unwrap();
    }
}

mod quaggans {
    use gw2api_rs::v2::quaggans::Quaggan;

    use crate::support::CLIENT;

    #[test]
    fn test_quaggans() {
        Quaggan::ids(&*CLIENT).unwrap();
        Quaggan::get_all(&*CLIENT).unwrap();
    }
}

mod raids {
    use gw2api_rs::v2::raids::Raid;

    use crate::support::CLIENT;

    #[test]
    fn test_raids() {
        Raid::ids(&*CLIENT).unwrap();
        Raid::get_all(&*CLIENT).unwrap();
    }
}

mod titles {
    use gw2api_rs::v2::titles::Title;

    use crate::support::CLIENT;

    #[test]
    fn test_titles() {
        Title::ids(&*CLIENT).unwrap();
        Title::get_all(&*CLIENT).unwrap();
    }
}

mod worlds {
    use gw2api_rs::v2::worlds::World;

    use crate::support::CLIENT;

    #[test]
    fn test_worlds() {
        World::ids(&*CLIENT).unwrap();
        World::get_all(&*CLIENT).unwrap();
    }
}

mod wvw {
    use gw2api_rs::v2::wvw::*;

    use crate::support::CLIENT;

    #[test]
    fn test_abilities() {
        Ability::ids(&*CLIENT).unwrap();
        Ability::get_all(&*CLIENT).unwrap();
    }

    #[test]
    fn test_ranks() {
        Rank::ids(&*CLIENT).unwrap();
        Rank::get_all(&*CLIENT).unwrap();
    }

    #[test]
    fn test_matches() {
        Match::ids(&*CLIENT).unwrap();
        Match::get_all(&*CLIENT).unwrap();
    }

    #[test]
    fn test_upgrades() {
        Upgrades::ids(&*CLIENT).unwrap();
        Upgrades::get_all(&*CLIENT).unwrap();
    }
}
