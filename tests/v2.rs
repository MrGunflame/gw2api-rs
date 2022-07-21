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
