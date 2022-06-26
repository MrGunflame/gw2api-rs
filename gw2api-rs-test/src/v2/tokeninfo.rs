use crate::CLIENT;

use gw2api_rs::v2::tokeninfo::TokenInfo;
use gw2api_rs::Error;

#[test]
fn tokeninfo() {
    assert!(matches!(
        TokenInfo::get(&*CLIENT).unwrap_err(),
        Error::NoAccessToken
    ));
}
