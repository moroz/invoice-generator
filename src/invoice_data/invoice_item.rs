use crate::invoice_data::VatRate;
use rust_decimal::prelude::*;
use rust_decimal_macros::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    title: String,
    amount: u32,
    unit_price: Decimal,
    vat_rate: VatRate,
}

#[test]
fn test_invoice_item_deserialization() {
    let input = r#"
        {
            "title": "Software Development",
            "amount": 1,
            "unit_price": "21.37",
            "vat_rate": "FIVE"
        }
    "#;

    let actual: InvoiceItem = serde_json::from_str(&input).unwrap();
    assert_eq!("Software Development", actual.title);
    assert_eq!(1, actual.amount);
    assert_eq!(dec!(21.37), actual.unit_price);
    assert_eq!(VatRate::Five, actual.vat_rate);
}
