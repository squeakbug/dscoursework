use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TicketPurchaseRequestJwt {
    request: TicketPurchaseRequest,
    access_token: String,
}
