use deadpool_lapin::Pool;
use futures::StreamExt;
use lapin::{
    message::Delivery, options::*, types::FieldTable
};

use crate::service::service_error::{Result, ServiceError};

pub fn handle_message(delivery: &Delivery) {
    println!("Received message: {:?}", delivery.data);
}

pub async fn init_rmq_listen(pool: Pool) -> Result<()> {
    let rmq_con = pool.get().await.map_err(|e| {
        eprintln!("could not get rmq con: {}", e);
        e
    }).map_err(|_| ServiceError::InternalError)?;
    let channel = rmq_con.create_channel().await.map_err(|_| ServiceError::InternalError)?;

    let queue = channel
        .queue_declare(
            "retryer_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await.map_err(|_| ServiceError::InternalError)?;
    println!("Declared queue {:?}", queue);

    let mut consumer = channel
        .basic_consume(
            "retryer_queue",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await.map_err(|_| ServiceError::InternalError)?;

    println!("rmq consumer connected, waiting for messages");
    while let Some(delivery) = consumer.next().await {
        if let Ok((channel, delivery)) = delivery {
            handle_message(&delivery);
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await.map_err(|_| ServiceError::InternalError)?
        }
    }
    Ok(())
}
