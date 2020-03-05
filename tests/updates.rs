#![allow(unused_imports)]
#![allow(dead_code)]

use std::fs::read_to_string;

use bodhi::Update;

const JSON_F33: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f33.json");
const JSON_F33C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f33c.json");
const JSON_F32: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f32.json");
const JSON_F32C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f32c.json");
const JSON_F32M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f32m.json");
const JSON_F31: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f31.json");
const JSON_F31C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f31c.json");
const JSON_F31F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f31f.json");
const JSON_F31M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f31m.json");
const JSON_F30: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f30.json");
const JSON_F30C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f30c.json");
const JSON_F30F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f30f.json");
const JSON_F30M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f30m.json");
const JSON_F29: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f29.json");
const JSON_F29C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f29c.json");
const JSON_F29F: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f29f.json");
const JSON_F29M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f29m.json");
const JSON_F28: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f28.json");
const JSON_F28C: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f28c.json");
const JSON_F28M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f28m.json");
const JSON_F27: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f27.json");
const JSON_F27M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f27m.json");
const JSON_F26: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f26.json");
const JSON_F25: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f25.json");
const JSON_F24: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f24.json");
const JSON_F23: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f23.json");
const JSON_F22: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f22.json");
const JSON_F21: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_f21.json");
const JSON_EPEL8: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel8.json");
const JSON_EPEL8M: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel8m.json");
const JSON_EPEL7: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_epel7.json");
const JSON_EL6: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_el6.json");
const JSON_EL5: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/updates_el5.json");

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f33() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F33).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f33c() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F33C).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f32() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F32).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f32c() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F32C).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f32m() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F32M).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f31() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F31).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f31c() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F31C).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f31f() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F31F).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f31m() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F31M).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f30() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F30).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f30c() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F30C).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f30f() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F30F).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f30m() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F30M).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f29() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F29).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f29c() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F29C).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f29f() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F29F).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f29m() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F29M).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f28() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F28).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f28c() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F28C).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f28m() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F28M).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f27() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F27).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f27m() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F27M).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f26() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F26).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f25() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F25).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f24() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F24).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f23() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F23).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f22() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F22).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_f21() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_F21).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel8() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL8).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel8m() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL8M).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_epel7() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EPEL7).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_el6() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EL6).unwrap()).unwrap();
}

#[cfg(feature = "data-tests")]
#[test]
fn updates_dejson_el5() {
    let _: Vec<Update> = serde_json::from_str(&read_to_string(JSON_EL5).unwrap()).unwrap();
}
