use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum VatRate {
    NA,
    #[serde(rename = "ZERO")]
    Zero,
    #[serde(rename = "FIVE")]
    Five,
    #[serde(rename = "SEVEN")]
    Seven,
    #[serde(rename = "EIGHT")]
    Eight,
    #[serde(rename = "TWENTY_THREE")]
    TwentyThree,
}

#[test]
fn test_vat_rate_serialization() {
    let actual = serde_json::to_string(&VatRate::NA).unwrap();
    assert_eq!("\"NA\"", actual);
    let actual = serde_json::to_string(&VatRate::Zero).unwrap();
    assert_eq!("\"ZERO\"", actual);
    let actual = serde_json::to_string(&VatRate::Five).unwrap();
    assert_eq!("\"FIVE\"", actual);
    let actual = serde_json::to_string(&VatRate::Seven).unwrap();
    assert_eq!("\"SEVEN\"", actual);
    let actual = serde_json::to_string(&VatRate::Eight).unwrap();
    assert_eq!("\"EIGHT\"", actual);
    let actual = serde_json::to_string(&VatRate::TwentyThree).unwrap();
    assert_eq!("\"TWENTY_THREE\"", actual);
}

#[test]
fn test_vat_rate_deserialization() -> std::io::Result<()> {
    let actual: VatRate = serde_json::from_str("\"NA\"")?;
    assert_eq!(VatRate::NA, actual);
    let actual: VatRate = serde_json::from_str("\"ZERO\"")?;
    assert_eq!(VatRate::Zero, actual);
    let actual: VatRate = serde_json::from_str("\"FIVE\"")?;
    assert_eq!(VatRate::Five, actual);
    let actual: VatRate = serde_json::from_str("\"SEVEN\"")?;
    assert_eq!(VatRate::Seven, actual);

    Ok(())
}
