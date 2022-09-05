mod support;

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
