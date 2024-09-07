use std::boxed::Box;
use std::vec;

use async_trait::async_trait;
use failsafe;
use failsafe::backoff::Exponential;
use failsafe::failure_policy::ConsecutiveFailures;
use failsafe::futures::CircuitBreaker;
use failsafe::StateMachine;
use futures::future::join;
use futures::{future, TryFutureExt};
use reqwest::Client;

use crate::models;
use crate::service::gateway_service::GatewayService;
use crate::service::service_error::{Result, ServiceError};

#[derive(Clone)]
pub struct GatewayServiceImpl {
    pub flight_base_path: String,
    pub ticket_base_path: String,
    pub bonus_base_path: String,

    // TODO: Убрать этот ужас: заменить на BoxContainerOf<CircuitBreaker>
    pub circuit_breaker: StateMachine<ConsecutiveFailures<Exponential>, ()>,

    pub client: Client,
}

impl GatewayServiceImpl {
    async fn get_privilege(&self, token: String, username: String) -> Result<models::PrivilegeResponse> {
        use bonus_service_api::apis as bapis;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let privileges = self
            .circuit_breaker
            .call(bapis::bonus_restapi_operations_api::list_privileges(
                token.clone(),
                &bonus_conf,
                Some(&username),
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    bapis::Error::Serde(_) => ServiceError::BadClientData,
                    bapis::Error::Io(_) => ServiceError::BadClientData,
                    bapis::Error::Reqwest(_) => ServiceError::BonusServiceUnavailable,
                    bapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::BonusServiceUnavailable,
            })?;

        // У одного пользователя может быть только 1 аккаунт
        let user_bonus = if let Some(bonus) = privileges.first() {
            Ok(bonus.to_owned())
        } else {
            Err(ServiceError::NotFoundError)
        }?;

        let result = models::PrivilegeResponse {
            id: user_bonus.id,
            balance: user_bonus.balance,
            username: user_bonus.username,
            status: user_bonus.status,
        };

        Ok(result)
    }

    async fn get_history(
        &self,
        token: String,
        username: Option<String>,
        ticket_uid: Option<uuid::Uuid>,
    ) -> Result<Vec<models::BalanceHistory>> {
        use bonus_service_api::apis as bapis;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let response = self
            .circuit_breaker
            .call(bapis::bonus_restapi_operations_api::list_bonus_history(
                token.clone(),
                &bonus_conf,
                username,
                ticket_uid.map(|x| x.to_string()),
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    bapis::Error::Serde(_) => ServiceError::BadClientData,
                    bapis::Error::Io(_) => ServiceError::BadClientData,
                    bapis::Error::Reqwest(_) => ServiceError::BonusServiceUnavailable,
                    bapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::BonusServiceUnavailable,
            })?;

        let result = response
            .into_iter()
            .map(|history| models::BalanceHistory {
                balance_diff: history.balance_diff,
                date: history.date,
                operation_type: history.operation_type,
                ticket_uid: history.ticket_uid,
            })
            .collect::<Vec<_>>();

        Ok(result)
    }

    async fn create_bonus(&self, token: String, input: models::PrivilegeRequest) -> Result<models::PrivilegeFullInfo> {
        use bonus_service_api::apis as bapis;
        use bonus_service_api::models as bmodels;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let privilege_req = bmodels::PrivilegeRequest {
            ticket_uid: input.ticket_uid,
            paid_from_balance: input.paid_from_balance,
            price: input.price,
            username: input.username,
        };

        let privilege_resp = self
            .circuit_breaker
            .call(bapis::bonus_restapi_operations_api::create_bonus(
                token.clone(),
                &bonus_conf,
                privilege_req,
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    bapis::Error::Serde(_) => ServiceError::BadClientData,
                    bapis::Error::Io(_) => ServiceError::BadClientData,
                    bapis::Error::Reqwest(_) => ServiceError::BonusServiceUnavailable,
                    bapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::BonusServiceUnavailable,
            })?;

        let result = models::PrivilegeFullInfo {
            id: privilege_resp.id,
            short_info: models::PrivilegeShortInfo {
                balance: privilege_resp.short_info.balance,
                status: privilege_resp.short_info.status,
            },
            paid_by_bonuses: privilege_resp.paid_by_bonuses,
            paid_by_money: privilege_resp.paid_by_money,
            date: privilege_resp.date,
        };

        Ok(result)
    }

