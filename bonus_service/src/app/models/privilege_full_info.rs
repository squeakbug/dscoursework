use serde::{Deserialize, Serialize};

use super::PrivilegeShortInfo;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrivilegeFullInfo {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "short_info")]
    pub short_info: PrivilegeShortInfo,
    #[serde(rename = "paid_by_bonuses")]
    pub paid_by_bonuses: i32,
    #[serde(rename = "paid_by_money")]
    pub paid_by_money: i32,
    #[serde(rename = "date")]
    pub date: String,
}
