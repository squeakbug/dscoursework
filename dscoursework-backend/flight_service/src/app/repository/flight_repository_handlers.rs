use actix::prelude::*;
use anyhow::Result;
use diesel::sql_types::{Int4, Timestamptz, VarChar};
use diesel::{self, RunQueryDsl};
use diesel::{prelude::*, sql_query};
use serde::{Deserialize, Serialize};

use super::database_executor::DatabaseExecutor;
use crate::app::models;

pub struct GetFlights {
    pub page_and_size: Option<(i32, i32)>,
    pub flight_number: Option<String>,
}

impl Message for GetFlights {
    type Result = Result<Vec<models::FlightResponse>>;
}

#[derive(QueryableByName, Serialize, Deserialize, Debug, Clone)]
struct QueryFlightResponse {
    #[diesel(sql_type = Int4)]
    pub id: i32,
    #[diesel(sql_type = VarChar)]
    pub flight_number: String,
    #[diesel(sql_type = Timestamptz)]
    pub datetime: chrono::NaiveDateTime,
    #[diesel(sql_type = VarChar)]
    pub src_country: String,
    #[diesel(sql_type = VarChar)]
    pub src_city: String,
    #[diesel(sql_type = VarChar)]
    pub src_name: String,
    #[diesel(sql_type = VarChar)]
    pub dst_country: String,
    #[diesel(sql_type = VarChar)]
    pub dst_city: String,
    #[diesel(sql_type = VarChar)]
    pub dst_name: String,
    #[diesel(sql_type = Int4)]
    pub price: i32,
}

use tracing::info;

impl Handler<GetFlights> for DatabaseExecutor {
    type Result = Result<Vec<models::FlightResponse>>;

    fn handle(&mut self, msg: GetFlights, _: &mut Self::Context) -> Self::Result {
        info!("msg: {:?}", "TEST");

        let mut conn = self.0.get()?;

        let (limit_str, offset_str) = msg.page_and_size.map_or((String::new(), String::new()), |page_ans_size| {
            let (page, size) = page_ans_size;
            (format!("LIMIT {}", size), format!("OFFSET {}", (page - 1) * size))
        });

        let flight_number_str = msg
            .flight_number
            .map_or(String::new(), |flight_number| format!("WHERE flight_number = '{}'", flight_number));

        let query_string = format!(
            "\
            SELECT flight.id as id, flight_number, datetime, \
            src_a.country as src_country, src_a.city as src_city, src_a.name as src_name, \
            dst_a.country as dst_country, dst_a.city as dst_city, dst_a.name as dst_name, price \
            FROM flight \
            JOIN (SELECT id FROM flight ORDER BY id {limit_str} {offset_str}) as tmp ON tmp.id = flight.id \
            JOIN airport as src_a ON flight.from_airport_id = src_a.id \
            JOIN airport as dst_a ON flight.to_airport_id = dst_a.id \
            {flight_number_str}"
        );

        let query = sql_query(query_string);

        let flight_list = query.load::<QueryFlightResponse>(&mut conn)?;
        
        info!("flight_list: {:?}", flight_list);

        let result = flight_list
            .into_iter()
            .map(|f| models::FlightResponse {
                date: Some(f.datetime.to_string()),
                flight_number: Some(f.flight_number),
                from_airport: Some(format!("{} {}", f.src_city, f.src_name)),
                to_airport: Some(format!("{} {}", f.dst_city, f.dst_name)),
                price: Some(f.price),
            })
            .collect::<Vec<_>>();

        Ok(result)
    }
}

pub struct GetFlight {
    pub id: i32,
}

impl Message for GetFlight {
    type Result = Result<models::FlightResponse>;
}

impl Handler<GetFlight> for DatabaseExecutor {
    type Result = Result<models::FlightResponse>;

    fn handle(&mut self, msg: GetFlight, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;

        let query = sql_query(
            "\
            SELECT flight.id as id, flight_number, datetime, \
               src_a.country as src_country, src_a.city as src_city, dst_a.country as dst_country, dst_a.city as dst_city, price \
            FROM flight \
            JOIN airport as src_a ON flight.from_airport_id == src_a.id \
            JOIN airport as dst_a ON flight.to_airport_id == dst_a.id \
            WHERE flight.id == ?"
                .to_string(),
        );

        let f = query.bind::<Int4, _>(msg.id).get_result::<QueryFlightResponse>(&mut conn)?;

        let result = models::FlightResponse {
            date: Some(f.datetime.to_string()),
            flight_number: Some(f.flight_number),
            from_airport: Some(format!("{} {}", f.dst_country, f.dst_city)),
            to_airport: Some(format!("{} {}", f.dst_country, f.dst_city)),
            price: Some(f.price),
        };

        Ok(result)
    }
}