    async fn delete_bonus(&self, token: String, username: String, ticket_uid: uuid::Uuid) -> Result<()> {
        use bonus_service_api::apis as bapis;

        let bonus_conf = bapis::configuration::Configuration {
            base_path: self.bonus_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        self.circuit_breaker
            .call(bapis::bonus_restapi_operations_api::delete_bonus(
                token.clone(),
                &bonus_conf,
                username,
                ticket_uid.to_string(),
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    bapis::Error::Serde(_) => ServiceError::BadClientData,
                    bapis::Error::Io(_) => ServiceError::BadClientData,
                    bapis::Error::Reqwest(_) => ServiceError::BonusServiceUnavailable,
                    bapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::BonusServiceUnavailable,
            })
    }

    async fn get_tickets(&self, token: String, username: String) -> Result<Vec<models::TicketInfo>> {
        use ticket_service_api::apis as tapis;

        let ticket_conf = tapis::configuration::Configuration {
            base_path: self.ticket_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let tickets = self
            .circuit_breaker
            .call(tapis::ticket_restapi_operations_api::list_tickets(
                token.clone(),
                &ticket_conf,
                Some(&username),
                None,
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    tapis::Error::Serde(_) => ServiceError::BadClientData,
                    tapis::Error::Io(_) => ServiceError::BadClientData,
                    tapis::Error::Reqwest(_) => ServiceError::TicketServiceUnavailable,
                    tapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::TicketServiceUnavailable,
            })?
            .into_iter()
            .map(|ticket| models::TicketInfo {
                id: ticket.id,
                ticket_uid: ticket.ticket_uid,
                flight_number: ticket.flight_number,
                price: ticket.price,
                status: ticket.status,
                username: ticket.username,
            })
            .collect::<Vec<_>>();

        Ok(tickets)
    }

    async fn get_ticket(&self, token: String, ticket_uid: uuid::Uuid) -> Result<models::TicketInfo> {
        use ticket_service_api::apis as tapis;

        let ticket_conf = tapis::configuration::Configuration {
            base_path: self.ticket_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        self.circuit_breaker
            .call(tapis::ticket_restapi_operations_api::get_ticket(
                token.clone(),
                &ticket_conf,
                &ticket_uid.to_string(),
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    tapis::Error::Serde(_) => ServiceError::BadClientData,
                    tapis::Error::Io(_) => ServiceError::BadClientData,
                    tapis::Error::Reqwest(_) => ServiceError::TicketServiceUnavailable,
                    tapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::TicketServiceUnavailable,
            })
            .map(|ticket| models::TicketInfo {
                id: ticket.id,
                ticket_uid: ticket.ticket_uid,
                flight_number: ticket.flight_number,
                price: ticket.price,
                status: ticket.status,
                username: ticket.username,
            })
    }

    async fn create_ticket(&self, token: String, input: models::TicketRequest) -> Result<models::TicketInfo> {
        use ticket_service_api::apis as tapis;
        use ticket_service_api::models as tmodels;

        let ticket_conf = tapis::configuration::Configuration {
            base_path: self.ticket_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let ticket_req = tmodels::TicketRequest {
            flight_number: input.flight_number,
            price: input.price,
            username: input.username.clone(),
        };
        self.circuit_breaker
            .call(tapis::ticket_restapi_operations_api::create_ticket(
                token.clone(),
                &ticket_conf,
                ticket_req,
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    tapis::Error::Serde(_) => ServiceError::BadClientData,
                    tapis::Error::Io(_) => ServiceError::BadClientData,
                    tapis::Error::Reqwest(_) => ServiceError::TicketServiceUnavailable,
                    tapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::TicketServiceUnavailable,
            })
            .map(|ticket| models::TicketInfo {
                id: ticket.id,
                flight_number: ticket.flight_number,
                price: ticket.price,
                status: ticket.status,
                ticket_uid: ticket.ticket_uid,
                username: input.username,
            })
    }

    async fn delete_ticket(&self, token: String, ticket_uid: uuid::Uuid) -> Result<()> {
        use ticket_service_api::apis as tapis;

        let ticket_conf = tapis::configuration::Configuration {
            base_path: self.ticket_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        self.circuit_breaker
            .call(tapis::ticket_restapi_operations_api::edit_ticket1(
                token.clone(),
                &ticket_conf,
                &ticket_uid.to_string(),
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    tapis::Error::Serde(_) => ServiceError::BadClientData,
                    tapis::Error::Io(_) => ServiceError::BadClientData,
                    tapis::Error::Reqwest(_) => ServiceError::TicketServiceUnavailable,
                    tapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::TicketServiceUnavailable,
            })
    }

    async fn get_flight(&self, token: String, flight_number: String) -> Result<models::FlightResponse> {
        use flight_service_api::apis as fapis;

        let flight_conf = fapis::configuration::Configuration {
            base_path: self.flight_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };
        let flight_resp = self
            .circuit_breaker
            .call(fapis::flight_restapi_operations_api::list_flights(
                token.clone(),
                &flight_conf,
                None,
                None,
                Some(&flight_number),
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    fapis::Error::Serde(_) => ServiceError::BadClientData,
                    fapis::Error::Io(_) => ServiceError::BadClientData,
                    fapis::Error::Reqwest(_) => ServiceError::FlightServiceUnavailable,
                    fapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::FlightServiceUnavailable,
            })?
            .items;

        let flight_by_number = match flight_resp {
            None => Err(ServiceError::NotFoundError),
            _ => Ok(flight_resp.unwrap().first().unwrap().to_owned()),
        }?;

        let result = models::FlightResponse {
            flight_number: flight_by_number.flight_number,
            from_airport: flight_by_number.from_airport,
            to_airport: flight_by_number.to_airport,
            date: flight_by_number.date,
            price: flight_by_number.price,
        };

        Ok(result)
    }

    async fn get_flights(&self, token: String, page: Option<i32>, size: Option<i32>) -> Result<models::PaginationResponse> {
        use flight_service_api::apis as fapis;

        let flight_conf = fapis::configuration::Configuration {
            base_path: self.flight_base_path.clone(),
            client: self.client.clone(),
            ..Default::default()
        };

        self.circuit_breaker
            .call(fapis::flight_restapi_operations_api::list_flights(
                token.clone(),
                &flight_conf,
                page,
                size,
                None,
            ))
            .await
            .map_err(|err| match err {
                failsafe::Error::Inner(err) => match err {
                    fapis::Error::Serde(_) => ServiceError::BadClientData,
                    fapis::Error::Io(_) => ServiceError::BadClientData,
                    fapis::Error::Reqwest(_) => ServiceError::FlightServiceUnavailable,
                    fapis::Error::ResponseError(_) => ServiceError::BadClientData,
                },
                failsafe::Error::Rejected => ServiceError::FlightServiceUnavailable,
            })
            .map(|flights| models::PaginationResponse {
                page: flights.page,
                page_size: flights.page_size,
                total_elements: flights.total_elements,
                items: Some(
                    flights
                        .items
                        .unwrap()
                        .into_iter()
                        .map(|flight| models::FlightResponse {
                            date: flight.date,
                            flight_number: flight.flight_number,
                            from_airport: flight.from_airport,
                            to_airport: flight.to_airport,
                            price: flight.price,
                        })
                        .collect::<Vec<_>>(),
                ),
            })
    }

    async fn get_full_tickets(&self, token: String, username: String) -> Result<Vec<models::TicketResponse>> {
        let my_token = &token;
        let futures = self.get_tickets(my_token.clone(), username).await?.into_iter().map(|ticket| async move {
            let flight = self.get_flight(my_token.clone(), ticket.flight_number).await.unwrap();
            models::TicketResponse {
                date: flight.date,
                flight_number: flight.flight_number,
                from_airport: flight.from_airport,
                to_airport: flight.to_airport,
                price: flight.price,
                status: Some(ticket.status),
                ticket_uid: Some(ticket.ticket_uid),
            }
        });

        let mut result = vec![];
        for f in futures {
            result.push(f.await);
        }

        Ok(result)
    }
}

#[async_trait]
impl GatewayService for GatewayServiceImpl {
    async fn get_flights(&self, token: String, page: Option<i32>, size: Option<i32>) -> Result<models::PaginationResponse> {
        self.get_flights(token.clone(), page, size).await
    }

    async fn get_privilege_with_history(&self, token: String, username: String) -> Result<models::PrivilegeInfoResponse> {
        let user_privilege = self.get_privilege(token.clone(), username.clone()).await?;
        let user_history = self.get_history(token.clone(), Some(username), None).await?;

        let balance = Some(user_privilege.balance);
        let status = Some(user_privilege.status);
        let history = Some(user_history);

        let result = models::PrivilegeInfoResponse {
            balance,
            history,
            status,
        };

        Ok(result)
    }

    async fn get_user_info(&self, token: String, username: String) -> Result<models::UserInfoResponse> {
        let fut_tickets = self.get_full_tickets(token.clone(), username.clone());
        let fut_privilege = self.get_privilege(token.clone(), username.clone());
        let fut_joined = join(fut_tickets, fut_privilege);
        let (tickets, privilege) = fut_joined.await;

        let privilege = privilege.as_ref().map_or(None, |_privel| {
            Some(models::PrivilegeShortInfo {
                balance: Some(_privel.balance),
                status: Some(_privel.status.clone()),
            })
        });
        let tickets = tickets.ok();

        let result = models::UserInfoResponse { privilege, tickets };

        Ok(result)
    }

    async fn get_user_tickets(&self, token: String, username: String) -> Result<Vec<models::TicketResponse>> {
        self.get_full_tickets(token, username).await
    }

    async fn get_ticket_by_uid(&self, token: String, _: String, ticket_uid: uuid::Uuid) -> Result<models::TicketResponse> {
        self.get_ticket(token.clone(), ticket_uid)
            .and_then(|ticket_info| {
                let flight_number = ticket_info.flight_number;
                self.get_flight(token.clone(), flight_number.clone()).and_then(|flight_response| {
                    future::ok(models::TicketResponse {
                        date: flight_response.date,
                        ticket_uid: Some(ticket_uid),
                        flight_number: Some(flight_number),
                        from_airport: flight_response.from_airport,
                        to_airport: flight_response.to_airport,
                        price: flight_response.price,
                        status: Some(ticket_info.status),
                    })
                })
            })
            .await
    }

    async fn buy_ticket(
        &self,
        token: String,
        username: String,
        ticket_req: models::TicketPurchaseRequest,
    ) -> Result<models::TicketPurchaseResponse> {
        let flight_number = if let Some(flight_number) = ticket_req.flight_number {
            Ok(flight_number)
        } else {
            Err(ServiceError::BadClientData)
        }?;

        self.get_flight(token.clone(), flight_number.clone())
            .and_then(|flight| {
                let ticket_service_req = models::TicketRequest {
                    flight_number: flight_number.clone(),
                    price: ticket_req.price.unwrap(),
                    username: username.clone(),
                };
                self.create_ticket(token.clone(), ticket_service_req).and_then(|new_ticket| {
                    let privilege_req = models::PrivilegeRequest {
                        paid_from_balance: ticket_req.paid_from_balance.unwrap(),
                        price: ticket_req.price.unwrap(),
                        ticket_uid: new_ticket.ticket_uid,
                        username: username.clone(),
                    };
                    self.create_bonus(token.clone(), privilege_req)
                        .and_then(move |user_privilege| {
                            future::ok(models::TicketPurchaseResponse {
                                date: Some(user_privilege.date),
                                flight_number: Some(flight_number),
                                from_airport: flight.from_airport,
                                to_airport: flight.to_airport,
                                paid_by_bonuses: Some(user_privilege.paid_by_bonuses),
                                paid_by_money: Some(user_privilege.paid_by_money),
                                ticket_uid: Some(new_ticket.ticket_uid),
                                status: Some(new_ticket.status),
                                price: Some(new_ticket.price),
                                privilege: Some(Box::new(user_privilege.short_info)),
                            })
                        })
                        .or_else(move |err| {
                            self.return_ticket(token.clone(), username, new_ticket.ticket_uid)
                                .and_then(move |_| future::err(err))
                        })
                })
            })
            .await
    }

    async fn return_ticket(&self, token: String, username: String, ticket_uid: uuid::Uuid) -> Result<()> {
        let _ = self
            .get_history(token.clone(), Some(username.clone()), Some(ticket_uid))
            .and_then(|_| self.delete_bonus(token.clone(), username.clone(), ticket_uid))
            .await?;
        let ticket_res = self.delete_ticket(token.clone(), ticket_uid).await;
        if ticket_res.is_err() {
            let req = models::TicketRequest {
                flight_number: String::from(""),
                username: username.clone(),
                price: 0,
            };
            let _ = self.create_ticket(token.clone(), req).await;
        }

        Ok(())
    }
}
