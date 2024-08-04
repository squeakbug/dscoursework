use std::boxed::Box;
use std::sync::Arc;

use crate::config::Config;

use crate::app::service::privilege_service::PrivilegeService;
use crate::app::repository::statistics_repository::StatisticsRepository;

/// Represents the state carried by the web server actors.
pub struct AppState {
    pub privilege_service: Box<dyn PrivilegeService + Sync + Send>,
    pub statistics_repository: Arc<StatisticsRepository>,
    pub config: Config,
}
