mod common;

use restful_booker_automation::builders::BookingBuilder;

#[tokio::test]
async fn update_without_token_fails() {
    let client = common::setup_client().await;
    let booking = BookingBuilder::new().build();
    let created = client.create_booking(&booking).await.expect("create");
    let result = client.update_booking(created.bookingid, &booking).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn delete_without_token_fails() {
    let client = common::setup_client().await;
    let booking = BookingBuilder::new().build();
    let created = client.create_booking(&booking).await.expect("create");
    let result = client.delete_booking(created.bookingid).await;
    assert!(result.is_err());
}
