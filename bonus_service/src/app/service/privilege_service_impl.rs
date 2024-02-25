use std::boxed::Box;

use async_trait::async_trait;
use futures::future::join_all;

use crate::app::domain::privilege_history::NewPrivilegeHistory;
use crate::app::models::{self, PrivilegeShortInfo};
use crate::app::repository::privilege_repository::PrivilegeRepository;
use crate::app::service::privilege_service::PrivilegeService;
use crate::app::service::service_error::{Result, ServiceError};

#[derive(Clone)]
pub struct PrivilegeServiceImpl {
    pub privilege_repository: Box<dyn PrivilegeRepository + Send + Sync>,
}

#[async_trait]
impl PrivilegeService for PrivilegeServiceImpl {
    async fn list_privileges(&self, username: Option<String>) -> Result<Vec<models::PrivilegeResponse>> {
        self.privilege_repository.get_privileges(username).await
    }

    async fn create_bonus(&self, request: &models::PrivilegeRequest) -> Result<models::PrivilegeFullInfo> {
        let username = request.username.clone();
        let mut current_privilege = match self.list_privileges(Some(username.clone())).await {
            Err(ServiceError::NotFoundError) => {
                let req = models::PrivilegeCreateRequest {
                    balance: 0,
                    username: username.clone(),
                    status: String::from("BRONZE"),
                };
                self.privilege_repository.create_privilege(&req).await
            }
            Err(err) => Err(err),
            Ok(privileges) => Ok(privileges.first().unwrap().to_owned()),
        }?;

        let balance_diff;
        let paid_by_money;
        let paid_by_bonuses;
        let operation;
        let paid_from_balance: bool = request.paid_from_balance;
        if paid_from_balance {
            let cur_balance = current_privilege.balance;
            let ticket_price = request.price;
            if cur_balance < ticket_price {
                balance_diff = cur_balance;
                paid_by_money = ticket_price - cur_balance;
                paid_by_bonuses = balance_diff;
            } else {
                balance_diff = cur_balance - ticket_price;
                paid_by_money = 0;
                paid_by_bonuses = balance_diff;
            }
            operation = String::from("DEBIT_THE_ACCOUNT");
        } else {
            balance_diff = request.price / 10;
            paid_by_money = request.price;
            paid_by_bonuses = 0;
            operation = String::from("FILL_IN_BALANCE");
        }

        let new_balance = current_privilege.balance + balance_diff;

        current_privilege.balance = new_balance;

        let upd_privilege = self
            .privilege_repository
            .edit_privilege(current_privilege.id, &current_privilege)
            .await?;

        let new_privilege_history = NewPrivilegeHistory {
            balance_diff,
            datetime: chrono::Local::now().naive_local(),
            operation_type: operation,
            privilege_id: Some(upd_privilege.id),
            ticket_uid: request.ticket_uid,
        };

        self.privilege_repository
            .create_privilege_history(new_privilege_history)
            .await?;

        let result = models::PrivilegeFullInfo {
            short_info: PrivilegeShortInfo {
                balance: Some(upd_privilege.balance),
                status: Some(String::from("PAID")),
            },
            id: upd_privilege.id,
            paid_by_bonuses,
            paid_by_money,
            date: chrono::Local::now().to_string(),
        };

        Ok(result)
    }

    async fn delete_bonus(&self, username: String, ticket_uid: uuid::Uuid) -> Result<()> {
        let ticket_history = self.get_privilege_history(None, Some(ticket_uid)).await?;

        let privileges = self.list_privileges(Some(username)).await?;
        let mut current_privilege = privileges.first().unwrap().to_owned();

        let last_history_entry = match ticket_history.last() {
            None => return Err(ServiceError::NotFoundError),
            Some(entry) => entry,
        };

        let last_op_type = last_history_entry.clone().operation_type.unwrap();
        let balance_diff;
        let operation_to_return;
        if last_op_type == *String::from("FILL_IN_BALANCE") {
            // Если бонусы были начислены, то списываем их
            let last_balance_diff = last_history_entry.balance_diff.unwrap();
            balance_diff = if current_privilege.balance > last_balance_diff {
                -last_balance_diff
            } else {
                -current_privilege.balance
            };
            operation_to_return = String::from("DEBIT_THE_ACCOUNT");
        } else if last_op_type == *String::from("DEBIT_THE_ACCOUNT") {
            // Если бонусы были списаны, то начисляем их
            balance_diff = last_history_entry.balance_diff.unwrap();
            operation_to_return = String::from("FILL_IN_BALANCE");
        } else if last_op_type == *String::from("FILLED_BY_MONEY") {
            // Если покупка совершена только за деньги, то ни списываем, ни начисляем
            balance_diff = 0;
            operation_to_return = String::from("FILL_IN_BALANCE");
        } else {
            return Err(ServiceError::InternalError);
        }

        let new_balance = current_privilege.balance + balance_diff;
        current_privilege.balance = new_balance;

        let upd_privilege = self
            .privilege_repository
            .edit_privilege(current_privilege.id, &current_privilege)
            .await?;

        let new_privilege_history = NewPrivilegeHistory {
            balance_diff,
            datetime: chrono::Local::now().naive_local(),
            operation_type: operation_to_return,
            privilege_id: Some(upd_privilege.id),
            ticket_uid,
        };

        self.privilege_repository
            .create_privilege_history(new_privilege_history)
            .await?;

        Ok(())
    }

    async fn get_privilege_history(
        &self,
        username: Option<String>,
        ticket_uid: Option<uuid::Uuid>,
    ) -> Result<Vec<models::BalanceHistory>> {
        let requests = self
            .privilege_repository
            .get_privileges(username)
            .await?
            .into_iter()
            .map(|privilege| {
                self.privilege_repository
                    .get_privilege_history(Some(privilege.id), ticket_uid)
            });

        let out = join_all(requests)
            .await
            .into_iter()
            .flat_map(|response| response.unwrap_or_default())
            .collect();

        Ok(out)
    }
}
