use std::time::Duration;

use deadpool_lapin::{Pool, PoolError};
use futures::StreamExt;
use lapin::{message::Delivery, options::*, types::FieldTable};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use thiserror::Error as ThisError;

pub type Connection = deadpool::managed::Object<deadpool_lapin::Manager>;
type StdResult<T, E> = std::result::Result<T, E>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("rmq error: {0}")]
    RMQError(#[from] lapin::Error),
    #[error("rmq pool error: {0}")]
    RMQPoolError(#[from] PoolError),
}

pub async fn get_rmq_con(pool: Pool) -> StdResult<Connection, PoolError> {
    let connection = pool.get().await?;
    Ok(connection)
}

enum ServiceError {
    ParseError,
}

type ServiceResult<T> = std::result::Result<T, ServiceError>;

#[derive(Serialize, Deserialize)]
enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
}

#[derive(Serialize, Deserialize)]
struct HttpRequest {
    method: HttpMethod,
    uri: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

fn http_method_to_reqwest_method(method: HttpMethod) -> ServiceResult<reqwest::Method> {
    match method {
        HttpMethod::Get => Ok(reqwest::Method::GET),
        HttpMethod::Patch => Ok(reqwest::Method::PATCH),
        HttpMethod::Post => Ok(reqwest::Method::POST),
        HttpMethod::Delete => Ok(reqwest::Method::DELETE),
    }
}

async fn send_smth(req: HttpRequest) -> ServiceResult<()> {
    let client = Client::new();

    let uri = req.uri;
    let method = http_method_to_reqwest_method(req.method)?;
    let mut local_var_req_builder = client.request(method, uri);

    for header in req.headers {
        local_var_req_builder = local_var_req_builder.header(header.0, header.1);
    }

    local_var_req_builder = local_var_req_builder.json(&req.body);

    let local_var_req = local_var_req_builder.build().map_err(|_| ServiceError::ParseError)?;
    let local_var_resp = client.execute(local_var_req).await.unwrap();

    let local_var_status = local_var_resp.status();
    let _ = local_var_resp.text().await.unwrap(); //.map_err(|_| ServiceError::ParseError)?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        Err(ServiceError::ParseError)
    }
}

async fn rmq_handle_message(delivery: Delivery) -> ServiceResult<()> {
    let data = std::str::from_utf8(&delivery.data).unwrap(); //.map_err(|_| ServiceError::ParseError)?;

    let req: HttpRequest = serde_json::from_str(data).unwrap(); //.map_err(|_| ServiceError::ParseError)?;

    send_smth(req).await
}

pub async fn rmq_listen(pool: Pool) -> StdResult<(), Error> {
    let mut retry_interval = tokio::time::interval(Duration::from_secs(5));
    loop {
        retry_interval.tick().await;
        println!("connecting rmq consumer...");
        match init_rmq_listen(pool.clone()).await {
            Ok(_) => println!("rmq listen returned"),
            Err(e) => eprintln!("rmq listen had an error: {}", e),
        };
    }
}

pub async fn init_rmq_listen(pool: Pool) -> StdResult<(), Error> {
    let rmq_con = get_rmq_con(pool).await.map_err(|e| {
        eprintln!("could not get rmq con: {}", e);
        e
    })?;
    let channel = rmq_con.create_channel().await?;

    let tbl = FieldTable::default();
    let queue = channel
        .queue_declare("hello", QueueDeclareOptions::default(), tbl)
        .await?;
    println!("Declared queue {:?}", queue);

    let mut consumer = channel
        .basic_consume(
            "hello",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("rmq consumer connected, waiting for messages");
    while let Some(delivery) = consumer.next().await {
        if let Ok((channel, delivery)) = delivery {
            let _ = rmq_handle_message(delivery.clone()).await;
            channel
                .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                .await?
        }
    }
    Ok(())
}
