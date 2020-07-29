# Credit Card

Credit Card is a library for adding credit cards
to any project.

```rust,norun
use crate::credit_card::CreditCard;

let mut cc = CreditCard {
    number: "4111111111111111".to_string(),
    cardholder_name: "Graydon Hoare".to_string(),
    expiration_month: "01".to_string(),
    expiration_year: "2023".to_string(),
    brand: None,
    security_code: None
};
cc.apply_brand();
```

# Current Features
- Create CreditCards
- Add brand to credit cards from card number
- Validate credit card number

# Future Features
- Validate CVV
- Validate Address
- ...
- Full card validation

### Notice:
This is under development right now, so interfaces
and apis will be changing.  If you are interested
in using this please create an issue or reach out
with your feature request so I can help add it.