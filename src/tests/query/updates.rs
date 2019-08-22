use super::{TEST_RETRIES, TEST_TIMEOUT};

use crate::data::*;
use crate::query::*;
use crate::service::*;

#[test]
fn deserialize_f32() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F32)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f31() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F31)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f31c() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F31C)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f30() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F30)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f30c() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F30C)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f30f() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F30F)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f30m() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F30M)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f29() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F29)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f29c() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F29C)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f29f() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F29F)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f29m() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F29M)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f28() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F28)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f28c() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F28C)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f28m() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F28M)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f27() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F27)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f27m() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F27M)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f26() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F26)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_f25() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::F25)
        .query(&bodhi)
        .unwrap();
}

#[test]
fn deserialize_epel8() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::EPEL8)
        .query(&bodhi)
        .unwrap();
}

/*
#[test]
fn deserialize_epel7() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    // query only updates for one release, and deserialize them
    UpdateQuery::new()
        .releases(FedoraRelease::EPEL7)
        .query(&bodhi)
        .unwrap();
}
*/

#[test]
fn id_query_some() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let update: Option<Update> = UpdateIDQuery::new(String::from("FEDORA-2019-227c137c3f"))
        .query(&bodhi)
        .unwrap();

    assert!(update.is_some());
}

#[test]
fn id_query_none() {
    let bodhi = BodhiServiceBuilder::new(String::from(FEDORA_BODHI_URL))
        .timeout(TEST_TIMEOUT)
        .retries(TEST_RETRIES)
        .build()
        .unwrap();

    let update: Option<Update> = UpdateIDQuery::new(String::from("NOPE"))
        .query(&bodhi)
        .unwrap();

    assert!(update.is_none());
}
