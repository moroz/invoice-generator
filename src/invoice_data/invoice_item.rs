use crate::invoice_data::VatRate;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    title: String,
    amount: u32,
    unit_price: Decimal,
    vat_rate: VatRate,
}
