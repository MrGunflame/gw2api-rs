use crate::CLIENT;

use gw2api_rs::v2::tokeninfo::TokenInfo;

#[test]
fn tokeninfo() {
    CLIENT.get::<TokenInfo>().unwrap();
}
