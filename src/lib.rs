#![doc = include_str!("../README.md")]

mod client;
pub mod de;
mod error;
pub mod types;

pub mod account;
pub mod auth;
pub mod billing;
pub mod channels;
pub mod countries;
pub mod couriers;
pub mod documents;
pub mod imports;
pub mod international;
pub mod inventory;
pub mod listings;
pub mod ndr;
pub mod orders;
pub mod pickup;
pub mod products;
pub mod returns;
pub mod shipments;
pub mod tracking;
pub mod wrapper;

pub use client::{
    DEFAULT_BASE_URL, DEFAULT_SERVICEABILITY_BASE_URL, Shiprocket, ShiprocketBuilder,
};
pub use error::Error;
pub use types::{MessageResponse, Paginated, PaymentMethod, Upload};
