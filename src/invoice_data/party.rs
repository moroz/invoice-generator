use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Party {
    pub name: String,
    pub tax_id: Option<String>,
    pub address_first_line: String,
    pub address_city: String,
    pub address_zip_code: String,
}

#[test]
fn test_deserialize_party() {
    let input = r#"
        {
            "name": "Test Buyer",
            "tax_id": "123456",
            "address_first_line": "123, Some Street",
            "address_city": "SimCity",
            "address_zip_code": "DE12345"
        }
    "#;

    let actual: Party = serde_json::from_str(&input).unwrap();

    assert_eq!(actual.name, "Test Buyer");
    assert_eq!(actual.tax_id, Some("123456".to_string()));
    assert_eq!(actual.address_first_line, "123, Some Street");
    assert_eq!(actual.address_city, "SimCity");
    assert_eq!(actual.address_zip_code, "DE12345");
}
