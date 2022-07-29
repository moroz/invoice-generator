use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize)]
pub enum VatRate {
    NA,
    Zero,
    Five,
    Seven,
    Eight,
    TwentyThree,
}

impl Serialize for VatRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match *self {
            VatRate::NA => "N_A",
            VatRate::Zero => "ZERO",
            VatRate::Five => "FIVE",
            VatRate::Seven => "SEVEN",
            VatRate::Eight => "EIGHT",
            VatRate::TwentyThree => "TWENTY_THREE",
        };

        serializer.serialize_str(value)
    }
}

// impl<'de> Deserialize<'de> for VatRate {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let value = deserializer.deserialize_str({d});
//     }
// }

#[test]
fn test_vat_rate_serialization() {
    let actual = serde_json::to_string(&VatRate::NA).unwrap();
    assert_eq!("\"N_A\"", actual);
    let actual = serde_json::to_string(&VatRate::Zero).unwrap();
    assert_eq!("\"ZERO\"", actual);
    let actual = serde_json::to_string(&VatRate::TwentyThree).unwrap();
    assert_eq!("\"TWENTY_THREE\"", actual);
}
