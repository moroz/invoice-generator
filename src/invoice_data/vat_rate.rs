use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum VatRate {
    NA,
    Zero,
    Five,
    Seven,
    Eight,
    TwentyThree,
}

#[test]
fn test_vat_rate_serialization() {
    let actual = serde_json::to_string(&VatRate::NA).unwrap();
    assert_eq!("\"NA\"", actual);
    let actual = serde_json::to_string(&VatRate::Zero).unwrap();
    assert_eq!("\"Zero\"", actual);
    let actual = serde_json::to_string(&VatRate::TwentyThree).unwrap();
    assert_eq!("\"TwentyThree\"", actual);
}
