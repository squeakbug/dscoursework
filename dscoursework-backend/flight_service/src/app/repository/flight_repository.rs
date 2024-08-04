use actix::prelude::*;
use async_trait::async_trait;

use crate::app::models;
use crate::app::repository::database_executor::DatabaseExecutor;
use crate::app::repository::flight_repository_handlers::{GetFlight, GetFlights};
use crate::app::service::service_error::{Result, ServiceError};

#[async_trait]
pub trait FlightRepository: FlightRepositoryClone {
    async fn get_flight(&self, id: i32) -> Result<models::FlightResponse>;
    async fn get_flights(&self, page: Option<i32>, size: Option<i32>, flight_number: Option<String>) -> Result<Vec<models::FlightResponse>>;
}

pub trait FlightRepositoryClone {
    fn clone_box(&self) -> Box<dyn FlightRepository + Send + Sync>;
}

impl<T> FlightRepositoryClone for T
where
    T: 'static + FlightRepository + Send + Sync + Clone,
{
    fn clone_box(&self) -> Box<dyn FlightRepository + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn FlightRepository + Send + Sync> {
    fn clone(&self) -> Box<dyn FlightRepository + Send + Sync> {
        self.clone_box()
    }
}

#[derive(Clone)]
pub struct FlightRepositoryImpl {
    pub db_addr: Addr<DatabaseExecutor>,
}

#[async_trait]
impl FlightRepository for FlightRepositoryImpl {
    async fn get_flight(&self, id: i32) -> Result<models::FlightResponse> {
        self.db_addr
            .send(GetFlight { id })
            .await
            .map_err(|_| ServiceError::InternalError)?
            .map_err(|_| ServiceError::NotFoundError)
    }

    async fn get_flights(&self, page: Option<i32>, size: Option<i32>, flight_number: Option<String>) -> Result<Vec<models::FlightResponse>> {
        let page_and_size = page.and_then(|p| size.map(|s| (p, s))).ok_or(ServiceError::InternalError)?;
        self.db_addr
            .send(GetFlights {
                page_and_size: Some(page_and_size),
                flight_number,
            })
            .await
            .map_err(|_| ServiceError::InternalError)?
            .map_err(|_| ServiceError::NotFoundError)
    }
}
