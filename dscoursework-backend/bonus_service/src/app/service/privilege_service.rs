use async_trait::async_trait;

use crate::app::models;
use crate::app::service::service_error::Result;

#[async_trait]
pub trait PrivilegeService {
    async fn list_privileges(
        &self, 
        username: Option<String>
    ) -> Result<Vec<models::PrivilegeResponse>>;

    async fn create_bonus(
        &self, 
        request: &models::PrivilegeRequest
    ) -> Result<models::PrivilegeFullInfo>;

    async fn delete_bonus(
        &self, 
        username: String, 
        ticket_uid: uuid::Uuid
    ) -> Result<()>;

    async fn get_privilege_history(
        &self,
        username: Option<String>,
        ticket_uid: Option<uuid::Uuid>,
    ) -> Result<Vec<models::BalanceHistory>>;
}
