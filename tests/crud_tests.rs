mod common;

use restful_booker_automation::builders::BookingBuilder;

#[tokio::test]
async fn full_crud_flow() {
    let mut client = common::setup_authenticated_client().await;

    // Create
    let booking = BookingBuilder::new()
        .firstname("CRUD")
        .lastname("Test")
        .totalprice(200)
        .depositpaid(true)
        .checkin("2026-06-01")
        .checkout("2026-06-10")
        .additionalneeds("Lunch")
        .build();
    let created = client.create_booking(&booking).await.expect("create");
    let id = created.bookingid;
    assert_eq!(created.booking.firstname, "CRUD");

    // Read
    let fetched = client.get_booking(id).await.expect("get");
    assert_eq!(fetched.lastname, "Test");

    // Update
    let updated_data = BookingBuilder::new()
        .firstname("Updated")
        .lastname("CRUD")
        .totalprice(250)
        .depositpaid(false)
        .checkin("2026-06-01")
        .checkout("2026-06-10")
        .build();
    let updated = client
        .update_booking(id, &updated_data)
        .await
        .expect("update");
    assert_eq!(updated.firstname, "Updated");

    // Delete
    client.delete_booking(id).await.expect("delete");

    // Confirm gone
    let gone = client.get_booking(id).await;
    assert!(gone.is_err());
}

#[tokio::test]
async fn list_booking_ids() {
    let client = common::setup_client().await;
    let ids = client.get_booking_ids().await.expect("list");
    assert!(!ids.is_empty());
}
