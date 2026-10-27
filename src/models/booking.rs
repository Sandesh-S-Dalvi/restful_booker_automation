use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BookingDates {
    pub checkin: String,
    pub checkout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Booking {
    pub firstname: String,
    pub lastname: String,
    pub totalprice: i32,
    pub depositpaid: bool,
    pub bookingdates: BookingDates,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additionalneeds: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingResponse {
    pub bookingid: i64,
    pub booking: Booking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingId {
    pub bookingid: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialBooking {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub totalprice: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depositpaid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookingdates: Option<BookingDates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additionalneeds: Option<String>,
}
