use crate::CLIENT;
use gw2api_rs::v2::achievements::Achievement;

#[test]
fn achievements() {
    Achievement::ids(&*CLIENT).unwrap();
    Achievement::get(&*CLIENT, 1840).unwrap();
    Achievement::get(&*CLIENT, 910).unwrap();
    Achievement::get(&*CLIENT, 2258).unwrap();
}
