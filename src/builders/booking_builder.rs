use crate::models::booking::{Booking, BookingDates};

#[derive(Debug, Default)]
pub struct BookingBuilder {
    firstname: Option<String>,
    lastname: Option<String>,
    totalprice: Option<i32>,
    depositpaid: Option<bool>,
    checkin: Option<String>,
    checkout: Option<String>,
    additionalneeds: Option<String>,
}

impl BookingBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn firstname(mut self, v: impl Into<String>) -> Self {
        self.firstname = Some(v.into());
        self
    }

    pub fn lastname(mut self, v: impl Into<String>) -> Self {
        self.lastname = Some(v.into());
        self
    }

    pub fn totalprice(mut self, v: i32) -> Self {
        self.totalprice = Some(v);
        self
    }

    pub fn depositpaid(mut self, v: bool) -> Self {
        self.depositpaid = Some(v);
        self
    }

    pub fn checkin(mut self, v: impl Into<String>) -> Self {
        self.checkin = Some(v.into());
        self
    }

    pub fn checkout(mut self, v: impl Into<String>) -> Self {
        self.checkout = Some(v.into());
        self
    }

    pub fn additionalneeds(mut self, v: impl Into<String>) -> Self {
        self.additionalneeds = Some(v.into());
        self
    }

    pub fn build(self) -> Booking {
        Booking {
            firstname: self.firstname.unwrap_or_else(|| "Test".into()),
            lastname: self.lastname.unwrap_or_else(|| "User".into()),
            totalprice: self.totalprice.unwrap_or(100),
            depositpaid: self.depositpaid.unwrap_or(true),
            bookingdates: BookingDates {
                checkin: self.checkin.unwrap_or_else(|| "2026-01-01".into()),
                checkout: self.checkout.unwrap_or_else(|| "2026-01-10".into()),
            },
            additionalneeds: self.additionalneeds,
        }
    }
}
