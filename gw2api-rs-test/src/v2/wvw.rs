use crate::CLIENT;

use gw2api_rs::v2::wvw::{Ability, Match, Rank, Upgrades};

#[test]
fn abilities() {
    Ability::ids(&*CLIENT).unwrap();
    Ability::get_all(&*CLIENT).unwrap();
}

#[test]
fn ranks() {
    Rank::ids(&*CLIENT).unwrap();
    Rank::get_all(&*CLIENT).unwrap();
}

#[test]
fn matches() {
    Match::ids(&*CLIENT).unwrap();
    Match::get_all(&*CLIENT).unwrap();
}

#[test]
fn upgrades() {
    Upgrades::ids(&*CLIENT).unwrap();
    Upgrades::get_all(&*CLIENT).unwrap();
}
