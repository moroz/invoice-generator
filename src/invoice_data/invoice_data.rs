use crate::invoice_data::InvoiceItem;
use crate::invoice_data::Party;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceData {
    items: Vec<InvoiceItem>,
    seller: Party,
    buyer: Party,
}
