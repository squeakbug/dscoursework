use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeShortInfo {
    /// Баланс бонусного счета
    #[serde(rename = "balance")]
    pub balance: Option<i32>,
    /// Статус в бонусной программе
    #[serde(rename = "status")]
    pub status: Option<String>,
}
