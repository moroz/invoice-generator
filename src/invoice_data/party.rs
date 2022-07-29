use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Party {
    name: String,
    tax_id: Option<String>,
    address_first_line: String,
    address_city: String,
    address_zip_code: String,
}
