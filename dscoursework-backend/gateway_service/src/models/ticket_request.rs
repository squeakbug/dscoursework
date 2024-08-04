use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketRequest {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "flight_number")]
    pub flight_number: String,
    #[serde(rename = "price")]
    pub price: i32,
}
