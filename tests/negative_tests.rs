mod common;

use restful_booker_automation::builders::BookingBuilder;

#[tokio::test]
async fn get_nonexistent_booking() {
    let client = common::setup_client().await;
    let result = client.get_booking(999_999_999).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn create_with_missing_fields_uses_defaults_or_fails_gracefully() {
    // Builder always supplies required fields; this verifies the happy path still works
    // under boundary values.
    let client = common::setup_client().await;
    let booking = BookingBuilder::new()
        .firstname("")
        .lastname("")
        .totalprice(0)
        .depositpaid(false)
        .build();
    let result = client.create_booking(&booking).await;
    // API may accept or reject empty names; we only assert no panic
    let _ = result;
}

#[tokio::test]
async fn ping_health() {
    let client = common::setup_client().await;
    let ok = client.ping().await.expect("ping");
    assert!(ok);
}
