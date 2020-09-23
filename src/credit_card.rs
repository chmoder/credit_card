use serde::{Serialize, Deserialize};
use crate::validate::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CreditCard {
    pub number: String,
    pub cardholder_name: String,
    pub expiration_month: String,
    pub expiration_year: String,
    pub brand: Option<String>,
    pub security_code: Option<String>
}

impl CreditCard {
    pub fn new() -> Self {
        CreditCard {
            number: "".to_string(),
            cardholder_name: "".to_string(),
            expiration_month: "".to_string(),
            expiration_year: "".to_string(),
            brand: None,
            security_code: None
        }
    }

    pub fn apply_brand(&mut self) {
        match Validate::from(self.number.clone().as_str()) {
            Ok(result) => {
                self.brand = Option::from(result.card_type.name());
            },
            Err(err) => println!("Card is invalid: {:?}", err)
        }
    }
}
