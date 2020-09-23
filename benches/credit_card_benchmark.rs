use tokio;
use criterion::{criterion_group, criterion_main, Criterion};
use credit_card::CreditCard;
use core::time::Duration;

#[tokio::main]
async fn brand_credit_card() {
    let mut cc = CreditCard {
        number: "4111111111111111".to_string(),
        cardholder_name: "Graydon Hoare".to_string(),
        expiration_month: "01".to_string(),
        expiration_year: "2023".to_string(),
        brand: None,
        security_code: None
    };

    cc.apply_brand();

    assert_eq!(cc.brand.unwrap(), "visa")
}

fn criterion_brand_credit_card(c: &mut Criterion) {
    c.bench_function("brand_credit_card", |b| b.iter(|| brand_credit_card()));
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10).nresamples(2000).measurement_time(Duration::new(5, 0));
    targets = criterion_brand_credit_card
}
criterion_main!(benches);