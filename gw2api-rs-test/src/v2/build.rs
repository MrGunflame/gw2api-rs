use crate::CLIENT;

use gw2api_rs::v2::build::Build;

#[test]
fn build() {
    CLIENT.get::<Build>().unwrap();
}
