pub mod health;
pub mod rfc;
pub mod translation;

use crate::config::Config;
use crate::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub config: Config,
}
