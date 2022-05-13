use crate::CLIENT;

use gw2api_rs::v2::wvw::{Ability, Match, Rank, Upgrades};

#[test]
fn abilities() {
    CLIENT.get::<Ability>().unwrap();
    CLIENT.get_all::<Ability>().unwrap();
}

#[test]
fn ranks() {
    CLIENT.get::<Rank>().unwrap();
    CLIENT.get_all::<Rank>().unwrap();
}

#[test]
fn matches() {
    CLIENT.get::<Match>().unwrap();
    CLIENT.get_all::<Match>().unwrap();
}

#[test]
fn upgrades() {
    CLIENT.get::<Upgrades>().unwrap();
    CLIENT.get_all::<Upgrades>().unwrap();
}
