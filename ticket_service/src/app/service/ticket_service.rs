use async_trait::async_trait;

use crate::app::models;
use crate::app::service::service_error::Result;

#[async_trait]
pub trait TicketService {
    async fn get_ticket(&self, ticket_uid: uuid::Uuid) -> Result<models::TicketResponse>;
    async fn get_tickets(
        &self,
        username: Option<String>,
        flight_number: Option<String>,
    ) -> Result<Vec<models::TicketResponse>>;
    async fn create_ticket(&self, create_request: &models::TicketCreateRequest) -> Result<models::TicketResponse>;
    async fn edit_ticket(
        &self,
        ticket_uid: uuid::Uuid,
        request: &models::TicketRequest,
    ) -> Result<models::TicketResponse>;
    async fn delete_ticket(&self, ticket_uuid: uuid::Uuid) -> Result<()>;
}
