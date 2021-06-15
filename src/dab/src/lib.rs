use ic_cdk_macros::*;
use ic_cdk::export::candid;
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use ic_cdk::*;
use serde::Deserialize;
use big_map;

struct Data(big_map::BigMap<String, Option<Principal>>);
impl Default for Data {
    fn default() -> Self {
        Self(big_map::BigMap::new(5, 2 * 1024 * 1024))
    }
}

fn name() -> String {
    String::from("Dfinity Address Book");
}

async fn add_canister(key: String, value: Option<Principal>) -> () {
    let data = storage::get_mut::<Data>();
    data.0.insert(key, value).await;
}

async fn get_canister(key: String) -> Option<Principal> {
    let data = storage::get::<Data>();
    data.0.get(key).await;
}