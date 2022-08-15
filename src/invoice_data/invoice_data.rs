use crate::invoice_data::InvoiceItem;
use crate::invoice_data::Party;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceData {
    pub items: Vec<InvoiceItem>,
    pub seller: Party,
    pub buyer: Party,
    pub currency: String,
    pub sold_on: NaiveDate,
    pub issued_at: Option<NaiveDateTime>,
}

#[test]
fn test_deserialize_invoice_data() {
    let input = r#"
    {
        "currency": "EUR",
        "sold_on": "2022-08-15",
        "issued_at": "2022-08-15T21:27:50",
        "buyer": {
            "name": "Test Buyer",
            "tax_id": "123456",
            "address_first_line": "123, Some Street",
            "address_city": "SimCity",
            "address_zip_code": "DE12345"
        },
        "seller": {
            "name": "Test Seller",
            "tax_id": "123456",
            "address_first_line": "123, Some Street",
            "address_city": "SimCity",
            "address_zip_code": "DE12345"
        },
        "items": [
            {
                "title": "Software Development",
                "amount": 1,
                "unit_price": "21.37",
                "vat_rate": "FIVE"
            }
        ]
    }
    "#;

    let actual: InvoiceData = serde_json::from_str(&input).unwrap();

    assert_eq!(actual.items.len(), 1);
}
