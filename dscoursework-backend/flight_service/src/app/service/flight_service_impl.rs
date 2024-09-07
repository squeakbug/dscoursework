use std::boxed::Box;

use async_trait::async_trait;

use crate::app::models::flight_response::FlightResponse;
use crate::app::models::{self, PaginationResponse};
use crate::app::repository::flight_repository::FlightRepository;
use crate::app::service::flight_service::FlightService;
use crate::app::service::service_error::Result;
use crate::app::service::service_error::ServiceError;

#[derive(Clone)]
pub struct FlightServiceImpl {
    pub flight_repository: Box<dyn FlightRepository + Send + Sync>,
}

use tracing::info;

#[async_trait]
impl FlightService for FlightServiceImpl {
    async fn get_flight(&self, id: i32) -> Result<FlightResponse> {
        self.flight_repository.get_flight(id).await
    }

    async fn get_flights(&self, page: Option<i32>, size: Option<i32>, flight_number: Option<String>) -> Result<PaginationResponse> {
        info!("page: {:?}; size: {:?}, flight_nymber: {:?}", page, size, flight_number);
        
        if let Some(pg) = page {
            if size.is_none() {
                return Err(ServiceError::BadClientData);
            }
            if pg < 1 {
                return Err(ServiceError::BadClientData);
            }
        }

        let result = self
            .flight_repository
            .get_flights(page, size, flight_number)
            .await
            .map(|flights| models::PaginationResponse {
                page,
                page_size: size,
                total_elements: Some(flights.len() as i32),
                items: Some(flights),
            })?;

        Ok(result)
    }
}
