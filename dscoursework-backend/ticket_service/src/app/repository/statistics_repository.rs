use std::time::Duration;

use futures::executor;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::app::service::service_error::ServiceError;

pub struct StatisticsRepository {
    producer: FutureProducer,
}

impl StatisticsRepository {
    pub fn new(producer: FutureProducer) -> Self {
        StatisticsRepository {
            producer,
        }
    }

    pub fn create_error_message(&self, err: &ServiceError) -> Result<(), ServiceError> {
        let payload = &serde_json::to_string(&err).map_err(|_| {
            ServiceError::InternalError
        })?;
        let key = "my_key";
        let record = FutureRecord::to("ticket-service")
            .payload(payload)
            .key(key);

        let _ = executor::block_on(self.producer.send(record, Duration::from_secs(0)));
        Ok(())
    }
}
