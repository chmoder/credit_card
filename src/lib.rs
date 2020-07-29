#![warn(warnings)]
#![allow(clippy::needless_doctest_main, clippy::type_complexity)]

//! Credit Card is a library for adding credit cards
//! to any project.
//!
//! ```rust,no_run
//! use credit_card::CreditCard;
//!
//! let mut cc = CreditCard {
//!     number: "4111111111111111".to_string(),
//!     cardholder_name: "Graydon Hoare".to_string(),
//!     expiration_month: "01".to_string(),
//!     expiration_year: "2023".to_string(),
//!     brand: None,
//!     security_code: None
//! };
//! cc.apply_brand();
//! ```
//!
//! # Current Features
//! - Create CreditCards
//! - Add brand to credit cards from card number
//! - Validate credit card number
//!
//! # Future Features
//! - Validate CVV
//! - Validate Address
//! - ...
//! - Full card validation
//!
//! ### Notice:
//! This is under development right now, so interfaces
//! and apis will be changing.  If you are interested
//! in using this please create an issue or reach out
//! with your feature request so I can help add it.
//!
mod luhn;
mod validate;
mod credit_card;


// lazy static is used by `validate`
// `macro_use` is required to be at the module root
#[macro_use]
extern crate lazy_static;

// re-export CreditCard
pub use self::credit_card::CreditCard;

#[cfg(test)]
mod tests {
    use crate::CreditCard;

    #[test]
    fn create() {
        let cc = CreditCard {
            number: "4111111111111111".to_string(),
            cardholder_name: "Graydon Hoare".to_string(),
            expiration_month: "01".to_string(),
            expiration_year: "2023".to_string(),
            brand: None,
            security_code: None
        };
        assert_eq!(cc.number, "4111111111111111".to_string())
    }

    #[test]
    fn is_visa() {
        let mut cc = CreditCard {
            number: "4111111111111111".to_string(),
            cardholder_name: "Graydon Hoare".to_string(),
            expiration_month: "01".to_string(),
            expiration_year: "2023".to_string(),
            brand: None,
            security_code: None
        };
        cc.apply_brand();

        assert_eq!(cc.brand.unwrap_or_default(), "visa")
    }

}
