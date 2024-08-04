use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserInfoResponse {
    /// Информация о билетах пользоватлея
    #[serde(rename = "tickets")]
    pub tickets: Option<Vec<crate::models::TicketResponse>>,
    #[serde(rename = "privilege")]
    pub privilege: Option<crate::models::PrivilegeShortInfo>,
}
