const s1: &str = r#"{
    "fallback_for_all": true,
    "serde1_ba": true,
    "c": true
}"#;

const s2: &str = r#"{
    "fallback_for_all": true,
    "b": {
        "serde2_ba": true
    },
    "only_for_serde2": true
}"#;

#[derive(serde1::Deserialize, serde2::Deserialize, Debug, PartialEq)]
#[serde1(crate = "serde1")]
#[serde2(crate = "serde2")]
struct A {
    #[serde(rename = "fallback_for_all")]
    a: bool,
    #[serde1(flatten)]
    b: B,
    #[serde2(rename = "only_for_serde2")]
    c: bool,
}

#[derive(serde1::Deserialize, serde2::Deserialize, Debug, PartialEq)]
#[serde1(crate = "serde1")]
#[serde2(crate = "serde2")]
struct B {
    #[serde1(rename = "serde1_ba")]
    #[serde2(rename = "serde2_ba")]
    ba: bool,
}

#[test]
fn test1() {
    let a1: A = serde_json1::from_str(s1).unwrap();
    let a2: A = serde_json2::from_str(s2).unwrap();
    assert_eq!(a1, a2);
}
